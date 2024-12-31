use super::{from::FromScript, into::IntoScript};
use crate::{
    bindings::{
        function::from::{Mut, Ref, Val},
        ReflectReference,
    },
    error::InteropError,
    prelude::{ScriptValue, WorldCallbackAccess},
};
use bevy::{
    prelude::{AppFunctionRegistry, IntoFunction, Reflect, Resource, World},
    reflect::{
        func::{args::GetOwnership, DynamicFunction, FunctionError, FunctionInfo, TypedFunction},
        FromReflect, GetTypeRegistration, PartialReflect, TypePath, TypeRegistration, TypeRegistry,
        Typed,
    },
};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::borrow::Cow;
use std::collections::HashMap;
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[diagnostic::on_unimplemented(
    message = "Only functions with all arguments impplementing FromScript and return values supporting IntoScript are supported. Registering functions also requires they implement GetInnerTypeDependencies",
    note = "If you're trying to return a non-primitive type, you might need to use Val<T> Ref<T> or Mut<T> wrappers"
)]
pub trait ScriptFunction<'env, Marker> {
    fn into_dynamic_script_function(self) -> DynamicScriptFunction;
}

#[diagnostic::on_unimplemented(
    message = "Only functions with all arguments impplementing FromScript and return values supporting IntoScript are supported. Registering functions also requires they implement GetInnerTypeDependencies",
    note = "If you're trying to return a non-primitive type, you might need to use Val<T> Ref<T> or Mut<T> wrappers"
)]
pub trait ScriptFunctionMut<'env, Marker> {
    fn into_dynamic_script_function_mut(self) -> DynamicScriptFunctionMut;
}

/// Functionally identical to [`GetTypeRegistration`] but without the 'static bound
pub trait GetInnerTypeDependencies {
    fn register_type_dependencies(registry: &mut TypeRegistry);
}

#[macro_export]
macro_rules! no_type_dependencies {
    ($($path:path),*) => {
        $(
            impl $crate::bindings::function::script_function::GetInnerTypeDependencies for $path {
                fn register_type_dependencies(_registry: &mut bevy::reflect::TypeRegistry) {}
            }
        )*
    };
}

#[macro_export]
macro_rules! self_type_dependency_only {
    ($($path:ty),*) => {
        $(
            impl $crate::bindings::function::script_function::GetInnerTypeDependencies for $path {
                fn register_type_dependencies(registry: &mut bevy::reflect::TypeRegistry) {
                    registry.register::<$path>();
                }
            }
        )*
    };
}

macro_rules! recursive_type_dependencies {
    ($( ($path:ty where $($bound:ident : $($bound_val:path);*),* $(,,const $const:ident : $const_ty:ty)? $(=> with $self_:ident)?) ),* )  => {
        $(
            impl<$($bound : $($bound_val +)*),* , $(const $const : $const_ty )?> GetInnerTypeDependencies for $path {
                fn register_type_dependencies(registry: &mut TypeRegistry) {
                    $(
                        registry.register::<$bound>();
                    )*
                    $(
                        registry.register::<$self_>();
                    )?
                }
            }
        )*
    };
}

macro_rules! register_tuple_dependencies {
    ($($ty:ident),*) => {
        impl<$($ty: GetTypeRegistration + Typed),*> GetInnerTypeDependencies for ($($ty,)*) {
            fn register_type_dependencies(registry: &mut TypeRegistry) {
                $(
                    registry.register::<$ty>();
                )*
            }
        }
    };
}

no_type_dependencies!(ReflectReference, InteropError);
self_type_dependency_only!(WorldCallbackAccess, CallerContext);

