//! Describes the types of script function arguments and returns for documentation, see [`TypedThrough`].
//!
//! Wrappers like [`V`], [`R`] and [`M`] are not [`Typed`] themselves, so the description is built from the [`TypeInfo`] of the types they wrap.
//! [`ThroughTypeInfo`] mirrors the LAD file's type kinds, with the type information of the leaf types in place of their ids.

use std::{any::TypeId, collections::VecDeque, ffi::OsString, path::PathBuf};

use crate::{
    ReflectReference, StaticTypeRef, VariadicTuple,
    function::{
        from::{M, R, Union, V},
        script_function::{DynamicScriptFunction, DynamicScriptFunctionMut, FunctionCallContext},
    },
    script_value::ScriptValue,
};
use crate::{error::InteropError, reflection_extensions::TypeInfoExtensions};
use bevy_mod_scripting_bindings_domain::ReflectionPrimitiveKind;
use bevy_mod_scripting_derive::DebugWithTypeInfo;
use bevy_platform::collections::{HashMap, HashSet};
use bevy_reflect::{TypeInfo, TypeRegistry, Typed};

/// How a type is passed to or from a script function, for documentation.
///
/// Leaves carry the [`TypeInfo`] of the type, containers the description of what they contain.
#[derive(Clone, DebugWithTypeInfo)]
#[debug_with_type_info(bms_display_path = "bevy_mod_scripting_display")]
pub enum ThroughTypeInfo {
    /// A reference to a value, i.e. [`R`]
    Ref(&'static TypeInfo),
    /// A mutable reference to a value, i.e. [`M`]
    Mut(&'static TypeInfo),
    /// A value, i.e. [`V`], or a type whose conversions are derived and so behave like one
    Val(&'static TypeInfo),
    /// A type used directly in a signature, converted by its own `FromScript`/`IntoScript`, so its representation in scripts is unknown.
    ///
    /// Produced by `#[derive(TypedThrough)]`, see its `value` attribute for types with derived conversions.
    Opaque(&'static TypeInfo),
    /// A primitive type, which mostly speaks for itself
    Primitive(ReflectionPrimitiveKind),
    /// An optional value
    Option(Box<ThroughTypeInfo>),
    /// A list, i.e. a `Vec` or `VecDeque`
    Vec(Box<ThroughTypeInfo>),
    /// A map
    HashMap(Box<ThroughTypeInfo>, Box<ThroughTypeInfo>),
    /// A set
    HashSet(Box<ThroughTypeInfo>),
    /// A fixed size array
    Array(Box<ThroughTypeInfo>, usize),
    /// A value which might also be an error, i.e. `Result<T, InteropError>`
    InteropResult(Box<ThroughTypeInfo>),
    /// A tuple
    Tuple(Vec<ThroughTypeInfo>),
    /// Any number of script values, i.e. [`VariadicTuple`]
    UntypedTuple,
    /// A list of values of the given types followed by any number of values of one type, i.e. a [`crate::ScriptQueryResult`]
    TupleWithRest {
        /// The types of the first values
        head: Vec<ThroughTypeInfo>,
        /// The type of the values after them
        rest: Box<ThroughTypeInfo>,
    },
    /// One of many possible types
    Union(Vec<ThroughTypeInfo>),
    /// A table of the fields of a type, i.e. the payload of a constructor.
    Fields {
        /// The type the fields belong to, a value or a generic parameter
        of: Box<ThroughTypeInfo>,
        /// For enums, the variant the fields belong to
        variant: Option<&'static str>,
    },
    /// A value of the type a generic parameter of the function stands for, i.e. [`crate::function::generics::A`]
    Param(&'static str),
    /// A generic type applied to arguments, i.e. `Handle<A>`, or `ComponentType<A>` for a component type itself.
    ///
    /// Reflection can't name a generic type without its arguments, so it is named by its path, see [`crate::function::generics`].
    Generic {
        /// The path of the generic type, without its arguments
        path: &'static str,
        /// Its arguments
        args: Vec<ThroughTypeInfo>,
    },
}

impl ThroughTypeInfo {
    /// The descriptions this one contains, i.e. `T` in `Vec<T>`.
    pub fn children(&self) -> Vec<&ThroughTypeInfo> {
        match self {
            ThroughTypeInfo::Option(inner)
            | ThroughTypeInfo::Vec(inner)
            | ThroughTypeInfo::HashSet(inner)
            | ThroughTypeInfo::Array(inner, _)
            | ThroughTypeInfo::InteropResult(inner) => vec![&**inner],
            ThroughTypeInfo::HashMap(key, value) => vec![&**key, &**value],
            ThroughTypeInfo::Tuple(inner) | ThroughTypeInfo::Union(inner) => inner.iter().collect(),
            ThroughTypeInfo::TupleWithRest { head, rest } => {
                head.iter().chain(std::iter::once(&**rest)).collect()
            }
            ThroughTypeInfo::Fields { of, .. } => vec![&**of],
            ThroughTypeInfo::Generic { args, .. } => args.iter().collect(),
            ThroughTypeInfo::Ref(_)
            | ThroughTypeInfo::Mut(_)
            | ThroughTypeInfo::Val(_)
            | ThroughTypeInfo::Opaque(_)
            | ThroughTypeInfo::Primitive(_)
            | ThroughTypeInfo::UntypedTuple
            | ThroughTypeInfo::Param(_) => vec![],
        }
    }

    /// Replaces the generic parameters with the given type, and for fields, the variant.
    ///
    /// Used for signatures which depend on what they are bound to, i.e. the constructor of each enum variant.
    pub fn bind_generics(
        &self,
        type_info: &'static TypeInfo,
        variant: Option<&'static str>,
    ) -> ThroughTypeInfo {
        let bind = |inner: &ThroughTypeInfo| Box::new(inner.bind_generics(type_info, variant));
        let bind_all = |inner: &[ThroughTypeInfo]| {
            inner
                .iter()
                .map(|i| i.bind_generics(type_info, variant))
                .collect()
        };
        match self {
            ThroughTypeInfo::Param(_) => ThroughTypeInfo::Val(type_info),
            ThroughTypeInfo::Fields { of, variant: own } => ThroughTypeInfo::Fields {
                of: bind(of),
                variant: own.or(variant),
            },
            ThroughTypeInfo::Generic { path, args } => ThroughTypeInfo::Generic {
                path,
                args: bind_all(args),
            },
            ThroughTypeInfo::Option(inner) => ThroughTypeInfo::Option(bind(inner)),
            ThroughTypeInfo::Vec(inner) => ThroughTypeInfo::Vec(bind(inner)),
            ThroughTypeInfo::HashMap(k, v) => ThroughTypeInfo::HashMap(bind(k), bind(v)),
            ThroughTypeInfo::HashSet(inner) => ThroughTypeInfo::HashSet(bind(inner)),
            ThroughTypeInfo::Array(inner, len) => ThroughTypeInfo::Array(bind(inner), *len),
            ThroughTypeInfo::InteropResult(inner) => ThroughTypeInfo::InteropResult(bind(inner)),
            ThroughTypeInfo::Tuple(inner) => ThroughTypeInfo::Tuple(bind_all(inner)),
            ThroughTypeInfo::TupleWithRest { head, rest } => ThroughTypeInfo::TupleWithRest {
                head: bind_all(head),
                rest: bind(rest),
            },
            ThroughTypeInfo::Union(inner) => ThroughTypeInfo::Union(bind_all(inner)),
            other => other.clone(),
        }
    }

    /// Whether this mentions a generic parameter.
    pub fn has_generics(&self) -> bool {
        match self {
            ThroughTypeInfo::Param(_) => true,
            other => other.children().into_iter().any(Self::has_generics),
        }
    }
}

/// A dynamic version of [`TypedThrough`], which can be used to convert a [`TypeInfo`] into a [`ThroughTypeInfo`].
///
/// Reflection does not expose the type info of every nested type (i.e. the values of sets are only known by their type id),
/// use [`into_through_type_info_with_registry`] to resolve those as well.
pub fn into_through_type_info(type_info: &'static TypeInfo) -> ThroughTypeInfo {
    convert_type_info(type_info, None)
}

/// Same as [`into_through_type_info`], but looks up the types which reflection only exposes as type ids in the registry.
pub fn into_through_type_info_with_registry(
    type_info: &'static TypeInfo,
    registry: &TypeRegistry,
) -> ThroughTypeInfo {
    convert_type_info(type_info, Some(registry))
}

fn convert_type_info(
    type_info: &'static TypeInfo,
    registry: Option<&TypeRegistry>,
) -> ThroughTypeInfo {
    let nested = |inner: &'static TypeInfo| Box::new(convert_type_info(inner, registry));
    let option = (|| {
        if let Ok(array) = type_info.as_array() {
            return Some(ThroughTypeInfo::Array(
                nested(array.item_info()?),
                array.capacity(),
            ));
        } else if let Ok(hash_map) = type_info.as_map() {
            return Some(ThroughTypeInfo::HashMap(
                nested(hash_map.key_info()?),
                nested(hash_map.value_info()?),
            ));
        } else if let Ok(set) = type_info.as_set() {
            // sets only expose the type id of their values
            let inner = registry?.get_type_info(set.value_ty().id())?;
            return Some(ThroughTypeInfo::HashSet(nested(inner)));
        } else if let Ok(list) = type_info.as_list() {
            return Some(ThroughTypeInfo::Vec(nested(list.item_info()?)));
        } else if type_info.is_option() {
            let inner = type_info
                .as_enum()
                .ok()?
                .variant("Some")?
                .as_tuple_variant()
                .ok()?
                .field_at(0)?
                .type_info()?;
            return Some(ThroughTypeInfo::Option(nested(inner)));
        } else if type_info.is_result() {
            // TODO verify the error variant is InteropError
            let inner = type_info
                .as_enum()
                .ok()?
                .variant("Ok")?
                .as_tuple_variant()
                .ok()?
                .field_at(0)?
                .type_info()?;
            return Some(ThroughTypeInfo::InteropResult(nested(inner)));
        } else if let Ok(tuple) = type_info.as_tuple() {
            let mut tuple_types = Vec::new();
            for i in 0..tuple.field_len() {
                tuple_types.push(*nested(tuple.field_at(i)?.type_info()?));
            }
            return Some(ThroughTypeInfo::Tuple(tuple_types));
        } else if let Some(primitive) = as_reflect_primitive(type_info) {
            return Some(ThroughTypeInfo::Primitive(primitive));
        }
        None
    })();

    option.unwrap_or(ThroughTypeInfo::Val(type_info))
}

/// The types BMS converts natively, and the primitive kinds they are documented as.
///
/// This is the one list of primitives, the LAD file builder maps between type ids and primitive kinds with it too.
pub fn primitive_type_ids() -> [(TypeId, ReflectionPrimitiveKind); 27] {
    use ReflectionPrimitiveKind as P;
    [
        (TypeId::of::<bool>(), P::Bool),
        (TypeId::of::<isize>(), P::Isize),
        (TypeId::of::<i8>(), P::I8),
        (TypeId::of::<i16>(), P::I16),
        (TypeId::of::<i32>(), P::I32),
        (TypeId::of::<i64>(), P::I64),
        (TypeId::of::<i128>(), P::I128),
        (TypeId::of::<usize>(), P::Usize),
        (TypeId::of::<u8>(), P::U8),
        (TypeId::of::<u16>(), P::U16),
        (TypeId::of::<u32>(), P::U32),
        (TypeId::of::<u64>(), P::U64),
        (TypeId::of::<u128>(), P::U128),
        (TypeId::of::<f32>(), P::F32),
        (TypeId::of::<f64>(), P::F64),
        (TypeId::of::<char>(), P::Char),
        // `&'static str` first, so it is the type of `Str`
        (TypeId::of::<&'static str>(), P::Str),
        (TypeId::of::<str>(), P::Str),
        (TypeId::of::<String>(), P::String),
        (TypeId::of::<OsString>(), P::OsString),
        (TypeId::of::<PathBuf>(), P::PathBuf),
        (TypeId::of::<FunctionCallContext>(), P::FunctionCallContext),
        (TypeId::of::<DynamicScriptFunction>(), P::DynamicFunction),
        (
            TypeId::of::<DynamicScriptFunctionMut>(),
            P::DynamicFunctionMut,
        ),
        (TypeId::of::<ScriptValue>(), P::ScriptValue),
        (TypeId::of::<ReflectReference>(), P::ReflectReference),
        (TypeId::of::<StaticTypeRef>(), P::StaticTypeRef),
    ]
}

/// Returns the primitive kind if the given type corresponds to one, see [`primitive_type_ids`]
pub fn primitive_kind_of(type_id: TypeId) -> Option<ReflectionPrimitiveKind> {
    primitive_type_ids()
        .into_iter()
        .find_map(|(id, kind)| (id == type_id).then_some(kind))
}

/// The type a primitive kind is the documentation of, see [`primitive_type_ids`]
pub fn primitive_type_id(kind: &ReflectionPrimitiveKind) -> Option<TypeId> {
    primitive_type_ids()
        .into_iter()
        .find_map(|(id, k)| (&k == kind).then_some(id))
}

/// Returns the primitive kind if the given type info corresponds to one
pub fn as_reflect_primitive(type_info: &'static TypeInfo) -> Option<ReflectionPrimitiveKind> {
    primitive_kind_of(type_info.type_id())
}

/// A trait for types that can be converted to a [`ThroughTypeInfo`].
pub trait TypedThrough {
    /// Get the [`ThroughTypeInfo`] for the type.
    fn through_type_info() -> ThroughTypeInfo;
}

impl<T1: TypedThrough, T2: TypedThrough> TypedThrough for Union<T1, T2> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::Union(vec![T1::through_type_info(), T2::through_type_info()])
    }
}

impl<T: Typed> TypedThrough for R<'_, T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::Ref(T::type_info())
    }
}

impl<T: Typed> TypedThrough for M<'_, T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::Mut(T::type_info())
    }
}

