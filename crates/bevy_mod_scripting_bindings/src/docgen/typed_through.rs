//! Defines a set of traits which destruture [`bevy_reflect::TypeInfo`] and implement a light weight wrapper around it, to allow types
//! which normally can't implement [`bevy_reflect::Typed`] to be used in a reflection context.

use std::{any::TypeId, ffi::OsString, path::PathBuf};

use crate::{
    ReflectReference,
    function::{
        from::{M, R, Union, V},
        script_function::{DynamicScriptFunction, DynamicScriptFunctionMut, FunctionCallContext},
    },
    script_value::ScriptValue,
};
use crate::{error::InteropError, reflection_extensions::TypeInfoExtensions};
use bevy_mod_scripting_bindings_domain::ReflectionPrimitiveKind;
use bevy_mod_scripting_derive::DebugWithTypeInfo;
use bevy_platform::collections::HashMap;
use bevy_reflect::{TypeInfo, Typed};

/// All Through types follow one rule:
/// - A through type can not contain a nested through type. It must always contain a fully typed inner type.
///
/// This means that:
/// - `R<R<T>>` is not allowed, but `R<T>` is.
///
/// i.e. `R`, `M` and `V` wrappers are `leaf` types, and can not contain other `leaf` types.
///
/// This is to keep the implementations of this trait simple, and to ultimately depend on the `TypeInfo` trait for the actual type information.
#[derive(Clone, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
pub enum ThroughTypeInfo {
    /// A wrapper around a typed type, which itself is not a `Typed` type.
    UntypedWrapper {
        /// The type information of the inner type.
        through_type: &'static TypeInfo,
        /// The name of the wrapper type.
        wrapper_kind: UntypedWrapperKind,
    },
    /// A wrapper around a through typed type, which itself is also a `Typed` type.
    TypedWrapper(TypedWrapperKind),
    /// an actual type info
    TypeInfo(&'static TypeInfo),
    /// A primitive type, which mostly speaks for itself
    Primitive(ReflectionPrimitiveKind),
}

#[derive(Clone, PartialEq, Eq, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
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
#[derive(Clone, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
pub enum TypedWrapperKind {
    /// A union of many possible types
    Union(Vec<ThroughTypeInfo>),
    /// Wraps a `Vec` of a through typed type.
    Vec(Box<ThroughTypeInfo>),
    /// Wraps a `HashMap` of a through typed type.
    HashMap(Box<ThroughTypeInfo>, Box<ThroughTypeInfo>),
    /// Wraps a `HashSet` of a through typed type.
    HashSet(Box<ThroughTypeInfo>),
    /// Wraps a `Result` of a through typed type.
    Array(Box<ThroughTypeInfo>, usize),
    /// Wraps an `Option` of a through typed type.
    Option(Box<ThroughTypeInfo>),
    /// Wraps a `Result` of a through typed type.
    InteropResult(Box<ThroughTypeInfo>),
    /// Wraps a tuple of through typed types.
    Tuple(Vec<ThroughTypeInfo>),
}

/// A dynamic version of [`TypedThrough`], which can be used to convert a [`TypeInfo`] into a [`ThroughTypeInfo`].
pub fn into_through_type_info(type_info: &'static TypeInfo) -> ThroughTypeInfo {
    let option = (|| {
        if let Ok(array) = type_info.as_array() {
            let len = array.capacity();
            let inner = array.item_info()?;
            return Some(ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Array(
                Box::new(into_through_type_info(inner)),
                len,
            )));
        } else if let Ok(hash_map) = type_info.as_map() {
            let key_type = hash_map.key_info()?;
            let value_type = hash_map.value_info()?;

            return Some(ThroughTypeInfo::TypedWrapper(TypedWrapperKind::HashMap(
                Box::new(into_through_type_info(key_type)),
                Box::new(into_through_type_info(value_type)),
            )));
        } else if let Ok(list) = type_info.as_list() {
            let inner = list.item_info()?;
            return Some(ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Vec(
                Box::new(into_through_type_info(inner)),
            )));
        } else if type_info.is_option() {
            let enum_ = type_info.as_enum().ok()?;
            let inner = enum_.variant("Some")?;
            let inner = inner.as_tuple_variant().ok()?;
            let inner = inner.field_at(0)?;
            let inner = inner.type_info()?;
            return Some(ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Option(
                Box::new(into_through_type_info(inner)),
            )));
        } else if type_info.is_result() {
            let enum_ = type_info.as_enum().ok()?;
            // let error_variant = enum_.variant("Err")?;
            // TODO verify error variant is InteropError

            let inner = enum_.variant("Ok")?;
            let inner = inner.as_tuple_variant().ok()?;
            let inner = inner.field_at(0)?;
            let inner = inner.type_info()?;
            return Some(ThroughTypeInfo::TypedWrapper(
                TypedWrapperKind::InteropResult(Box::new(into_through_type_info(inner))),
            ));
        } else if let Ok(tuple) = type_info.as_tuple() {
            let mut tuple_types = Vec::new();
            for i in 0..tuple.field_len() {
                let field = tuple.field_at(i)?;
                let field_type = field.type_info()?;
                tuple_types.push(into_through_type_info(field_type));
            }
            return Some(ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Tuple(
                tuple_types,
            )));
        } else if let Some(primitive) = as_reflect_primitive(type_info) {
            return Some(ThroughTypeInfo::Primitive(primitive));
        }
        None
    })();

    option.unwrap_or(ThroughTypeInfo::UntypedWrapper {
        through_type: type_info,
        wrapper_kind: UntypedWrapperKind::Val,
    })
}

