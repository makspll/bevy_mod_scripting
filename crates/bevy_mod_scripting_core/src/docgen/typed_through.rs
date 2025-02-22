//! Defines a set of traits which destruture [`bevy::reflect::TypeInfo`] and implement a light weight wrapper around it, to allow types
//! which normally can't implement [`bevy::reflect::Typed`] to be used in a reflection context.

use std::{any::TypeId, ffi::OsString, path::PathBuf};

use bevy::reflect::{TypeInfo, Typed};

use crate::{
    bindings::{
        function::{
            from::{Mut, Ref, Union, Val},
            script_function::{
                DynamicScriptFunction, DynamicScriptFunctionMut, FunctionCallContext,
            },
        },
        script_value::ScriptValue,
        ReflectReference,
    },
    error::InteropError,
};

/// All Through types follow one rule:
/// - A through type can not contain a nested through type. It must always contain a fully typed inner type.
///
/// This means that:
/// - `Ref<Ref<T>>` is not allowed, but `Ref<T>` is.
///
/// i.e. `Ref`, `Mut` and `Val` wrappers are `leaf` types, and can not contain other `leaf` types.
///
/// This is to keep the implementations of this trait simple, and to ultimately depend on the `TypeInfo` trait for the actual type information.
#[derive(Debug, Clone)]
pub enum ThroughTypeInfo {
    /// A wrapper around a typed type, which itself is not a `Typed` type.
    UntypedWrapper {
        /// The type information of the inner type.
        through_type: &'static TypeInfo,
        /// The type id of the wrapper type.
        wrapper_type_id: TypeId,
        /// The name of the wrapper type.
        wrapper_kind: UntypedWrapperKind,
    },
    /// A wrapper around a through typed type, which itself is also a `Typed` type.
    TypedWrapper(TypedWrapperKind),
    /// an actual type info
    TypeInfo(&'static TypeInfo),
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// The kind of untyped wrapper.
pub enum UntypedWrapperKind {
    /// A reference wrapper.
    Ref,
    /// A mutable reference wrapper.
    Mut,
    /// A value wrapper.
    Val,
}

/// The kind of typed wrapper.
#[derive(Debug, Clone)]
pub enum TypedWrapperKind {
    /// A union of many possible types
    Union(Vec<ThroughTypeInfo>),
    /// Wraps a `Vec` of a through typed type.
    Vec(Box<ThroughTypeInfo>),
    /// Wraps a `HashMap` of a through typed type.
    HashMap(Box<ThroughTypeInfo>, Box<ThroughTypeInfo>),
    /// Wraps a `Result` of a through typed type.
    Array(Box<ThroughTypeInfo>, usize),
    /// Wraps an `Option` of a through typed type.
    Option(Box<ThroughTypeInfo>),
    /// Wraps a `Result` of a through typed type.
    InteropResult(Box<ThroughTypeInfo>),
    /// Wraps a tuple of through typed types.
    Tuple(Vec<ThroughTypeInfo>),
}

/// A trait for types that can be converted to a [`ThroughTypeInfo`].
pub trait TypedThrough {
    /// Get the [`ThroughTypeInfo`] for the type.
    fn through_type_info() -> ThroughTypeInfo;
}

impl<T1: TypedThrough, T2: TypedThrough> TypedThrough for Union<T1, T2> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Union(vec![
            T1::through_type_info(),
            T2::through_type_info(),
        ]))
    }
}

impl<T: Typed> TypedThrough for Ref<'_, T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::UntypedWrapper {
            through_type: T::type_info(),
            wrapper_type_id: TypeId::of::<Ref<T>>(),
            wrapper_kind: UntypedWrapperKind::Ref,
        }
    }
}

impl<T: Typed> TypedThrough for Mut<'_, T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::UntypedWrapper {
            through_type: T::type_info(),
            wrapper_type_id: TypeId::of::<Mut<T>>(),
            wrapper_kind: UntypedWrapperKind::Mut,
        }
    }
}

impl<T: Typed> TypedThrough for Val<T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::UntypedWrapper {
            through_type: T::type_info(),
            wrapper_type_id: TypeId::of::<Val<T>>(),
            wrapper_kind: UntypedWrapperKind::Val,
        }
    }
}

impl<T: TypedThrough> TypedThrough for Vec<T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Vec(Box::new(T::through_type_info())))
    }
}

