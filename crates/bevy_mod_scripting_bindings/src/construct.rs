//! Construction of reflected values from script payloads.
//!
//! This is the language-agnostic core behind `construct` and the type level `new` constructors.

use crate::{
    EnumVariantRef, ReflectReference, ScriptTypeRegistration, StaticTypeRef, WorldExtensions,
    error::InteropError,
    function::{
        from::{FromScript, V},
        from_ref::FromScriptRef,
        namespace::{Namespace, NamespaceBuilder},
        script_function::FunctionCallContext,
    },
    reflection_extensions::PartialReflectExt,
    script_value::ScriptValue,
};
use bevy_app::{App, Plugin};
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
use std::{any::TypeId, borrow::Cow};

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
    type_: &ScriptTypeRegistration,
    variant: Option<&str>,
    mut payload: HashMap<String, ScriptValue>,
    opts: ConstructOptions,
) -> Result<Box<dyn PartialReflect>, InteropError> {
    let type_info = type_.registration.type_info();
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

/// The type level constructor, registered once on [`StaticTypeRef`] and bound to the type
/// when accessed through a static type reference, i.e. `Type.new{...}`.
fn construct_type(
    ctxt: FunctionCallContext,
    type_ref: V<StaticTypeRef>,
    payload: HashMap<String, ScriptValue>,
    use_type_default: Option<bool>,
    use_field_default: Option<bool>,
) -> Result<ReflectReference, InteropError> {
    let guard = ctxt.world()?;
    let registration = type_ref.into_inner().registration;
    let opts = ConstructOptions {
        one_indexed: ctxt.convert_to_0_indexed(),
        use_type_default: use_type_default.unwrap_or(true),
        use_field_default: use_field_default.unwrap_or(true),
    };
    let value = construct(&guard, &registration, None, payload, opts)?;
    allocate_constructed(&guard, registration.type_id(), value)
}

/// The constructor of a variant, i.e. `MyEnum.Variant:new{...}`, shares its options with [`construct_type`].
fn construct_variant(
    ctxt: FunctionCallContext,
    variant: V<EnumVariantRef>,
    payload: HashMap<String, ScriptValue>,
    use_type_default: Option<bool>,
    use_field_default: Option<bool>,
) -> Result<ReflectReference, InteropError> {
    let guard = ctxt.world()?;
    let variant = variant.into_inner();
    let opts = ConstructOptions {
        one_indexed: ctxt.convert_to_0_indexed(),
        use_type_default: use_type_default.unwrap_or(true),
        use_field_default: use_field_default.unwrap_or(true),
    };
    let value = construct(
        &guard,
        &variant.registration,
        Some(variant.variant_name()?),
        payload,
        opts,
    )?;
    allocate_constructed(&guard, variant.registration.type_id(), value)
}

/// Adds a type level constructor to scripts, i.e. `MyType.new{ field = value }`.
///
/// The constructor is a single function registered on [`StaticTypeRef`], and is made available on
/// every type's static reference. Functions registered on the type itself with the same name take priority.
///
/// The constructor takes a table of fields, and optionally:
/// - `use_type_default` (default `true`): use the type's `Default` as the base, only overwriting the given fields
/// - `use_field_default` (default `true`): use the `Default` of a field's type for fields which are still missing
pub struct ConstructorsPlugin {
    /// The name the constructor is available under in scripts.
    pub constructor_name: Cow<'static, str>,
}

impl Default for ConstructorsPlugin {
    fn default() -> Self {
        Self {
            constructor_name: Cow::Borrowed("new"),
        }
    }
}

impl Plugin for ConstructorsPlugin {
    fn build(&self, app: &mut App) {
        NamespaceBuilder::<EnumVariantRef>::new(app.world_mut()).register_documented(
            self.constructor_name.clone(),
            construct_variant,
            "Constructs a value of this enum variant from a table of its fields.\n\n\
             Arguments:\n\
             * `variant`: The variant to construct.\n\
             * `payload`: The values to use to construct the variant.\n\
             * `use_type_default`: Whether to use the enum's default as the base if it is this variant, true by default.\n\
             * `use_field_default`: Whether to default missing fields from their types' defaults, true by default.\n\
             Returns:\n\
             * `reference`: The constructed value.",
            &[
                "variant",
                "payload",
                "use_type_default",
                "use_field_default",
            ],
        );
        NamespaceBuilder::<StaticTypeRef>::new(app.world_mut())
            .register_documented(
                self.constructor_name.clone(),
                construct_type,
                "Constructs a new instance of the type from a table of its fields.\n\
                 For enums use the `variant` key, or construct from a variant directly.\n\n\
                 Arguments:\n\
                 * `type`: The type to construct.\n\
                 * `payload`: The values to use to construct the type.\n\
                 * `use_type_default`: Whether to use the type's default as the base, true by default.\n\
                 * `use_field_default`: Whether to default missing fields from their types' defaults, true by default.\n\
                 Returns:\n\
                 * `reference`: The constructed value.",
                &[
                    "type",
                    "payload",
                    "use_type_default",
                    "use_field_default",
                ],
            );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use crate::CurrentScriptAttachment;
    use bevy_mod_scripting_world::WorldAccessGuard;
    use bevy_reflect::Reflect;
    use std::sync::Arc;
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

    fn run<T: Reflect + bevy_reflect::GetTypeRegistration>(
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
        let registration = guard
            .type_registry()
            .read()
            .get(TypeId::of::<T>())
            .unwrap()
            .clone();
        let payload = payload
            .into_iter()
            .map(|(k, v)| (k.to_owned(), ScriptValue::Integer(v)))
            .collect();
        construct(
            &guard,
            &ScriptTypeRegistration::new(Arc::new(registration)),
            variant,
            payload,
            opts,
        )
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

    #[test]
    fn constructor_name_is_configurable() {
        use crate::function::script_function::AppScriptFunctionRegistry;

        let mut app = App::new();
        app.add_plugins(ConstructorsPlugin {
            constructor_name: "create".into(),
        });
        let registry = app
            .world()
            .get_resource::<AppScriptFunctionRegistry>()
            .unwrap()
            .read();
        for type_id in [
            TypeId::of::<StaticTypeRef>(),
            TypeId::of::<EnumVariantRef>(),
        ] {
            assert!(registry.contains(Namespace::OnType(type_id), "create"));
            assert!(!registry.contains(Namespace::OnType(type_id), "new"));
        }
    }
}
