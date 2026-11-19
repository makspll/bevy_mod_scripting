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
/// size and alignment of the type's vtable layout, which it then keeps as the native value (dropping it with the vtable). It
/// returns false without calling `write` otherwise.
pub unsafe trait ReturnSink {
    /// Returns nothing.
    fn unit(&mut self);
    /// Returns a number.
    fn number(&mut self, n: f64);
    /// Returns an integer.
    fn integer(&mut self, i: i64);
    /// Returns a boolean.
    fn boolean(&mut self, b: bool);
    /// Returns a new native value of type `native`: makes [`NativeStorage`] for it and initialises it with `write`.
    ///
    /// Returns false, without calling `write`, if the language has no native form for the type.
    ///
    /// # Safety
    /// `write` writes a valid value of the type's vtable to the pointer it's given.
    unsafe fn native(
        &mut self,
        native: &std::sync::Arc<NativeType>,
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
pub struct NativeVTable {
    /// The type of the values
    pub type_id: TypeId,
    /// The name of the type, for errors
    pub type_name: &'static str,
    /// The layout of a value
    pub layout: Layout,
    /// Drops a value in place, `None` if the type needs no drop
    pub drop: Option<unsafe fn(*mut u8)>,
    clone_into: unsafe fn(*const u8, *mut u8),
    to_reference: unsafe fn(*const u8) -> ReflectReference,
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
            Box::leak(Box::new(Self {
                type_id: TypeId::of::<T>(),
                type_name: std::any::type_name::<T>(),
                layout: Layout::new::<T>(),
                drop: std::mem::needs_drop::<T>().then_some(drop_value::<T> as unsafe fn(*mut u8)),
                clone_into: clone_into::<T>,
                to_reference: to_reference::<T>,
            }))
        })
    }
}

impl std::fmt::Debug for NativeVTable {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.type_name)
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
        // one allocation holding the copy
        Ok(ScriptValue::Reference(unsafe {
            (header.vtable.to_reference)(Self::value(base))
        }))
    }

    /// Copies the value of initialised storage into memory for a value of its type.
    ///
    /// # Safety
    /// `base` is the base of initialised storage, `dst` points to memory for a value of its type.
    pub unsafe fn clone_value(base: *mut u8, dst: *mut u8) {
        // Safety: see the function's contract
        unsafe { (Self::header(base).vtable.clone_into)(Self::value(base), dst) }
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
}

impl NativeType {
    /// A native type with no fields yet.
    pub fn new<T: Reflect + Clone>() -> Self {
        Self {
            vtable: NativeVTable::new::<T>(),
            fields: Default::default(),
            methods: Default::default(),
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
        };
        self.fields.insert(name.into(), field);
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

/// The native type of a typed function's result, looked up in the registry on its first return and kept by its thunk, so
/// returning a native value takes no lock.
///
/// Kept weakly: the native type holds its functions, which hold their thunks. If the registry replaced the type since
/// (functions registered on it after the first return), it's looked up again.
#[derive(Default)]
pub struct NativeReturnType(std::sync::OnceLock<(TypeId, Option<std::sync::Weak<NativeType>>)>);

impl NativeReturnType {
    /// The native type of `type_id`, if it is one: cached for the first type asked for, looked up for others (i.e. the
    /// other side of a `Union`).
    pub fn get(&self, world: &WorldGuard, type_id: TypeId) -> Option<std::sync::Arc<NativeType>> {
        use crate::WorldExtensions;
        let look_up = || {
            world
                .script_function_registry()
                .read()
                .native_type(type_id)
                .cloned()
        };
        let (cached_id, cached) = self
            .0
            .get_or_init(|| (type_id, look_up().as_ref().map(std::sync::Arc::downgrade)));
        if *cached_id != type_id {
            return look_up();
        }
        // not native, the common case: no lookup
        cached.as_ref()?.upgrade().or_else(look_up)
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

/// Declares a [`NativeType`] with typed accessors for the given fields, and registers it.
///
/// ```rust,ignore
/// script_native!(world, bevy_math::Vec3 { x: f32, y: f32, z: f32 });
/// ```
#[macro_export]
macro_rules! script_native {
    ($world:expr, $ty:ty { $($field:ident : $field_ty:ty),* $(,)? }) => {{
        let native = $crate::native::NativeType::new::<$ty>();
        $(
            let native = {
                fn get(value: $crate::R<$ty>) -> $field_ty {
                    ::core::clone::Clone::clone(&value.$field)
                }
                fn set(mut value: $crate::M<$ty>, field: $field_ty) {
                    value.$field = field;
                }
                native.with_field(stringify!($field), get, set)
            };
        )*
        native.register($world);
    }};
}

#[cfg(test)]
mod test {
    use std::sync::atomic::{AtomicUsize, Ordering};

    use bevy_mod_scripting_asset::Language;
    use bevy_mod_scripting_world::WorldAccessGuard;

    use super::*;
    use bevy_mod_scripting_world::ThreadWorldContainer;

    use crate::{CurrentScriptAttachment, FunctionCallContext, M, R, V, WorldExtensions};

    static DROPS: AtomicUsize = AtomicUsize::new(0);

    #[derive(Reflect, Clone, Debug, PartialEq)]
    struct Tracked {
        value: f32,
    }

    impl Drop for Tracked {
        fn drop(&mut self) {
            DROPS.fetch_add(1, Ordering::SeqCst);
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
            native: &std::sync::Arc<NativeType>,
            write: &mut dyn FnMut(*mut u8),
        ) -> bool {
            // Safety: `write` writes a value of the vtable's type, see the method's contract
            self.native = unsafe { NativeBox::new(native.vtable, write) }.ok();
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
            let before = DROPS.load(Ordering::SeqCst);
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
            drops = DROPS.load(Ordering::SeqCst) - before;
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