impl<T: Typed> TypedThrough for V<T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::Val(T::type_info())
    }
}

impl<T: TypedThrough> TypedThrough for Vec<T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::Vec(Box::new(T::through_type_info()))
    }
}

impl<T: TypedThrough> TypedThrough for VecDeque<T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::Vec(Box::new(T::through_type_info()))
    }
}

impl<K: TypedThrough, V: TypedThrough> TypedThrough for HashMap<K, V> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::HashMap(
            Box::new(K::through_type_info()),
            Box::new(V::through_type_info()),
        )
    }
}

impl<K: TypedThrough, V: TypedThrough> TypedThrough for std::collections::HashMap<K, V> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::HashMap(
            Box::new(K::through_type_info()),
            Box::new(V::through_type_info()),
        )
    }
}

impl<T: TypedThrough> TypedThrough for HashSet<T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::HashSet(Box::new(T::through_type_info()))
    }
}

impl<T: TypedThrough> TypedThrough for std::collections::HashSet<T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::HashSet(Box::new(T::through_type_info()))
    }
}

impl<T: TypedThrough> TypedThrough for Result<T, InteropError> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::InteropResult(Box::new(T::through_type_info()))
    }
}

impl<T: TypedThrough, const N: usize> TypedThrough for [T; N] {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::Array(Box::new(T::through_type_info()), N)
    }
}

