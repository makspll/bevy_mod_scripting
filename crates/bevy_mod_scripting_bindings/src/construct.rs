//! Construction of reflected values from script payloads.
//!
//! This is the language-agnostic core behind `construct` and the type level `new` constructors.

use crate::{
    ReflectReference, WorldExtensions,
    error::InteropError,
    function::{from::FromScript, from_ref::FromScriptRef, namespace::Namespace},
    reflection_extensions::PartialReflectExt,
    script_value::ScriptValue,
};
use bevy_ecs::world::World;
use bevy_mod_scripting_world::WorldGuard;
use bevy_platform::collections::HashMap;
use bevy_reflect::{
    PartialReflect, ReflectRef, TypeInfo,
    enums::{DynamicEnum, DynamicVariant, VariantInfo},
    std_traits::ReflectDefault,
    structs::DynamicStruct,
    tuple::DynamicTuple,
    tuple_struct::DynamicTupleStruct,
};
use std::any::TypeId;

/// Options controlling how values are constructed from a payload.
#[derive(Clone, Copy, Debug)]
pub struct ConstructOptions {
    /// Whether tuple-like fields are keyed from `1` instead of `0` in the payload.
    pub one_indexed: bool,
    /// If set and the type has a [`ReflectDefault`], the default value is used as the base,
    /// and only fields present in the payload are overwritten.
    pub use_type_default: bool,
    /// If set, fields which are not in the payload (and not provided by the type default)
    /// are filled in with the [`ReflectDefault`] of the field's type, if it has one.
    /// Otherwise a missing field is an error.
    pub use_field_default: bool,
}

impl Default for ConstructOptions {
    fn default() -> Self {
        Self {
            one_indexed: false,
            use_type_default: true,
            use_field_default: true,
        }
    }
}

