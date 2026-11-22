//! Typed calls and native values: the fast path next to [`ScriptValue`] conversions.
//!
//! - Every function reads its arguments from an [`ArgSource`] and writes its result to a [`ReturnSink`] (see
//!   [`DynamicScriptFunction::call_with`]), both implemented by each language over its own values, without building
//!   [`ScriptValue`]s. Arguments the language can't give in their native form are converted the usual way
//!   ([`FromScript::from_arg`](crate::FromScript::from_arg)'s slow arm). Rust callers use [`ScriptValueArgs`] and
//!   [`ScriptValueReturn`] (see [`DynamicScriptFunction::call`]).
//! - A [`NativeType`] is stored by the language itself, i.e. in a Lua userdata, as `[NativeHeader][value]`, see [`NativeStorage`].
//!   Its fields are read and written through typed accessor functions, see [`NativeBuilder`].
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
use bevy_mod_scripting_world::WorldGuard;
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
/// and which stays alive and in place until the call returns. The sources [`ArgSource::list`] and [`ArgSource::map`] give
/// keep to the same.
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
    /// The argument as a plain value of the type `type_id` without a [`NativeHeader`] (i.e. a LuaJIT cdata), only for types
    /// with an [`FfiLayout`]: a pointer to the value, alive and in place until the call returns, which needn't be
    /// aligned, and whether it's read-only (see [`ReturnSink::read_only`]).
    fn plain(&self, _i: usize, _type_id: TypeId) -> Option<(*mut u8, bool)> {
        None
    }
    /// The argument as a string, borrowed from the language, if it is one (numbers aren't converted).
    fn string(&self, _i: usize) -> Option<&str> {
        None
    }
    /// Calls `items` with the items of the argument as arguments, if it's a list: what [`ScriptValue::List`] the argument
    /// would convert to. False if it isn't one, or the language has no fast way to read it.
    fn list(&self, _i: usize, _items: &mut dyn FnMut(&dyn ArgSource)) -> bool {
        false
    }
    /// Calls `entry` with each entry of the argument as arguments (the key at 0, the value at 1), if it's a map with string
    /// keys: what [`ScriptValue::Map`] the argument would convert to. `None` if it isn't one, or the language has no fast way
    /// to read it, in which case `entry` may have been called for some entries already.
    fn map(
        &self,
        _i: usize,
        _entry: &mut dyn FnMut(&dyn ArgSource) -> Result<(), InteropError>,
    ) -> Option<Result<(), InteropError>> {
        None
    }
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
    /// Makes the native value returned next a read-only copy: a copy of a field of another value, which scripts can't
    /// write, as the write would be lost (`t.translation.x = 5`), see [`NativeHeader::read_only`]. Native values have
    /// no write-through: scripts assign the whole field. Ignored by languages without read-only values.
    fn read_only(&mut self) {}
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
    pub(crate) clone_into: unsafe fn(*const u8, *mut u8),
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

    /// A reference to a copy of the value at `value`, the slow path of values stored without a [`NativeHeader`] (i.e.
    /// LuaJIT cdata), see [`NativeStorage::to_script_value`].
    ///
    /// # Safety
    /// `value` points to a value of this vtable's type (not a place).
    pub unsafe fn plain_reference(&self, value: *const u8) -> ReflectReference {
        // Safety: the caller's
        unsafe { (self.to_reference)(value) }
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

/// A value of a [`NativeType`] in the world, a whole component or resource, stored by the language in place of a value
/// (see [`NativeVTable::place_vtable`]). Typed functions read and write the world through it, claiming access for the
/// call. Its fields are read as copies: only the whole value has a place.
#[derive(Clone, Copy, Debug)]
pub struct NativePlace {
    /// The component or resource
    pub base: PlaceBase,
    /// The type of the component or resource
    pub base_type: TypeId,
}

impl NativePlace {
    /// The place of a whole component or resource of type `base_type`.
    pub fn new(base: PlaceBase, base_type: TypeId) -> Self {
        Self { base, base_type }
    }

    /// The value in the world, after claiming access to its component or resource for the call (`write` for exclusive access).
    ///
    /// # Safety
    /// - The place was made for a value of the type the pointer is used as.
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
        let cell = scope.world()?.as_unsafe_world_cell()?;
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
        Ok(ptr)
    }

    /// A reference to the value, for everything which isn't typed.
    pub fn to_reference(&self) -> ReflectReference {
        // Safety: the type is the one of the component or resource the place was made for
        unsafe {
            match self.base {
                PlaceBase::Component(entity, id) => {
                    ReflectReference::new_component_ref_by_id(entity, id, self.base_type)
                }
                PlaceBase::Resource(id) => {
                    ReflectReference::new_resource_ref_by_id(id, self.base_type)
                }
            }
        }
    }
}

/// The start of the storage of every native value, see [`NativeStorage`].
#[repr(C)]
pub struct NativeHeader {
    /// The type of the value
    pub vtable: &'static NativeVTable,
    /// Whether the value is a read-only copy (of a field of another value, see [`ReturnSink::read_only`]): scripts and
    /// calls can't change it
    pub read_only: std::cell::Cell<bool>,
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
        // Size(Header) divides the alignment, so the padding then must get us to the Align boundary of values
        // The difference between these two align values gives us the MAXIMUM required distance to such a point in memory,
        // for example with Align(Header) = 8, and Align(Value) = 8, 8 - 8 = 0, as they are already aligned,
        // with Align(Header) = 8 and Align(Value) = 16, we need to make 8 bytes of space to reach the next 16 block
        // With Align(Header) = 8 and Align(Value) = 4 because only powers of 2 are allowed, any lower alignment for the value is already aligned
        // this is a higher bound, as the address might end up not needing padding say in the case of 8 and 16, if we end up at address 8 instead of 0.
        // value_ptr calculates the actual padding necessary given the exact address
        size_of::<NativeHeader>()
            + vtable.layout.align().saturating_sub(Self::ALIGN)
            + vtable.layout.size()
    }

    fn value_ptr(base: *mut u8, vtable: &NativeVTable) -> *mut u8 {
        // the storage has room for the header, the padding and the value, see `size`
        let start = base.wrapping_add(size_of::<NativeHeader>());
        let align = vtable.layout.align();
        // start & (align - 1) is equivalent to start % align, align - 1 is a bitmask over the remainder
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
                read_only: Default::default(),
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
        // a place is referenced where it is
        if let Some(place) = unsafe { Self::place(base) } {
            return Ok(ScriptValue::Reference(place.to_reference()));
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
    /// Where values store the field, if languages may read and write it in place instead of calling the accessors
    pub direct: Option<DirectField>,
}

/// A primitive field at an offset of every value of a [`NativeType`], which languages read and write in place in the values
/// they store (not in places), unless the value is a read-only copy (see [`NativeHeader::read_only`]).
///
/// The accessors do the same: reading converts to the language's number or boolean, writing a number truncates it with `as`.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct DirectField {
    /// The offset of the field in the value
    pub offset: usize,
    /// The type of the field
    pub kind: DirectKind,
}

