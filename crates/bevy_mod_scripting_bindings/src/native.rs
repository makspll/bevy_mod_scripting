//! Typed calls and native values: the fast path next to [`ScriptValue`] conversions.
//!
//! - Every function reads its arguments from an [`ArgSource`] and writes its result to a [`ReturnSink`] (see
//!   [`DynamicScriptFunction::call_with`]), both implemented by each language over its own values, without building
//!   [`ScriptValue`]s. Arguments the language can't give in their native form are converted the usual way
//!   ([`FromScript::from_arg`](crate::FromScript::from_arg)'s slow arm). Rust callers use [`ScriptValueArgs`] and
//!   [`ScriptValueReturn`] (see [`DynamicScriptFunction::call`]).
//! - A [`NativeType`] is stored by the language itself, i.e. in a Lua userdata, as `[NativeHeader][value]`, see [`NativeStorage`].
//!   Its fields are read and written through typed accessor functions, see [`crate::script_native`].
//!
//! Everything which isn't typed sees a copy of a native value (a reference to an allocation, see
//! [`crate::ReflectAllocationId::is_native_copy`]), which nothing can change in place (write access to it is refused, `M<T>`
//! errors), so changes are never silently lost.
//!
//! Languages look up a native type's fields and methods when they first need them (a Lua state makes the metatable once, a
//! Rune value keeps its type from when it was made): register native types and their functions before scripts run. Methods
//! registered later are still found through the slow path, on a copy.

use std::{alloc::Layout, any::TypeId, borrow::Cow, ptr::NonNull};

use bevy_ecs::world::World;
use bevy_mod_scripting_world::{BorrowFlag, WorldGuard};
use bevy_platform::collections::HashMap;
use bevy_reflect::Reflect;

use crate::{
    AppScriptFunctionRegistry, DynamicScriptFunction, FunctionScope, GetFunctionInfo, InteropError,
    ReflectReference, ScriptFunction, ScriptValue, function::script_function::documented_function,
};

/// The arguments of a typed call, implemented by each language over its own values.
///
/// # Safety
/// [`ArgSource::native`] must only return the base of a [`NativeStorage`] which was initialised with [`NativeStorage::init`],
/// and which stays alive and in place until the call returns.
pub unsafe trait ArgSource {
    /// The number of arguments.
    fn len(&self) -> usize;
    /// Whether there are no arguments.
    fn is_empty(&self) -> bool {
        self.len() == 0
    }
    /// The argument as a number, if it is one.
    fn number(&self, i: usize) -> Option<f64>;
    /// The argument as an integer, if it is one.
    fn integer(&self, i: usize) -> Option<i64>;
    /// The argument as a boolean, if it is one.
    fn boolean(&self, i: usize) -> Option<bool>;
    /// The base of the [`NativeStorage`] of the argument, if it is a native value.
    fn native(&self, i: usize) -> Option<*mut u8>;
    /// The argument converted to a [`ScriptValue`], the slow path.
    fn script_value(&self, i: usize) -> Result<ScriptValue, InteropError>;
}

/// Where the result of a typed call goes, implemented by each language.
///
/// # Safety
/// [`ReturnSink::native`] returns true only if it called `write` exactly once, with a pointer to writable memory with the
/// size and alignment of the given vtable's layout, which it then keeps as the native value (dropping it with that vtable).
/// It returns false without calling `write` otherwise.
pub unsafe trait ReturnSink {
    /// Returns nothing.
    fn unit(&mut self);
    /// Returns a number.
    fn number(&mut self, n: f64);
    /// Returns an integer.
    fn integer(&mut self, i: i64);
    /// Returns a boolean.
    fn boolean(&mut self, b: bool);
    /// Returns a new native value of type `native`: makes [`NativeStorage`] for `vtable` (the type's own, or its
    /// [`NativeVTable::place_vtable`]) and initialises it with `write`.
    ///
    /// Returns false, without calling `write`, if the language has no native form for the type.
    ///
    /// # Safety
    /// `vtable` is `native.vtable` or its place vtable, `write` writes a valid value of the vtable to the pointer it's given.
    unsafe fn native(
        &mut self,
        native: &std::sync::Arc<NativeType>,
        vtable: &'static NativeVTable,
        write: &mut dyn FnMut(*mut u8),
    ) -> bool;
    /// Returns any value, the slow path.
    fn script_value(&mut self, value: ScriptValue) -> Result<(), InteropError>;
}

/// What every function runs, see [`DynamicScriptFunction::call_with`]. Its result's native type is cached in it, see
/// [`NativeReturnType`].
pub type NativeThunk = dyn Fn(crate::FunctionCallContext, &dyn ArgSource, &mut dyn ReturnSink) -> Result<(), InteropError>
    + Send
    + Sync;

/// [`NativeThunk`] of a [`crate::DynamicScriptFunctionMut`].
pub type NativeThunkMut = dyn FnMut(
        crate::FunctionCallContext,
        &dyn ArgSource,
        &mut dyn ReturnSink,
    ) -> Result<(), InteropError>
    + Send
    + Sync;

/// Arguments given as [`ScriptValue`]s, i.e. by Rust callers, see [`DynamicScriptFunction::call`].
///
/// Each argument is converted to a [`ScriptValue`] at most once ([`FromScript::from_arg`](crate::FromScript::from_arg)), so
/// [`ArgSource::script_value`] moves it out instead of cloning it.
pub struct ScriptValueArgs(Vec<std::cell::Cell<ScriptValue>>);

impl ScriptValueArgs {
    /// The arguments, in order.
    pub fn new(args: impl IntoIterator<Item = ScriptValue>) -> Self {
        Self(args.into_iter().map(std::cell::Cell::new).collect())
    }

    /// Runs `f` with argument `i`, if there is one.
    fn with<O>(&self, i: usize, f: impl FnOnce(&ScriptValue) -> Option<O>) -> Option<O> {
        let cell = self.0.get(i)?;
        let value = cell.take();
        let out = f(&value);
        cell.set(value);
        out
    }
}

/// The argument as a number, see [`ArgSource::number`].
fn number_of(value: &ScriptValue) -> Option<f64> {
    match value {
        ScriptValue::Float(f) => Some(*f),
        ScriptValue::Integer(i) => Some(*i as f64),
        _ => None,
    }
}

// Safety: never gives native values
unsafe impl ArgSource for ScriptValueArgs {
    fn len(&self) -> usize {
        self.0.len()
    }
    fn number(&self, i: usize) -> Option<f64> {
        self.with(i, number_of)
    }
    fn integer(&self, i: usize) -> Option<i64> {
        self.with(i, |v| match v {
            ScriptValue::Integer(i) => Some(*i),
            _ => None,
        })
    }
    fn boolean(&self, i: usize) -> Option<bool> {
        self.with(i, |v| match v {
            ScriptValue::Bool(b) => Some(*b),
            _ => None,
        })
    }
    fn native(&self, _: usize) -> Option<*mut u8> {
        None
    }
    fn script_value(&self, i: usize) -> Result<ScriptValue, InteropError> {
        Ok(self.0.get(i).map(std::cell::Cell::take).unwrap_or_default())
    }
}

/// The arguments of a curried function, see [`DynamicScriptFunction::curry`]: `first`, then `rest`.
pub(crate) struct CurriedArgs<'a> {
    first: &'a ScriptValue,
    rest: &'a dyn ArgSource,
}

impl<'a> CurriedArgs<'a> {
    pub(crate) fn new(first: &'a ScriptValue, rest: &'a dyn ArgSource) -> Self {
        Self { first, rest }
    }
}

