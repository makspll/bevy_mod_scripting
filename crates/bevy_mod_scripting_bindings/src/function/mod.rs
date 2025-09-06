//! Abstractions to do with dynamic script functions

pub mod arg_meta;
pub mod from;
pub mod from_ref;
pub mod into;
pub mod into_ref;
pub mod magic_functions;
pub mod namespace;
pub mod script_function;
pub mod type_dependencies;

pub use arg_meta::*;
pub use from::*;
pub use from_ref::*;
pub use into::*;
pub use into_ref::*;
pub use magic_functions::*;
pub use namespace::*;
pub use script_function::*;
pub use type_dependencies::*;

#[cfg(test)]
#[allow(dead_code)]
mod test {
    use bevy_ecs::world::World;
    use bevy_mod_scripting_derive::script_bindings;
    use bevy_platform::collections::HashMap;
    use bevy_reflect::{FromReflect, GetTypeRegistration, Reflect, Typed};

    use crate::{
        function::{
            from::{Ref, Union, Val},
            namespace::IntoNamespace,
            script_function::AppScriptFunctionRegistry,
        },
        script_value::ScriptValue,
    };
    use bevy_ecs::prelude::AppTypeRegistry;

    use super::arg_meta::{ScriptArgument, ScriptReturn, TypedScriptArgument, TypedScriptReturn};
    #[test]
    fn test_macro_generates_correct_registrator_function() {
        #[derive(Reflect)]
        struct TestStruct;

        #[script_bindings(bms_bindings_path = "crate", name = "test_fn")]
        impl TestStruct {
            /// My docs !!
            fn test_fn(_self: Ref<TestStruct>, mut _arg1: usize) {}
        }

        let mut test_world = World::default();

        register_test_fn(&mut test_world);

        let app_registry = test_world
            .get_resource::<AppScriptFunctionRegistry>()
            .unwrap();
        let app_registry = app_registry.read();

        let test_fn = app_registry
            .get_function(TestStruct::into_namespace(), "test_fn")
            .unwrap();

        assert_eq!(test_fn.info.docs, Some("My docs !!".into()));
        assert_eq!(test_fn.info.arg_info.len(), 2);

        assert_eq!(
            test_fn.info.arg_info[0].type_id,
            std::any::TypeId::of::<Ref<TestStruct>>()
        );
        assert_eq!(test_fn.info.arg_info[0].name, Some("_self".into()));

        assert_eq!(
            test_fn.info.arg_info[1].type_id,
            std::any::TypeId::of::<usize>()
        );
        assert_eq!(test_fn.info.arg_info[1].name, Some("_arg1".into()));

        assert_eq!(
            test_fn.info.return_info.type_id,
            std::any::TypeId::of::<()>()
        );
    }

    fn test_is_valid_return<T: TypedScriptReturn>() {}
    fn test_is_valid_arg<T: TypedScriptArgument>() {}
    fn test_is_valid_arg_and_return<T: TypedScriptReturn + TypedScriptArgument>() {}

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
        test_is_valid_arg_and_return::<ScriptValue>();
    }

    #[test]
    fn strings_are_valid_args() {
        test_is_valid_arg_and_return::<String>();
        test_is_valid_arg_and_return::<std::path::PathBuf>();
        test_is_valid_arg_and_return::<std::ffi::OsString>();
        test_is_valid_arg_and_return::<char>();
        test_is_valid_return::<&'static str>();
    }

    #[test]
    fn composites_are_valid_args() {
        test_is_valid_arg::<Union<usize, usize>>();

        fn test_val<T>()
        where
            T: ScriptArgument + ScriptReturn,
            T: GetTypeRegistration + FromReflect + Typed,
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

        fn test_union<T>()
        where
            T: TypedScriptArgument + TypedScriptReturn,
            T::Underlying: FromReflect + Typed + GetTypeRegistration,
            for<'a> T::This<'a>: Into<T>,
        {
            test_is_valid_arg_and_return::<Union<T, T>>();
            test_is_valid_arg_and_return::<Union<T, Union<T, T>>>();
        }

        fn test_array<T, const N: usize>()
        where
            T: TypedScriptArgument + TypedScriptReturn + 'static,
            T::Underlying: FromReflect + Typed + GetTypeRegistration,
            for<'a> T::This<'a>: Into<T>,
        {
            test_is_valid_arg_and_return::<[T; N]>();
        }

        fn test_tuple<T>()
        where
            T: TypedScriptArgument + TypedScriptReturn + 'static,
            T::Underlying: FromReflect + Typed + GetTypeRegistration,
            for<'a> T::This<'a>: Into<T>,
        {
            test_is_valid_arg_and_return::<()>();
            test_is_valid_arg_and_return::<(T,)>();
            test_is_valid_arg_and_return::<(T, T)>();
            test_is_valid_arg_and_return::<(T, T, T, T, T, T, T, T, T, T)>();
        }

        fn test_option<T>()
        where
            T: TypedScriptArgument + TypedScriptReturn,
            T::Underlying: FromReflect + Typed + GetTypeRegistration,
            for<'a> T::This<'a>: Into<T>,
        {
            test_is_valid_arg_and_return::<Option<T>>();
        }

        fn test_vec<T>()
        where
            T: TypedScriptArgument + TypedScriptReturn + 'static,
            T::Underlying: FromReflect + Typed + GetTypeRegistration,
            for<'a> T::This<'a>: Into<T>,
        {
            test_is_valid_arg_and_return::<Vec<T>>();
        }

        fn test_hashmap<V>()
        where
            V: TypedScriptArgument + TypedScriptReturn + 'static,
            V::Underlying: FromReflect + Typed + GetTypeRegistration + Eq,
            for<'a> V::This<'a>: Into<V>,
        {
            test_is_valid_arg_and_return::<HashMap<String, V>>();
        }
    }

    #[test]
    fn test_dynamic_functions() {
        test_is_valid_arg_and_return::<crate::function::script_function::DynamicScriptFunction>();
        test_is_valid_arg_and_return::<crate::function::script_function::DynamicScriptFunctionMut>(
        );
    }
}