impl<T: TypedThrough> TypedThrough for Option<T> {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::Option(Box::new(T::through_type_info()))
    }
}

impl TypedThrough for VariadicTuple {
    fn through_type_info() -> ThroughTypeInfo {
        ThroughTypeInfo::UntypedTuple
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
    StaticTypeRef => StaticTypeRef,
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
                ThroughTypeInfo::Tuple(vec![$($ty::through_type_info()),*])
            }
        }
    };
}

variadics_please::all_tuples!(impl_through_typed_tuple, 0, 13, T);

#[cfg(test)]
mod test {
    use std::collections::BTreeSet;

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
                _ => panic!("Expected ThroughTypeInfo::Primitive"),
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
        assert_type_info_is_primitive::<StaticTypeRef>(ReflectionPrimitiveKind::StaticTypeRef);
        assert_type_info_is_primitive::<ScriptValue>(ReflectionPrimitiveKind::ScriptValue);
        assert_type_info_is_primitive::<&'static str>(ReflectionPrimitiveKind::Str);
    }

    #[test]
    fn primitive_kinds_map_back_to_their_types() {
        for (type_id, kind) in primitive_type_ids() {
            if kind == ReflectionPrimitiveKind::Str && type_id == TypeId::of::<str>() {
                continue;
            }
            assert_eq!(primitive_type_id(&kind), Some(type_id), "{kind}");
        }
    }

    // Every variant is listed exactly once here. The generated name lookup is an exhaustive match,
    // so adding a variant does not compile until it is listed, and the coverage tests below fail until it is exercised.
    macro_rules! variant_names {
        ($ty:ident, $all:ident, $name:ident: $($variant:ident),+) => {
            const $all: &[&str] = &[$(stringify!($variant)),+];

            fn $name(value: &$ty) -> &'static str {
                match value {
                    $($ty::$variant { .. } => stringify!($variant)),+
                }
            }
        };
    }

    variant_names!(
        ThroughTypeInfo, ALL_THROUGH_TYPES, through_name:
        Ref, Mut, Val, Opaque, Primitive, Option, Vec, HashMap, HashSet, Array, InteropResult, Tuple,
        UntypedTuple, TupleWithRest, Union, Fields, Param, Generic
    );

    /// Every variant, i.e. `ThroughTypeInfo::Vec`
    fn all_variants() -> BTreeSet<String> {
        ALL_THROUGH_TYPES
            .iter()
            .map(|name| format!("ThroughTypeInfo::{name}"))
            .collect()
    }

    /// Records every variant found anywhere in the given type info
    fn collect_variants(info: &ThroughTypeInfo, seen: &mut BTreeSet<String>) {
        seen.insert(format!("ThroughTypeInfo::{}", through_name(info)));
        for inner in info.children() {
            collect_variants(inner, seen);
        }
    }

    /// The structure of the type info, i.e. `Vec<Val>`, ignoring which types are involved
    fn shape(info: &ThroughTypeInfo) -> String {
        match info {
            ThroughTypeInfo::Primitive(primitive) => format!("{primitive}"),
            other if other.children().is_empty() => through_name(other).to_owned(),
            other => format!(
                "{}<{}>",
                through_name(other),
                other
                    .children()
                    .into_iter()
                    .map(shape)
                    .collect::<Vec<_>>()
                    .join(", ")
            ),
        }
    }

    #[derive(bevy_reflect::Reflect)]
    struct Value;

    /// The static and the dynamic description of the same type, i.e. `Vec<V<Value>>` and `Vec<Value>`
    type StaticDynamicPair = (ThroughTypeInfo, &'static TypeInfo);

    fn static_dynamic_pairs() -> Vec<(&'static str, StaticDynamicPair)> {
        vec![
            ("primitive", (i32::through_type_info(), i32::type_info())),
            (
                "value",
                (V::<Value>::through_type_info(), Value::type_info()),
            ),
            (
                "vec",
                (
                    Vec::<V<Value>>::through_type_info(),
                    Vec::<Value>::type_info(),
                ),
            ),
            (
                "hash map",
                (
                    HashMap::<String, f32>::through_type_info(),
                    HashMap::<String, f32>::type_info(),
                ),
            ),
            (
                "hash set",
                (
                    HashSet::<i32>::through_type_info(),
                    HashSet::<i32>::type_info(),
                ),
            ),
            (
                "std hash set",
                (
                    std::collections::HashSet::<i32>::through_type_info(),
                    std::collections::HashSet::<i32>::type_info(),
                ),
            ),
            (
                "array",
                (<[u8; 4]>::through_type_info(), <[u8; 4]>::type_info()),
            ),
            (
                "option",
                (
                    Option::<String>::through_type_info(),
                    Option::<String>::type_info(),
                ),
            ),
            (
                "result",
                (
                    Result::<i32, InteropError>::through_type_info(),
                    Result::<i32, InteropError>::type_info(),
                ),
            ),
            (
                "tuple",
                (<(i32, f32)>::through_type_info(), <(i32, f32)>::type_info()),
            ),
            (
                "nested",
                (
                    Vec::<HashSet<Option<(i32, String)>>>::through_type_info(),
                    Vec::<HashSet<Option<(i32, String)>>>::type_info(),
                ),
            ),
        ]
    }

    fn test_registry() -> TypeRegistry {
        let mut registry = TypeRegistry::default();
        registry.register::<Value>();
        registry.register::<HashSet<i32>>();
        registry.register::<std::collections::HashSet<i32>>();
        registry.register::<HashSet<Option<(i32, String)>>>();
        registry
    }

    /// The variants which are only produced by [`TypedThrough`], as reflection can not express them.
    const STATIC_ONLY_VARIANTS: &[&str] = &[
        // how an argument is passed is a property of the function, not of the type
        "ThroughTypeInfo::Ref",
        "ThroughTypeInfo::Mut",
        // reflection has no notion of unions, variadics, or generic parameters
        "ThroughTypeInfo::Union",
        "ThroughTypeInfo::UntypedTuple",
        "ThroughTypeInfo::TupleWithRest",
        "ThroughTypeInfo::Fields",
        "ThroughTypeInfo::Param",
        "ThroughTypeInfo::Generic",
        // only produced by `#[derive(TypedThrough)]`, the dynamic conversion uses a value instead
        "ThroughTypeInfo::Opaque",
    ];

    #[test]
    fn static_conversion_produces_every_variant() {
        use crate::function::generics::{A, ComponentOf, FieldsOf, HandleOf};

        let mut seen = BTreeSet::new();
        let mut collect = |info: ThroughTypeInfo| collect_variants(&info, &mut seen);

        collect(R::<Value>::through_type_info());
        collect(M::<Value>::through_type_info());
        collect(V::<Value>::through_type_info());
        collect(i32::through_type_info());
        collect(Union::<V<Value>, V<Value>>::through_type_info());
        collect(Vec::<i32>::through_type_info());
        collect(VecDeque::<i32>::through_type_info());
        collect(HashMap::<i32, i32>::through_type_info());
        collect(HashSet::<i32>::through_type_info());
        collect(<[i32; 2]>::through_type_info());
        collect(Option::<i32>::through_type_info());
        collect(Result::<i32, InteropError>::through_type_info());
        collect(<(i32, i32)>::through_type_info());
        collect(VariadicTuple::through_type_info());
        collect(crate::ScriptQueryResult::through_type_info());
        collect(ThroughTypeInfo::Opaque(Value::type_info()));
        // generic parameters, and what they turn into when bound
        for generic in [
            A::through_type_info(),
            FieldsOf::<A>::through_type_info(),
            HandleOf::<A>::through_type_info(),
            ComponentOf::<A>::through_type_info(),
        ] {
            collect(generic.bind_generics(Value::type_info(), None));
            collect(generic);
        }

        let missing: Vec<_> = all_variants().difference(&seen).cloned().collect();
        assert!(
            missing.is_empty(),
            "variants which are never produced by the static conversion: {missing:?}"
        );
    }

    #[test]
    fn dynamic_conversion_produces_every_variant_but_the_static_only_ones() {
        let registry = test_registry();

        let mut seen = BTreeSet::new();
        for (_, (_, type_info)) in static_dynamic_pairs() {
            collect_variants(
                &into_through_type_info_with_registry(type_info, &registry),
                &mut seen,
            );
        }

        let static_only: BTreeSet<String> = STATIC_ONLY_VARIANTS
            .iter()
            .map(|name| name.to_string())
            .collect();
        let all = all_variants();
        assert!(
            static_only.is_subset(&all),
            "unknown variants in the static only list: {:?}",
            static_only.difference(&all)
        );

        let missing: Vec<_> = all
            .difference(&static_only)
            .filter(|variant| !seen.contains(*variant))
            .collect();
        assert!(
            missing.is_empty(),
            "variants which are never produced by the dynamic conversion, and are not listed as static only: {missing:?}"
        );

        let wrongly_listed: Vec<_> = static_only.intersection(&seen).collect();
        assert!(
            wrongly_listed.is_empty(),
            "variants listed as static only which the dynamic conversion produces: {wrongly_listed:?}"
        );
    }

    #[test]
    fn dynamic_conversion_describes_types_like_the_static_one() {
        let registry = test_registry();

        for (name, (static_info, type_info)) in static_dynamic_pairs() {
            let dynamic_info = into_through_type_info_with_registry(type_info, &registry);
            assert_eq!(
                shape(&static_info),
                shape(&dynamic_info),
                "the dynamic conversion of the {name} type differs from the static one"
            );
        }
    }

    #[test]
    fn dynamic_conversion_without_a_registry_falls_back_to_a_value_for_sets() {
        // sets only expose the type id of their values
        assert_eq!(
            shape(&into_through_type_info(HashSet::<i32>::type_info())),
            "Val"
        );
        // while everything else is fully described
        assert_eq!(
            shape(&into_through_type_info(Vec::<i32>::type_info())),
            "Vec<I32>"
        );
    }
}
