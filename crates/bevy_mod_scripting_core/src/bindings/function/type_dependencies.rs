//! This module contains the [`GetTypeDependencies`] trait and its implementations for various types.

use super::{
    from::{Mut, Ref, Union, Val},
    script_function::FunctionCallContext,
};
use crate::{
    bindings::{ReflectReference, WorldGuard},
    error::InteropError,
};
use bevy::reflect::{FromReflect, GetTypeRegistration, TypeRegistry, Typed};
use std::collections::HashMap;
use std::hash::Hash;

/// Functionally identical to [`GetTypeRegistration`] but without the 'static bound
pub trait GetTypeDependencies {
    /// Registers the type dependencies of the implementing type with the given [`TypeRegistry`].
    fn register_type_dependencies(registry: &mut TypeRegistry);
}

#[macro_export]
/// A macro for implementing [`GetTypeDependencies`] for types with no type dependencies.
macro_rules! no_type_dependencies {
    ($($path:path),*) => {
        $(
            impl $crate::bindings::function::type_dependencies::GetTypeDependencies for $path {
                fn register_type_dependencies(_registry: &mut bevy::reflect::TypeRegistry) {}
            }
        )*
    };
}

#[macro_export]
/// A macro for implementing [`GetTypeDependencies`] for types that only depend on themselves.
macro_rules! self_type_dependency_only {
    ($($path:ty),*) => {
        $(
            impl $crate::bindings::function::type_dependencies::GetTypeDependencies for $path {
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
            impl<$($bound : $($bound_val +)*),* , $(const $const : $const_ty )?> GetTypeDependencies for $path {
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
        impl<$($ty: GetTypeRegistration + Typed),*> GetTypeDependencies for ($($ty,)*) {
            fn register_type_dependencies(registry: &mut TypeRegistry) {
                $(
                    registry.register::<$ty>();
                )*
            }
        }
    };
}

no_type_dependencies!(InteropError);
no_type_dependencies!(WorldGuard<'static>);
self_type_dependency_only!(FunctionCallContext, ReflectReference);

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

impl<T1: GetTypeDependencies, T2: GetTypeDependencies> GetTypeDependencies for Union<T1, T2> {
    fn register_type_dependencies(registry: &mut TypeRegistry) {
        T1::register_type_dependencies(registry);
        T2::register_type_dependencies(registry);
    }
}

bevy::utils::all_tuples!(register_tuple_dependencies, 1, 14, T);

/// A trait collecting type dependency information for a whole function. Used to register everything used by a function with the type registry
pub trait GetFunctionTypeDependencies<Marker> {
    /// Registers the type dependencies of the implementing type with the given [`TypeRegistry`].
    fn register_type_dependencies(registry: &mut TypeRegistry);
}

macro_rules! impl_script_function_type_dependencies{
    ($( $param:ident ),* ) => {
        impl<F, $( $param: GetTypeDependencies ,)* O: GetTypeDependencies> GetFunctionTypeDependencies<fn($($param),*) -> O> for F
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

bevy::utils::all_tuples!(impl_script_function_type_dependencies, 0, 13, T);