// Safety: native values come from `rest`, an `ArgSource`
unsafe impl ArgSource for CurriedArgs<'_> {
    fn len(&self) -> usize {
        self.rest.len() + 1
    }
    fn number(&self, i: usize) -> Option<f64> {
        match i {
            0 => number_of(self.first),
            i => self.rest.number(i - 1),
        }
    }
    fn integer(&self, i: usize) -> Option<i64> {
        match (i, self.first) {
            (0, ScriptValue::Integer(i)) => Some(*i),
            (0, _) => None,
            (i, _) => self.rest.integer(i - 1),
        }
    }
    fn boolean(&self, i: usize) -> Option<bool> {
        match (i, self.first) {
            (0, ScriptValue::Bool(b)) => Some(*b),
            (0, _) => None,
            (i, _) => self.rest.boolean(i - 1),
        }
    }
    fn native(&self, i: usize) -> Option<*mut u8> {
        self.rest.native(i.checked_sub(1)?)
    }
    fn script_value(&self, i: usize) -> Result<ScriptValue, InteropError> {
        match i {
            0 => Ok(self.first.clone()),
            i => self.rest.script_value(i - 1),
        }
    }
}

/// Takes the result as a [`ScriptValue`], i.e. for Rust callers, see [`DynamicScriptFunction::call`].
#[derive(Default)]
pub struct ScriptValueReturn(ScriptValue);

impl ScriptValueReturn {
    /// The result, [`ScriptValue::Unit`] if there was none.
    pub fn into_value(self) -> ScriptValue {
        self.0
    }
}

// Safety: never makes native values
unsafe impl ReturnSink for ScriptValueReturn {
    fn unit(&mut self) {
        self.0 = ScriptValue::Unit;
    }
    fn number(&mut self, n: f64) {
        self.0 = ScriptValue::Float(n);
    }
    fn integer(&mut self, i: i64) {
        self.0 = ScriptValue::Integer(i);
    }
    fn boolean(&mut self, b: bool) {
        self.0 = ScriptValue::Bool(b);
    }
    unsafe fn native(
        &mut self,
        _: &std::sync::Arc<NativeType>,
        _: &'static NativeVTable,
        _: &mut dyn FnMut(*mut u8),
    ) -> bool {
        false
    }
    fn script_value(&mut self, value: ScriptValue) -> Result<(), InteropError> {
        self.0 = value;
        Ok(())
    }
}

/// What a language needs to store, copy and drop values of a [`NativeType`] it knows nothing about.
///
/// Each type has two: one for values stored by the language, one for places in the world holding one ([`NativePlace`], i.e.
/// a component), see [`NativeVTable::place_vtable`].
pub struct NativeVTable {
    /// The type of the values
    pub type_id: TypeId,
    /// The name of the type, for errors
    pub type_name: &'static str,
    /// The layout of what's stored: a value, or a [`NativePlace`]
    pub layout: Layout,
    /// Drops a value in place, `None` if the type needs no drop
    pub drop: Option<unsafe fn(*mut u8)>,
    /// Whether this stores a [`NativePlace`] of the type instead of a value
    pub place: bool,
    clone_into: unsafe fn(*const u8, *mut u8),
    to_reference: unsafe fn(*const u8) -> ReflectReference,
    /// The value vtable of a place vtable, and the other way around
    counterpart: std::sync::OnceLock<&'static NativeVTable>,
}

impl NativeVTable {
    /// The vtable of `T`, made once per type and kept for the program's lifetime, so native values can outlive their app.
    fn new<T: Reflect + Clone>() -> &'static Self {
        unsafe fn drop_value<T>(p: *mut u8) {
            // Safety: called with a pointer to an initialised T, which isn't used afterwards
            unsafe { std::ptr::drop_in_place(p as *mut T) }
        }
        unsafe fn clone_into<T: Clone>(src: *const u8, dst: *mut u8) {
            // Safety: called with a pointer to an initialised T, and one to memory for a T
            unsafe { (dst as *mut T).write((*(src as *const T)).clone()) }
        }
        unsafe fn to_reference<T: Reflect + Clone>(src: *const u8) -> ReflectReference {
            // Safety: called with a pointer to an initialised T
            ReflectReference::new_allocated_raw(
                TypeId::of::<T>(),
                crate::ReflectAllocationId::new_native_copy(unsafe {
                    (*(src as *const T)).clone()
                }),
            )
        }
        static VTABLES: std::sync::OnceLock<
            std::sync::Mutex<HashMap<TypeId, &'static NativeVTable>>,
        > = std::sync::OnceLock::new();
        let mut vtables = VTABLES
            .get_or_init(Default::default)
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        vtables.entry(TypeId::of::<T>()).or_insert_with(|| {
            let value: &'static Self = Box::leak(Box::new(Self {
                type_id: TypeId::of::<T>(),
                type_name: std::any::type_name::<T>(),
                layout: Layout::new::<T>(),
                drop: std::mem::needs_drop::<T>().then_some(drop_value::<T> as unsafe fn(*mut u8)),
                place: false,
                clone_into: clone_into::<T>,
                to_reference: to_reference::<T>,
                counterpart: Default::default(),
            }));
            let place: &'static Self = Box::leak(Box::new(Self {
                layout: Layout::new::<NativePlace>(),
                drop: None,
                place: true,
                // places are plain data, see `NativeStorage::to_script_value` for their references
                clone_into: clone_into::<NativePlace>,
                counterpart: Default::default(),
                ..*value
            }));
            let _ = value.counterpart.set(place);
            let _ = place.counterpart.set(value);
            value
        })
    }

    /// The vtable of places holding a value of this type, see [`NativePlace`].
    pub fn place_vtable(&'static self) -> &'static NativeVTable {
        match self.place {
            true => self,
            false => self.counterpart(),
        }
    }

    /// The vtable of values of this type, the one a place vtable is of.
    pub fn value_vtable(&'static self) -> &'static NativeVTable {
        match self.place {
            false => self,
            true => self.counterpart(),
        }
    }

    /// The other vtable of the type, set in [`NativeVTable::new`] before either is handed out.
    #[allow(
        clippy::expect_used,
        reason = "both vtables are linked under the vtables lock before either is shared, a fallback would hand out the wrong layout"
    )]
    fn counterpart(&'static self) -> &'static NativeVTable {
        self.counterpart
            .get()
            .copied()
            .expect("native vtables are linked when they're made")
    }
}

impl std::fmt::Debug for NativeVTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.type_name)
    }
}

/// Where a [`NativePlace`] is: a component of an entity or a resource.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum PlaceBase {
    /// A component of an entity
    Component(bevy_ecs::entity::Entity, bevy_ecs::component::ComponentId),
    /// A resource
    Resource(bevy_ecs::component::ComponentId),
}

impl From<PlaceBase> for crate::ReflectBase {
    fn from(base: PlaceBase) -> Self {
        match base {
            PlaceBase::Component(entity, id) => crate::ReflectBase::Component(entity, id),
            PlaceBase::Resource(id) => crate::ReflectBase::Resource(id),
        }
    }
}

/// A value of a [`NativeType`] in the world, i.e. a component, or a field of one (a proxy, see [`crate::script_native`]),
/// stored by the language in place of a value (see [`NativeVTable::place_vtable`]). Typed functions read and write the
/// world through it, claiming access for the call.
#[derive(Clone, Copy, Debug)]
pub struct NativePlace {
    /// The component or resource
    pub base: PlaceBase,
    /// The type of the component or resource
    pub base_type: TypeId,
    /// The offset of the value in the component or resource
    pub offset: usize,
    /// The indices of the struct fields leading to the value, `path[..depth]`
    path: [u8; NativePlace::MAX_DEPTH],
    depth: u8,
}

impl NativePlace {
    /// How many fields deep a place can be.
    pub const MAX_DEPTH: usize = 8;

    /// The place of a whole component or resource of type `base_type`.
    pub fn new(base: PlaceBase, base_type: TypeId) -> Self {
        Self {
            base,
            base_type,
            offset: 0,
            path: [0; Self::MAX_DEPTH],
            depth: 0,
        }
    }

    /// The place of the field at `offset` in this place's value, the struct field with `index`.
    pub fn field(&self, offset: usize, index: u8) -> Option<Self> {
        let depth = self.depth as usize;
        if depth >= Self::MAX_DEPTH {
            return None;
        }
        let mut field = *self;
        field.offset += offset;
        field.path[depth] = index;
        field.depth += 1;
        Some(field)
    }

