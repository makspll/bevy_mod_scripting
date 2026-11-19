//! Typed calls and native values: the fast path next to [`ScriptValue`] conversions.
//!
//! - A function registered as [`Native`] also gets a typed entry (see [`DynamicScriptFunction::call_native`]), which reads its
//!   arguments from an [`ArgSource`] and writes its result to a [`ReturnSink`], both implemented by each language over its own
//!   values, without building [`ScriptValue`]s. Arguments the language can't give in their native form are converted the usual
//!   way ([`FromScript::from_arg`](crate::FromScript::from_arg)'s slow arm), so a typed call accepts everything the dynamic one does.
//! - A [`NativeType`] is stored by the language itself, i.e. in a Lua userdata, as `[NativeHeader][value]`, see [`NativeStorage`].
//!   Its fields are read and written through typed accessor functions, see [`crate::script_native`].

use std::{alloc::Layout, any::TypeId, borrow::Cow, ptr::NonNull};

use bevy_ecs::world::World;
use bevy_mod_scripting_world::{BorrowFlag, WorldGuard};
use bevy_platform::collections::HashMap;
use bevy_reflect::Reflect;

use crate::{
    AppScriptFunctionRegistry, DynamicScriptFunction, FunctionScope, GetFunctionInfo, InteropError,
    ReflectReference, ScriptFunction, ScriptValue,
    function::script_function::{NativeScriptFunction, documented_function},
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
pub trait ReturnSink {
    /// Returns nothing.
    fn unit(&mut self);
    /// Returns a number.
    fn number(&mut self, n: f64);
    /// Returns an integer.
    fn integer(&mut self, i: i64);
    /// Returns a boolean.
    fn boolean(&mut self, b: bool);
    /// Returns a new native value: makes [`NativeStorage`] for it and initialises it with `write`.
    ///
    /// Returns false, without calling `write`, if the language has no native form for the type.
    fn native(&mut self, vtable: &'static NativeVTable, write: &mut dyn FnMut(*mut u8)) -> bool;
    /// Returns any value, the slow path.
    fn script_value(&mut self, value: ScriptValue) -> Result<(), InteropError>;
}

/// The typed entry of a function, see [`DynamicScriptFunction::call_native`].
pub type NativeThunk = dyn Fn(crate::FunctionCallContext, &dyn ArgSource, &mut dyn ReturnSink) -> Result<(), InteropError>
    + Send
    + Sync;

/// Registers a function with a typed entry next to its dynamic one, see the [module docs](self).
///
/// ```rust,ignore
/// NamespaceBuilder::<Vec3>::new(world).register("length", Native(|v: R<Vec3>| v.length()));
/// ```
pub struct Native<F>(pub F);

impl<'env, M, F> ScriptFunction<'env, M> for Native<F>
where
    F: ScriptFunction<'env, M> + NativeScriptFunction<'env, M> + Clone,
{
    fn into_dynamic_script_function(self) -> DynamicScriptFunction {
        self.0
            .clone()
            .into_dynamic_script_function()
            .with_native(self.0.into_native_thunk())
    }
}

impl<M, F: GetFunctionInfo<M>> GetFunctionInfo<M> for Native<F> {
    fn function_info(name: Cow<'static, str>, namespace: FunctionScope) -> crate::FunctionInfo {
        F::function_info(name, namespace)
    }
}

impl<M, F: crate::GetFunctionTypeDependencies<M>> crate::GetFunctionTypeDependencies<M>
    for Native<F>
{
    fn register_type_dependencies(registry: &mut bevy_reflect::TypeRegistry) {
        F::register_type_dependencies(registry)
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
    to_reflect: unsafe fn(*const u8) -> Box<dyn Reflect>,
}

impl NativeVTable {
    /// The vtable of `T`, made once per registration.
    fn new<T: Reflect + Clone>() -> &'static Self {
        unsafe fn drop_value<T>(p: *mut u8) {
            // Safety: called with a pointer to an initialised T, which isn't used afterwards
            unsafe { std::ptr::drop_in_place(p as *mut T) }
        }
        unsafe fn clone_into<T: Clone>(src: *const u8, dst: *mut u8) {
            // Safety: called with a pointer to an initialised T, and one to memory for a T
            unsafe { (dst as *mut T).write((*(src as *const T)).clone()) }
        }
        unsafe fn to_reflect<T: Reflect + Clone>(src: *const u8) -> Box<dyn Reflect> {
            // Safety: called with a pointer to an initialised T
            Box::new(unsafe { (*(src as *const T)).clone() })
        }
        Box::leak(Box::new(Self {
            type_id: TypeId::of::<T>(),
            type_name: std::any::type_name::<T>(),
            layout: Layout::new::<T>(),
            drop: std::mem::needs_drop::<T>().then_some(drop_value::<T> as unsafe fn(*mut u8)),
            clone_into: clone_into::<T>,
            to_reflect: to_reflect::<T>,
        }))
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
        // Safety (of the arithmetic): the storage has room for the header, the padding and the value, see `size`
        let start = base.wrapping_add(size_of::<NativeHeader>());
        start.wrapping_add(start.align_offset(vtable.layout.align()))
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
        if header.borrow.is_borrowed() {
            return Err(InteropError::string(format!(
                "a {} is borrowed by a call",
                header.vtable.type_name
            )));
        }
        let copy = unsafe { (header.vtable.to_reflect)(Self::value(base)) };
        Ok(ScriptValue::Reference(
            ReflectReference::new_allocated_boxed(copy),
        ))
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
    pub fn new(
        vtable: &'static NativeVTable,
        write: &mut dyn FnMut(*mut u8),
    ) -> Result<Self, InteropError> {
        let layout = Layout::from_size_align(NativeStorage::size(vtable), NativeStorage::ALIGN)
            .map_err(|e| InteropError::string(e.to_string()))?;
        // Safety: the layout isn't zero sized, it has a header
        let base = NonNull::new(unsafe { std::alloc::alloc(layout) })
            .ok_or_else(|| InteropError::str("out of memory"))?;
        // Safety: freshly allocated with the size and alignment storage needs
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

// Safety: native values are `Reflect`, so `Send + Sync`, the header's borrow flag is atomic
unsafe impl Send for NativeBox {}
// Safety: see above
unsafe impl Sync for NativeBox {}

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
/// Its methods are the functions registered on the type; those which are [`Native`] are called without conversions.
#[derive(Clone)]
pub struct NativeType {
    /// How to store, copy and drop the values
    pub vtable: &'static NativeVTable,
    /// The fields, by name
    pub fields: HashMap<Cow<'static, str>, NativeField>,
}

impl NativeType {
    /// A native type with no fields yet.
    pub fn new<T: Reflect + Clone>() -> Self {
        Self {
            vtable: NativeVTable::new::<T>(),
            fields: Default::default(),
        }
    }

    /// Adds a field with a getter and a setter, both typed.
    pub fn with_field<'env, G, GM, S, SM>(mut self, name: &'static str, get: G, set: S) -> Self
    where
        G: ScriptFunction<'env, GM> + NativeScriptFunction<'env, GM> + GetFunctionInfo<GM> + Clone,
        S: ScriptFunction<'env, SM> + NativeScriptFunction<'env, SM> + GetFunctionInfo<SM> + Clone,
    {
        let scope = FunctionScope::OnType(self.vtable.type_id);
        let field = NativeField {
            get: documented_function(scope, name.into(), Native(get), None::<&'static str>, None),
            set: Some(documented_function(
                scope,
                name.into(),
                Native(set),
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

    impl ReturnSink for Return {
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
        fn native(
            &mut self,
            vtable: &'static NativeVTable,
            write: &mut dyn FnMut(*mut u8),
        ) -> bool {
            self.native = NativeBox::new(vtable, write).ok();
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
        NativeBox::new(vtable, &mut |slot| {
            if let Some(value) = value.take() {
                // Safety: the slot is for a Tracked
                unsafe { (slot as *mut Tracked).write(value) }
            }
        })
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
        let result = function
            .call_native(&args, &mut ret, FunctionCallContext::new(Language::Lua))
            .expect("the function is native");
        (result, ret)
    }

    #[test]
    fn typed_calls_read_and_return_native_values() {
        let scale = Native(|v: R<Tracked>, by: f32| {
            V(Tracked {
                value: v.value * by,
            })
        })
        .into_dynamic_script_function();
        let set = Native(|mut v: M<Tracked>, to: f32| v.value = to).into_dynamic_script_function();
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
        let both = Native(|_a: M<Tracked>, _b: R<Tracked>| ()).into_dynamic_script_function();
        let shared = Native(|_a: R<Tracked>, _b: R<Tracked>| ()).into_dynamic_script_function();
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
        let copy = Native(|v: V<Tracked>| v).into_dynamic_script_function();
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
}