/// The type of a [`DirectField`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DirectKind {
    /// `f32`
    F32,
    /// `f64`
    F64,
    /// `i32`
    I32,
    /// `u32`
    U32,
    /// `bool`
    Bool,
}

impl DirectKind {
    /// The kind of `F`, if it's one of the direct types
    pub fn of<F: 'static>() -> Option<Self> {
        let id = TypeId::of::<F>();
        [
            (TypeId::of::<f32>(), Self::F32),
            (TypeId::of::<f64>(), Self::F64),
            (TypeId::of::<i32>(), Self::I32),
            (TypeId::of::<u32>(), Self::U32),
            (TypeId::of::<bool>(), Self::Bool),
        ]
        .into_iter()
        .find_map(|(t, kind)| (t == id).then_some(kind))
    }
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
    /// The C layout of the values, if languages may store them as plain C structs (i.e. LuaJIT cdata), see
    /// [`NativeBuilder::ffi`].
    pub ffi: Option<FfiLayout>,
}

/// The C layout of the values of a [`NativeType`]: a struct of scalar fields, checked against the Rust type when declared
/// (see [`NativeBuilder::ffi`]). Values with a layout are `Copy` and plain bytes, so languages may store them without a
/// [`NativeHeader`]: typed calls read them with [`ArgSource::plain`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FfiLayout {
    /// The name of the C struct, i.e. `bms_Vec3`
    pub name: Cow<'static, str>,
    /// The size of a value
    pub size: usize,
    /// The alignment of a value
    pub align: usize,
    /// The fields, in order
    pub fields: Vec<FfiField>,
}

/// A field of an [`FfiLayout`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct FfiField {
    /// The field's name
    pub name: &'static str,
    /// The field's type: a scalar, or a native type with an [`FfiLayout`] of scalar fields (i.e. a matrix' columns)
    pub ty: FfiType,
    /// The offset of the field in a value
    pub offset: usize,
}

impl FfiLayout {
    /// The scalar fields of the C struct, `(name, C type, offset)`. A field of a native type is flattened into the scalar
    /// fields of its layout (`layout_of`), named `_{field}_{inner}`: a nested struct field would be read as a reference,
    /// which doesn't keep the value alive. `None` if a nested type has no layout, or fields of native types itself.
    pub fn c_fields<'a>(
        &'a self,
        layout_of: &dyn Fn(TypeId) -> Option<&'a FfiLayout>,
    ) -> Option<Vec<(Cow<'static, str>, &'static str, usize)>> {
        let mut out = Vec::with_capacity(self.fields.len());
        for field in &self.fields {
            // the name is a field of the C struct
            if !is_c_field_name(field.name) {
                return None;
            }
            match field.ty {
                FfiType::Scalar(c) => out.push((field.name.into(), c, field.offset)),
                FfiType::Value(type_id) => {
                    for inner in &layout_of(type_id)?.fields {
                        let FfiType::Scalar(c) = inner.ty else {
                            return None;
                        };
                        let name = format!("_{}_{}", field.name, inner.name);
                        out.push((name.into(), c, field.offset + inner.offset));
                    }
                }
            }
        }
        Some(out)
    }

    /// The C declaration of the struct, i.e. `typedef struct __attribute__((aligned(4))) { float x; float y; float z; }
    /// bms_Vec3;`, aligned like the Rust type, with the fields of [`Self::c_fields`] and padding where the Rust type has
    /// some (a `uint8_t _padN` for each byte: scalars only, see [`Self::c_fields`]). `None` if a field can't be flattened
    /// or named in C.
    pub fn c_declaration<'a>(
        &'a self,
        layout_of: &dyn Fn(TypeId) -> Option<&'a FfiLayout>,
    ) -> Option<String> {
        use std::fmt::Write as _;
        let mut fields = String::new();
        let mut end = 0;
        let pad = |fields: &mut String, from: usize, to: usize| {
            // a scalar for each byte: an array field would be read as a reference, which is neither bounds-checked nor
            // keeps the value alive
            for at in from..to {
                let _ = write!(fields, "uint8_t _pad{at}; ");
            }
        };
        for (name, c_type, offset) in self.c_fields(layout_of)? {
            pad(&mut fields, end, offset);
            let _ = write!(fields, "{c_type} {name}; ");
            end = offset + c_size(c_type)?;
        }
        pad(&mut fields, end, self.size);
        Some(format!(
            "typedef struct __attribute__((aligned({}))) {{ {fields}}} {};",
            self.align, self.name
        ))
    }
}