    /// The value in the world, after claiming access to its component or resource for the call (`write` for exclusive access).
    ///
    /// # Safety
    /// - The place was made for a value of the type the pointer is used as, its offset is inside the component or resource
    ///   (nothing checks it).
    /// - The pointer is valid until `scope` ends (the claim keeps the value where it is), for reads only unless `write`.
    pub unsafe fn resolve(
        &self,
        scope: &bevy_mod_scripting_world::CallScope<'_>,
        write: bool,
    ) -> Result<*mut u8, InteropError> {
        let base = crate::ReflectBase::from(self.base);
        let claimed = match write {
            true => scope.claim_write_access(&base),
            false => scope.claim_read_access(&base),
        };
        if !claimed {
            return Err(InteropError::cannot_claim_access(
                (&base).into(),
                format!("In a native value of {base:?}"),
            ));
        }
        let cell = scope.world().as_unsafe_world_cell()?;
        // Safety: access is claimed until the call ends, which the pointer doesn't outlive
        let ptr = unsafe {
            match write {
                true => base
                    .clone()
                    .into_ptr_mut(cell)
                    .map(|ptr| ptr.into_inner().as_ptr()),
                false => base
                    .clone()
                    .into_ptr(cell)
                    .map(|ptr| ptr.as_ptr() as *mut u8),
            }
        };
        let ptr = ptr.ok_or_else(|| {
            InteropError::string(format!("the entity or its component is gone: {base:?}"))
        })?;
        Ok(ptr.wrapping_add(self.offset))
    }

    /// A reference to the value, for everything which isn't typed.
    pub fn to_reference(&self, world: &WorldGuard) -> Result<ReflectReference, InteropError> {
        // Safety: the type is the one of the component or resource the place was made for
        let mut reference = unsafe {
            match self.base {
                PlaceBase::Component(entity, id) => {
                    ReflectReference::new_component_ref_by_id(entity, id, self.base_type)
                }
                PlaceBase::Resource(id) => {
                    ReflectReference::new_resource_ref_by_id(id, self.base_type)
                }
            }
        };
        let registry = world.type_registry();
        let registry = registry.read();
        let mut type_id = self.base_type;
        for index in &self.path[..self.depth as usize] {
            let field = match registry.get_type_info(type_id) {
                Some(bevy_reflect::TypeInfo::Struct(info)) => info.field_at(*index as usize),
                _ => None,
            }
            .ok_or_else(|| InteropError::str("the path of a native value isn't a struct field"))?;
            reference.push_path(crate::ReferencePart::StringAccess(Cow::Borrowed(
                field.name(),
            )));
            type_id = field.type_id();
        }
        Ok(reference)
    }
}

/// The start of the storage of every native value, see [`NativeStorage`].
#[repr(C)]
pub struct NativeHeader {
    /// The type of the value
    pub vtable: &'static NativeVTable,
    /// The borrows of the value made by calls, see [`bevy_mod_scripting_world::CallScope::borrow`]
    pub borrow: BorrowFlag,
}

/// Helpers for the memory of a native value: `[NativeHeader][padding][value]`.
///
/// The language allocates [`NativeStorage::size`] bytes, aligned for a [`NativeHeader`], which must never move.
pub struct NativeStorage;

impl NativeStorage {
    /// The alignment the storage must have
    pub const ALIGN: usize = align_of::<NativeHeader>();

    /// The number of bytes the storage of a value with this vtable needs
    pub fn size(vtable: &NativeVTable) -> usize {
        size_of::<NativeHeader>()
            + vtable.layout.align().saturating_sub(Self::ALIGN)
            + vtable.layout.size()
    }

    fn value_ptr(base: *mut u8, vtable: &NativeVTable) -> *mut u8 {
        // the storage has room for the header, the padding and the value, see `size`
        let start = base.wrapping_add(size_of::<NativeHeader>());
        let align = vtable.layout.align();
        let padding = (align - (start as usize & (align - 1))) & (align - 1);
        start.wrapping_add(padding)
    }

    /// Initialises the storage: the value is written by `write`, which is given a pointer to memory for it.
    ///
    /// # Safety
    /// `base` points to [`NativeStorage::size`] writable bytes aligned to [`NativeStorage::ALIGN`], and `write` writes a valid
    /// value of the vtable's type.
    pub unsafe fn init(
        base: *mut u8,
        vtable: &'static NativeVTable,
        write: &mut dyn FnMut(*mut u8),
    ) {
        write(Self::value_ptr(base, vtable));
        // Safety: the caller gives aligned, writable memory with room for the header
        unsafe {
            (base as *mut NativeHeader).write(NativeHeader {
                vtable,
                borrow: BorrowFlag::default(),
            })
        }
    }

    /// The header of initialised storage.
    ///
    /// # Safety
    /// `base` is the base of initialised storage, alive for `'a`.
    pub unsafe fn header<'a>(base: *const u8) -> &'a NativeHeader {
        // Safety: see the function's contract
        unsafe { &*(base as *const NativeHeader) }
    }

    /// The value of initialised storage.
    ///
    /// # Safety
    /// `base` is the base of initialised storage.
    pub unsafe fn value(base: *mut u8) -> *mut u8 {
        // Safety: see the function's contract
        Self::value_ptr(base, unsafe { Self::header(base) }.vtable)
    }

    /// Drops the value of initialised storage, the memory itself is the language's.
    ///
    /// # Safety
    /// `base` is the base of initialised storage, which isn't used afterwards.
    pub unsafe fn drop_value(base: *mut u8) {
        // Safety: see the function's contract
        unsafe {
            if let Some(drop) = Self::header(base).vtable.drop {
                drop(Self::value(base))
            }
        }
    }

    /// The value as a [`ScriptValue`]: a reference to a copy of it, for everything which isn't typed.
    ///
    /// # Safety
    /// `base` is the base of initialised storage.
    pub unsafe fn to_script_value(base: *mut u8) -> Result<ScriptValue, InteropError> {
        // Safety: see the function's contract
        let header = unsafe { Self::header(base) };
        if header.borrow.is_borrowed_mut() {
            return Err(InteropError::string(format!(
                "a {} is borrowed mutably by a call",
                header.vtable.type_name
            )));
        }
        // a place is referenced where it is
        if let Some(place) = unsafe { Self::place(base) } {
            let world = bevy_mod_scripting_world::ThreadWorldContainer::try_get_context()?.world;
            return place.to_reference(&world).map(ScriptValue::Reference);
        }
        // one allocation holding the copy
        Ok(ScriptValue::Reference(unsafe {
            (header.vtable.to_reference)(Self::value(base))
        }))
    }

    /// The place stored instead of a value, if it is one, see [`NativeVTable::place_vtable`].
    ///
    /// # Safety
    /// `base` is the base of initialised storage, alive for `'a`.
    pub unsafe fn place<'a>(base: *mut u8) -> Option<&'a NativePlace> {
        // Safety: see the function's contract, place vtables store a `NativePlace`
        unsafe {
            Self::header(base)
                .vtable
                .place
                .then(|| &*(Self::value(base) as *const NativePlace))
        }
    }

    /// Copies the value `src` of the storage's type (its own value, or the one its place resolved to) into memory for a value
    /// of the type.
    ///
    /// # Safety
    /// `base` is the base of initialised storage, `src` points to a value of its type, `dst` to memory for one.
    pub unsafe fn clone_value(base: *mut u8, src: *const u8, dst: *mut u8) {
        // Safety: see the function's contract
        unsafe { (Self::header(base).vtable.value_vtable().clone_into)(src, dst) }
    }
}

/// Storage for a native value on the Rust heap, for languages which can't hold raw memory themselves.
pub struct NativeBox {
    base: NonNull<u8>,
    layout: Layout,
}

