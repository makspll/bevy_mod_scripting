//! The `new` constructors available on every type and enum variant, i.e. `MyType.new{...}` and `MyEnum.Variant:new{...}`.

use std::borrow::Cow;

use bevy_app::{App, Plugin};
use bevy_mod_scripting_bindings::{
    EnumVariantRef, InteropError, StaticTypeRef,
    construct::construct_reference,
    function::{
        bound::{BoundFields, BoundValue},
        from::V,
        namespace::NamespaceBuilder,
        script_function::FunctionCallContext,
    },
};

/// The type level constructor, registered once on [`StaticTypeRef`] and bound to the type
/// when accessed through a static type reference, i.e. `Type.new{...}`.
fn construct_type(
    ctxt: FunctionCallContext,
    type_ref: V<StaticTypeRef>,
    payload: BoundFields,
) -> Result<BoundValue, InteropError> {
    let world = ctxt.world()?;
    let type_info = type_ref.type_info(&world)?;
    construct_reference(
        &world,
        type_info,
        None,
        payload,
        ctxt.convert_to_0_indexed(),
    )
    .map(BoundValue)
}

/// The constructor of a variant, i.e. `MyEnum.Variant:new{...}`.
fn construct_variant(
    ctxt: FunctionCallContext,
    variant: V<EnumVariantRef>,
    payload: BoundFields,
) -> Result<BoundValue, InteropError> {
    let world = ctxt.world()?;
    construct_reference(
        &world,
        variant.type_info(&world)?,
        Some(variant.variant_name(&world)?),
        payload,
        ctxt.convert_to_0_indexed(),
    )
    .map(BoundValue)
}

/// Adds a type level constructor to scripts, i.e. `MyType.new{ field = value }`, and to enum variants, i.e. `MyEnum.Variant:new{ field = value }`.
///
/// The constructor is a single function registered on [`StaticTypeRef`], and is made available on
/// every type's static reference. Functions registered on the type itself with the same name take priority.
/// The global `construct` function is always available for types whose own functions shadow the constructor.
///
/// The constructor takes a table of fields, keyed by name or index, or a list of the fields in order.
/// Missing fields are filled in from the type's `Default` (for enums, only if it is the variant being constructed),
/// then from the `Default` of the field's type.
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
            "Constructs a value of this enum variant from a table of its fields.\n\
             Missing fields are filled in from the enum's default if it is this variant, then from the defaults of the fields' types.\n\n\
             Arguments:\n\
             * `variant`: The variant to construct.\n\
             * `payload`: The fields of the variant, keyed by name or index, or a list of the fields in order.\n\
             Returns:\n\
             * `reference`: The constructed value.",
            &["ctxt", "variant", "payload"],
        );
        NamespaceBuilder::<StaticTypeRef>::new(app.world_mut()).register_documented(
            self.constructor_name.clone(),
            construct_type,
            "Constructs a new instance of the type from a table of its fields.\n\
             Missing fields are filled in from the type's default, then from the defaults of the fields' types.\n\
             For enums use the `variant` key, or construct from a variant directly.\n\n\
             Arguments:\n\
             * `type`: The type to construct.\n\
             * `payload`: The fields of the type, keyed by name or index, or a list of the fields in order.\n\
             Returns:\n\
             * `reference`: The constructed value.",
            &["ctxt", "type", "payload"],
        );
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use bevy_mod_scripting_bindings::function::{
        namespace::Namespace, script_function::AppScriptFunctionRegistry,
    };
    use std::any::TypeId;

    #[test]
    fn constructor_name_is_configurable() {
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

    #[test]
    fn arguments_are_named_after_the_context() {
        let mut app = App::new();
        app.add_plugins(ConstructorsPlugin::default());
        let registry = app
            .world()
            .get_resource::<AppScriptFunctionRegistry>()
            .unwrap()
            .read();
        let func = registry
            .get_function(Namespace::OnType(TypeId::of::<StaticTypeRef>()), "new")
            .unwrap_or_else(|_| panic!("missing constructor"));
        let names = func
            .info
            .arg_info
            .iter()
            .map(|a| a.name.as_deref().unwrap_or_default().to_owned())
            .collect::<Vec<_>>();
        assert_eq!(names, ["ctxt", "type", "payload"]);
    }
}