/// Returns the primitive kind if the given type info corresponds to one
pub fn as_reflect_primitive(type_info: &'static TypeInfo) -> Option<ReflectionPrimitiveKind> {
    let type_id = type_info.type_id();
    Some(if type_id == TypeId::of::<bool>() {
        ReflectionPrimitiveKind::Bool
    } else if type_id == TypeId::of::<isize>() {
        ReflectionPrimitiveKind::Isize
    } else if type_id == TypeId::of::<i8>() {
        ReflectionPrimitiveKind::I8
    } else if type_id == TypeId::of::<i16>() {
        ReflectionPrimitiveKind::I16
    } else if type_id == TypeId::of::<i32>() {
        ReflectionPrimitiveKind::I32
    } else if type_id == TypeId::of::<i64>() {
        ReflectionPrimitiveKind::I64
    } else if type_id == TypeId::of::<i128>() {
        ReflectionPrimitiveKind::I128
    } else if type_id == TypeId::of::<usize>() {
        ReflectionPrimitiveKind::Usize
    } else if type_id == TypeId::of::<u8>() {
        ReflectionPrimitiveKind::U8
    } else if type_id == TypeId::of::<u16>() {
        ReflectionPrimitiveKind::U16
    } else if type_id == TypeId::of::<u32>() {
        ReflectionPrimitiveKind::U32
    } else if type_id == TypeId::of::<u64>() {
        ReflectionPrimitiveKind::U64
    } else if type_id == TypeId::of::<u128>() {
        ReflectionPrimitiveKind::U128
    } else if type_id == TypeId::of::<f32>() {
        ReflectionPrimitiveKind::F32
    } else if type_id == TypeId::of::<f64>() {
        ReflectionPrimitiveKind::F64
    } else if type_id == TypeId::of::<char>() {
        ReflectionPrimitiveKind::Char
    } else if type_id == TypeId::of::<&'static str>() || type_id == TypeId::of::<str>() {
        ReflectionPrimitiveKind::Str
    } else if type_id == TypeId::of::<String>() {
        ReflectionPrimitiveKind::String
    } else if type_id == TypeId::of::<OsString>() {
        ReflectionPrimitiveKind::OsString
    } else if type_id == TypeId::of::<PathBuf>() {
        ReflectionPrimitiveKind::PathBuf
    } else if type_id == TypeId::of::<FunctionCallContext>() {
        ReflectionPrimitiveKind::FunctionCallContext
    } else if type_id == TypeId::of::<DynamicScriptFunction>() {
        ReflectionPrimitiveKind::DynamicFunction
    } else if type_id == TypeId::of::<DynamicScriptFunctionMut>() {
        ReflectionPrimitiveKind::DynamicFunctionMut
    } else if type_id == TypeId::of::<ReflectReference>() {
        ReflectionPrimitiveKind::ReflectReference
    } else {
        return None;
    })
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

impl<T: Typed> TypedThrough for R<'_, T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::UntypedWrapper {
            through_type: T::type_info(),
            wrapper_kind: UntypedWrapperKind::Ref,
        }
    }
}

impl<T: Typed> TypedThrough for M<'_, T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::UntypedWrapper {
            through_type: T::type_info(),
            wrapper_kind: UntypedWrapperKind::Mut,
        }
    }
}

impl<T: Typed> TypedThrough for V<T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::UntypedWrapper {
            through_type: T::type_info(),
            wrapper_kind: UntypedWrapperKind::Val,
        }
    }
}

impl<T: TypedThrough> TypedThrough for Vec<T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Vec(Box::new(T::through_type_info())))
    }
}

impl<K: TypedThrough, V: TypedThrough> TypedThrough for HashMap<K, V> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::TypedWrapper(TypedWrapperKind::HashMap(
            Box::new(K::through_type_info()),
            Box::new(V::through_type_info()),
        ))
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
    ($($ty:ty => $ident:ident),*) => {
        $(
            impl $crate::docgen::typed_through::TypedThrough for $ty {
                fn through_type_info() -> $crate::docgen::typed_through::ThroughTypeInfo {
                    $crate::docgen::typed_through::ThroughTypeInfo::Primitive(ReflectionPrimitiveKind::$ident)
                }
            }
        )*
    };
}