recursive_type_dependencies!(
    (Val<T> where T: GetTypeRegistration),
    (Ref<'_, T>  where T: GetTypeRegistration),
    (Mut<'_, T>  where T: GetTypeRegistration),
    (Result<T, InteropError>  where T: GetTypeRegistration),
    ([T; N]  where T: GetTypeRegistration;Typed,, const N: usize => with Self),
    (Option<T>  where T: GetTypeRegistration;FromReflect;Typed => with Self),
    (Vec<T>  where T: GetTypeRegistration;FromReflect;Typed => with Self),
    (HashMap<K,V> where K: GetTypeRegistration;FromReflect;Typed;Hash;Eq, V: GetTypeRegistration;FromReflect;Typed => with Self)
);

bevy::utils::all_tuples!(register_tuple_dependencies, 1, 14, T);
pub trait GetFunctionTypeDependencies<Marker> {
    fn register_type_dependencies(registry: &mut TypeRegistry);
}

/// The caller context when calling a script function.
/// Functions can choose to react to caller preferences such as converting 1-indexed numbers to 0-indexed numbers
#[derive(Clone, Copy, Debug, Reflect)]
pub struct CallerContext {
    pub convert_to_0_indexed: bool,
}

/// The Script Function equivalent for dynamic functions. Currently unused
#[derive(Clone, Reflect)]
#[reflect(opaque)]
pub struct DynamicScriptFunction {
    name: Cow<'static, str>,
    // TODO: info about the function, this is hard right now because of non 'static lifetimes in wrappers, we can't use TypePath etc
    func: Arc<
        dyn Fn(CallerContext, WorldCallbackAccess, Vec<ScriptValue>) -> ScriptValue
            + Send
            + Sync
            + 'static,
    >,
}

impl PartialEq for DynamicScriptFunction {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

#[derive(Clone, Reflect)]
#[reflect(opaque)]
pub struct DynamicScriptFunctionMut {
    name: Cow<'static, str>,
    func: Arc<
        RwLock<
            // I'd rather consume an option or something instead of having the RWLock but I just wanna get this release out
            dyn FnMut(CallerContext, WorldCallbackAccess, Vec<ScriptValue>) -> ScriptValue
                + Send
                + Sync
                + 'static,
        >,
    >,
}

impl PartialEq for DynamicScriptFunctionMut {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

impl DynamicScriptFunction {
    pub fn call(
        &self,
        context: CallerContext,
        world: WorldCallbackAccess,
        args: Vec<ScriptValue>,
    ) -> ScriptValue {
        (self.func)(context, world, args)
    }

    pub fn name(&self) -> &Cow<'static, str> {
        &self.name
    }

    pub fn with_name<N: Into<Cow<'static, str>>>(self, name: N) -> Self {
        Self {
            name: name.into(),
            func: self.func,
        }
    }
}

impl DynamicScriptFunctionMut {
    pub fn call(
        &mut self,
        context: CallerContext,
        world: WorldCallbackAccess,
        args: Vec<ScriptValue>,
    ) -> ScriptValue {
        let mut write = self.func.write();
        write(context, world, args)
    }

    pub fn name(&self) -> &Cow<'static, str> {
        &self.name
    }

    pub fn with_name<N: Into<Cow<'static, str>>>(self, name: N) -> Self {
        Self {
            name: name.into(),
            func: self.func,
        }
    }
}

impl std::fmt::Debug for DynamicScriptFunction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicScriptFunction")
            .field("name", self.name())
            .finish()
    }
}

impl std::fmt::Debug for DynamicScriptFunctionMut {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("DynamicScriptFunctionMut")
            .field("name", self.name())
            .finish()
    }
}

impl<F> From<F> for DynamicScriptFunction
where
    F: Fn(CallerContext, WorldCallbackAccess, Vec<ScriptValue>) -> ScriptValue
        + Send
        + Sync
        + 'static,
{
    fn from(fn_: F) -> Self {
        DynamicScriptFunction {
            name: std::any::type_name::<F>().into(),
            func: Arc::new(fn_),
        }
    }
}

impl<F> From<F> for DynamicScriptFunctionMut
where
    F: FnMut(CallerContext, WorldCallbackAccess, Vec<ScriptValue>) -> ScriptValue
        + Send
        + Sync
        + 'static,
{
    fn from(fn_: F) -> Self {
        DynamicScriptFunctionMut {
            name: std::any::type_name::<F>().into(),
            func: Arc::new(RwLock::new(fn_)),
        }
    }
}

/// Equivalent to [`AppFunctionRegistry`] but stores functions with a more convenient signature for scripting to avoid boxing every argument.
#[derive(Clone, Debug, Default, Resource)]
pub struct AppScriptFunctionRegistry(ScriptFunctionRegistryArc);

