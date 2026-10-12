//! Construction of reflected values from script payloads.
//!
//! This is the language-agnostic core behind `construct` and the type level `new` constructors.

use crate::{
    ReflectReference, WorldExtensions,
    error::InteropError,
    function::{
        bound::BoundFields, from::FromScript, from_ref::FromScriptRef, namespace::Namespace,
    },
    reflection_extensions::PartialReflectExt,
    script_value::ScriptValue,
};
use bevy_ecs::world::World;
use bevy_mod_scripting_world::WorldGuard;
use bevy_platform::collections::HashMap;
use bevy_reflect::{
    NamedField, PartialReflect, ReflectRef, TypeInfo, UnnamedField,
    enums::{DynamicEnum, DynamicVariant, VariantInfo},
    std_traits::ReflectDefault,
    structs::DynamicStruct,
    tuple::DynamicTuple,
    tuple_struct::DynamicTupleStruct,
};
use std::{any::TypeId, cell::OnceCell};

#[derive(Clone, Copy)]
enum FieldKey {
    Name(&'static str),
    Index(usize),
}

/// Where the values of fields come from, in order of priority:
/// the payload, the type's default, the defaults of the fields' types.
struct FieldSources<'a, 'w> {
    guard: &'a WorldGuard<'w>,
    payload: HashMap<String, ScriptValue>,
    /// Whether tuple-like fields are keyed from `1` instead of `0` in the payload.
    one_indexed: bool,
    /// The type being constructed, whose default is used for missing fields.
    type_id: TypeId,
    /// For enums, the variant being constructed. The type's default is only used if it is this variant.
    variant: Option<String>,
    /// The type's default, only created once a field is missing.
    base: OnceCell<Option<Box<dyn PartialReflect>>>,
}

impl FieldSources<'_, '_> {
    /// The type's default, if it has one and, for enums, it is the variant being constructed.
    fn base(&self) -> Option<&dyn PartialReflect> {
        self.base
            .get_or_init(|| {
                default_value(self.guard, self.type_id).filter(|base| {
                    match (base.reflect_ref(), &self.variant) {
                        (ReflectRef::Enum(e), Some(variant)) => e.variant_name() == variant,
                        (_, variant) => variant.is_none(),
                    }
                })
            })
            .as_deref()
    }

    fn resolve(
        &mut self,
        key: FieldKey,
        type_id: TypeId,
    ) -> Result<Box<dyn PartialReflect>, InteropError> {
        let payload_key = match key {
            FieldKey::Name(name) => name.to_owned(),
            FieldKey::Index(idx) => (idx + self.one_indexed as usize).to_string(),
        };
        if let Some(value) = self.payload.remove(&payload_key) {
            return <Box<dyn PartialReflect>>::from_script_ref(type_id, value, self.guard.clone());
        }

        let base_field = self
            .base()
            .and_then(|base| match (base.reflect_ref(), key) {
                (ReflectRef::Struct(s), FieldKey::Name(name)) => s.field(name),
                (ReflectRef::TupleStruct(t), FieldKey::Index(idx)) => t.field(idx),
                (ReflectRef::Tuple(t), FieldKey::Index(idx)) => t.field(idx),
                (ReflectRef::Enum(e), FieldKey::Name(name)) => e.field(name),
                (ReflectRef::Enum(e), FieldKey::Index(idx)) => e.field_at(idx),
                _ => None,
            });
        if let Some(field) = base_field {
            return <dyn PartialReflect>::from_reflect_or_clone(field, self.guard.clone());
        }

        default_value(self.guard, type_id).ok_or_else(|| {
            construct_error(format!(
                "field missing and no default provided: '{payload_key}'"
            ))
        })
    }

    fn named<'f>(
        &mut self,
        fields: impl Iterator<Item = &'f NamedField>,
    ) -> Result<DynamicStruct, InteropError> {
        let mut dynamic = DynamicStruct::default();
        for field in fields {
            let value = self.resolve(FieldKey::Name(field.name()), field.type_id())?;
            dynamic.insert_boxed(field.name(), value);
        }
        Ok(dynamic)
    }

