use super::{from::FromScript, into::IntoScript, namespace::Namespace};
use crate::bindings::function::arg_meta::ArgMeta;
use crate::docgen::info::{FunctionInfo, GetFunctionInfo};
use crate::{
    bindings::{ThreadWorldContainer, WorldContainer, WorldGuard},
    error::InteropError,
    ScriptValue,
};
use bevy::{
    prelude::{Reflect, Resource},
    reflect::func::FunctionError,
};
use parking_lot::{RwLock, RwLockReadGuard, RwLockWriteGuard};
use std::borrow::Cow;
use std::collections::{HashMap, VecDeque};
use std::hash::Hash;
use std::ops::{Deref, DerefMut};
use std::sync::Arc;

#[diagnostic::on_unimplemented(
    message = "This function does not fulfil the requirements to be a script callable function. All arguments must implement the ScriptArgument trait and all return values must implement the ScriptReturn trait",
    note = "If you're trying to return a non-primitive type, you might need to use Val<T> Ref<T> or Mut<T> wrappers"
)]
pub trait ScriptFunction<'env, Marker> {
    fn into_dynamic_script_function(self) -> DynamicScriptFunction;
}

#[diagnostic::on_unimplemented(
    message = "Only functions with all arguments impplementing FromScript and return values supporting IntoScript are supported. Registering functions also requires they implement GetTypeDependencies",
    note = "If you're trying to return a non-primitive type, you might need to use Val<T> Ref<T> or Mut<T> wrappers"
)]
pub trait ScriptFunctionMut<'env, Marker> {
    fn into_dynamic_script_function_mut(self) -> DynamicScriptFunctionMut;
}

/// The caller context when calling a script function.
/// Functions can choose to react to caller preferences such as converting 1-indexed numbers to 0-indexed numbers
#[derive(Clone, Copy, Debug, Reflect, Default)]
#[reflect(opaque)]
pub struct FunctionCallContext {
    pub convert_to_0_indexed: bool,
}
impl FunctionCallContext {
    pub fn new(convert_to_0_indexed: bool) -> Self {
        Self {
            convert_to_0_indexed,
        }
    }

    /// Tries to access the world, returning an error if the world is not available
    pub fn world<'l>(&self) -> Result<WorldGuard<'l>, InteropError> {
        ThreadWorldContainer.try_get_world()
    }
}

/// The Script Function equivalent for dynamic functions. Currently unused
#[derive(Clone, Reflect)]
#[reflect(opaque)]
pub struct DynamicScriptFunction {
    pub info: FunctionInfo,
    // TODO: info about the function, this is hard right now because of non 'static lifetimes in wrappers, we can't use TypePath etc
    func: Arc<
        dyn Fn(FunctionCallContext, VecDeque<ScriptValue>) -> ScriptValue + Send + Sync + 'static,
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
            dyn FnMut(FunctionCallContext, VecDeque<ScriptValue>) -> ScriptValue
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
        context: FunctionCallContext,
    ) -> Result<ScriptValue, InteropError> {
        profiling::scope!("Dynamic Call ", self.name().clone());
        let args = args.into_iter().collect::<VecDeque<_>>();
        // should we be inlining call errors into the return value?
        let return_val = (self.func)(context, args);
        match return_val {
            ScriptValue::Error(e) => Err(InteropError::function_interop_error(
                self.name(),
                self.info.namespace,
                e,
            )),
            v => Ok(v),
        }
    }

    pub fn name(&self) -> &Cow<'static, str> {
        &self.info.name
    }

    pub fn with_info(self, info: FunctionInfo) -> Self {
        Self { info, ..self }
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
        context: FunctionCallContext,
    ) -> Result<ScriptValue, InteropError> {
        profiling::scope!("Dynamic Call Mut", self.name().clone());
        let args = args.into_iter().collect::<VecDeque<_>>();
        // should we be inlining call errors into the return value?
        let mut write = self.func.write();
        let return_val = (write)(context, args);
        match return_val {
            ScriptValue::Error(e) => Err(InteropError::function_interop_error(
                self.name(),
                self.info.namespace,
                e,
            )),
            v => Ok(v),
        }
    }
    pub fn name(&self) -> &Cow<'static, str> {
        &self.info.name
    }

    pub fn with_info(self, info: FunctionInfo) -> Self {
        Self { info, ..self }
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
    F: Fn(FunctionCallContext, VecDeque<ScriptValue>) -> ScriptValue + Send + Sync + 'static,
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
    F: FnMut(FunctionCallContext, VecDeque<ScriptValue>) -> ScriptValue + Send + Sync + 'static,
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

#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub struct FunctionKey {
    pub name: Cow<'static, str>,
    pub namespace: Namespace,
}