impl Deref for AppScriptFunctionRegistry {
    type Target = ScriptFunctionRegistryArc;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for AppScriptFunctionRegistry {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

#[derive(Clone, Debug, Default)]
pub struct ScriptFunctionRegistryArc(pub Arc<RwLock<ScriptFunctionRegistry>>);

impl ScriptFunctionRegistryArc {
    pub fn read(&self) -> RwLockReadGuard<ScriptFunctionRegistry> {
        self.0.read()
    }

    pub fn write(&mut self) -> RwLockWriteGuard<ScriptFunctionRegistry> {
        self.0.write()
    }
}

#[derive(Debug, Default)]
pub struct ScriptFunctionRegistry {
    functions: HashMap<Cow<'static, str>, DynamicScriptFunction>,
}

impl ScriptFunctionRegistry {
    /// Register a script function with the given name. If the name already exists,
    /// the new function will be registered as an overload of the function.
    pub fn register<F, M>(&mut self, name: impl Into<Cow<'static, str>>, func: F)
    where
        F: ScriptFunction<'static, M>,
    {
        self.register_overload(name, func);
    }

    pub fn register_overload<F, M>(&mut self, name: impl Into<Cow<'static, str>>, func: F)
    where
        F: ScriptFunction<'static, M>,
    {
        // always start with non-suffixed registration
        let name = name.into().clone();

        if !self.contains(&name) {
            let func = func.into_dynamic_script_function().with_name(name.clone());
            self.functions.insert(name, func);
            return;
        }

        for i in 1..16 {
            let overload = format!("{name}-{i}");
            if !self.contains(&overload) {
                self.register(overload, func);
                return;
            }
        }

        panic!(
            "Could not register overload for function {name}. Maximum number of overloads reached"
        );
    }

    pub fn contains(&self, name: impl AsRef<str>) -> bool {
        self.functions.contains_key(name.as_ref())
    }

    pub fn get_first(&self, name: impl AsRef<str>) -> Option<&DynamicScriptFunction> {
        self.functions.get(name.as_ref())
    }

    pub fn iter_overloads(
        &self,
        name: impl Into<Cow<'static, str>>,
    ) -> impl Iterator<Item = &DynamicScriptFunction> {
        let name = name.into();
        (0..16)
            .map(move |i| {
                if i == 0 {
                    self.functions.get(&name)
                } else {
                    let name: Cow<'static, str> = format!("{}-{i}", name).into();
                    self.functions.get(&name)
                }
            })
            .take_while(|o| o.is_some())
            .map(|o| o.unwrap())
    }
}

macro_rules! count {
    () => (0usize);
    ( $x:tt $($xs:tt)* ) => (1usize + count!($($xs)*));
}

macro_rules! impl_script_function {

    ($( $param:ident ),* ) => {
        // all of this is pretty heavy on the compile time.
        // ideally we'd do less, but for now this will suffice

        // Fn(T1...Tn) -> O
        impl_script_function!(@ ScriptFunction Fn DynamicScriptFunction into_dynamic_script_function $( $param ),* : -> O => O );
        // FnMut(T1...Tn) -> O
        impl_script_function!(@ ScriptFunctionMut FnMut DynamicScriptFunctionMut into_dynamic_script_function_mut $( $param ),* : -> O => O );

        // Fn(WorldCallbackAccess, T1...Tn) -> O
        impl_script_function!(@ ScriptFunction Fn DynamicScriptFunction into_dynamic_script_function $( $param ),* : ,(callback: WorldCallbackAccess) -> O => O);
        // FnMut(WorldCallbackAccess, T1...Tn) -> O
        impl_script_function!(@ ScriptFunctionMut FnMut DynamicScriptFunctionMut into_dynamic_script_function_mut $( $param ),* : ,(callback: WorldCallbackAccess) -> O => O);

        // Fn(CallerContext, WorldCallbackAccess, T1...Tn) -> O
        impl_script_function!(@ ScriptFunction Fn DynamicScriptFunction into_dynamic_script_function $( $param ),* : (context: CallerContext),(callback: WorldCallbackAccess) -> O => O);
        // FnMut(CallerContext, WorldCallbackAccess, T1...Tn) -> O
        impl_script_function!(@ ScriptFunctionMut FnMut DynamicScriptFunctionMut into_dynamic_script_function_mut $( $param ),* : (context: CallerContext),(callback: WorldCallbackAccess) -> O => O);

        // Fn(T1...Tn) -> Result<O, InteropError>
        impl_script_function!(@ ScriptFunction Fn DynamicScriptFunction into_dynamic_script_function $( $param ),* : -> O => Result<O, InteropError> where s);
        // FnMut(T1...Tn) -> Result<O, InteropError>
        impl_script_function!(@ ScriptFunctionMut FnMut DynamicScriptFunctionMut into_dynamic_script_function_mut $( $param ),* : -> O => Result<O, InteropError> where s);

        // Fn(WorldCallbackAccess, T1...Tn) -> Result<O, InteropError>
        impl_script_function!(@ ScriptFunction Fn DynamicScriptFunction into_dynamic_script_function $( $param ),* : ,(callback: WorldCallbackAccess) -> O => Result<O, InteropError> where s);
        // FnMut(WorldCallbackAccess, T1...Tn) -> Result<O, InteropError>
        impl_script_function!(@ ScriptFunctionMut FnMut DynamicScriptFunctionMut into_dynamic_script_function_mut $( $param ),* : ,(callback: WorldCallbackAccess) -> O => Result<O, InteropError> where s);

        // Fn(CallerContext, WorldCallbackAccess, T1...Tn) -> Result<O, InteropError>
        impl_script_function!(@ ScriptFunction Fn DynamicScriptFunction into_dynamic_script_function $( $param ),* : (context: CallerContext),(callback: WorldCallbackAccess) -> O => Result<O, InteropError> where s);
        // FnMut(CallerContext, WorldCallbackAccess, T1...Tn) -> Result<O, InteropError>
        impl_script_function!(@ ScriptFunctionMut FnMut DynamicScriptFunctionMut into_dynamic_script_function_mut $( $param ),* : (context: CallerContext),(callback: WorldCallbackAccess) -> O => Result<O, InteropError> where s);


    };

    (@ $trait_type:ident $fn_type:ident $dynamic_type:ident $trait_fn_name:ident $( $param:ident ),* :  $(($context:ident: $contextty:ty))? $(,($callback:ident: $callbackty:ty))? -> O => $res:ty $(where $out:ident)?) => {
        #[allow(non_snake_case)]
        impl<
            'env,
            $( $param: FromScript, )*
            O,
            F
        > $trait_type<'env,
            fn( $($contextty,)? $( $callbackty, )? $($param ),* ) -> $res
        > for F
        where
            O: IntoScript + TypePath + GetOwnership,
            F: $fn_type(  $($contextty,)? $( $callbackty, )? $($param ),* ) -> $res + Send + Sync + 'static,
            $( $param::This<'env>: Into<$param>,)*
        {
            #[allow(unused_mut,unused_variables)]
            fn $trait_fn_name(mut self) -> $dynamic_type {
                let func = (move |caller_context: CallerContext, world: WorldCallbackAccess, args: Vec<ScriptValue> | {
                    let res: Result<ScriptValue, InteropError> = (|| {
                        let expected_arg_count = count!($($param )*);
                        if args.len() != expected_arg_count {
                            return Err(InteropError::function_call_error(FunctionError::ArgCountMismatch{
                                expected: expected_arg_count,
                                received: args.len()
                            }));
                        }
                        $( let $context = caller_context; )?
                        $( let $callback = world.clone(); )?
                        let world = world.try_read()?;
                        world.begin_access_scope()?;
                        let ret = {
                            let mut current_arg = 0;
                            let mut arg_iter = args.into_iter();
                            $(
                                current_arg += 1;
                                let $param = <$param>::from_script(arg_iter.next().expect("invariant"), world.clone())
                                    .map_err(|e| InteropError::function_arg_conversion_error(current_arg.to_string(), e))?;
                            )*
                            let out = self( $( $context,)? $( $callback, )? $( $param.into(), )* );
                            $(
                                let $out = out?;
                                let out = $out;
                            )?
                            out.into_script(world.clone()).map_err(|e| InteropError::function_arg_conversion_error("return value".to_owned(), e))
                        };
                        // Safety: we're not holding any references to the world, the arguments which might have aliased have been dropped
                        unsafe { world.end_access_scope()? };
                        ret
                    })();
                    let script_value: ScriptValue = res.into();
                    script_value
                });

                func.into()
            }
        }
    };
}

macro_rules! impl_script_function_type_dependencies{
    ($( $param:ident ),* ) => {
        impl<F, $( $param: GetInnerTypeDependencies ,)* O: GetInnerTypeDependencies> GetFunctionTypeDependencies<fn($($param),*) -> O> for F
            where F: Fn( $( $param ),* ) -> O
        {
            fn register_type_dependencies(registry: &mut TypeRegistry) {
                $(
                    $param::register_type_dependencies(registry);
                )*

                O::register_type_dependencies(registry);
            }
        }
    };
}

bevy::utils::all_tuples!(impl_script_function, 0, 13, T);
bevy::utils::all_tuples!(impl_script_function_type_dependencies, 0, 13, T);

/// Utility for quickly checking your type can be used as an argument in a script function
///
/// Usage:
/// ```
/// assert_impls_into_script!(i32);
/// ```
#[macro_export]
macro_rules! assert_impls_into_script {
    ($ty:ty) => {
        trait Check: $crate::bindings::function::into::IntoScript {}
        impl Check for $ty {}
    };
}

/// Utility for quickly checking your type can be used as a return value in a script function
///
/// Usage:
/// ```
/// assert_impls_from_script!(i32);
/// ```
#[macro_export]
macro_rules! assert_impls_from_script {
    ($ty:ty) => {
        trait Check: $crate::bindings::function::from::FromScript {}
        impl Check for $ty {}
    };
    ($l:lifetime $ty:ty) => {
        trait Check: $crate::bindings::function::from::FromScript {}
        impl<$l> Check for $ty {}
    };
}

/// Utility for quickly checking your function can be used as a script function
///
/// Usage:
/// ```
/// assert_is_script_function!(|a: i32, b: i32| a + b);
/// ```
#[macro_export]
macro_rules! assert_is_script_function {
    ($($tt:tt)*) => {
        fn _check<'env,M,F: ScriptFunction<'env, M>>(f: F) {

        }

        fn test() {
            _check($($tt)*);
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::bindings::function::script_function::ScriptFunction;
    use crate::prelude::AppReflectAllocator;
    use bevy::reflect::func::{ArgList, ArgValue, Return};
    use test_utils::test_data::*;

    fn test_setup_world() -> World {
        setup_world(|w, _| w.insert_resource(AppReflectAllocator::default()))
    }

    fn assert_function_info_eq(a: &FunctionInfo, b: &FunctionInfo) {
        assert_eq!(a.name(), b.name(), "Function names do not match");
        assert_eq!(
            a.args().len(),
            b.args().len(),
            "Function arg count does not match"
        );
        for (a, b) in a.args().iter().zip(b.args().iter()) {
            assert_eq!(a.type_id(), b.type_id(), "Function arg types do not match");
            assert_eq!(a.name(), b.name(), "Function arg names do not match");
        }
    }

    #[test]
    fn test_register_script_function() {
        let mut registry = ScriptFunctionRegistry::default();
        let fn_ = |a: usize, b: usize| a + b;
        registry.register("test", fn_);
        registry.get_first("test").expect("Failed to get function");
    }

    #[test]
    fn test_overloaded_script_function() {
        let mut registry = ScriptFunctionRegistry::default();
        let fn_ = |a: usize, b: usize| a + b;
        registry.register("test", fn_);
        let fn_2 = |a: usize, b: i32| a + (b as usize);
        registry.register("test", fn_2);

        registry.get_first("test").expect("Failed to get function");

        assert_eq!(registry.iter_overloads("test").collect::<Vec<_>>().len(), 2);
    }
}