impl<K: TypedThrough, V: TypedThrough> TypedThrough for std::collections::HashMap<K, V> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::TypedWrapper(TypedWrapperKind::HashMap(
            Box::new(K::through_type_info()),
            Box::new(V::through_type_info()),
        ))
    }
}

impl<T: TypedThrough> TypedThrough for Result<T, InteropError> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::TypedWrapper(TypedWrapperKind::InteropResult(Box::new(
            T::through_type_info(),
        )))
    }
}

impl<T: TypedThrough, const N: usize> TypedThrough for [T; N] {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Array(Box::new(T::through_type_info()), N))
    }
}

impl<T: TypedThrough> TypedThrough for Option<T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Option(Box::new(T::through_type_info())))
    }
}

macro_rules! impl_through_typed {
    ($($ty:ty),*) => {
        $(
            impl $crate::docgen::typed_through::TypedThrough for $ty {
                fn through_type_info() -> $crate::docgen::typed_through::ThroughTypeInfo {
                    $crate::docgen::typed_through::ThroughTypeInfo::TypeInfo(<$ty as bevy::reflect::Typed>::type_info())
                }
            }
        )*
    };
}

impl_through_typed!(
    FunctionCallContext,
    ReflectReference,
    DynamicScriptFunctionMut,
    DynamicScriptFunction,
    ScriptValue,
    bool,
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
    f32,
    f64,
    usize,
    isize,
    String,
    PathBuf,
    OsString,
    char,
    &'static str
);

macro_rules! impl_through_typed_tuple {
    ($($ty:ident),*) => {
        impl<$($ty: TypedThrough),*> TypedThrough for ($($ty,)*) {
            fn through_type_info() -> ThroughTypeInfo {
                ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Tuple(vec![$($ty::through_type_info()),*]))
            }
        }
    };
}

bevy::utils::all_tuples!(impl_through_typed_tuple, 0, 13, T);

#[cfg(test)]
mod test {
    use super::*;

    fn assert_type_info_is_through<T: Typed + TypedThrough>() {
        let type_info = T::type_info();
        let through_type_info = T::through_type_info();

        match through_type_info {
            ThroughTypeInfo::TypeInfo(info) => {
                assert_eq!(info.type_id(), type_info.type_id());
                assert_eq!(info.type_path(), type_info.type_path());
            }
            _ => panic!("Expected ThroughTypeInfo::TypeInfo"),
        }
    }

    #[test]
    fn test_typed_through_primitives() {
        assert_type_info_is_through::<bool>();
        assert_type_info_is_through::<i8>();
        assert_type_info_is_through::<i16>();
        assert_type_info_is_through::<i32>();
        assert_type_info_is_through::<i64>();
        assert_type_info_is_through::<i128>();
        assert_type_info_is_through::<u8>();
        assert_type_info_is_through::<u16>();
        assert_type_info_is_through::<u32>();
        assert_type_info_is_through::<u64>();
        assert_type_info_is_through::<u128>();
        assert_type_info_is_through::<f32>();
        assert_type_info_is_through::<f64>();
        assert_type_info_is_through::<usize>();
        assert_type_info_is_through::<isize>();
        assert_type_info_is_through::<String>();
        assert_type_info_is_through::<PathBuf>();
        assert_type_info_is_through::<OsString>();
        assert_type_info_is_through::<char>();
        assert_type_info_is_through::<ReflectReference>();
        assert_type_info_is_through::<&'static str>();
    }

    #[test]
    fn test_typed_wrapper_outer_variant_matches() {
        assert!(matches!(
            Vec::<i32>::through_type_info(),
            ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Vec(..))
        ));

        assert!(matches!(
            std::collections::HashMap::<i32, f32>::through_type_info(),
            ThroughTypeInfo::TypedWrapper(TypedWrapperKind::HashMap(..))
        ));

        assert!(matches!(
            Result::<i32, InteropError>::through_type_info(),
            ThroughTypeInfo::TypedWrapper(TypedWrapperKind::InteropResult(..))
        ));

        assert!(matches!(
            <[i32; 3]>::through_type_info(),
            ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Array(..))
        ));

        assert!(matches!(
            Option::<i32>::through_type_info(),
            ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Option(..))
        ));

        assert!(matches!(
            <(i32, f32)>::through_type_info(),
            ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Tuple(..))
        ));
    }
}
