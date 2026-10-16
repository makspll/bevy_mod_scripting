//! The plugin setting up the namespaces scripts import

use std::{any::TypeId, sync::Arc};

use bevy_app::{App, Plugin};
use bevy_ecs::world::World;
use bevy_mod_scripting_bindings_domain::ReflectionPrimitiveKind;
use bevy_reflect::TypeRegistration;

use super::{AppScriptNamespaceMembers, AppScriptTypeCatalogue, NamespaceValue};
use crate::{ScriptValue, docgen::typed_through::ThroughTypeInfo};

/// A plugin setting up the namespaces scripts import, i.e. `import_namespace("math").Vec3`.
///
/// There are no globals besides the builtins: types, and functions or values registered into namespaces, are reached by importing their namespace,
/// which avoids types with the same name shadowing each other.
pub struct CoreScriptGlobalsPlugin {
    /// The filter function used to determine which types scripts can look up by name.
    /// When `true` for the given type registration, the type is part of the script type catalogue.
    pub filter: fn(&TypeRegistration) -> bool,
}

impl Default for CoreScriptGlobalsPlugin {
    fn default() -> Self {
        Self { filter: |_| true }
    }
}

impl Plugin for CoreScriptGlobalsPlugin {
    fn build(&self, app: &mut App) {
        let members = app
            .world_mut()
            .get_resource_or_init::<AppScriptNamespaceMembers>()
            .clone();
        // the world is a type: scripts call its functions, i.e. `bms.world.spawn()`, it holds no data
        members.write().register_value(
            "bms",
            "world",
            NamespaceValue {
                maker: Arc::new(|_| Ok(ScriptValue::Type(TypeId::of::<World>()))),
                type_information: ThroughTypeInfo::Primitive(
                    ReflectionPrimitiveKind::StaticTypeRef,
                ),
                documentation: Some(
                    "The ECS world, main point of interaction with the app.".into(),
                ),
            },
        );
        app.insert_resource(AppScriptTypeCatalogue::new(self.filter));
    }
}