/// A parameter or the result of an [`FfiFunction`].
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum FfiType {
    /// A scalar passed by value, its C type, i.e. `float`
    Scalar(&'static str),
    /// A pointer to a value of a native type with an [`FfiLayout`], read only (`R<T>`, `V<T>`), or for results, where the
    /// value is written (an out-pointer after the parameters)
    Value(TypeId),
}

/// A function of a native type callable through a language's FFI (i.e. LuaJIT's) with C types: an `extern "C"` form of a
/// registered function, made from a Rust function by [`FfiFn`] (see [`FfiArg`] and [`FfiReturn`]), which is what
/// `ScriptFunctionRegistry::register_ffi` and `#[ffi]` in `#[script_bindings]` blocks use.
///
/// A function has one FFI form for each C signature: a function taking a [`crate::Union`] is registered once for each
/// alternative, i.e. `add(R<Vec3>, R<Vec3>)` and `add(R<Vec3>, f32)`.
///
/// A panic of the function aborts the process, as it can't unwind through C: only for functions which don't panic (no
/// integer arithmetic which may overflow, no indexing).
#[derive(Clone, Debug)]
pub struct FfiFunction {
    /// The name of the function, the same as the registered one's
    pub name: &'static str,
    /// The `extern "C"` function
    pub function: *const (),
    /// Its parameters, in order
    pub params: Vec<FfiType>,
    /// Its result: a scalar returned, or a value written to an out-pointer after the parameters
    pub result: FfiType,
    /// Whether it may fail (`Result`, `Option`): it takes a pointer to a byte after the out-pointer, which it writes
    /// [`FFI_ERROR`] or [`FFI_NONE`] to when it does, else it's given null
    pub fallible: bool,
}

/// A parameter of an FFI function (see [`FfiFunction`]): a scalar passed by value, or `R<T>`/`V<T>` passed as a pointer
/// to the value. `T` needs an [`FfiLayout`] ([`NativeBuilder::ffi`]), which languages check when they make their
/// C types: it can't be a bound, the orphan rule would keep binding crates from declaring it for other crates' types.
///
/// A [`crate::Union`] is no parameter: each alternative has another C type, so each is registered as an FFI form of its
/// own.
///
/// # Safety
/// `C` is FFI-safe and the parameter of the C type [`Self::kind`] says.
#[diagnostic::on_unimplemented(
    message = "`{Self}` can't be a parameter of an FFI function",
    label = "not an FFI parameter",
    note = "FFI parameters are `f32`, `f64`, `i32`, `u32`, `bool`, `R<T>` and `V<T>` (`T` with a C layout, see `NativeBuilder::ffi`). A function taking a `Union` has an FFI form for each alternative: register them with `ScriptFunctionRegistry::register_ffi`"
)]
pub unsafe trait FfiArg {
    /// How the wrapper takes it
    type C;
    /// Its C type
    fn kind() -> FfiType;
    /// The parameter from what the wrapper is given.
    ///
    /// # Safety
    /// `c` is what a caller passes for [`Self::kind`]: pointers point to valid values, which outlive the call.
    unsafe fn from_c(c: Self::C) -> Self;
}

/// The result of an FFI function (see [`FfiFunction`]): a scalar returned, `V<T>` written to the out-pointer (`T` with
/// an [`FfiLayout`], see [`FfiArg`]), or a `Result`/`Option` of one of those, which fails by writing [`FFI_ERROR`] or
/// [`FFI_NONE`] to the `failed` pointer.
///
/// # Safety
/// `C` is FFI-safe, valid when zeroed (returned when failing) and what [`Self::result`] says is returned,
/// [`Self::into_c`] writes what it says is written.
#[diagnostic::on_unimplemented(
    message = "`{Self}` can't be the result of an FFI function",
    label = "not an FFI result",
    note = "FFI results are `f32`, `f64`, `i32`, `u32`, `bool` and `V<T>` (`T` with a C layout, see `NativeBuilder::ffi`), or a `Result<_, InteropError>` or `Option` of one"
)]
pub unsafe trait FfiReturn {
    /// What the wrapper returns
    type C;
    /// How it's returned
    fn result() -> FfiType;
    /// Whether it may fail, see [`FfiFunction::fallible`]
    fn fallible() -> bool {
        false
    }
    /// The wrapper's return value, writing to `out` what [`Self::result`] says is written there, or [`FFI_ERROR`] (the
    /// error kept for [`take_ffi_error`]) or [`FFI_NONE`] to `failed` when it fails.
    ///
    /// # Safety
    /// `out` points to memory big and aligned enough for the result, `failed` to a byte if [`Self::fallible`].
    unsafe fn into_c(self, out: *mut u8, failed: *mut u8) -> Self::C;
}

/// Written to the `failed` pointer of an [`FfiFunction`] when it returns an error, see [`take_ffi_error`].
pub const FFI_ERROR: u8 = 1;
/// Written to the `failed` pointer of an [`FfiFunction`] when it returns `None`.
pub const FFI_NONE: u8 = 2;

thread_local! {
    /// The error of the last [`FfiFunction`] which failed on this thread
    static FFI_ERROR_VALUE: std::cell::RefCell<Option<InteropError>> = const { std::cell::RefCell::new(None) };
}

/// Takes the error of the last [`FfiFunction`] which failed on this thread (with [`FFI_ERROR`]).
pub fn take_ffi_error() -> Option<InteropError> {
    FFI_ERROR_VALUE.with(|e| e.borrow_mut().take())
}

/// A zeroed `C`, returned by a failing [`FfiReturn`].
///
/// # Safety
/// `C` is valid when zeroed.
unsafe fn zeroed<C>() -> C {
    // Safety: upheld by the caller
    unsafe { std::mem::MaybeUninit::zeroed().assume_init() }
}

// Safety: `T`'s result, or a zeroed one and `FFI_ERROR` written to `failed`
unsafe impl<T: FfiReturn> FfiReturn for Result<T, InteropError> {
    type C = T::C;
    fn result() -> FfiType {
        T::result()
    }
    fn fallible() -> bool {
        true
    }
    unsafe fn into_c(self, out: *mut u8, failed: *mut u8) -> Self::C {
        match self {
            // Safety: upheld by the caller
            Ok(value) => unsafe { value.into_c(out, failed) },
            Err(error) => {
                FFI_ERROR_VALUE.with(|e| *e.borrow_mut() = Some(error));
                // Safety: upheld by the caller, `T::C` is valid when zeroed (see the trait)
                unsafe {
                    failed.write(FFI_ERROR);
                    zeroed()
                }
            }
        }
    }
}

// Safety: `T`'s result, or a zeroed one and `FFI_NONE` written to `failed`
unsafe impl<T: FfiReturn> FfiReturn for Option<T> {
    type C = T::C;
    fn result() -> FfiType {
        T::result()
    }
    fn fallible() -> bool {
        true
    }
    unsafe fn into_c(self, out: *mut u8, failed: *mut u8) -> Self::C {
        match self {
            // Safety: upheld by the caller
            Some(value) => unsafe { value.into_c(out, failed) },
            // Safety: upheld by the caller, `T::C` is valid when zeroed (see the trait)
            None => unsafe {
                failed.write(FFI_NONE);
                zeroed()
            },
        }
    }
}

macro_rules! ffi_scalar {
    ($($ty:ty => $c:literal),*) => {$(
        // Safety: passed and returned by value as the C type of the same size and representation
        unsafe impl FfiArg for $ty {
            type C = $ty;
            fn kind() -> FfiType {
                FfiType::Scalar($c)
            }
            unsafe fn from_c(c: Self::C) -> Self {
                c
            }
        }
        // Safety: see above
        unsafe impl FfiReturn for $ty {
            type C = $ty;
            fn result() -> FfiType {
                FfiType::Scalar($c)
            }
            unsafe fn into_c(self, _: *mut u8, _: *mut u8) -> Self::C {
                self
            }
        }
    )*};
}