    fn indexed<'f>(
        &mut self,
        fields: impl Iterator<Item = &'f UnnamedField>,
    ) -> Result<DynamicTuple, InteropError> {
        fields
            .map(|field| self.resolve(FieldKey::Index(field.index()), field.type_id()))
            .collect()
    }
}

fn construct_error(message: impl Into<String>) -> InteropError {
    InteropError::function_interop_error(
        "construct",
        Namespace::OnType(TypeId::of::<World>()),
        InteropError::string(message.into()),
        None,
    )
}

fn default_value(guard: &WorldGuard, type_id: TypeId) -> Option<Box<dyn PartialReflect>> {
    let type_registry = guard.type_registry();
    let type_registry = type_registry.read();
    type_registry
        .get_type_data::<ReflectDefault>(type_id)
        .map(|data| data.default().into_partial_reflect())
}

/// Errors if the payload has entries left which are not fields of the type, i.e. a misspelled field name.
fn ensure_no_unknown_fields(
    type_info: &TypeInfo,
    payload: &HashMap<String, ScriptValue>,
) -> Result<(), InteropError> {
    if payload.is_empty() {
        return Ok(());
    }
    let mut keys = payload.keys().map(String::as_str).collect::<Vec<_>>();
    keys.sort_unstable();
    Err(construct_error(format!(
        "unknown fields for '{}': {}",
        type_info.type_path(),
        keys.join(", ")
    )))
}

/// Constructs a value of the given type from a script payload.
///
/// Fields missing from the payload are filled in from the type's `Default` (for enums, only if it is the variant being constructed),
/// then from the `Default` of the field's type. Types without fields, such as opaque types, can only be built from their default.
///
/// For enums the variant is taken from `variant` if provided, otherwise from the `"variant"` entry of the payload.
/// Entries of the payload which are not fields of the type are an error.
pub fn construct(
    guard: &WorldGuard,
    type_info: &'static TypeInfo,
    variant: Option<&str>,
    fields: BoundFields,
    one_indexed: bool,
) -> Result<Box<dyn PartialReflect>, InteropError> {
    if variant.is_some() && !matches!(type_info, TypeInfo::Enum(_)) {
        return Err(construct_error(format!(
            "a variant was provided but '{}' is not an enum",
            type_info.type_path()
        )));
    }

    let mut sources = FieldSources {
        guard,
        payload: fields.into_map(one_indexed),
        one_indexed,
        type_id: type_info.type_id(),
        variant: None,
        base: OnceCell::new(),
    };

    let dynamic: Box<dyn PartialReflect> = match type_info {
        TypeInfo::Struct(info) => {
            let mut dynamic = sources.named(info.iter())?;
            dynamic.set_represented_type(Some(type_info));
            Box::new(dynamic)
        }
        TypeInfo::TupleStruct(info) => {
            let mut dynamic = DynamicTupleStruct::from(sources.indexed(info.iter())?);
            dynamic.set_represented_type(Some(type_info));
            Box::new(dynamic)
        }
        TypeInfo::Tuple(info) => {
            let mut dynamic = sources.indexed(info.iter())?;
            dynamic.set_represented_type(Some(type_info));
            Box::new(dynamic)
        }
        TypeInfo::Enum(enum_info) => {
            let variant_name = match variant {
                Some(name) => name.to_owned(),
                None => {
                    let value = sources.payload.remove("variant").ok_or_else(|| {
                        construct_error("missing 'variant' field in enum constructor payload")
                    })?;
                    String::from_script(value, guard.clone())?
                }
            };

            let variant_info = enum_info.variant(&variant_name).ok_or_else(|| {
                construct_error(format!(
                    "invalid variant name '{}' for enum '{}'",
                    variant_name,
                    enum_info.type_path()
                ))
            })?;
            sources.variant = Some(variant_name.clone());

            let dynamic_variant = match variant_info {
                VariantInfo::Struct(info) => DynamicVariant::Struct(sources.named(info.iter())?),
                VariantInfo::Tuple(info) => DynamicVariant::Tuple(sources.indexed(info.iter())?),
                VariantInfo::Unit(_) => DynamicVariant::Unit,
            };

            let mut dynamic = DynamicEnum::new(variant_name, dynamic_variant);
            dynamic.set_represented_type(Some(type_info));
            Box::new(dynamic)
        }
        _ => {
            let Some(default) = default_value(guard, type_info.type_id()) else {
                return Err(InteropError::unsupported_operation(
                    Some(type_info.type_id()),
                    Some(Box::new(sources.payload)),
                    "Type constructor not supported",
                ));
            };
            ensure_no_unknown_fields(type_info, &sources.payload)?;
            return Ok(default);
        }
    };
    ensure_no_unknown_fields(type_info, &sources.payload)?;

    // TODO: it would be nice to have a <dyn PartialReflect>::from_reflect_with_fallback equivalent, that does exactly that
    // only using this as it's already there and convenient, the clone variant hitting will be confusing to end users
    <dyn PartialReflect>::from_reflect_or_clone(dynamic.as_ref(), guard.clone())
}