impl NativeBox {
    /// Makes storage for a value with this vtable, initialised by `write`, see [`NativeStorage::init`].
    ///
    /// # Safety
    /// `write` writes a valid value of the vtable's type to the pointer it's given.
    pub unsafe fn new(
        vtable: &'static NativeVTable,
        write: &mut dyn FnMut(*mut u8),
    ) -> Result<Self, InteropError> {
        let layout = Layout::from_size_align(NativeStorage::size(vtable), NativeStorage::ALIGN)
            .map_err(|e| InteropError::string(e.to_string()))?;
        // Safety: the layout isn't zero sized, it has a header
        let base = NonNull::new(unsafe { std::alloc::alloc(layout) })
            .ok_or_else(|| InteropError::str("out of memory"))?;
        // Safety: freshly allocated with the size and alignment storage needs, `write` writes a valid value
        unsafe { NativeStorage::init(base.as_ptr(), vtable, write) };
        Ok(Self { base, layout })
    }

    /// The base of the storage
    pub fn base(&self) -> *mut u8 {
        self.base.as_ptr()
    }
}

impl Drop for NativeBox {
    fn drop(&mut self) {
        // Safety: initialised in `new`, freed here
        unsafe {
            NativeStorage::drop_value(self.base.as_ptr());
            std::alloc::dealloc(self.base.as_ptr(), self.layout);
        }
    }
}

// Safety: native values are `Reflect`, so `Send`, and the box owns its storage, header included. It isn't `Sync`: the
// header's borrow flag isn't atomic, values are only used by the thread of their script.
unsafe impl Send for NativeBox {}

/// A field of a [`NativeType`]: typed accessor functions.
#[derive(Clone)]
pub struct NativeField {
    /// Reads the field, called with the value
    pub get: DynamicScriptFunction,
    /// Writes the field, called with the value and the new value of the field
    pub set: Option<DynamicScriptFunction>,
    /// The type of the field's values, if known
    pub type_id: Option<TypeId>,
}

/// The offset of `field` (an `F`) in the `T` at `value`, if it's inside it, not reached through a `Deref` to elsewhere.
fn field_offset<T, F>(value: *const u8, field: *const u8) -> Option<usize> {
    let offset = (field as usize).checked_sub(value as usize)?;
    (offset.checked_add(size_of::<F>())? <= size_of::<T>()).then_some(offset)
}

/// A type whose values the languages store natively, see the [module docs](self).
///
/// Its methods are the functions registered on the type.
#[derive(Clone)]
pub struct NativeType {
    /// How to store, copy and drop the values
    pub vtable: &'static NativeVTable,
    /// The fields, by name
    pub fields: HashMap<Cow<'static, str>, NativeField>,
    /// The functions registered on the type, kept up to date by the registry so languages can look them up without cloning
    pub methods: HashMap<Cow<'static, str>, DynamicScriptFunction>,
    /// Whether values are stored natively, not only places (see [`NativePlace`]): false for types with fields of native
    /// types, which would be copies a write to i.e. `t.translation.x` is lost on. Kept up to date by the registry.
    pub inline: bool,
}

impl NativeType {
    /// A native type with no fields yet.
    pub fn new<T: Reflect + Clone>() -> Self {
        Self {
            vtable: NativeVTable::new::<T>(),
            fields: Default::default(),
            methods: Default::default(),
            inline: true,
        }
    }

    /// Adds a field with a getter and a setter, both typed.
    pub fn with_field<'env, G, GM, S, SM>(mut self, name: &'static str, get: G, set: S) -> Self
    where
        G: ScriptFunction<'env, GM> + GetFunctionInfo<GM>,
        S: ScriptFunction<'env, SM> + GetFunctionInfo<SM>,
    {
        let scope = FunctionScope::OnType(self.vtable.type_id);
        let field = NativeField {
            get: documented_function(scope, name.into(), get, None::<&'static str>, None),
            set: Some(documented_function(
                scope,
                name.into(),
                set,
                None::<&'static str>,
                None,
            )),
            type_id: None,
        };
        self.fields.insert(name.into(), field);
        self
    }

    /// Adds the struct field `name` of `T` of type `F`, at `at`. On a place (i.e. a component), a field of a native type at an
    /// offset is a place too (a proxy): `t.translation.x = 5` writes the component. See [`crate::script_native`].
    ///
    /// `ret` and `arg` convert the field's values, see [`FieldConv`].
    ///
    /// # Safety
    /// [`FieldAt::Offset`]: every `T` has an `F` at the offset. [`FieldAt::Deref`]: the projection, given a pointer to a
    /// valid `T`, returns a pointer to a valid `F` reachable from it, valid as long as the `T` is.
    pub unsafe fn with_struct_field<T, F>(
        mut self,
        name: &'static str,
        at: FieldAt,
        ret: FieldRet<F>,
        arg: FieldArg<F>,
    ) -> Self
    where
        T: bevy_reflect::FromReflect + bevy_reflect::Typed + 'static,
        F: Reflect + Clone + 'static,
    {
        let index = match T::type_info() {
            bevy_reflect::TypeInfo::Struct(info) => {
                info.index_of(name).and_then(|i| u8::try_from(i).ok())
            }
            _ => None,
        };
        let info = || {
            crate::FunctionInfo::default()
                .with_name(name)
                .with_namespace(FunctionScope::OnType(TypeId::of::<T>()))
        };
        let native_return = NativeReturnType::default();
        let get = DynamicScriptFunction::from_thunk::<T>(move |ctx, args, sink| {
            let world = ctx.world()?;
            world.with_call_scope(|scope| {
                // a field of a native type in a place is a place too
                if let Some(base) = args.native(0)
                    // Safety: `ArgSource` gives initialised storage
                    && unsafe { NativeStorage::header(base) }.vtable.type_id == TypeId::of::<T>()
                    && let Some(place) = unsafe { NativeStorage::place(base) }
                    && let Some(field_native) = native_return.get(&world, TypeId::of::<F>())
                {
                    if let (FieldAt::Offset(offset), Some(index)) = (at, index) {
                        let field_place = place.field(offset, index).ok_or_else(|| {
                            InteropError::str("a native value is nested too deeply")
                        })?;
                        let mut field_place = Some(field_place);
                        // Safety: the place vtable stores a `NativePlace`, of an `F`
                        let written = unsafe {
                            sink.native(
                                &field_native,
                                field_native.vtable.place_vtable(),
                                &mut |slot| {
                                    if let Some(place) = field_place.take() {
                                        (slot as *mut NativePlace).write(place)
                                    }
                                },
                            )
                        };
                        if written {
                            return Ok(());
                        }
                    }
                    // a reference to where the field is, not a copy a write would be lost on
                    let mut reference = place.to_reference(&world)?;
                    reference.push_path(crate::ReferencePart::StringAccess(Cow::Borrowed(name)));
                    return sink.script_value(ScriptValue::Reference(reference));
                }
                let value = <crate::R<T> as crate::FromScript>::from_arg(args, 0, scope)?;
                let value = &*value as *const T as *const u8;
                let field = match at {
                    FieldAt::Offset(offset) => value.wrapping_add(offset),
                    FieldAt::Deref(project) => project(value),
                };
                // Safety: an `F` of the T, see the function's contract
                let field = unsafe { &*(field as *const F) };
                ret(field.clone(), sink, world.clone(), &native_return)
            })
        })
        .with_info(info());
        let set = DynamicScriptFunction::from_thunk::<T>(move |ctx, args, sink| {
            if args.len() < 2 {
                return Err(InteropError::argument_count_mismatch(2, args.len()));
            }
            let world = ctx.world()?;
            world.with_call_scope(|scope| {
                let mut target = <crate::M<T> as crate::FromScript>::from_arg(args, 0, scope)?;
                let value = arg(args, 1, scope)?;
                let target = &mut *target as *mut T as *mut u8;
                let offset = match at {
                    FieldAt::Offset(offset) => Some(offset),
                    // only the offset comes from the projection, which reads through a shared reference: the write goes
                    // through the exclusive borrow, so only to a field inside this value
                    FieldAt::Deref(project) => field_offset::<T, F>(target, project(target)),
                }
                .ok_or_else(|| InteropError::string(format!("the field `{name}` is read only")))?;
                // Safety: an `F` inside the T (see the function's contract, checked for `Deref`), borrowed exclusively
                unsafe { *(target.add(offset) as *mut F) = value };
                sink.unit();
                Ok(())
            })
        })
        .with_info(info());
        self.fields.insert(
            name.into(),
            NativeField {
                get,
                set: Some(set),
                type_id: Some(TypeId::of::<F>()),
            },
        );
        self
    }

    /// Registers the native type, values of it made by typed functions are then stored natively by the languages.
    pub fn register(self, world: &mut World) {
        let mut registry = world
            .get_resource_or_init::<AppScriptFunctionRegistry>()
            .0
            .clone();
        registry.write().register_native_type(self);
    }
}

