use bevy::{
    prelude::{AppFunctionRegistry, IntoFunction, World},
    reflect::{func::DynamicFunction, PartialReflect},
};

use crate::{
    error::InteropError,
    prelude::{ScriptValue, WorldCallbackAccess},
};

use super::{from::FromScript, into::IntoScript};

#[diagnostic::on_unimplemented(
    message = "Only functions with all arguments impplementing FromScript and return values supporting IntoScript are supported. use assert_impls_into_script!(MyArg) and assert_impls_from_script!(MyReturnType) to verify yours do.",
    note = "If you're trying to return a non-primitive type, you might need to use Val<T> Ref<T> or Mut<T> wrappers"
)]
pub trait ScriptFunction<Marker> {
    fn into_dynamic_function(self) -> DynamicFunction<'static>;
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
            $( $param: FromScript, )*
            O,
            F
        > ScriptFunction<
            fn( $( $callbackty, )? $($param ),* ) -> $res
        > for F
        where
            O: IntoScript,
            F: Fn( $( $callbackty, )? $($param ),* ) -> $res + Send + Sync + 'static,
            $( for<'a> $param::This<'a>: Into<$param>, )*
        {
            fn into_dynamic_function(self) -> DynamicFunction<'static> {
                (move |world: WorldCallbackAccess, $( $param: ScriptValue ),* | {
                    let res: Result<ScriptValue, InteropError> = (|| {
                        $( let $callback = world.clone(); )?
                        let world = world.read().ok_or_else(|| InteropError::stale_world_access())?;
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

bevy::utils::all_tuples!(impl_script_function, 0, 14, T);

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
    () => {};
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
        fn _check<M,F: ScriptFunction<M>>(f: F) {

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
