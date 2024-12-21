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
    prelude::{AppFunctionRegistry, IntoFunction, World},
    reflect::{
        func::{DynamicFunction, FunctionInfo},
        GetTypeRegistration, PartialReflect, TypeRegistration, TypeRegistry,
    },
};
use std::collections::HashMap;
use std::sync::Arc;

#[diagnostic::on_unimplemented(
    message = "Only functions with all arguments impplementing FromScript and return values supporting IntoScript are supported. use assert_impls_into_script!(MyArg) and assert_impls_from_script!(MyReturnType) to verify yours do.",
    note = "If you're trying to return a non-primitive type, you might need to use Val<T> Ref<T> or Mut<T> wrappers"
)]
pub trait ScriptFunction<'env, Marker> {
    fn into_dynamic_function(self) -> DynamicFunction<'static>;
}

pub trait GetInnerTypeDependencies {
    fn register_type_dependencies(registry: &mut TypeRegistry);
}

#[macro_export]
macro_rules! no_type_dependencies {
    ($($path:path),*) => {
        $(
            impl GetInnerTypeDependencies for $path {
                fn register_type_dependencies(_registry: &mut TypeRegistry) {}
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
    ($(($path:path where $($bound:ident : $bound_val:path),*)),*)  => {
        $(
            impl<$($bound : $bound_val),*> GetInnerTypeDependencies for $path {
                fn register_type_dependencies(registry: &mut TypeRegistry) {
                    $(
                        registry.register::<$bound>();
                    )*
                }
            }
        )*
    };
}

no_type_dependencies!(ReflectReference, InteropError);
self_type_dependency_only!(WorldCallbackAccess);

recursive_type_dependencies!(
    (Val<T> where T: GetTypeRegistration),
    (Ref<'_, T>  where T: GetTypeRegistration),
    (Mut<'_, T>  where T: GetTypeRegistration),
    (Result<T, InteropError>  where T: GetTypeRegistration),
    (Option<T>  where T: GetTypeRegistration),
    (Vec<T>  where T: GetTypeRegistration)
);

recursive_type_dependencies!(
    (HashMap<K,V> where K: GetTypeRegistration, V: GetTypeRegistration)
);

pub trait GetFunctionTypeDependencies<Marker> {
    fn register_type_dependencies(registry: &mut TypeRegistry);
}

/// The Script Function equivalent for dynamic functions
/// TODO: have a separate function registry to avoid the need for boxing script args every time
pub struct DynamicScriptFunction {
    pub info: FunctionInfo,
    pub func:
        Arc<dyn Fn(WorldCallbackAccess, Vec<ScriptValue>) -> Result<ScriptValue, InteropError>>,
}

macro_rules! impl_script_function {

    ($( $param:ident ),* ) => {
        // fn(T1...Tn) -> O
        impl_script_function!(@ $( $param ),* : -> O => O );
        // fn(WorldCallbackAccess, T1...Tn) -> O
        impl_script_function!(@ $( $param ),* : (callback: WorldCallbackAccess) -> O => O);
        // fn(T1...Tn) -> Result<O, InteropError>
        impl_script_function!(@ $( $param ),* : -> O => Result<O, InteropError> where s);
        // fn(WorldCallbackAccess, T1...Tn) -> Result<O, InteropError>
        impl_script_function!(@ $( $param ),* : (callback: WorldCallbackAccess) -> O => Result<O, InteropError> where s);
    };

    (@ $( $param:ident ),* : $(($callback:ident: $callbackty:ty))? -> O => $res:ty $(where $out:ident)?) => {
        #[allow(non_snake_case)]
        impl<
            'env,
            $( $param: FromScript, )*
            O,
            F
        > ScriptFunction<'env,
            fn( $( $callbackty, )? $($param ),* ) -> $res
        > for F
        where
            O: IntoScript,
            F: Fn( $( $callbackty, )? $($param ),* ) -> $res + Send + Sync + 'static,
            $( $param::This<'env>: Into<$param>),*
        {
            fn into_dynamic_function(self) -> DynamicFunction<'static> {
                (move |world: WorldCallbackAccess, $( $param: ScriptValue ),* | {
                    let res: Result<ScriptValue, InteropError> = (|| {
                        $( let $callback = world.clone(); )?
                        let world = world.try_read()?;
                        // TODO: snapshot the accesses and release them after
                        $( let $param = <$param>::from_script($param, world.clone())?; )*
                        let out = self( $( $callback, )? $( $param.into(), )* );
                        $(
                            let $out = out?;
                            let out = $out;
                        )?
                        out.into_script(world.clone())
                    })();
                    let script_value: ScriptValue = res.into();
                    script_value
                }).into_function()
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

bevy::utils::all_tuples!(impl_script_function, 0, 14, T);
bevy::utils::all_tuples!(impl_script_function_type_dependencies, 0, 14, T);

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
    use crate::prelude::AppReflectAllocator;

    use super::*;
    use bevy::reflect::func::{ArgList, ArgValue, Return};
    use test_utils::test_data::*;

    fn test_setup_world() -> World {
        setup_world(|w, _| w.insert_resource(AppReflectAllocator::default()))
    }

    macro_rules! call_script_function_with {
        ($world:ident, $fun:expr, ($($args: expr),*)) => {
            {
                let f = $fun;
                let f = f.into_dynamic_function();

                let o = WorldCallbackAccess::with_callback_access(&mut $world, |world| {
                    let mut arg_list = ArgList::new();
                    arg_list = arg_list.push_arg(ArgValue::Owned(Box::new(world.clone())));
                    $(
                        arg_list = arg_list.push_arg($args);
                    )*
                    f.call(arg_list)
                }).expect("Failed to call function");

                match o {
                    Return::Owned(v) => v.try_take().expect("Failed to convert to target type"),
                    _ => panic!("Expected owned value")
                }
            }
        };
    }

    #[test]
    fn primitive_function_should_work() {
        let mut world = test_setup_world();

        let out: ScriptValue = call_script_function_with!(
            world,
            |a: usize, b: usize| a + b,
            (
                ArgValue::Owned(Box::new(ScriptValue::Integer(1))),
                ArgValue::Owned(Box::new(ScriptValue::Integer(1)))
            )
        );
        assert_eq!(out, ScriptValue::Integer(2));
    }

    #[test]
    fn primitive_result_function_should_work() {
        let mut world = test_setup_world();

        let out: ScriptValue = call_script_function_with!(
            world,
            |a: usize, b: usize| Ok(a + b),
            (
                ArgValue::Owned(Box::new(ScriptValue::Integer(1))),
                ArgValue::Owned(Box::new(ScriptValue::Integer(1)))
            )
        );
        assert_eq!(out, ScriptValue::Integer(2));

        let out: ScriptValue = call_script_function_with!(
            world,
            || Err::<usize, _>(InteropError::missing_world()),
            ()
        );
        assert!(matches!(out, ScriptValue::Error(_)));
    }

    #[test]
    fn primitive_function_with_world_should_work() {
        let mut world = test_setup_world();

        let out: ScriptValue = call_script_function_with!(
            world,
            |_w: WorldCallbackAccess, a: usize, b: usize| a + b,
            (
                ArgValue::Owned(Box::new(ScriptValue::Integer(1))),
                ArgValue::Owned(Box::new(ScriptValue::Integer(1)))
            )
        );
        assert_eq!(out, ScriptValue::Integer(2));
    }

    #[test]
    fn primitive_result_function_with_world_should_work() {
        let mut world = test_setup_world();

        let out: ScriptValue = call_script_function_with!(
            world,
            |_w: WorldCallbackAccess, a: usize, b: usize| Ok(a + b),
            (
                ArgValue::Owned(Box::new(ScriptValue::Integer(1))),
                ArgValue::Owned(Box::new(ScriptValue::Integer(1)))
            )
        );
        assert_eq!(out, ScriptValue::Integer(2));

        let out: ScriptValue = call_script_function_with!(
            world,
            || Err::<usize, _>(InteropError::missing_world()),
            ()
        );
        assert!(matches!(out, ScriptValue::Error(_)));
    }
}