/// Bumped whenever a native type is registered or changes (i.e. functions are registered on it), so caches of native types
/// look them up again, see [`NativeReturnType`].
pub(crate) static NATIVE_TYPES_GENERATION: std::sync::atomic::AtomicU64 =
    std::sync::atomic::AtomicU64::new(0);

/// The native types of a typed function's results, looked up in the registry on their first return and kept by its thunk,
/// so returning a native value takes no registry lock. A few types are kept, i.e. the components `get_component` returns.
///
/// Kept weakly: the native type holds its functions, which hold their thunks. Entries are only used while no native type
/// changed since (see [`NATIVE_TYPES_GENERATION`]): values made earlier can keep a replaced type alive.
#[derive(Default)]
pub struct NativeReturnType(parking_lot::RwLock<Vec<CachedNativeType>>);

/// An entry of [`NativeReturnType`].
struct CachedNativeType {
    type_id: TypeId,
    generation: u64,
    native: Option<std::sync::Weak<NativeType>>,
}

impl NativeReturnType {
    /// How many types are kept, others are looked up on every return.
    const CAPACITY: usize = 8;

    /// The native type of `type_id`, if it is one.
    pub fn get(&self, world: &WorldGuard, type_id: TypeId) -> Option<std::sync::Arc<NativeType>> {
        use crate::WorldExtensions;
        let generation = NATIVE_TYPES_GENERATION.load(std::sync::atomic::Ordering::Acquire);
        // whether there's room for the type, if it needs looking up
        let cacheable = {
            let cache = self.0.read();
            match cache.iter().find(|c| c.type_id == type_id) {
                Some(cached) if cached.generation == generation => {
                    // not native, the common case: no lookup
                    let native = cached.native.as_ref()?;
                    if let Some(native) = native.upgrade() {
                        return Some(native);
                    }
                    true
                }
                Some(_) => true,
                None => cache.len() < Self::CAPACITY,
            }
        };
        let native = world
            .script_function_registry()
            .read()
            .native_type(type_id)
            .cloned();
        if !cacheable {
            return native;
        }
        let mut cache = self.0.write();
        let entry = CachedNativeType {
            type_id,
            generation,
            native: native.as_ref().map(std::sync::Arc::downgrade),
        };
        if let Some(cached) = cache.iter_mut().find(|c| c.type_id == type_id) {
            *cached = entry;
        } else if cache.len() < Self::CAPACITY {
            cache.push(entry);
        }
        native
    }
}

/// The native type of `type_id`, if it is one.
pub fn native_vtable(world: &WorldGuard, type_id: TypeId) -> Option<&'static NativeVTable> {
    use crate::WorldExtensions;
    world
        .script_function_registry()
        .read()
        .native_type(type_id)
        .map(|t| t.vtable)
}

/// Gives the field of a value, from a pointer to it, see [`FieldAt::Deref`].
pub type FieldProject = fn(*const u8) -> *const u8;

/// Where a field of a native type is, see [`NativeType::with_struct_field`].
#[derive(Clone, Copy)]
pub enum FieldAt {
    /// At this offset in every value (`offset_of!`): written in place, and a place (a proxy) on places.
    Offset(usize),
    /// Reached through `Deref`, i.e. glam's SIMD vectors: read through the projection, written only when it's inside the
    /// value, and a reference on places (its location may depend on the value, so it's never cached).
    Deref(FieldProject),
}

/// Converts the value of a field read by a script, see [`FieldConv`].
pub type FieldRet<F> = for<'w> fn(
    F,
    &mut dyn ReturnSink,
    WorldGuard<'w>,
    &NativeReturnType,
) -> Result<(), InteropError>;

/// Converts the value a script writes to a field, see [`FieldConv`].
pub type FieldArg<F> = for<'s, 'w> fn(
    &dyn ArgSource,
    usize,
    &'s bevy_mod_scripting_world::CallScope<'w>,
) -> Result<F, InteropError>;

/// Picks how [`crate::script_native`] converts the values of a field of type `F` (autoref specialization):
/// [`FieldConvScript`] for types converted like arguments and results (numbers, strings), else [`FieldConvValue`] (`V<F>`,
/// i.e. native types).
///
/// ```rust,ignore
/// use bevy_mod_scripting_bindings::native::{FieldConv, FieldConvScript as _, FieldConvValue as _};
/// let ret = (&FieldConv::<f32>::new()).ret();
/// ```
pub struct FieldConv<F>(std::marker::PhantomData<F>);

impl<F> FieldConv<F> {
    /// The converter of `F`
    pub fn new() -> Self {
        Self(std::marker::PhantomData)
    }
}

impl<F> Default for FieldConv<F> {
    fn default() -> Self {
        Self::new()
    }
}

/// Fields converted like arguments and results, see [`FieldConv`].
pub trait FieldConvScript<F> {
    /// The conversion of a read value
    fn ret(&self) -> FieldRet<F>;
    /// The conversion of a written value
    fn arg(&self) -> FieldArg<F>;
}

impl<F> FieldConvScript<F> for FieldConv<F>
where
    F: crate::IntoScript + for<'s> crate::FromScript<This<'s> = F>,
{
    fn ret(&self) -> FieldRet<F> {
        |value, sink, world, native| value.into_ret(sink, world, native)
    }
    fn arg(&self) -> FieldArg<F> {
        |args, index, scope| F::from_arg(args, index, scope)
    }
}

/// Fields converted as `V<F>`, i.e. of native types, see [`FieldConv`].
pub trait FieldConvValue<F> {
    /// The conversion of a read value
    fn ret(&self) -> FieldRet<F>;
    /// The conversion of a written value
    fn arg(&self) -> FieldArg<F>;
}

impl<F> FieldConvValue<F> for &FieldConv<F>
where
    crate::V<F>: crate::IntoScript + for<'s> crate::FromScript<This<'s> = crate::V<F>>,
{
    fn ret(&self) -> FieldRet<F> {
        |value, sink, world, native| {
            crate::IntoScript::into_ret(crate::V(value), sink, world, native)
        }
    }
    fn arg(&self) -> FieldArg<F> {
        |args, index, scope| {
            <crate::V<F> as crate::FromScript>::from_arg(args, index, scope).map(|v| v.0)
        }
    }
}