ffi_scalar!(f32 => "float", f64 => "double", i32 => "int32_t", u32 => "uint32_t", bool => "bool");

// Safety: a pointer to a value, which outlives the call (upheld by the caller of `from_c`)
unsafe impl<'w, T: 'static> FfiArg for crate::R<'w, T> {
    type C = *const T;
    fn kind() -> FfiType {
        FfiType::Value(TypeId::of::<T>())
    }
    unsafe fn from_c(c: Self::C) -> Self {
        // Safety: upheld by the caller
        crate::R(unsafe { &*c })
    }
}

// Safety: see above, the value is copied
unsafe impl<T: Copy + 'static> FfiArg for crate::V<T> {
    type C = *const T;
    fn kind() -> FfiType {
        FfiType::Value(TypeId::of::<T>())
    }
    unsafe fn from_c(c: Self::C) -> Self {
        // Safety: upheld by the caller
        crate::V(unsafe { *c })
    }
}

// Safety: the value is written to the out-pointer
unsafe impl<T: Copy + 'static> FfiReturn for crate::V<T> {
    type C = ();
    fn result() -> FfiType {
        FfiType::Value(TypeId::of::<T>())
    }
    unsafe fn into_c(self, out: *mut u8, _: *mut u8) -> Self::C {
        // Safety: upheld by the caller
        unsafe { out.cast::<T>().write(self.0) }
    }
}

// Safety: a pointer to a function, which can be called from any thread
unsafe impl Send for FfiFunction {}
// Safety: see above
unsafe impl Sync for FfiFunction {}

/// The C type of `F` if it's a scalar a field of an [`FfiLayout`] may have: the same size, alignment and representation,
/// and every bit pattern a valid value (languages may write any), which rules out `bool`.
fn c_scalar<F: 'static>() -> Option<&'static str> {
    let id = TypeId::of::<F>();
    [
        (TypeId::of::<f32>(), "float"),
        (TypeId::of::<f64>(), "double"),
        (TypeId::of::<i32>(), "int32_t"),
        (TypeId::of::<u32>(), "uint32_t"),
    ]
    .into_iter()
    .find_map(|(t, c)| (t == id).then_some(c))
}

/// Whether a Rust field name can name a field of a C struct: not a keyword of C or a type name C declarations use.
fn is_c_field_name(name: &str) -> bool {
    const RESERVED: &[&str] = &[
        "auto", "bool", "char", "complex", "default", "double", "float", "inline", "int", "long",
        "register", "restrict", "short", "signed", "sizeof", "typedef", "union", "unsigned",
        "void", "volatile", "int8_t", "int16_t", "int32_t", "int64_t", "uint8_t", "uint16_t",
        "uint32_t", "uint64_t", "size_t",
    ];
    !RESERVED.contains(&name) && !name.starts_with("r#")
}

/// The size of the C type of a scalar, see [`c_scalar`].
fn c_size(c_type: &str) -> Option<usize> {
    match c_type {
        "float" | "int32_t" | "uint32_t" => Some(4),
        "double" => Some(8),
        "bool" => Some(1),
        _ => None,
    }
}

/// The C layout of a struct of fields of the given `(size, alignment)`, in order, aligned to at least `align`: its size
/// and alignment. The offset of each field is written to `offsets`, as far as it has room.
///
/// It's what a type declared with [`NativeBuilder::ffi`] must have, checked when it's declared, and at compile time by
/// [`crate::assert_ffi_layout`].
pub const fn c_layout(
    fields: &[(usize, usize)],
    align: usize,
    offsets: &mut [usize],
) -> (usize, usize) {
    let mut offset: usize = 0;
    let mut max_align = align;
    let mut i = 0;
    while i < fields.len() {
        let (size, field_align) = fields[i];
        offset = offset.div_ceil(field_align) * field_align;
        if i < offsets.len() {
            offsets[i] = offset;
        }
        offset += size;
        if field_align > max_align {
            max_align = field_align;
        }
        i += 1;
    }
    (offset.div_ceil(max_align) * max_align, max_align)
}

/// Makes the `extern "C"` form of a Rust function, see [`FfiFunction`]: implemented for functions and closures which
/// capture nothing, of up to 5 parameters which are [`FfiArg`]s, returning an [`FfiReturn`].
///
/// ```rust,ignore
/// // `v:dot(w)` as a C call, next to the registered `dot`
/// registry.register_ffi(TypeId::of::<Vec3>(), "dot", |a: R<Vec3>, b: R<Vec3>| a.dot(*b));
/// // a function taking a `Union` has a form for each alternative
/// registry.register_ffi(TypeId::of::<Vec3>(), "add", |a: R<Vec3>, b: R<Vec3>| V(*a + *b));
/// registry.register_ffi(TypeId::of::<Vec3>(), "add", |a: R<Vec3>, b: f32| V(*a + b));
/// ```
///
/// A panic of the function aborts the process, as it can't unwind through C.
///
/// # Safety
/// [`Self::ffi_function`] gives an `extern "C"` function with the parameters and result it says.
#[diagnostic::on_unimplemented(
    message = "this function has no FFI form",
    note = "FFI functions capture nothing, take up to 5 `FfiArg`s (`f32`, `f64`, `i32`, `u32`, `bool`, `R<T>`, `V<T>`) and return an `FfiReturn`"
)]
pub unsafe trait FfiFn<Marker>: Copy + 'static {
    /// The FFI form of the function, registered as `name`
    fn ffi_function(self, name: &'static str) -> FfiFunction;
}

macro_rules! impl_ffi_fn {
    ($($arg:ident),*) => {
        // Safety: the trampoline takes each parameter as `FfiArg::C`, then the out-pointer and the `failed` pointer, and
        // returns `FfiReturn::C`
        unsafe impl<Function, Return, $($arg),*> FfiFn<fn($($arg),*) -> Return> for Function
        where
            Function: Fn($($arg),*) -> Return + Copy + 'static,
            Return: FfiReturn,
            $($arg: FfiArg,)*
        {
            fn ffi_function(self, name: &'static str) -> FfiFunction {
                // the trampoline makes its own: the function is its type
                let _ = self;
                #[allow(non_snake_case, clippy::not_unsafe_ptr_arg_deref)]
                extern "C" fn trampoline<Function, Return, $($arg),*>(
                    $($arg: $arg::C,)*
                    out: *mut u8,
                    failed: *mut u8,
                ) -> Return::C
                where
                    Function: Fn($($arg),*) -> Return + Copy + 'static,
                    Return: FfiReturn,
                    $($arg: FfiArg,)*
                {
                    const { assert!(size_of::<Function>() == 0, "FFI functions capture nothing") };
                    // Safety: a type without bytes (asserted) which has a value (`ffi_function` was given one) and is
                    // `Copy`: this is that value
                    let function = unsafe { std::ptr::NonNull::<Function>::dangling().as_ptr().read() };
                    // Safety (callers): arguments as `FfiArg::from_c` says, the pointers as `FfiReturn::into_c` says
                    let output = function($(unsafe { $arg::from_c($arg) }),*);
                    unsafe { output.into_c(out, failed) }
                }
                FfiFunction {
                    name,
                    function: trampoline::<Function, Return, $($arg),*> as *const (),
                    params: vec![$($arg::kind()),*],
                    result: Return::result(),
                    fallible: Return::fallible(),
                }
            }
        }
    };
}