enum FieldKey {
    Name(&'static str),
    Index(usize),
}

impl FieldKey {
    /// The key under which the field is looked up in the script payload.
    fn payload_key(&self, one_indexed: bool) -> String {
        match self {
            FieldKey::Name(name) => (*name).to_owned(),
            FieldKey::Index(idx) => (idx + one_indexed as usize).to_string(),
        }
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

fn base_field<'a>(base: &'a dyn PartialReflect, key: &FieldKey) -> Option<&'a dyn PartialReflect> {
    match (base.reflect_ref(), key) {
        (ReflectRef::Struct(s), FieldKey::Name(name)) => s.field(name),
        (ReflectRef::TupleStruct(t), FieldKey::Index(idx)) => t.field(*idx),
        (ReflectRef::Tuple(t), FieldKey::Index(idx)) => t.field(*idx),
        (ReflectRef::Enum(e), FieldKey::Name(name)) => e.field(name),
        (ReflectRef::Enum(e), FieldKey::Index(idx)) => e.field_at(*idx),
        _ => None,
    }
}

/// Resolves a single field, in order of priority from: the payload, the type default, the field default.
fn resolve_field(
    guard: &WorldGuard,
    key: FieldKey,
    type_id: TypeId,
    payload: &mut HashMap<String, ScriptValue>,
    base: Option<&dyn PartialReflect>,
    opts: &ConstructOptions,
) -> Result<Box<dyn PartialReflect>, InteropError> {
    let payload_key = key.payload_key(opts.one_indexed);
    if let Some(value) = payload.remove(&payload_key) {
        return <Box<dyn PartialReflect>>::from_script_ref(type_id, value, guard.clone());
    }

    if let Some(field) = base.and_then(|base| base_field(base, &key)) {
        return <dyn PartialReflect>::from_reflect_or_clone(field, guard.clone());
    }

    if opts.use_field_default {
        if let Some(default) = default_value(guard, type_id) {
            return Ok(default);
        }
        return Err(construct_error(format!(
            "field missing and no default provided: '{payload_key}'"
        )));
    }

    Err(construct_error(format!(
        "field missing and field defaults are disabled: '{payload_key}'"
    )))
}

fn resolve_named(
    guard: &WorldGuard,
    payload: &mut HashMap<String, ScriptValue>,
    fields: Vec<(&'static str, TypeId)>,
    base: Option<&dyn PartialReflect>,
    opts: &ConstructOptions,
) -> Result<DynamicStruct, InteropError> {
    let mut dynamic = DynamicStruct::default();
    for (name, type_id) in fields {
        let value = resolve_field(guard, FieldKey::Name(name), type_id, payload, base, opts)?;
        dynamic.insert_boxed(name, value);
    }
    Ok(dynamic)
}

fn resolve_indexed(
    guard: &WorldGuard,
    payload: &mut HashMap<String, ScriptValue>,
    fields: Vec<TypeId>,
    base: Option<&dyn PartialReflect>,
    opts: &ConstructOptions,
) -> Result<Vec<Box<dyn PartialReflect>>, InteropError> {
    fields
        .into_iter()
        .enumerate()
        .map(|(idx, type_id)| {
            resolve_field(guard, FieldKey::Index(idx), type_id, payload, base, opts)
        })
        .collect()
}

fn missing_info() -> InteropError {
    InteropError::invariant("field in field_names should have reflection information")
}

/// Constructs a value of the given type from a script payload.
///
/// For enums the variant is taken from `variant` if provided, otherwise from the `"variant"` entry of the payload.
pub fn construct(
    guard: &WorldGuard,
    type_info: &'static TypeInfo,
    variant: Option<&str>,
    mut payload: HashMap<String, ScriptValue>,
    opts: ConstructOptions,
) -> Result<Box<dyn PartialReflect>, InteropError> {
    let base = if opts.use_type_default {
        default_value(guard, type_info.type_id())
    } else {
        None
    };
    let base_ref = base.as_deref();

    if variant.is_some() && !matches!(type_info, TypeInfo::Enum(_)) {
        return Err(construct_error(format!(
            "a variant was provided but '{}' is not an enum",
            type_info.type_path()
        )));
    }

    let dynamic: Box<dyn PartialReflect> = match type_info {
        TypeInfo::Struct(info) => {
            let fields = info
                .field_names()
                .iter()
                .map(|f| Ok((*f, info.field(f).ok_or_else(missing_info)?.type_id())))
                .collect::<Result<Vec<_>, InteropError>>()?;
            let mut dynamic = resolve_named(guard, &mut payload, fields, base_ref, &opts)?;
            dynamic.set_represented_type(Some(type_info));
            Box::new(dynamic)
        }
        TypeInfo::TupleStruct(info) => {
            let fields = (0..info.field_len())
                .map(|i| Ok(info.field_at(i).ok_or_else(missing_info)?.type_id()))
                .collect::<Result<Vec<_>, InteropError>>()?;
            let mut dynamic = DynamicTupleStruct::default();
            for value in resolve_indexed(guard, &mut payload, fields, base_ref, &opts)? {
                dynamic.insert_boxed(value);
            }
            dynamic.set_represented_type(Some(type_info));
            Box::new(dynamic)
        }
        TypeInfo::Tuple(info) => {
            let fields = (0..info.field_len())
                .map(|i| Ok(info.field_at(i).ok_or_else(missing_info)?.type_id()))
                .collect::<Result<Vec<_>, InteropError>>()?;
            let mut dynamic = DynamicTuple::default();
            for value in resolve_indexed(guard, &mut payload, fields, base_ref, &opts)? {
                dynamic.insert_boxed(value);
            }
            dynamic.set_represented_type(Some(type_info));
            Box::new(dynamic)
        }
        TypeInfo::Enum(enum_info) => {
            let variant_name = match variant {
                Some(name) => name.to_owned(),
                None => {
                    let value = payload.remove("variant").ok_or_else(|| {
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

            // the type default is only a useful base if it is the variant we are building
            let base_ref = base_ref.filter(|base| match base.reflect_ref() {
                ReflectRef::Enum(e) => e.variant_name() == variant_name,
                _ => false,
            });

            let dynamic_variant = match variant_info {
                VariantInfo::Struct(info) => {
                    let fields = info
                        .field_names()
                        .iter()
                        .map(|f| Ok((*f, info.field(f).ok_or_else(missing_info)?.type_id())))
                        .collect::<Result<Vec<_>, InteropError>>()?;
                    DynamicVariant::Struct(resolve_named(
                        guard,
                        &mut payload,
                        fields,
                        base_ref,
                        &opts,
                    )?)
                }
                VariantInfo::Tuple(info) => {
                    let fields = (0..info.field_len())
                        .map(|i| Ok(info.field_at(i).ok_or_else(missing_info)?.type_id()))
                        .collect::<Result<Vec<_>, InteropError>>()?;
                    let mut dynamic = DynamicTuple::default();
                    for value in resolve_indexed(guard, &mut payload, fields, base_ref, &opts)? {
                        dynamic.insert_boxed(value);
                    }
                    DynamicVariant::Tuple(dynamic)
                }
                VariantInfo::Unit(_) => DynamicVariant::Unit,
            };

            let mut dynamic = DynamicEnum::new(variant_name, dynamic_variant);
            dynamic.set_represented_type(Some(type_info));
            Box::new(dynamic)
        }
        _ => {
            // opaque and container types can only be built from their default
            return match base {
                Some(default) => Ok(default),
                None => Err(InteropError::unsupported_operation(
                    Some(type_info.type_id()),
                    Some(Box::new(payload)),
                    "Type constructor not supported",
                )),
            };
        }
    };

    // TODO: it would be nice to have a <dyn PartialReflect>::from_reflect_with_fallback equivalent, that does exactly that
    // only using this as it's already there and convenient, the clone variant hitting will be confusing to end users
    <dyn PartialReflect>::from_reflect_or_clone(dynamic.as_ref(), guard.clone())
}

/// Allocates a freshly constructed value, returning a reference to it.
pub fn allocate_constructed(
    guard: &WorldGuard,
    type_id: TypeId,
    value: Box<dyn PartialReflect>,
) -> Result<ReflectReference, InteropError> {
    let value = value.try_into_reflect().map_err(|_| {
        InteropError::failed_from_reflect(Some(type_id), "Could not construct the type")
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
    use test_utils::test_data::setup_world;

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

    fn run<T: Reflect + bevy_reflect::Typed + bevy_reflect::GetTypeRegistration>(
        variant: Option<&str>,
        payload: Vec<(&str, i64)>,
        opts: ConstructOptions,
    ) -> Result<Box<dyn PartialReflect>, InteropError> {
        let mut world = setup_world(|_, registry| {
            registry.register::<T>();
            registry.register::<usize>();
        });
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::default());
        let guard = WorldAccessGuard::new_exclusive(&mut world, cache);
        let payload = payload
            .into_iter()
            .map(|(k, v)| (k.to_owned(), ScriptValue::Integer(v)))
            .collect();
        construct(&guard, T::type_info(), variant, payload, opts)
    }

    fn opts(use_type_default: bool, use_field_default: bool) -> ConstructOptions {
        ConstructOptions {
            one_indexed: false,
            use_type_default,
            use_field_default,
        }
    }

    fn assert_is<T: Reflect + Debug + PartialEq>(
        result: Result<Box<dyn PartialReflect>, InteropError>,
        expected: T,
    ) {
        let result = result.unwrap();
        let result = result.try_downcast_ref::<T>().unwrap();
        assert_eq!(result, &expected);
    }

    use std::fmt::Debug;

    #[test]
    fn type_default_provides_missing_fields() {
        assert_is(
            run::<WithDefault>(None, vec![("a", 1)], opts(true, true)),
            WithDefault { a: 1, b: 7 },
        );
    }

    #[test]
    fn field_default_used_without_type_default() {
        assert_is(
            run::<WithDefault>(None, vec![("a", 1)], opts(false, true)),
            WithDefault { a: 1, b: 0 },
        );
    }

    #[test]
    fn missing_field_errors_without_any_default() {
        assert!(run::<WithDefault>(None, vec![("a", 1)], opts(false, false)).is_err());
    }

    #[test]
    fn enum_type_default_only_applies_to_matching_variant() {
        assert_is(
            run::<EnumWithDefault>(Some("X"), vec![("a", 1)], opts(true, true)),
            EnumWithDefault::X { a: 1, b: 7 },
        );
        // the default is `X`, so `Y` falls back to field defaults
        assert_is(
            run::<EnumWithDefault>(Some("Y"), vec![], opts(true, true)),
            EnumWithDefault::Y(0),
        );
    }

    #[test]
    fn variant_argument_on_non_enum_errors() {
        assert!(run::<WithDefault>(Some("X"), vec![], opts(true, true)).is_err());
    }

    #[derive(Reflect, Debug, PartialEq)]
    struct Pair(usize, usize);

    #[test]
    fn positional_fields_build_tuple_structs() {
        use crate::function::bound::BoundFields;

        for one_indexed in [false, true] {
            let fields =
                BoundFields::Positional([ScriptValue::Integer(1), ScriptValue::Integer(2)].into())
                    .into_map(one_indexed);
            let mut keys = fields.keys().cloned().collect::<Vec<_>>();
            keys.sort();
            let expected = if one_indexed { ["1", "2"] } else { ["0", "1"] };
            assert_eq!(keys, expected);

            let payload = fields
                .into_iter()
                .map(|(k, v)| match v {
                    ScriptValue::Integer(i) => (k, i),
                    _ => unreachable!(),
                })
                .collect::<Vec<_>>();
            let payload = payload.iter().map(|(k, v)| (k.as_str(), *v)).collect();
            let opts = ConstructOptions {
                one_indexed,
                ..opts(false, false)
            };
            assert_is(run::<Pair>(None, payload, opts), Pair(1, 2));
        }
    }
}