/// Declares a [`NativeType`] with typed accessors for the given struct fields, and registers it.
///
/// ```rust,ignore
/// script_native!(world, bevy_math::Vec3 { x: f32, y: f32, z: f32 });
/// script_native!(world, Transform { translation: Vec3, rotation: Quat, scale: Vec3 });
/// // fields reached through `Deref`, i.e. glam's SIMD vectors: read and written, but never places (proxies)
/// script_native!(world, deref bevy_math::Vec4 { x: f32, y: f32, z: f32, w: f32 });
/// ```
///
/// Values of types with fields of native types aren't stored natively (writes to `t.translation.x` would be lost on a copy),
/// but components and resources of every native type are: `world.get_component(e, Transform)` returns a native handle
/// ([`NativePlace`]) whose `t.translation` is a place too, read and written through.
#[macro_export]
macro_rules! script_native {
    ($world:expr, deref $ty:ty { $($field:ident : $field_ty:ty),* $(,)? }) => {{
        #[allow(unused_imports)]
        use $crate::native::{FieldConvScript as _, FieldConvValue as _};
        let native = $crate::native::NativeType::new::<$ty>();
        $(
            // Safety: the projection gives the field `$field` of the value, a `$field_ty` (the compiler checks both),
            // through `Deref`: borrowed from the value, so valid as long as it is
            let native = unsafe {
                native.with_struct_field::<$ty, $field_ty>(
                    stringify!($field),
                    $crate::native::FieldAt::Deref(|value| {
                        &(&*(value as *const $ty)).$field as *const $field_ty as *const u8
                    }),
                    (&$crate::native::FieldConv::<$field_ty>::new()).ret(),
                    (&$crate::native::FieldConv::<$field_ty>::new()).arg(),
                )
            };
        )*
        native.register($world);
    }};
    ($world:expr, $ty:ty { $($field:ident : $field_ty:ty),* $(,)? }) => {{
        #[allow(unused_imports)]
        use $crate::native::{FieldConvScript as _, FieldConvValue as _};
        let native = $crate::native::NativeType::new::<$ty>();
        $(
            // a field of the type itself, at the same offset in every value
            const _: fn(&$ty) -> &$field_ty = |value| &value.$field;
            // Safety: every value has a `$field_ty` (checked above) at the offset
            let native = unsafe {
                native.with_struct_field::<$ty, $field_ty>(
                    stringify!($field),
                    $crate::native::FieldAt::Offset(::core::mem::offset_of!($ty, $field)),
                    (&$crate::native::FieldConv::<$field_ty>::new()).ret(),
                    (&$crate::native::FieldConv::<$field_ty>::new()).arg(),
                )
            };
        )*
        native.register($world);
    }};
}

#[cfg(test)]
mod test {
    use bevy_mod_scripting_asset::Language;
    use bevy_mod_scripting_world::WorldAccessGuard;

    use super::*;
    use bevy_mod_scripting_world::ThreadWorldContainer;

    use crate::{CurrentScriptAttachment, FunctionCallContext, M, R, V, WorldExtensions};

    std::thread_local! {
        /// Per thread, tests running in parallel drop their own values
        static DROPS: std::cell::Cell<usize> = const { std::cell::Cell::new(0) };
    }

    #[derive(Reflect, Clone, Debug, PartialEq)]
    struct Tracked {
        value: f32,
    }

    impl Drop for Tracked {
        fn drop(&mut self) {
            DROPS.set(DROPS.get() + 1);
        }
    }

