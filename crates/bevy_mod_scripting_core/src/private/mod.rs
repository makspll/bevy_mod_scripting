//! This module contains private functions and modules that are used internally by the crate.

macro_rules! export_all_in_modules {
    ( $( $module:ident ),* $(,)? ) => {
        $(
            pub mod $module;
        )*
        pub use {
            $( $module::*, )*
        };
    };
}

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

pub(crate) use {export_all_in_modules, no_type_dependencies, self_type_dependency_only};