/// [`construct`]s a value and allocates it, returning a reference to it.
pub fn construct_reference(
    guard: &WorldGuard,
    type_info: &'static TypeInfo,
    variant: Option<&str>,
    fields: BoundFields,
    one_indexed: bool,
) -> Result<ReflectReference, InteropError> {
    let value = construct(guard, type_info, variant, fields, one_indexed)?
        .try_into_reflect()
        .map_err(|_| {
            InteropError::failed_from_reflect(
                Some(type_info.type_id()),
                "Could not construct the type",
            )
        })?;
    let allocator = guard.allocator();
    let mut allocator = allocator.write();
    Ok(ReflectReference::new_allocated_boxed(value, &mut allocator))
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::CurrentScriptAttachment;
    use bevy_mod_scripting_world::WorldAccessGuard;
    use bevy_reflect::Reflect;
    use std::fmt::Debug;
    use test_utils::test_data::{SimpleEnum, SimpleStruct, setup_world};

    #[derive(Reflect, Debug, PartialEq)]
    #[reflect(Default)]
    struct WithDefault {
        a: usize,
        b: usize,
    }

    impl Default for WithDefault {
        fn default() -> Self {
            Self { a: 5, b: 7 }
        }
    }

    #[derive(Reflect, Debug, PartialEq)]
    struct WithoutDefault {
        a: usize,
        b: usize,
    }

    #[derive(Reflect, Debug, PartialEq)]
    struct NoDefaults {
        a: WithoutDefault,
    }

    #[derive(Reflect, Debug, PartialEq)]
    #[reflect(Default)]
    enum EnumWithDefault {
        X { a: usize, b: usize },
        Y(usize),
    }

    impl Default for EnumWithDefault {
        fn default() -> Self {
            Self::X { a: 5, b: 7 }
        }
    }

    #[derive(Reflect, Debug, PartialEq)]
    struct Pair(usize, usize);

    fn run<
        T: bevy_reflect::FromReflect + bevy_reflect::Typed + bevy_reflect::GetTypeRegistration,
    >(
        variant: Option<&str>,
        fields: BoundFields,
        one_indexed: bool,
    ) -> Result<Box<dyn PartialReflect>, InteropError> {
        let mut world = setup_world(|_, registry| {
            registry.register::<T>();
            // not registered automatically for all types, i.e. tuples, see https://github.com/bevyengine/bevy/issues/17981
            registry.register_type_data::<T, bevy_reflect::ReflectFromReflect>();
            registry.register::<usize>();
        });
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        let guard = WorldAccessGuard::new_exclusive(&mut world, cache);
        construct(&guard, T::type_info(), variant, fields, one_indexed)
    }

    fn keyed(payload: Vec<(&str, i64)>) -> BoundFields {
        BoundFields::Keyed(
            payload
                .into_iter()
                .map(|(k, v)| (k.to_owned(), ScriptValue::Integer(v)))
                .collect(),
        )
    }

    fn assert_is<T: Reflect + Debug + PartialEq>(
        result: Result<Box<dyn PartialReflect>, InteropError>,
        expected: T,
    ) {
        let result = result.unwrap();
        let result = result.try_downcast_ref::<T>().unwrap();
        assert_eq!(result, &expected);
    }

    #[test]
    fn type_default_provides_missing_fields() {
        assert_is(
            run::<WithDefault>(None, keyed(vec![("a", 1)]), false),
            WithDefault { a: 1, b: 7 },
        );
    }

    #[test]
    fn field_default_used_without_type_default() {
        assert_is(
            run::<WithoutDefault>(None, keyed(vec![("a", 1)]), false),
            WithoutDefault { a: 1, b: 0 },
        );
    }

    #[test]
    fn missing_field_errors_without_any_default() {
        assert!(run::<NoDefaults>(None, keyed(vec![]), false).is_err());
    }

    #[test]
    fn enum_type_default_only_applies_to_matching_variant() {
        assert_is(
            run::<EnumWithDefault>(Some("X"), keyed(vec![("a", 1)]), false),
            EnumWithDefault::X { a: 1, b: 7 },
        );
        // the default is `X`, so `Y` falls back to field defaults
        assert_is(
            run::<EnumWithDefault>(Some("Y"), keyed(vec![]), false),
            EnumWithDefault::Y(0),
        );
    }

    #[test]
    fn unknown_fields_error() {
        // a misspelled field must not silently fall back to the default
        let result = run::<WithDefault>(None, keyed(vec![("a", 1), ("bb", 2)]), false);
        assert!(format!("{result:?}").contains("bb"), "{result:?}");
        // positional fields beyond the last field too
        let fields =
            BoundFields::Positional([1, 2, 3].map(ScriptValue::Integer).into_iter().collect());
        assert!(run::<Pair>(None, fields, false).is_err());
    }

    #[test]
    fn variant_argument_on_non_enum_errors() {
        assert!(run::<WithDefault>(Some("X"), keyed(vec![]), false).is_err());
    }

    #[test]
    fn positional_fields_build_tuple_structs() {
        for one_indexed in [false, true] {
            let fields =
                BoundFields::Positional([ScriptValue::Integer(1), ScriptValue::Integer(2)].into());
            assert_is(run::<Pair>(None, fields, one_indexed), Pair(1, 2));
        }
    }

    #[test]
    fn keyed_tuple_fields_follow_the_indexing_convention() {
        assert_is(
            run::<Pair>(None, keyed(vec![("0", 1), ("1", 2)]), false),
            Pair(1, 2),
        );
        assert_is(
            run::<Pair>(None, keyed(vec![("1", 1), ("2", 2)]), true),
            Pair(1, 2),
        );
    }

    #[test]
    fn constructs_structs_and_tuples() {
        assert_is(
            run::<SimpleStruct>(None, keyed(vec![("foo", 1)]), false),
            SimpleStruct { foo: 1 },
        );
        assert_is(
            run::<(usize, usize)>(None, keyed(vec![("1", 1), ("2", 2)]), true),
            (1usize, 2usize),
        );
    }

    #[test]
    fn enums_take_the_variant_from_the_payload() {
        let with_variant = |variant: &str, fields: Vec<(&str, i64)>| {
            let BoundFields::Keyed(mut map) = keyed(fields) else {
                unreachable!()
            };
            map.insert(
                "variant".into(),
                ScriptValue::String(variant.to_owned().into()),
            );
            run::<SimpleEnum>(None, BoundFields::Keyed(map), false)
        };
        assert_is(
            with_variant("Struct", vec![("foo", 1)]),
            SimpleEnum::Struct { foo: 1 },
        );
        assert_is(
            with_variant("TupleStruct", vec![("0", 1)]),
            SimpleEnum::TupleStruct(1),
        );
        assert_is(with_variant("Unit", vec![]), SimpleEnum::Unit);
        assert!(with_variant("Nonexistent", vec![]).is_err());
        // the variant is required
        assert!(run::<SimpleEnum>(None, keyed(vec![]), false).is_err());
    }
}
