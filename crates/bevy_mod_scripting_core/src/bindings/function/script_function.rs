use super::{from::FromScript, into::IntoScript, namespace::Namespace};
use crate::{
    bindings::{
        function::from::{Mut, Ref, Val},
        ReflectReference, WorldGuard,
    },
    error::InteropError,
    ScriptValue, WorldCallbackAccess,
};
use bevy::{
    prelude::{Reflect, Resource},
    reflect::{
        func::{args::GetOwnership, FunctionError},
        FromReflect, GetTypeRegistration, TypePath, TypeRegistry, Typed,
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

no_type_dependencies!(InteropError);
self_type_dependency_only!(WorldCallbackAccess, CallerContext, ReflectReference);

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
#[derive(Clone, Copy, Debug, Reflect, Default)]
#[reflect(opaque)]
pub struct CallerContext {
    pub convert_to_0_indexed: bool,
}

#[derive(Clone, Debug, PartialEq, Default)]
pub struct FunctionInfo {
    pub name: Cow<'static, str>,
    pub namespace: Namespace,
}

impl FunctionInfo {
    /// The name of the function
    pub fn name(&self) -> &Cow<'static, str> {
        &self.name
    }

    /// If the function is namespaced to a specific type, this will return the type id of that type
    pub fn namespace(&self) -> Namespace {
        self.namespace
    }
}

/// The Script Function equivalent for dynamic functions. Currently unused
#[derive(Clone, Reflect)]
#[reflect(opaque)]
pub struct DynamicScriptFunction {
    pub info: FunctionInfo,
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
        self.info == other.info
    }
}

#[derive(Clone, Reflect)]
#[reflect(opaque)]
pub struct DynamicScriptFunctionMut {
    pub info: FunctionInfo,
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
        self.info == other.info
    }
}

impl DynamicScriptFunction {
    /// Call the function with the given arguments and caller context.
    ///
    /// In the case of errors wraps the error in a [`InteropError::function_interop_error`] to provide more context.
    pub fn call<I: IntoIterator<Item = ScriptValue>>(
        &self,
        args: I,
        world: WorldGuard,
        context: CallerContext,
    ) -> Result<ScriptValue, InteropError> {
        let args = args.into_iter().collect::<Vec<_>>();
        let world_callback_access = WorldCallbackAccess::from_guard(world.clone());
        // should we be inlining call errors into the return value?
        let return_val = (self.func)(context, world_callback_access, args);
        match return_val {
            ScriptValue::Error(e) => Err(InteropError::function_interop_error(
                self.name(),
                self.info.namespace(),
                e,
            )),
            v => Ok(v),
        }
    }

    pub fn name(&self) -> &Cow<'static, str> {
        &self.info.name
    }

    pub fn with_name<N: Into<Cow<'static, str>>>(self, name: N) -> Self {
        Self {
            info: FunctionInfo {
                name: name.into(),
                ..self.info
            },
            func: self.func,
        }
    }

    pub fn with_namespace(self, namespace: Namespace) -> Self {
        Self {
            info: FunctionInfo {
                namespace,
                ..self.info
            },
            func: self.func,
        }
    }
}

impl DynamicScriptFunctionMut {
    /// Call the function with the given arguments and caller context.
    ///
    /// In the case of errors wraps the error in a [`InteropError::function_interop_error`] to provide more context.
    pub fn call<I: IntoIterator<Item = ScriptValue>>(
        &self,
        args: I,
        world: WorldGuard,
        context: CallerContext,
    ) -> Result<ScriptValue, InteropError> {
        let args = args.into_iter().collect::<Vec<_>>();
        let world_callback_access = WorldCallbackAccess::from_guard(world.clone());
        // should we be inlining call errors into the return value?
        let mut write = self.func.write();
        let return_val = (write)(context, world_callback_access, args);
        match return_val {
            ScriptValue::Error(e) => Err(InteropError::function_interop_error(
                self.name(),
                self.info.namespace(),
                e,
            )),
            v => Ok(v),
        }
    }
    pub fn name(&self) -> &Cow<'static, str> {
        &self.info.name
    }

    pub fn with_name<N: Into<Cow<'static, str>>>(self, name: N) -> Self {
        Self {
            info: FunctionInfo {
                name: name.into(),
                ..self.info
            },
            func: self.func,
        }
    }

    pub fn with_namespace(self, namespace: Namespace) -> Self {
        Self {
            info: FunctionInfo {
                namespace,
                ..self.info
            },
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
            info: FunctionInfo::default(),
            func: Arc::new(fn_),
        }
        .with_name(std::any::type_name::<F>())
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
            info: FunctionInfo::default(),
            func: Arc::new(RwLock::new(fn_)),
        }
        .with_name(std::any::type_name::<F>())
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

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct FunctionKey {
    name: Cow<'static, str>,
    namespace: Namespace,
}