#[derive(Debug, Default)]
pub struct ScriptFunctionRegistry {
    functions: HashMap<FunctionKey, DynamicScriptFunction>,
}
#[profiling::all_functions]
impl ScriptFunctionRegistry {
    /// Register a script function with the given name. If the name already exists,
    /// the new function will be registered as an overload of the function.
    ///
    /// If you want to overwrite an existing function, use [`ScriptFunctionRegistry::overwrite`]
    pub fn register<'env, F, M>(
        &mut self,
        namespace: Namespace,
        name: impl Into<Cow<'static, str>>,
        func: F,
    ) where
        F: ScriptFunction<'env, M> + GetFunctionInfo<M>,
    {
        self.register_overload(namespace, name, func, false, None::<&'static str>);
    }

    /// Equivalent to [`ScriptFunctionRegistry::register`] but with the ability to provide documentation for the function.
    ///
    /// The docstring will be added to the function's metadata and can be accessed at runtime.
    pub fn register_documented<F, M>(
        &mut self,
        namespace: Namespace,
        name: impl Into<Cow<'static, str>>,
        func: F,
        docs: &'static str,
    ) where
        F: ScriptFunction<'static, M> + GetFunctionInfo<M>,
    {
        self.register_overload(namespace, name, func, false, Some(docs));
    }

    /// Overwrite a function with the given name. If the function does not exist, it will be registered as a new function.
    pub fn overwrite<F, M>(
        &mut self,
        namespace: Namespace,
        name: impl Into<Cow<'static, str>>,
        func: F,
    ) where
        F: ScriptFunction<'static, M> + GetFunctionInfo<M>,
    {
        self.register_overload(namespace, name, func, true, None::<&'static str>);
    }

    pub fn overwrite_documented<F, M>(
        &mut self,
        namespace: Namespace,
        name: impl Into<Cow<'static, str>>,
        func: F,
        docs: &'static str,
    ) where
        F: ScriptFunction<'static, M> + GetFunctionInfo<M>,
    {
        self.register_overload(namespace, name, func, true, Some(docs));
    }

    /// Remove a function from the registry if it exists. Returns the removed function if it was found.
    ///
    /// Note if the function is overloaded, you will need to remove each overload individually.
    /// Use [`ScriptFunctionRegistry::remove_all_overloads`] to remove all overloads at once.
    pub fn remove(
        &mut self,
        namespace: Namespace,
        name: impl Into<Cow<'static, str>>,
    ) -> Option<DynamicScriptFunction> {
        let name = name.into();
        self.functions.remove(&FunctionKey { name, namespace })
    }

    pub fn remove_all_overloads(
        &mut self,
        namespace: Namespace,
        name: impl Into<Cow<'static, str>>,
    ) -> Result<Vec<DynamicScriptFunction>, Cow<'static, str>> {
        let overloads: Vec<_> = self.iter_overloads(namespace, name)?.cloned().collect();
        for overload in overloads.iter() {
            self.functions.remove(&FunctionKey {
                name: overload.info.name.clone(),
                namespace,
            });
        }
        Ok(overloads)
    }

    fn register_overload<'env, F, M>(
        &mut self,
        namespace: Namespace,
        name: impl Into<Cow<'static, str>>,
        func: F,
        overwrite: bool,
        docs: Option<impl Into<Cow<'static, str>>>,
    ) where
        F: ScriptFunction<'env, M> + GetFunctionInfo<M>,
    {
        // always start with non-suffixed registration
        // TODO: we do alot of string work, can we make this all more efficient?
        let name: Cow<'static, str> = name.into();
        if overwrite || !self.contains(namespace, name.clone()) {
            let info = func.get_function_info(name.clone(), namespace);
            let info = match docs {
                Some(docs) => info.with_docs(docs.into()),
                None => info,
            };
            let func = func.into_dynamic_script_function().with_info(info);
            self.functions.insert(FunctionKey { name, namespace }, func);
            return;
        }

        for i in 1.. {
            let overload = format!("{name}-{i}");
            if !self.contains(namespace, overload.clone()) {
                self.register(namespace, overload, func);
                return;
            }
        }
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

        let overloads = (1..)
            .map(move |i| {
                if i == 0 {
                    self.get_function(namespace, name.clone())
                } else {
                    let name: Cow<'static, str> = format!("{}-{i}", name).into();
                    self.get_function(namespace, name)
                }
            })
            .take_while(|o| o.is_ok())
            .filter_map(|o| o.ok());

        Ok(seed.chain(overloads))
    }

    /// Iterates over all functions including overloads
    pub fn iter_all(&self) -> impl Iterator<Item = (&FunctionKey, &DynamicScriptFunction)> {
        self.functions.iter()
    }

    pub fn iter_namespace(
        &self,
        namespace: Namespace,
    ) -> impl Iterator<Item = (&FunctionKey, &DynamicScriptFunction)> {
        self.functions
            .iter()
            .filter(move |(key, _)| key.namespace == namespace)
    }

    /// Insert a function into the registry with the given key, this will not perform any overloading logic.
    /// Do not use unless you really need to.
    pub fn raw_insert(
        &mut self,
        namespace: Namespace,
        name: impl Into<Cow<'static, str>>,
        func: DynamicScriptFunction,
    ) {
        self.functions.insert(
            FunctionKey {
                name: name.into(),
                namespace,
            },
            func,
        );
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

        // Fn(CallerContext, T1...Tn) -> O
        impl_script_function!(@ ScriptFunction Fn DynamicScriptFunction into_dynamic_script_function $( $param ),* : (context: FunctionCallContext) -> O => O);
        // FnMut(FunctionCallContext, T1...Tn) -> O
        impl_script_function!(@ ScriptFunctionMut FnMut DynamicScriptFunctionMut into_dynamic_script_function_mut $( $param ),* : (context: FunctionCallContext) -> O => O);

        // Fn(T1...Tn) -> Result<O, InteropError>
        impl_script_function!(@ ScriptFunction Fn DynamicScriptFunction into_dynamic_script_function $( $param ),* : -> O => Result<O, InteropError> where s);
        // FnMut(T1...Tn) -> Result<O, InteropError>
        impl_script_function!(@ ScriptFunctionMut FnMut DynamicScriptFunctionMut into_dynamic_script_function_mut $( $param ),* : -> O => Result<O, InteropError> where s);

        // Fn(FunctionCallContext, WorldGuard<'w>, T1...Tn) -> Result<O, InteropError>
        impl_script_function!(@ ScriptFunction Fn DynamicScriptFunction into_dynamic_script_function $( $param ),* : (context: FunctionCallContext)-> O => Result<O, InteropError> where s);
        // FnMut(FunctionCallContext, WorldGuard<'w>, T1...Tn) -> Result<O, InteropError>
        impl_script_function!(@ ScriptFunctionMut FnMut DynamicScriptFunctionMut into_dynamic_script_function_mut $( $param ),* : (context: FunctionCallContext) -> O => Result<O, InteropError> where s);


    };

    (@ $trait_type:ident $fn_type:ident $dynamic_type:ident $trait_fn_name:ident $( $param:ident ),* :  $(($context:ident: $contextty:ty))? -> O => $res:ty $(where $out:ident)?) => {
        #[allow(non_snake_case)]
        impl<
            'env,
            $( $param: FromScript + ArgMeta,)*
            O,
            F
        > $trait_type<'env,
            fn( $($contextty,)? $($param ),* ) -> $res
        > for F
        where
            O: IntoScript,
            F: $fn_type(  $($contextty,)? $($param ),* ) -> $res + Send + Sync + 'static,
            $( $param::This<'env>: Into<$param>,)*
        {
            #[allow(unused_mut,unused_variables)]
            #[profiling::function]
            fn $trait_fn_name(mut self) -> $dynamic_type {

                let func = (move |caller_context: FunctionCallContext, mut args: VecDeque<ScriptValue> | {
                    let res: Result<ScriptValue, InteropError> = (|| {
                        let received_args_len = args.len();
                        let expected_arg_count = count!($($param )*);

                        $( let $context = caller_context; )?
                        let world = caller_context.world()?;
                        world.begin_access_scope()?;
                        let mut current_arg = 0;

                        $(
                            current_arg += 1;
                            let $param = args.pop_front();
                            let $param = match $param {
                                Some($param) => $param,
                                None => {
                                    if let Some(default) = <$param>::default_value() {
                                        default
                                    } else {
                                        return Err(InteropError::function_call_error(FunctionError::ArgCountMismatch{
                                            expected: expected_arg_count,
                                            received: received_args_len
                                        }));
                                    }
                                }
                            };
                            let $param = <$param>::from_script($param, world.clone())
                                .map_err(|e| InteropError::function_arg_conversion_error(current_arg.to_string(), e))?;
                        )*

                        let ret = {
                            let out = self( $( $context,)?  $( $param.into(), )* );
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

bevy::utils::all_tuples!(impl_script_function, 0, 13, T);

#[cfg(test)]
mod test {
    use super::*;

    fn with_local_world<F: Fn()>(f: F) {
        let mut world = bevy::prelude::World::default();
        WorldGuard::with_static_guard(&mut world, |world| {
            ThreadWorldContainer.set_world(world).unwrap();
            f()
        });
    }

    #[test]
    fn test_register_script_function() {
        let mut registry = ScriptFunctionRegistry::default();
        let fn_ = |a: usize, b: usize| a + b;

        let namespace = Namespace::Global;
        registry.register(namespace, "test", fn_);
        let function = registry
            .get_function(namespace, "test")
            .expect("Failed to get function");

        assert_eq!(function.info.name, "test");
        assert_eq!(function.info.namespace, namespace);
    }

    #[test]
    fn test_optional_argument_not_required() {
        let fn_ = |a: usize, b: Option<usize>| a + b.unwrap_or(0);
        let script_function = fn_.into_dynamic_script_function();

        with_local_world(|| {
            let out = script_function
                .call(vec![ScriptValue::from(1)], FunctionCallContext::default())
                .unwrap();

            assert_eq!(out, ScriptValue::from(1));
        });
    }

    #[test]
    fn test_invalid_amount_of_args_errors_nicely() {
        let fn_ = |a: usize, b: usize| a + b;
        let script_function = fn_.into_dynamic_script_function().with_name("my_fn");

        with_local_world(|| {
            let out =
                script_function.call(vec![ScriptValue::from(1)], FunctionCallContext::default());

            assert!(out.is_err());
            assert_eq!(
                out.unwrap_err().into_inner().unwrap(),
                InteropError::function_interop_error(
                    "my_fn",
                    Namespace::Global,
                    InteropError::function_call_error(FunctionError::ArgCountMismatch {
                        expected: 2,
                        received: 1
                    })
                )
                .into_inner()
                .unwrap()
            );
        });
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

        assert_eq!(first_function.info.name, "test");
        assert_eq!(first_function.info.namespace, namespace);

        let all_functions = registry
            .iter_overloads(namespace, "test")
            .expect("Failed to get overloads")
            .collect::<Vec<_>>();

        assert_eq!(all_functions.len(), 2);
        assert_eq!(all_functions[0].info.name, "test");
        assert_eq!(all_functions[1].info.name, "test-1");
    }

    #[test]
    fn test_overwrite_script_function() {
        let mut registry = ScriptFunctionRegistry::default();
        let fn_ = |a: usize, b: usize| a + b;
        let namespace = Namespace::Global;
        registry.register(namespace, "test", fn_);
        let fn_2 = |a: usize, b: i32| a + (b as usize);
        registry.overwrite(namespace, "test", fn_2);

        let all_functions = registry
            .iter_overloads(namespace, "test")
            .expect("Failed to get overloads")
            .collect::<Vec<_>>();

        assert_eq!(all_functions.len(), 1);
        assert_eq!(all_functions[0].info.name, "test");
    }

    #[test]
    fn test_remove_script_function() {
        let mut registry = ScriptFunctionRegistry::default();
        let fn_ = |a: usize, b: usize| a + b;
        let namespace = Namespace::Global;
        registry.register(namespace, "test", fn_);
        let removed = registry.remove(namespace, "test");
        assert!(removed.is_some());
        let removed = registry.remove(namespace, "test");
        assert!(removed.is_none());
    }

    #[test]
    fn test_remove_all_overloads() {
        let mut registry = ScriptFunctionRegistry::default();
        let fn_ = |a: usize, b: usize| a + b;
        let namespace = Namespace::Global;
        registry.register(namespace, "test", fn_);
        let fn_2 = |a: usize, b: i32| a + (b as usize);
        registry.register(namespace, "test", fn_2);

        let removed = registry
            .remove_all_overloads(namespace, "test")
            .expect("Failed to remove overloads");
        assert_eq!(removed.len(), 2);
        assert!(registry.get_function(namespace, "test").is_err());
    }
}