    /// The arguments of a call: the native values, then the numbers
    struct Args<'a> {
        natives: &'a [&'a NativeBox],
        numbers: &'a [f64],
    }

    // Safety: bases are of `NativeBox`es which outlive the call
    unsafe impl ArgSource for Args<'_> {
        fn len(&self) -> usize {
            self.natives.len() + self.numbers.len()
        }
        fn number(&self, i: usize) -> Option<f64> {
            self.numbers
                .get(i.checked_sub(self.natives.len())?)
                .copied()
        }
        fn integer(&self, _: usize) -> Option<i64> {
            None
        }
        fn boolean(&self, _: usize) -> Option<bool> {
            None
        }
        fn native(&self, i: usize) -> Option<*mut u8> {
            self.natives.get(i).map(|n| n.base())
        }
        fn script_value(&self, i: usize) -> Result<ScriptValue, InteropError> {
            match self.native(i) {
                // Safety: see above
                Some(base) => unsafe { NativeStorage::to_script_value(base) },
                None => Ok(self.number(i).map(ScriptValue::Float).unwrap_or_default()),
            }
        }
    }

    #[derive(Default)]
    struct Return {
        native: Option<NativeBox>,
        value: Option<ScriptValue>,
    }

    // Safety: `native` keeps the value in a `NativeBox`, which calls `write` once when it's made
    unsafe impl ReturnSink for Return {
        fn unit(&mut self) {}
        fn number(&mut self, n: f64) {
            self.value = Some(ScriptValue::Float(n));
        }
        fn integer(&mut self, i: i64) {
            self.value = Some(ScriptValue::Integer(i));
        }
        fn boolean(&mut self, b: bool) {
            self.value = Some(ScriptValue::Bool(b));
        }
        unsafe fn native(
            &mut self,
            _: &std::sync::Arc<NativeType>,
            vtable: &'static NativeVTable,
            write: &mut dyn FnMut(*mut u8),
        ) -> bool {
            // Safety: `write` writes a value of the vtable's type, see the method's contract
            self.native = unsafe { NativeBox::new(vtable, write) }.ok();
            self.native.is_some()
        }
        fn script_value(&mut self, value: ScriptValue) -> Result<(), InteropError> {
            self.value = Some(value);
            Ok(())
        }
    }

    fn tracked(value: f32) -> NativeBox {
        let vtable = native_vtable(
            &ThreadWorldContainer::try_get_context().unwrap().world,
            TypeId::of::<Tracked>(),
        )
        .unwrap();
        let mut value = Some(Tracked { value });
        // Safety: writes the Tracked, `NativeBox::new` calls it once
        unsafe {
            NativeBox::new(vtable, &mut |slot| {
                if let Some(value) = value.take() {
                    (slot as *mut Tracked).write(value)
                }
            })
        }
        .unwrap()
    }

    fn value_of(native: &NativeBox) -> f32 {
        // Safety: made by `tracked`
        unsafe { (*(NativeStorage::value(native.base()) as *const Tracked)).value }
    }

    fn with_native_world(f: impl FnOnce()) {
        let mut world = World::default();
        NativeType::new::<Tracked>().register(&mut world);
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        WorldAccessGuard::with_static_guard(&mut world, cache, |_| f());
    }

    fn call(function: &DynamicScriptFunction, args: Args) -> (Result<(), InteropError>, Return) {
        let mut ret = Return::default();
        let result = function.call_with(&args, &mut ret, FunctionCallContext::new(Language::Lua));
        (result, ret)
    }

    #[test]
    fn typed_calls_read_and_return_native_values() {
        let scale = (|v: R<Tracked>, by: f32| {
            V(Tracked {
                value: v.value * by,
            })
        })
        .into_dynamic_script_function();
        let set = (|mut v: M<Tracked>, to: f32| v.value = to).into_dynamic_script_function();
        with_native_world(|| {
            let native = tracked(2.0);
            let (result, ret) = call(
                &scale,
                Args {
                    natives: &[&native],
                    numbers: &[3.0],
                },
            );
            result.unwrap();
            assert_eq!(
                value_of(ret.native.as_ref().unwrap()),
                6.0,
                "a native result"
            );

            let (result, _) = call(
                &set,
                Args {
                    natives: &[&native],
                    numbers: &[3.0],
                },
            );
            result.unwrap();
            assert_eq!(value_of(&native), 3.0, "changed in place");

            let (result, _) = call(
                &scale,
                Args {
                    natives: &[],
                    numbers: &[1.0, 2.0],
                },
            );
            assert!(result.is_err(), "a number isn't a Tracked");
        });
    }

    #[test]
    fn conflicting_borrows_of_one_value_error() {
        let both = (|_a: M<Tracked>, _b: R<Tracked>| ()).into_dynamic_script_function();
        let shared = (|_a: R<Tracked>, _b: R<Tracked>| ()).into_dynamic_script_function();
        with_native_world(|| {
            let native = tracked(1.0);
            let (result, _) = call(
                &both,
                Args {
                    natives: &[&native, &native],
                    numbers: &[],
                },
            );
            assert!(result.is_err(), "&mut and & of one value");
            let (result, _) = call(
                &shared,
                Args {
                    natives: &[&native, &native],
                    numbers: &[],
                },
            );
            assert!(
                result.is_ok(),
                "two & of one value, after the first call released its borrows"
            );
        });
    }

    #[test]
    fn native_values_are_dropped_once() {
        let copy = (|v: V<Tracked>| v).into_dynamic_script_function();
        let mut drops = 0;
        with_native_world(|| {
            let native = tracked(1.0);
            let before = DROPS.get();
            let (result, ret) = call(
                &copy,
                Args {
                    natives: &[&native],
                    numbers: &[],
                },
            );
            result.unwrap();
            drop(ret);
            drop(native);
            drops = DROPS.get() - before;
        });
        // the argument and the copy it returned
        assert_eq!(drops, 2);
    }

    /// Not Copy: dropping and cloning go through the vtable
    #[derive(Reflect, Clone, Debug, PartialEq)]
    struct Labelled {
        value: f32,
        label: String,
    }

    /// Needs more alignment than the storage's header gives
    #[derive(Reflect, Clone, Copy, Debug, PartialEq)]
    #[repr(C, align(16))]
    struct Aligned {
        values: [f32; 4],
    }

    fn native_of<T: Reflect + Clone>(value: T) -> NativeBox {
        let vtable = native_vtable(
            &ThreadWorldContainer::try_get_context().unwrap().world,
            TypeId::of::<T>(),
        )
        .unwrap();
        let mut value = Some(value);
        // Safety: writes the T, `NativeBox::new` calls it once
        unsafe {
            NativeBox::new(vtable, &mut |slot| {
                if let Some(value) = value.take() {
                    (slot as *mut T).write(value)
                }
            })
        }
        .unwrap()
    }

    fn value_in<T>(native: &NativeBox) -> &T {
        // Safety: the box holds a T
        unsafe { &*(NativeStorage::value(native.base()) as *const T) }
    }

    /// The whole chain a language goes through: registering a native type with `script_native!`, making values with typed
    /// functions, fields, a value passed twice, non native arguments (the slow path), and copies into references.
    #[test]
    fn native_values_through_the_whole_chain() {
        let make = (|value: f32| {
            V(Labelled {
                value,
                label: "made".to_owned(),
            })
        })
        .into_dynamic_script_function();
        // `Union` takes a native value as it is, also the receiver passed again
        let add = (|a: R<Labelled>, b: crate::Union<R<Labelled>, f32>| {
            V(Labelled {
                value: a.value
                    + match b.into_left() {
                        Ok(b) => b.value,
                        Err(n) => n,
                    },
                label: a.label.clone(),
            })
        })
        .into_dynamic_script_function();

        let mut world = World::default();
        crate::script_native!(
            &mut world,
            Labelled {
                value: f32,
                label: String
            }
        );
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        WorldAccessGuard::with_static_guard(&mut world, cache, |world| {
            // a native value returned by a typed call
            let (result, made) = call(
                &make,
                Args {
                    natives: &[],
                    numbers: &[2.0],
                },
            );
            result.unwrap();
            let made = made.native.unwrap();
            assert_eq!(value_in::<Labelled>(&made).label, "made");

            // fields, through the accessors `script_native!` registered
            let fields = {
                let registry = world.script_function_registry();
                let registry = registry.read();
                registry
                    .native_type(TypeId::of::<Labelled>())
                    .unwrap()
                    .fields
                    .clone()
            };
            let (result, got) = call(
                &fields["value"].get,
                Args {
                    natives: &[&made],
                    numbers: &[],
                },
            );
            result.unwrap();
            assert_eq!(got.value, Some(ScriptValue::Float(2.0)));
            let (result, _) = call(
                fields["value"].set.as_ref().unwrap(),
                Args {
                    natives: &[&made],
                    numbers: &[5.0],
                },
            );
            result.unwrap();
            assert_eq!(value_in::<Labelled>(&made).value, 5.0);

            // the same value twice, and a number for the union
            let (result, sum) = call(
                &add,
                Args {
                    natives: &[&made, &made],
                    numbers: &[],
                },
            );
            result.unwrap();
            assert_eq!(
                value_in::<Labelled>(sum.native.as_ref().unwrap()).value,
                10.0
            );
            let (result, sum) = call(
                &add,
                Args {
                    natives: &[&made],
                    numbers: &[1.0],
                },
            );
            result.unwrap();
            assert_eq!(
                value_in::<Labelled>(sum.native.as_ref().unwrap()).value,
                6.0
            );

            // a copy, as everything which isn't typed sees it, goes back in through the slow path
            let copy = unsafe { NativeStorage::to_script_value(made.base()) }.unwrap();
            let mut ret = Return::default();
            add.call_with(
                &ScriptValueArgs::new([copy.clone(), copy.clone()]),
                &mut ret,
                FunctionCallContext::new(Language::Lua),
            )
            .unwrap();
            assert_eq!(
                value_in::<Labelled>(ret.native.as_ref().unwrap()).value,
                10.0
            );

            // a function without a typed entry can't change a copy in place, the change would be lost
            let set = (|mut v: crate::M<Labelled>| v.value = 1.0).into_dynamic_script_function();
            assert!(
                set.call(vec![copy.clone()], FunctionCallContext::new(Language::Lua))
                    .is_err(),
                "changing a copy errors"
            );
            // nor through the reference itself, i.e. by the generic `set`
            let ScriptValue::Reference(copy) = copy else {
                panic!("a copy is a reference")
            };
            assert!(
                copy.with_reflect_mut(world.clone(), |_| ()).is_err(),
                "write access to a copy is refused"
            );
            assert!(
                copy.with_reflect(world.clone(), |_| ()).is_ok(),
                "reads work"
            );
            assert_eq!(value_in::<Labelled>(&made).value, 5.0);
        });
    }

    #[derive(bevy_ecs::resource::Resource, Reflect, Clone, Debug, PartialEq, Default)]
    struct Inner {
        x: f32,
    }

    #[derive(bevy_ecs::component::Component, Reflect, Clone, Debug, PartialEq, Default)]
    struct Outer {
        inner: Inner,
        n: f32,
    }

    /// A component of a native type is a handle to it (a place), its native fields are places too: reads and writes go to
    /// the world, conflicting accesses error, places follow the component (archetype moves, removal, despawning), resources
    /// work alike, and outside typed calls a place is a reference to where it is.
    #[test]
    fn places_read_and_write_components() {
        fn with_guard<O>(world: &mut World, f: impl FnOnce(WorldGuard<'_>) -> O) -> O {
            let cache = WorldAccessGuard::setup_cache(world, CurrentScriptAttachment::empty());
            WorldAccessGuard::with_static_guard(world, cache, f)
        }
        let mut world = World::default();
        {
            let registry = world.get_resource_or_init::<bevy_ecs::reflect::AppTypeRegistry>();
            let mut registry = registry.write();
            registry.register::<Outer>();
            registry.register::<Inner>();
        }
        // the outer type first: it stops being stored as values once its field's type is native
        crate::script_native!(
            &mut world,
            Outer {
                inner: Inner,
                n: f32
            }
        );
        crate::script_native!(&mut world, Inner { x: f32 });
        let outer_value = |x| Outer {
            inner: Inner { x },
            n: 2.0,
        };
        let entity = world.spawn(outer_value(1.0)).id();
        let component_id = world.component_id::<Outer>().unwrap();
        let natives = |world: &WorldGuard, type_id| {
            let registry = world.script_function_registry();
            let registry = registry.read();
            registry.native_type(type_id).unwrap().clone()
        };
        let handle_of = |world: &WorldGuard, reference: ReflectReference| {
            let mut ret = Return::default();
            crate::IntoScript::into_ret(
                reference,
                &mut ret,
                world.clone(),
                &NativeReturnType::default(),
            )
            .unwrap();
            ret.native
                .expect("a component or resource of a native type is a native handle")
        };
        let get = |function: &DynamicScriptFunction, native: &NativeBox| {
            call(
                function,
                Args {
                    natives: &[native],
                    numbers: &[],
                },
            )
        };
        let read_x = (|v: V<Inner>| v.x).into_dynamic_script_function();

        let (handle, proxy) = with_guard(&mut world, |world| {
            let outer = natives(&world, TypeId::of::<Outer>());
            let inner = natives(&world, TypeId::of::<Inner>());
            assert!(
                !outer.inline && inner.inline,
                "a type with native fields is only a place"
            );

            // Safety: the component is an Outer
            let handle = handle_of(&world, unsafe {
                ReflectReference::new_component_ref_by_id(
                    entity,
                    component_id,
                    TypeId::of::<Outer>(),
                )
            });
            let (result, got) = get(&outer.fields["inner"].get, &handle);
            result.unwrap();
            let proxy = got.native.expect("a native field of a place is a place");
            // Safety: initialised storage
            assert!(unsafe { NativeStorage::place(proxy.base()) }.is_some());

            let (result, _) = call(
                inner.fields["x"].set.as_ref().unwrap(),
                Args {
                    natives: &[&proxy],
                    numbers: &[5.0],
                },
            );
            result.unwrap();
            let (result, got) = get(&outer.fields["n"].get, &handle);
            result.unwrap();
            assert_eq!(
                got.value,
                Some(ScriptValue::Float(2.0)),
                "plain fields are read"
            );
            let (result, got) = get(&read_x, &proxy);
            result.unwrap();
            assert_eq!(
                got.value,
                Some(ScriptValue::Float(5.0)),
                "a value copied out of a place"
            );

            // the component, or its field, written while it's used otherwise
            let write_read = (|_a: M<Outer>, _b: R<Inner>| ()).into_dynamic_script_function();
            let write_write = (|_a: M<Inner>, _b: M<Inner>| ()).into_dynamic_script_function();
            for (function, a, b) in [
                (&write_read, &handle, &proxy),
                (&write_write, &proxy, &proxy),
            ] {
                let (result, _) = call(
                    function,
                    Args {
                        natives: &[a, b],
                        numbers: &[],
                    },
                );
                assert!(result.is_err(), "conflicting accesses of a component error");
            }

            // a getter called the dynamic way: a reference to the field, which writes the component
            let mut sink = ScriptValueReturn::default();
            outer.fields["inner"]
                .get
                .call_with(
                    &Args {
                        natives: &[&handle],
                        numbers: &[],
                    },
                    &mut sink,
                    FunctionCallContext::new(Language::Lua),
                )
                .unwrap();
            let ScriptValue::Reference(field) = sink.into_value() else {
                panic!("a field of a place is a reference when not native")
            };
            field
                .with_reflect_mut(world.clone(), |r| {
                    r.try_downcast_mut::<Inner>().unwrap().x = 6.0;
                })
                .unwrap();

            // outside typed calls, a place is a reference to where it is
            let ScriptValue::Reference(copy) =
                (unsafe { NativeStorage::to_script_value(proxy.base()) }).unwrap()
            else {
                panic!("a place is a reference")
            };
            assert_eq!(
                copy.with_reflect(world.clone(), |r| r.try_downcast_ref::<Inner>().cloned())
                    .unwrap(),
                Some(Inner { x: 6.0 })
            );
            (handle, proxy)
        });
        assert_eq!(
            world.get::<Outer>(entity).unwrap().inner.x,
            6.0,
            "written to the world"
        );

        let x_of = |world: &mut World| {
            with_guard(world, |_| {
                let (result, got) = get(&read_x, &proxy);
                result.map(|()| got.value)
            })
        };
        // the entity moves to another archetype
        world
            .entity_mut(entity)
            .insert(bevy_ecs::name::Name::default());
        assert_eq!(
            x_of(&mut world).unwrap(),
            Some(ScriptValue::Float(6.0)),
            "after a move"
        );
        world.entity_mut(entity).remove::<Outer>();
        assert!(x_of(&mut world).is_err(), "a removed component errors");
        world.entity_mut(entity).insert(outer_value(7.0));
        assert_eq!(
            x_of(&mut world).unwrap(),
            Some(ScriptValue::Float(7.0)),
            "inserted again"
        );
        world.despawn(entity);
        assert!(x_of(&mut world).is_err(), "a despawned entity errors");
        drop(handle);

        // resources are places too
        world.insert_resource(Inner { x: 3.0 });
        let resource_id = world.component_id::<Inner>().unwrap();
        with_guard(&mut world, |world| {
            // Safety: the resource is an Inner
            let handle = handle_of(&world, unsafe {
                ReflectReference::new_resource_ref_by_id(resource_id, TypeId::of::<Inner>())
            });
            let (result, got) = get(&read_x, &handle);
            result.unwrap();
            assert_eq!(got.value, Some(ScriptValue::Float(3.0)), "a resource");
        });

        // places nest so deep only
        let mut place = NativePlace::new(PlaceBase::Resource(resource_id), TypeId::of::<Outer>());
        for _ in 0..NativePlace::MAX_DEPTH {
            place = place.field(0, 0).unwrap();
        }
        assert!(place.field(0, 0).is_none(), "nested too deeply");
    }

    /// A `Deref` whose target is in the value or on the heap, depending on the value.
    #[derive(Reflect, Clone, Debug)]
    enum Small {
        Inline(Inner),
        Boxed(#[reflect(ignore)] Box<Inner>),
    }

    impl std::ops::Deref for Small {
        type Target = Inner;
        fn deref(&self) -> &Inner {
            match self {
                Small::Inline(inner) => inner,
                Small::Boxed(inner) => inner,
            }
        }
    }

    /// Fields reached through `Deref` (i.e. glam's SIMD vectors) are found per value: written when they're inside the value,
    /// only read when they're elsewhere, also for values of one type.
    #[test]
    fn fields_through_deref() {
        let mut world = World::default();
        crate::script_native!(&mut world, deref Small { x: f32 });
        let inline = (|| V(Small::Inline(Inner { x: 0.0 }))).into_dynamic_script_function();
        let boxed = (|| V(Small::Boxed(Box::new(Inner { x: 0.0 })))).into_dynamic_script_function();
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        WorldAccessGuard::with_static_guard(&mut world, cache, |world| {
            let field = {
                let registry = world.script_function_registry();
                let registry = registry.read();
                registry.native_type(TypeId::of::<Small>()).unwrap().fields["x"].clone()
            };
            // the inline value first: nothing about it is kept for the boxed one
            for (make, writable) in [(&inline, true), (&boxed, false), (&inline, true)] {
                let (result, value) = call(
                    make,
                    Args {
                        natives: &[],
                        numbers: &[],
                    },
                );
                result.unwrap();
                let value = value.native.unwrap();
                let (result, _) = call(
                    field.set.as_ref().unwrap(),
                    Args {
                        natives: &[&value],
                        numbers: &[4.0],
                    },
                );
                assert_eq!(
                    result.is_ok(),
                    writable,
                    "only fields inside the value are written"
                );
                let (result, got) = call(
                    &field.get,
                    Args {
                        natives: &[&value],
                        numbers: &[],
                    },
                );
                result.unwrap();
                let expected = if writable { 4.0 } else { 0.0 };
                assert_eq!(got.value, Some(ScriptValue::Float(expected)));
            }
        });
    }

    #[test]
    fn values_needing_more_alignment_than_the_header() {
        let double = (|v: R<Aligned>| {
            V(Aligned {
                values: v.values.map(|x| x * 2.0),
            })
        })
        .into_dynamic_script_function();
        let mut world = World::default();
        NativeType::new::<Aligned>().register(&mut world);
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        WorldAccessGuard::with_static_guard(&mut world, cache, |_| {
            let native = native_of(Aligned {
                values: [1.0, 2.0, 3.0, 4.0],
            });
            let value = unsafe { NativeStorage::value(native.base()) };
            assert_eq!(value as usize % 16, 0, "the value is aligned");
            let (result, ret) = call(
                &double,
                Args {
                    natives: &[&native],
                    numbers: &[],
                },
            );
            result.unwrap();
            assert_eq!(
                *value_in::<Aligned>(ret.native.as_ref().unwrap()),
                Aligned {
                    values: [2.0, 4.0, 6.0, 8.0]
                }
            );
        });
    }
}