impl_ffi_fn!();
impl_ffi_fn!(A1);
impl_ffi_fn!(A1, A2);
impl_ffi_fn!(A1, A2, A3);
impl_ffi_fn!(A1, A2, A3, A4);
impl_ffi_fn!(A1, A2, A3, A4, A5);

impl NativeType {
    /// A native type with no fields yet.
    pub fn new<T: Reflect + Clone>() -> Self {
        Self {
            vtable: NativeVTable::new::<T>(),
            fields: Default::default(),
            methods: Default::default(),
            ffi: None,
        }
    }

    /// Declares the C layout of the values, see [`FfiLayout`].
    ///
    /// # Safety
    /// The layout is the type's: its size and alignment, and fields of its types (scalars, or native types with a layout)
    /// at their offsets. The type is `Copy`.
    pub unsafe fn with_ffi(mut self, layout: FfiLayout) -> Self {
        self.ffi = Some(layout);
        self
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
            direct: None,
        };
        self.fields.insert(name.into(), field);
        self
    }

    /// Adds the struct field `name` of `T` of type `F`, at `offset`. Reading it gives a copy, read-only if `F` is a native type
    /// (`t.translation.x = 5` is an error, `t.translation = v` writes the field). What [`NativeBuilder`]'s fields are made with.
    ///
    /// `ret` and `arg` convert the field's values.
    ///
    /// # Safety
    /// Every `T` has an `F` at `offset`.
    pub unsafe fn with_struct_field<T, F>(
        mut self,
        name: &'static str,
        offset: usize,
        ret: FieldRet<F>,
        arg: FieldArg<F>,
    ) -> Self
    where
        T: bevy_reflect::FromReflect + bevy_reflect::Typed + 'static,
        F: Reflect + Clone + 'static,
    {
        let info = || {
            crate::FunctionInfo::default()
                .with_name(name)
                .with_namespace(FunctionScope::OnType(TypeId::of::<T>()))
        };
        let native_return = NativeReturnType::default();
        let get = DynamicScriptFunction::from_thunk::<T>(move |_ctx, args, sink| {
            // the world is only looked up when needed (places, conversions)
            bevy_mod_scripting_world::CallScope::with_lazy(|scope| {
                let value = <crate::R<T> as crate::FromScript>::from_arg(args, 0, scope)?;
                let field = (&*value as *const T as *const u8).wrapping_add(offset);
                // Safety: an `F` of the T, see the function's contract
                let field = unsafe { &*(field as *const F) };
                // a copy: native values have no write-through, a native field is read-only so writes to it aren't lost
                sink.read_only();
                ret(field.clone(), sink, scope, &native_return)
            })
        })
        .with_info(info());
        let set = DynamicScriptFunction::from_thunk::<T>(move |_ctx, args, sink| {
            if args.len() < 2 {
                return Err(InteropError::argument_count_mismatch(2, args.len()));
            }
            bevy_mod_scripting_world::CallScope::with_lazy(|scope| {
                let mut target = <crate::M<T> as crate::FromScript>::from_arg(args, 0, scope)?;
                let value = arg(args, 1, scope)?;
                let target = &mut *target as *mut T as *mut u8;
                // Safety: an `F` inside the T (see the function's contract), borrowed exclusively
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
                direct: DirectKind::of::<F>().map(|kind| DirectField { offset, kind }),
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

    /// Starts declaring the native type `T`, field by field, see [`NativeBuilder`].
    pub fn of<T>() -> NativeBuilder<T>
    where
        T: bevy_reflect::FromReflect + bevy_reflect::Typed + Clone + 'static,
    {
        NativeBuilder {
            native: Self::new::<T>(),
            fields: Vec::new(),
            _type: std::marker::PhantomData,
        }
    }
}

/// Declares a [`NativeType`]: a type languages store themselves, whose fields are read and written with typed accessors.
///
/// Every method which adds something is `unsafe`: languages read and write the fields at the offsets given, unchecked.
///
/// ```rust,ignore
/// // Safety: the fields are at their offsets
/// unsafe {
///     NativeType::of::<Vec3>()
///         .field("x", offset_of!(Vec3, x), |v| &v.x)
///         .field("y", offset_of!(Vec3, y), |v| &v.y)
///         .field("z", offset_of!(Vec3, z), |v| &v.z)
/// }
/// .register(world);
/// ```
///
/// Each kind of field has its method, nothing is inferred:
///
/// | Method | The field is | Scripts get |
/// |---|---|---|
/// | [`field`](Self::field) | converted like an argument: a number, a `bool`, a `String` | the value, read and written in place where the language can |
/// | [`value_field`](Self::value_field) | a reflected type passed as `V<F>`, i.e. another native type | a read-only copy, written whole |
/// | [`deref_field`](Self::deref_field) | like `field`, reached through `Deref` (glam's SIMD types) | the same as `field` |
///
/// Native types are values, with no write-through: a field of a native type is read as a read-only copy and written whole
/// (`t.translation.x = 5` is an error, `t.translation = v` writes the field), `clone` (registered with the type) gives a
/// copy scripts can change. Components and resources of native types are native handles ([`NativePlace`]):
/// `world.get_component(e, Transform)` returns one, which reads and writes the world.
///
/// [`ffi`](Self::ffi) also lets languages with an FFI (LuaJIT) store the values as plain C structs.
pub struct NativeBuilder<T> {
    native: NativeType,
    /// The fields declared so far, for [`Self::ffi`]
    fields: Vec<BuilderField>,
    _type: std::marker::PhantomData<fn(T)>,
}

/// A field of a [`NativeBuilder`].
struct BuilderField {
    name: &'static str,
    ty: FfiType,
    offset: usize,
    size: usize,
    align: usize,
}

impl<T> NativeBuilder<T>
where
    T: bevy_reflect::FromReflect + bevy_reflect::Typed + Clone + 'static,
{
    fn add<F>(
        mut self,
        name: &'static str,
        offset: usize,
        ret: FieldRet<F>,
        arg: FieldArg<F>,
    ) -> Self
    where
        F: Reflect + Clone + 'static,
    {
        self.fields.push(BuilderField {
            name,
            ty: match c_scalar::<F>() {
                Some(c) => FfiType::Scalar(c),
                None => FfiType::Value(TypeId::of::<F>()),
            },
            offset,
            size: size_of::<F>(),
            align: align_of::<F>(),
        });
        // Safety: the caller's, see the methods adding fields
        self.native = unsafe {
            self.native
                .with_struct_field::<T, F>(name, offset, ret, arg)
        };
        self
    }

    /// Adds the field `name` at `offset`, whose values are converted like arguments and results: numbers, `bool`s,
    /// `String`s. `field` names it (`|v| &v.x`), which gives its type.
    ///
    /// # Safety
    /// Every `T` has an `F` at `offset` (i.e. `offset_of!(T, x)` for the field `field` gives).
    pub unsafe fn field<F>(self, name: &'static str, offset: usize, field: fn(&T) -> &F) -> Self
    where
        F: Reflect + Clone + crate::IntoScript + for<'s> crate::FromScript<This<'s> = F>,
    {
        let _ = field;
        self.add::<F>(
            name,
            offset,
            |value, sink, scope, native| value.into_ret(sink, scope, native),
            |args, index, scope| F::from_arg(args, index, scope),
        )
    }

    /// Adds the field `name` at `offset`, whose values are passed as `V<F>`: reflected types, i.e. other native types
    /// (`Transform::translation`). Scripts read it as a read-only copy and write it whole.
    ///
    /// # Safety
    /// Every `T` has an `F` at `offset` (i.e. `offset_of!(T, x)` for the field `field` gives).
    pub unsafe fn value_field<F>(
        self,
        name: &'static str,
        offset: usize,
        field: fn(&T) -> &F,
    ) -> Self
    where
        F: Reflect + Clone,
        crate::V<F>: crate::IntoScript + for<'s> crate::FromScript<This<'s> = crate::V<F>>,
    {
        let _ = field;
        self.add::<F>(
            name,
            offset,
            |value, sink, scope, native| {
                crate::IntoScript::into_ret(crate::V(value), sink, scope, native)
            },
            |args, index, scope| {
                <crate::V<F> as crate::FromScript>::from_arg(args, index, scope).map(|v| v.0)
            },
        )
    }

    /// Adds the field `name` reached through `T`'s `Deref`, like [`Self::field`]: its offset is measured once, in the
    /// default value, so it's read and written in place like any field.
    ///
    /// # Safety
    /// `T`'s `Deref` is to the value itself, the field at the same offset in every value: glam's SIMD types cast `self`
    /// to a `repr(C)` view of their components. Never when the target depends on the value (an enum holding it inline or
    /// boxed) or is elsewhere (a `Box`, an `Arc`).
    ///
    /// # Panics
    /// If the field isn't inside the default value, or wouldn't be aligned in every value.
    pub unsafe fn deref_field<F>(self, name: &'static str, field: fn(&T) -> &F) -> Self
    where
        T: Default,
        F: Reflect + Clone + crate::IntoScript + for<'s> crate::FromScript<This<'s> = F>,
    {
        // Safety: the caller's
        unsafe {
            let offset = deref_field_offset(field);
            self.field(name, offset, field)
        }
    }

    /// Declares the C layout of the values, the fields declared so far in order (see [`FfiLayout`]): languages with an
    /// FFI (LuaJIT) then store them as plain C structs, which scripts can write byte by byte at any time. Fields which
    /// aren't `f32`, `f64`, `i32`, `u32` or a native type with such fields keep the values as they were.
    ///
    /// Check the layout at compile time too, with [`crate::assert_ffi_layout`].
    ///
    /// # Safety
    /// Every bit pattern of the fields is a valid `T`: there's no invariant between or within them (a vector, not a
    /// normalised direction), and no field left out.
    ///
    /// # Panics
    /// If `T` isn't laid out like the C struct of the fields: another size or alignment, or a field somewhere else.
    pub unsafe fn ffi(mut self) -> Self
    where
        T: Copy,
    {
        let sizes = self
            .fields
            .iter()
            .map(|f| (f.size, f.align))
            .collect::<Vec<_>>();
        let mut offsets = vec![0; sizes.len()];
        let (size, align) = c_layout(&sizes, align_of::<T>(), &mut offsets);
        let type_name = std::any::type_name::<T>();
        assert!(
            size == size_of::<T>()
                && align == align_of::<T>()
                && self.fields.iter().map(|f| f.offset).eq(offsets),
            "{type_name} isn't laid out like the C struct of its fields, it can't be declared with `ffi`",
        );
        self.native.ffi = Some(FfiLayout {
            name: format!(
                "bms_{}",
                type_name.replace(|c: char| !c.is_ascii_alphanumeric(), "_")
            )
            .into(),
            size,
            align,
            fields: self
                .fields
                .iter()
                .map(|f| FfiField {
                    name: f.name,
                    ty: f.ty,
                    offset: f.offset,
                })
                .collect(),
        });
        self
    }

    /// The native type declared.
    pub fn build(self) -> NativeType {
        self.native
    }

    /// Registers the native type, and `clone` on it (see [`register_clone`]): values of it made by typed functions are
    /// then stored natively by the languages.
    pub fn register(self, world: &mut World) {
        self.native.register(world);
        register_clone::<T>(world);
    }
}

/// The native types of a typed function's results, looked up in the registry on their first return and kept by its thunk,
/// so returning a native value takes no registry lock. A few types are kept, i.e. the components `get_component` returns.
///
/// Kept weakly: the native type holds its functions, which hold their thunks. Kept for good: native types and their
/// functions are registered before scripts run.
#[derive(Default)]
pub struct NativeReturnType(parking_lot::RwLock<Vec<CachedNativeType>>);

/// An entry of [`NativeReturnType`].
struct CachedNativeType {
    type_id: TypeId,
    native: Option<std::sync::Weak<NativeType>>,
}

impl NativeReturnType {
    /// How many types are kept, others are looked up on every return.
    const CAPACITY: usize = 8;

    /// The native type of `type_id`, if it is one. The scope's world is only needed when it isn't cached.
    pub fn get(
        &self,
        scope: &bevy_mod_scripting_world::CallScope,
        type_id: TypeId,
    ) -> Option<std::sync::Arc<NativeType>> {
        use crate::WorldExtensions;
        // whether there's room for the type, if it needs looking up
        let cacheable = {
            let cache = self.0.read();
            match cache.iter().find(|c| c.type_id == type_id) {
                Some(cached) => {
                    // not native, the common case: no lookup
                    let native = cached.native.as_ref()?;
                    if let Some(native) = native.upgrade() {
                        return Some(native);
                    }
                    true
                }
                None => cache.len() < Self::CAPACITY,
            }
        };
        let native = scope
            .world()
            .ok()?
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

/// The offset of the field `F` of `T` reached through `Deref`, for [`NativeBuilder::deref_field`]: measured in
/// the default value (glam's SIMD vectors deref to themselves), checked to be inside it and aligned in every value.
///
/// # Safety
/// `project` gives a field of the `T` it's given, and `T`'s `Deref` puts it at the same offset in every
/// value, inside the value: a `Deref` to the value itself (i.e. casting `self` to a `repr(C)` view of it). Not when the
/// target depends on the value (an enum with an inline or a boxed target) or is elsewhere (a `Box`).
///
/// # Panics
/// If the field isn't inside the default value, or wouldn't be aligned in every value.
unsafe fn deref_field_offset<T: Default, F>(project: fn(&T) -> &F) -> usize {
    let value = T::default();
    let base = &value as *const T;
    let offset = field_offset::<T, F>(base as *const u8, project(&value) as *const F as *const u8)
        .filter(|offset| {
            offset.is_multiple_of(align_of::<F>())
                && align_of::<T>().is_multiple_of(align_of::<F>())
        });
    assert!(
        offset.is_some(),
        "`Deref` of {} doesn't put a field inside the value, it can't be declared with `deref_field`",
        std::any::type_name::<T>()
    );
    offset.unwrap_or_default()
}

/// Converts the value of a field read by a script, see [`FieldConv`].
pub type FieldRet<F> = for<'s, 'w> fn(
    F,
    &mut dyn ReturnSink,
    &'s bevy_mod_scripting_world::CallScope<'w>,
    &NativeReturnType,
) -> Result<(), InteropError>;

/// Converts the value a script writes to a field, see [`FieldConv`].
pub type FieldArg<F> = for<'s, 'w> fn(
    &dyn ArgSource,
    usize,
    &'s bevy_mod_scripting_world::CallScope<'w>,
) -> Result<F, InteropError>;

/// Registers `clone` on the native type `T`, unless it has one: a copy scripts can change, i.e. of the read-only copy
/// reading a field gives (`local p = t.translation:clone(); p.x = 5; t.translation = p`).
pub fn register_clone<T>(world: &mut World)
where
    T: bevy_reflect::FromReflect + bevy_reflect::Typed + 'static,
{
    let mut registry = world
        .get_resource_or_init::<AppScriptFunctionRegistry>()
        .0
        .clone();
    let mut registry = registry.write();
    if !registry.contains(TypeId::of::<T>(), "clone") {
        registry.register(TypeId::of::<T>(), "clone", |value: crate::V<T>| value);
    }
}

/// Checks at compile time that `$ty` is laid out like the C struct of the given fields, as a type declared with
/// [`NativeBuilder::ffi`] must be: it and its fields are `Copy`, and it has the size and alignment of the C struct
/// (aligned like the type). The offsets of the fields are checked when the type is declared, as fields reached through
/// `Deref` have none at compile time.
///
/// ```rust
/// #[derive(Clone, Copy)]
/// struct Health {
///     current: f32,
///     max: f32,
/// }
/// bevy_mod_scripting_bindings::assert_ffi_layout!(Health { current: f32, max: f32 });
/// ```
///
/// ```rust,compile_fail
/// #[derive(Clone, Copy)]
/// struct Health {
///     current: f32,
///     max: f32,
///     regenerates: bool,
/// }
/// // a field is missing: the C struct of these is smaller
/// bevy_mod_scripting_bindings::assert_ffi_layout!(Health { current: f32, max: f32 });
/// ```
#[macro_export]
macro_rules! assert_ffi_layout {
    ($ty:ty { $($field:ident : $field_ty:ty),* $(,)? }) => {
        const _: fn() = || {
            fn copy<T: Copy>() {}
            copy::<$ty>();
            $(copy::<$field_ty>();)*
        };
        const _: () = {
            let (size, align) = $crate::native::c_layout(
                &[$((::core::mem::size_of::<$field_ty>(), ::core::mem::align_of::<$field_ty>())),*],
                ::core::mem::align_of::<$ty>(),
                &mut [],
            );
            assert!(size == ::core::mem::size_of::<$ty>(), "the C struct of the fields has another size");
            assert!(align == ::core::mem::align_of::<$ty>(), "the C struct of the fields has another alignment");
        };
    };
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
        read_only: bool,
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
            if let Some(native) = &self.native {
                // Safety: initialised storage
                unsafe { NativeStorage::header(native.base()) }
                    .read_only
                    .set(self.read_only && !vtable.place);
            }
            self.native.is_some()
        }
        fn script_value(&mut self, value: ScriptValue) -> Result<(), InteropError> {
            self.value = Some(value);
            Ok(())
        }
        fn read_only(&mut self) {
            self.read_only = true;
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

    /// Calls get copies of the values languages store, so one value can be passed as `&mut` and `&` at once; a
    /// read-only copy can't be changed.
    #[test]
    fn calls_copy_values_and_read_only_ones_stay() {
        let both = (|_a: M<Tracked>, _b: R<Tracked>| ()).into_dynamic_script_function();
        let shared = (|_a: R<Tracked>, _b: R<Tracked>| ()).into_dynamic_script_function();
        with_native_world(|| {
            let native = tracked(1.0);
            let natives = [&native, &native];
            let args = || Args {
                natives: &natives,
                numbers: &[],
            };
            let (result, _) = call(&both, args());
            assert!(result.is_ok(), "&mut and & of one value are two copies");
            // Safety: initialised storage
            unsafe { NativeStorage::header(native.base()) }
                .read_only
                .set(true);
            let (result, _) = call(&both, args());
            assert!(result.is_err(), "&mut of a read-only copy");
            let (result, _) = call(&shared, args());
            assert!(result.is_ok(), "& of a read-only copy");
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

    /// The whole chain a language goes through: registering a native type with `NativeBuilder`, making values with typed
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
        // Safety: the fields are at their offsets
        unsafe {
            NativeType::of::<Labelled>()
                .field("value", std::mem::offset_of!(Labelled, value), |l| &l.value)
                .field("label", std::mem::offset_of!(Labelled, label), |l| &l.label)
        }
        .register(&mut world);
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

            // fields, through the accessors `NativeBuilder` registered
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

    /// A component of a native type is a handle to it (a place): reads and writes go to the world, conflicting accesses
    /// error, places follow the component (archetype moves, removal, despawning), resources work alike, and outside typed
    /// calls a place is a reference to where it is. Its native fields are read as read-only copies and written whole.
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
        // Safety: the fields are at their offsets
        unsafe {
            NativeType::of::<Outer>()
                .value_field("inner", std::mem::offset_of!(Outer, inner), |o| &o.inner)
                .field("n", std::mem::offset_of!(Outer, n), |o| &o.n)
                .register(&mut world);
            NativeType::of::<Inner>()
                .field("x", std::mem::offset_of!(Inner, x), |i| &i.x)
                .register(&mut world);
        }
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
            world
                .with_call_scope(|scope| {
                    crate::IntoScript::into_ret(
                        reference,
                        &mut ret,
                        scope,
                        &NativeReturnType::default(),
                    )
                })
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
        let read_outer_x = (|v: V<Outer>| v.inner.x).into_dynamic_script_function();

        let handle = with_guard(&mut world, |world| {
            let outer = natives(&world, TypeId::of::<Outer>());
            let inner = natives(&world, TypeId::of::<Inner>());

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
            let copy = got.native.expect("a native field of a place is a value");
            // Safety: initialised storage
            assert!(unsafe { NativeStorage::place(copy.base()) }.is_none());

            // the copy is read-only: a write to it would be lost
            let set_x = |target: &NativeBox| {
                call(
                    inner.fields["x"].set.as_ref().unwrap(),
                    Args {
                        natives: &[target],
                        numbers: &[5.0],
                    },
                )
                .0
            };
            assert!(set_x(&copy).is_err(), "a field of a read-only copy");
            // the whole field is written instead, with a value which isn't a copy of a field
            let (result, got) = call(
                &(|x: f32| V(Inner { x })).into_dynamic_script_function(),
                Args {
                    natives: &[],
                    numbers: &[4.0],
                },
            );
            result.unwrap();
            let value = got.native.expect("a native value");
            set_x(&value).unwrap();
            let (result, _) = call(
                outer.fields["inner"].set.as_ref().unwrap(),
                Args {
                    natives: &[&handle, &value],
                    numbers: &[],
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
            let (result, got) = get(&read_x, &copy);
            result.unwrap();
            assert_eq!(
                got.value,
                Some(ScriptValue::Float(1.0)),
                "the copy is what the field was"
            );

            // the component written while it's read
            let write_read = (|_a: M<Outer>, _b: R<Outer>| ()).into_dynamic_script_function();
            let (result, _) = call(
                &write_read,
                Args {
                    natives: &[&handle, &handle],
                    numbers: &[],
                },
            );
            assert!(result.is_err(), "conflicting accesses of a component error");

            // outside typed calls, a place is a reference to where it is
            let ScriptValue::Reference(reference) =
                (unsafe { NativeStorage::to_script_value(handle.base()) }).unwrap()
            else {
                panic!("a place is a reference")
            };
            assert_eq!(
                reference
                    .with_reflect(world.clone(), |r| r.try_downcast_ref::<Outer>().cloned())
                    .unwrap()
                    .map(|outer| outer.inner),
                Some(Inner { x: 5.0 })
            );
            handle
        });
        assert_eq!(
            world.get::<Outer>(entity).unwrap().inner.x,
            5.0,
            "written to the world"
        );

        let x_of = |world: &mut World| {
            with_guard(world, |_| {
                let (result, got) = get(&read_outer_x, &handle);
                result.map(|()| got.value)
            })
        };
        // the entity moves to another archetype
        world
            .entity_mut(entity)
            .insert(bevy_ecs::name::Name::default());
        assert_eq!(
            x_of(&mut world).unwrap(),
            Some(ScriptValue::Float(5.0)),
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
    }

    /// A `Deref` to a field of the value itself, like glam's SIMD vectors, after a field of its own.
    #[derive(Reflect, Clone, Debug, Default)]
    struct Small {
        pad: u32,
        inner: Inner,
    }

    impl std::ops::Deref for Small {
        type Target = Inner;
        fn deref(&self) -> &Inner {
            &self.inner
        }
    }

    /// A `Deref` to elsewhere, which can't be declared with `deref`.
    #[derive(Reflect, Clone, Debug, Default)]
    struct Boxed(#[reflect(ignore)] Box<Inner>);

    impl std::ops::Deref for Boxed {
        type Target = Inner;
        fn deref(&self) -> &Inner {
            &self.0
        }
    }

    /// Fields reached through `Deref` (i.e. glam's SIMD vectors) are at the offset measured in the default value: read and
    /// written in place like any field. A `Deref` to elsewhere is refused when it's declared.
    #[test]
    fn fields_through_deref() {
        let mut world = World::default();
        // Safety: `Small` derefs to a field of itself
        unsafe { NativeType::of::<Small>().deref_field("x", |s| &s.x) }.register(&mut world);
        let make = (|| {
            V(Small {
                pad: 7,
                inner: Inner { x: 0.0 },
            })
        })
        .into_dynamic_script_function();
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        WorldAccessGuard::with_static_guard(&mut world, cache, |world| {
            let field = {
                let registry = world.script_function_registry();
                let registry = registry.read();
                registry.native_type(TypeId::of::<Small>()).unwrap().fields["x"].clone()
            };
            let offset = std::mem::offset_of!(Small, inner) + std::mem::offset_of!(Inner, x);
            assert_eq!(
                field.direct.map(|d| d.offset),
                Some(offset),
                "in place, at the measured offset"
            );
            let (result, value) = call(
                &make,
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
            result.unwrap();
            let (result, got) = call(
                &field.get,
                Args {
                    natives: &[&value],
                    numbers: &[],
                },
            );
            result.unwrap();
            assert_eq!(got.value, Some(ScriptValue::Float(4.0)));
        });
        let boxed = std::panic::catch_unwind(|| {
            // Safety: not upheld, which is caught when it's declared
            unsafe { NativeType::of::<Boxed>().deref_field("x", |b| &b.x) };
        });
        assert!(boxed.is_err(), "a `Deref` to elsewhere can't be declared");
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
