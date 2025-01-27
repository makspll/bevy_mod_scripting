pub mod arg_meta;
pub mod from;
pub mod from_ref;
pub mod into;
pub mod into_ref;
pub mod namespace;
pub mod script_function;
pub mod type_dependencies;

#[cfg(test)]
#[allow(dead_code)]
mod test {
    use bevy::reflect::{FromReflect, GetTypeRegistration, Typed};

    use crate::{
        bindings::function::from::{Ref, Val},
        error::InteropError,
    };

    use super::arg_meta::{ScriptArgument, ScriptReturn};

    fn test_is_valid_return<T: ScriptReturn>() {}
    fn test_is_valid_arg<T: ScriptArgument>() {}
    fn test_is_valid_arg_and_return<T: ScriptArgument + ScriptReturn>() {}

    #[test]
    fn primitives_are_valid_args() {
        test_is_valid_arg_and_return::<bool>();
        test_is_valid_arg_and_return::<i8>();
        test_is_valid_arg_and_return::<i16>();
        test_is_valid_arg_and_return::<i32>();
        test_is_valid_arg_and_return::<i64>();
        test_is_valid_arg_and_return::<i128>();
        test_is_valid_arg_and_return::<u8>();
        test_is_valid_arg_and_return::<u16>();
        test_is_valid_arg_and_return::<u32>();
        test_is_valid_arg_and_return::<u64>();
        test_is_valid_arg_and_return::<u128>();
        test_is_valid_arg_and_return::<f32>();
        test_is_valid_arg_and_return::<f64>();
        test_is_valid_arg_and_return::<usize>();
        test_is_valid_arg_and_return::<isize>();
    }

    #[test]
    fn strings_are_valid_args() {
        test_is_valid_arg_and_return::<String>();
        test_is_valid_arg_and_return::<std::path::PathBuf>();
        test_is_valid_arg_and_return::<std::ffi::OsString>();
        test_is_valid_arg_and_return::<char>();
    }

    #[test]
    fn composites_are_valid_args() {
        fn test_val<T>()
        where
            T: ScriptArgument + ScriptReturn,
            T: GetTypeRegistration + FromReflect,
        {
            test_is_valid_arg_and_return::<Val<T>>();
        }

        fn test_ref<T>()
        where
            T: ScriptArgument,
            T: GetTypeRegistration + FromReflect + Typed,
        {
            test_is_valid_arg::<Ref<'_, T>>();
        }

        fn test_mut<T>()
        where
            T: ScriptArgument,
            T: GetTypeRegistration + FromReflect + Typed,
        {
            test_is_valid_arg::<Ref<'_, T>>();
        }

        test_is_valid_return::<InteropError>();

        fn test_array<T, const N: usize>()
        where
            T: ScriptArgument + ScriptReturn,
            T: GetTypeRegistration + FromReflect + Typed,
            for<'a> T::This<'a>: Into<T>,
        {
            test_is_valid_arg_and_return::<[T; N]>();
        }

        fn test_tuple<T>()
        where
            T: ScriptArgument + ScriptReturn,
            T: GetTypeRegistration + FromReflect + Typed,
            for<'a> T::This<'a>: Into<T>,
        {
            test_is_valid_arg_and_return::<()>();
            test_is_valid_return::<(T,)>();
            test_is_valid_return::<(T, T)>();
            test_is_valid_return::<(T, T, T, T, T, T, T, T, T, T)>();
        }

        fn test_option<T>()
        where
            T: ScriptArgument + ScriptReturn,
            T: GetTypeRegistration + FromReflect + Typed,
            for<'a> T::This<'a>: Into<T>,
        {
            test_is_valid_arg_and_return::<Option<T>>();
        }

        fn test_vec<T>()
        where
            T: ScriptArgument + ScriptReturn,
            T: GetTypeRegistration + FromReflect + Typed,
            for<'a> T::This<'a>: Into<T>,
        {
            test_is_valid_arg_and_return::<Vec<T>>();
        }

        fn test_hashmap<V>()
        where
            V: ScriptArgument + ScriptReturn,
            V: GetTypeRegistration + FromReflect + Typed,
            for<'a> V::This<'a>: Into<V>,
        {
            test_is_valid_arg_and_return::<std::collections::HashMap<String, V>>();
        }
    }

    #[test]
    fn test_dynamic_functions() {
        test_is_valid_arg_and_return::<
            crate::bindings::function::script_function::DynamicScriptFunction,
        >();
        test_is_valid_arg_and_return::<
            crate::bindings::function::script_function::DynamicScriptFunctionMut,
        >();
    }
}