impl_through_typed!(
    FunctionCallContext => FunctionCallContext,
    ReflectReference => ReflectReference,
    DynamicScriptFunctionMut => DynamicFunctionMut,
    DynamicScriptFunction => DynamicFunction,
    ScriptValue => ScriptValue,
    bool => Bool,
    i8 => I8,
    i16 => I16,
    i32 => I32,
    i64 => I64,
    i128 => I128,
    u8 => U8,
    u16 => U16,
    u32 => U32,
    u64 => U64,
    u128 => U128,
    f32 => F32,
    f64 => F64,
    usize => Usize,
    isize => Isize,
    String => String,
    PathBuf => PathBuf,
    OsString => OsString,
    char => Char,
    &'static str => Str
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

variadics_please::all_tuples!(impl_through_typed_tuple, 0, 13, T);

#[cfg(test)]
mod test {
    use super::*;

    fn assert_type_info_is_primitive<T: Typed + TypedThrough>(kind: ReflectionPrimitiveKind) {
        let type_info = T::type_info();
        let through_type_info = T::through_type_info();
        let dynamic_through_type_info = into_through_type_info(type_info);

        for (test, info) in [
            ("static", through_type_info),
            ("dynamic", dynamic_through_type_info),
        ] {
            match info {
                ThroughTypeInfo::Primitive(prim) => {
                    assert_eq!(
                        prim,
                        kind,
                        "expected {} to have primitive {test} type info: {kind}",
                        std::any::type_name::<T>()
                    )
                }
                _ => panic!("Expected ThroughTypeInfo::TypeInfo"),
            }
        }
    }

    #[test]
    fn test_typed_through_primitives() {
        assert_type_info_is_primitive::<bool>(ReflectionPrimitiveKind::Bool);
        assert_type_info_is_primitive::<i8>(ReflectionPrimitiveKind::I8);
        assert_type_info_is_primitive::<i16>(ReflectionPrimitiveKind::I16);
        assert_type_info_is_primitive::<i32>(ReflectionPrimitiveKind::I32);
        assert_type_info_is_primitive::<i64>(ReflectionPrimitiveKind::I64);
        assert_type_info_is_primitive::<i128>(ReflectionPrimitiveKind::I128);
        assert_type_info_is_primitive::<u8>(ReflectionPrimitiveKind::U8);
        assert_type_info_is_primitive::<u16>(ReflectionPrimitiveKind::U16);
        assert_type_info_is_primitive::<u32>(ReflectionPrimitiveKind::U32);
        assert_type_info_is_primitive::<u64>(ReflectionPrimitiveKind::U64);
        assert_type_info_is_primitive::<u128>(ReflectionPrimitiveKind::U128);
        assert_type_info_is_primitive::<f32>(ReflectionPrimitiveKind::F32);
        assert_type_info_is_primitive::<f64>(ReflectionPrimitiveKind::F64);
        assert_type_info_is_primitive::<usize>(ReflectionPrimitiveKind::Usize);
        assert_type_info_is_primitive::<isize>(ReflectionPrimitiveKind::Isize);
        assert_type_info_is_primitive::<String>(ReflectionPrimitiveKind::String);
        assert_type_info_is_primitive::<PathBuf>(ReflectionPrimitiveKind::PathBuf);
        assert_type_info_is_primitive::<OsString>(ReflectionPrimitiveKind::OsString);
        assert_type_info_is_primitive::<char>(ReflectionPrimitiveKind::Char);
        assert_type_info_is_primitive::<ReflectReference>(
            ReflectionPrimitiveKind::ReflectReference,
        );
        assert_type_info_is_primitive::<&'static str>(ReflectionPrimitiveKind::Str);
    }

    #[test]
    fn test_typed_wrapper_outer_variant_matches() {
        assert!(matches!(
            Vec::<i32>::through_type_info(),
            ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Vec(..))
        ));

        assert!(matches!(
            HashMap::<i32, f32>::through_type_info(),
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

    #[test]
    fn test_dynamic_typed_wrapper_outer_variant_matches() {
        assert!(matches!(
            into_through_type_info(Vec::<i32>::type_info()),
            ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Vec(..))
        ));

        assert!(matches!(
            into_through_type_info(HashMap::<i32, f32>::type_info()),
            ThroughTypeInfo::TypedWrapper(TypedWrapperKind::HashMap(..))
        ));

        assert!(matches!(
            into_through_type_info(Result::<i32, InteropError>::type_info()),
            ThroughTypeInfo::TypedWrapper(TypedWrapperKind::InteropResult(..))
        ));

        assert!(matches!(
            into_through_type_info(<[i32; 3]>::type_info()),
            ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Array(..))
        ));

        assert!(matches!(
            into_through_type_info(Option::<i32>::type_info()),
            ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Option(..))
        ));

        assert!(matches!(
            into_through_type_info(<(i32, f32)>::type_info()),
            ThroughTypeInfo::TypedWrapper(TypedWrapperKind::Tuple(..))
        ));
    }
}
