//! This module contains the [`GetTypeDependencies`] trait and its implementations for various types.

use super::{
    DynamicScriptFunction, DynamicScriptFunctionMut,
    from::{Mut, Ref, Union, Val},
    script_function::FunctionCallContext,
};
use crate::{ReflectReference, ScriptValue, error::InteropError};
use bevy_mod_scripting_derive::impl_get_type_dependencies;
use bevy_platform::collections::HashMap;
use bevy_reflect::{FromReflect, GetTypeRegistration, TypeRegistry, Typed};
use std::collections::HashMap as StdHashMap;
use std::{ffi::OsString, hash::Hash, path::PathBuf};

macro_rules! impl_get_type_dependencies_primitives {
    ($($ty:ty),*) => {
        $(
            impl_get_type_dependencies!(
                #[derive(GetTypeDependencies)]
                #[get_type_dependencies(bms_bindings_path="crate")]
                struct $ty where {}
            );
        )*
    };
}

impl_get_type_dependencies_primitives!(
    i8,
    i16,
    i32,
    i64,
    i128,
    u8,
    u16,
    u32,
    u64,
    u128,
    usize,
    isize,
    f32,
    f64,
    bool,
    ScriptValue,
    DynamicScriptFunction,
    DynamicScriptFunctionMut,
    InteropError,
    String,
    PathBuf,
    OsString,
    char
);

impl GetTypeDependencies for () {
    type Underlying = ();
    fn register_type_dependencies(registry: &mut TypeRegistry) {
        registry.register::<()>();
    }
}

impl GetTypeDependencies for &'static str {
    type Underlying = &'static str;
    fn register_type_dependencies(registry: &mut TypeRegistry) {
        registry.register::<&'static str>();
    }
}

/// Functionally identical to [`GetTypeRegistration`] but without the 'static bound
pub trait GetTypeDependencies {
    /// In the majority of the implementations, this will be `Self`
    /// However some types might be `facades` for other types, in which case this will be the underlying type
    type Underlying;

    /// Registers the type dependencies of the implementing type with the given [`TypeRegistry`].
    fn register_type_dependencies(registry: &mut TypeRegistry);
}

impl_get_type_dependencies!(
    #[derive(GetTypeDependencies)]
    #[get_type_dependencies(bms_bindings_path = "crate")]
    struct HashMap<K, V>
    where
        K::Underlying: FromReflect + Eq + Hash + Typed,
        V::Underlying: FromReflect + Typed, {}
);

impl_get_type_dependencies!(
    #[derive(GetTypeDependencies)]
    #[get_type_dependencies(bms_bindings_path = "crate")]
    struct StdHashMap<K, V>
    where
        K::Underlying: FromReflect + Eq + Hash + Typed,
        V::Underlying: FromReflect + Typed, {}
);

impl_get_type_dependencies!(
    #[derive(GetTypeDependencies)]
    #[get_type_dependencies(bms_bindings_path = "crate")]
    struct Result<T, E>
    where
        T::Underlying: FromReflect + Typed,
        E::Underlying: FromReflect + Typed, {}
);

impl_get_type_dependencies!(
    #[derive(GetTypeDependencies)]
    #[get_type_dependencies(bms_bindings_path = "crate")]
    struct Option<T>
    where
        T::Underlying: FromReflect + Typed, {}
);

impl_get_type_dependencies!(
    #[derive(GetTypeDependencies)]
    #[get_type_dependencies(bms_bindings_path = "crate")]
    struct Vec<T>
    where
        T::Underlying: FromReflect + Typed, {}
);

impl_get_type_dependencies!(
    #[derive(GetTypeDependencies)]
    #[get_type_dependencies(
        bms_bindings_path = "crate",
        underlying = "Result<T1::Underlying,T2::Underlying>"
    )]
    struct Union<T1, T2>
    where
        T1::Underlying: FromReflect + Typed,
        T2::Underlying: FromReflect + Typed, {}
);

impl_get_type_dependencies!(
    #[derive(GetTypeDependencies)]
    #[get_type_dependencies(bms_bindings_path = "crate", underlying = "T", dont_recurse)]
    struct Val<T> {}
);

impl_get_type_dependencies!(
    #[derive(GetTypeDependencies)]
    #[get_type_dependencies(bms_bindings_path = "crate", underlying = "T", dont_recurse)]
    struct Ref<'a, T> {}
);

impl_get_type_dependencies!(
    #[derive(GetTypeDependencies)]
    #[get_type_dependencies(bms_bindings_path = "crate", underlying = "T", dont_recurse)]
    struct Mut<'a, T> {}
);

impl_get_type_dependencies!(
    #[derive(GetTypeDependencies)]
    #[get_type_dependencies(bms_bindings_path = "crate")]
    struct ReflectReference {}
);

impl_get_type_dependencies!(
    #[derive(GetTypeDependencies)]
    #[get_type_dependencies(bms_bindings_path = "crate")]
    struct FunctionCallContext {}
);

impl<T, const N: usize> GetTypeDependencies for [T; N]
where
    T: GetTypeDependencies,
    T::Underlying: FromReflect + Typed,
{
    type Underlying = [T::Underlying; N];
    fn register_type_dependencies(registry: &mut TypeRegistry) {
        T::register_type_dependencies(registry);
    }
}

macro_rules! register_tuple_dependencies {
    ($($param:ident),*) => {
        impl <$($param),*> $crate::function::GetTypeDependencies for ($($param,)*) where
            $(
                $param: GetTypeDependencies,
                <$param>::Underlying: FromReflect + Typed + GetTypeRegistration,
            )*
        {
            type Underlying = ($(<$param as GetTypeDependencies>::Underlying,)*);
            fn register_type_dependencies(registry: &mut TypeRegistry) {
                $(
                    registry.register::<<$param>::Underlying>();
                    <$param>::register_type_dependencies(registry);
                )*
            }
        }
    };
}

variadics_please::all_tuples!(register_tuple_dependencies, 1, 14, T);

/// A trait collecting type dependency information for a whole function. Used to register everything used by a function with the type registry
pub trait GetFunctionTypeDependencies<Marker> {
    /// Registers the type dependencies of the implementing type with the given [`TypeRegistry`].
    fn register_type_dependencies(registry: &mut TypeRegistry);
}

macro_rules! impl_script_function_type_dependencies{
    ($( $param:ident ),* ) => {
        impl<F, $( $param,)* O > GetFunctionTypeDependencies<fn($($param),*) -> O> for F where
            O: GetTypeDependencies,
            F: Fn( $( $param ),* ) -> O,
            $(
                $param: GetTypeDependencies,
            )*
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

variadics_please::all_tuples!(impl_script_function_type_dependencies, 0, 13, T);