#[derive(Debug, Default)]
pub struct ScriptFunctionRegistry {
    functions: HashMap<FunctionKey, DynamicScriptFunction>,
}

impl ScriptFunctionRegistry {
    /// Register a script function with the given name. If the name already exists,
    /// the new function will be registered as an overload of the function.
    pub fn register<F, M>(
        &mut self,
        namespace: Namespace,
        name: impl Into<Cow<'static, str>>,
        func: F,
    ) where
        F: ScriptFunction<'static, M>,
    {
        self.register_overload(namespace, name, func);
    }

    fn register_overload<F, M>(
        &mut self,
        namespace: Namespace,
        name: impl Into<Cow<'static, str>>,
        func: F,
    ) where
        F: ScriptFunction<'static, M>,
    {
        // always start with non-suffixed registration
        // TODO: we do alot of string work, can we make this all more efficient?
        let name: Cow<'static, str> = name.into();
        if !self.contains(namespace, name.clone()) {
            let func = func
                .into_dynamic_script_function()
                .with_name(name.clone())
                .with_namespace(namespace);
            self.functions.insert(FunctionKey { name, namespace }, func);
            return;
        }

        for i in 1..16 {
            let overload = format!("{name}-{i}");
            if !self.contains(namespace, overload.clone()) {
                self.register(namespace, overload, func);
                return;
            }
        }

        panic!(
            "Could not register overload for function {name}. Maximum number of overloads reached"
        );
    }

    pub fn contains(&self, namespace: Namespace, name: impl Into<Cow<'static, str>>) -> bool {
        self.functions.contains_key(&FunctionKey {
            name: name.into(),
            namespace,
        })
    }

    /// Get the first overload for the function with the given name and namespace
    pub fn get_function(
        &self,
        namespace: Namespace,
        name: impl Into<Cow<'static, str>>,
    ) -> Result<&DynamicScriptFunction, Cow<'static, str>> {
        let name = name.into();
        let key = FunctionKey { name, namespace };
        if let Some(func) = self.functions.get(&key) {
            Ok(func)
        } else {
            Err(key.name)
        }
    }

    /// Iterate over all overloads for the function with the given name and namespace
    /// If the iterator variant is returned it is guaranteed to contain at least one element
    pub fn iter_overloads(
        &self,
        namespace: Namespace,
        name: impl Into<Cow<'static, str>>,
    ) -> Result<impl Iterator<Item = &DynamicScriptFunction>, Cow<'static, str>> {
        let name: Cow<'static, str> = name.into();
        let seed = match self.get_function(namespace, name.clone()) {
            Ok(func) => std::iter::once(func),
            Err(name) => return Err(name),
        };

        let overloads = (1..16)
            .map(move |i| {
                if i == 0 {
                    self.get_function(namespace, name.clone())
                } else {
                    let name: Cow<'static, str> = format!("{}-{i}", name).into();
                    self.get_function(namespace, name)
                }
            })
            .take_while(|o| o.is_ok())
            .map(|o| o.unwrap());

        Ok(seed.chain(overloads))
    }

    /// Iterates over all functions including overloads
    pub fn iter_all(&self) -> impl Iterator<Item = (&FunctionKey, &DynamicScriptFunction)> {
        self.functions.iter()
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
                        if args.len() < expected_arg_count {
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

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn test_register_script_function() {
        let mut registry = ScriptFunctionRegistry::default();
        let fn_ = |a: usize, b: usize| a + b;
        let namespace = Namespace::Global;
        registry.register(namespace, "test", fn_);
        let function = registry
            .get_function(namespace, "test")
            .expect("Failed to get function");

        assert_eq!(function.info.name(), "test");
        assert_eq!(function.info.namespace(), namespace);
    }

    #[test]
    fn test_overloaded_script_function() {
        let mut registry = ScriptFunctionRegistry::default();
        let fn_ = |a: usize, b: usize| a + b;
        let namespace = Namespace::Global;
        registry.register(namespace, "test", fn_);
        let fn_2 = |a: usize, b: i32| a + (b as usize);
        registry.register(namespace, "test", fn_2);

        let first_function = registry
            .get_function(namespace, "test")
            .expect("Failed to get function");

        assert_eq!(first_function.info.name(), "test");
        assert_eq!(first_function.info.namespace(), namespace);

        let all_functions = registry
            .iter_overloads(namespace, "test")
            .expect("Failed to get overloads")
            .collect::<Vec<_>>();

        assert_eq!(all_functions.len(), 2);
        assert_eq!(all_functions[0].info.name(), "test");
        assert_eq!(all_functions[1].info.name(), "test-1");
    }
}
