//! The system sets scripts put and order their systems in, see [`ReflectSystemSet`].
//!
//! Scripts either register their own sets into a `script.*` namespace (see [`ScriptNamedSet`]),
//! or use the Rust sets registered for them with [`ScriptSystemSetAppExt::register_script_system_set`].
//! Both are members of their namespace, i.e. `import_namespace("transform.TransformSystems").Propagate`.

use std::sync::Arc;

use bevy_app::App;
use bevy_ecs::{
    resource::Resource,
    schedule::{InternedSystemSet, SystemSet},
};
use bevy_log::warn;
use bevy_platform::collections::HashMap;
use bevy_system_reflection::ReflectSystemSet;

use crate::{
    AppScriptNamespaceMembers, IntoScript, NamespaceValue, SCRIPT_NAMESPACE_PREFIX,
    docgen::typed_through::TypedThrough, function::from::V, script_path,
};

/// A system set registered by a script, identified by its full path, i.e. `script.game.ai`.
///
/// The set stays the same when the script registers it again (i.e. after a reload), and stays in the schedules once
/// the script unloads, only its namespace entry is removed. Interned once per distinct path.
#[derive(SystemSet, Clone, Hash, PartialEq, Eq, Debug)]
pub struct ScriptNamedSet(pub String);

/// The Rust system sets scripts can use, by their full path, i.e. `transform.TransformSystems.Propagate`.
///
/// Script registered sets are not in here, see [`crate::ScriptComponentRegistry::register_system_set`].
#[derive(Resource, Default)]
pub struct ScriptSystemSetRegistry {
    sets: HashMap<String, InternedSystemSet>,
}

impl ScriptSystemSetRegistry {
    /// Registers the set under the given full path, replacing a set registered with the same path
    pub fn register(&mut self, path: String, set: InternedSystemSet) {
        self.sets.insert(path, set);
    }

    /// Returns the set registered with the given full path
    pub fn get(&self, path: &str) -> Option<InternedSystemSet> {
        self.sets.get(path).copied()
    }
}

/// Registers Rust system sets for scripts, see [`ScriptSystemSetRegistry`].
pub trait ScriptSystemSetAppExt {
    /// Lets scripts put their systems in the set and order them against it, as the member `name` of `namespace`.
    ///
    /// The variants of a set enum go into a namespace named after the enum, i.e.
    /// `("transform.TransformSystems", "Propagate")`, unit struct sets into the namespace of their crate, i.e. `("time", "TimeSystems")`.
    /// Script namespaces (`script.*`) are reserved for sets registered by scripts.
    fn register_script_system_set(
        &mut self,
        namespace: &'static str,
        name: &'static str,
        set: impl SystemSet,
    ) -> &mut Self;
}

impl ScriptSystemSetAppExt for App {
    fn register_script_system_set(
        &mut self,
        namespace: &'static str,
        name: &'static str,
        set: impl SystemSet,
    ) -> &mut Self {
        if namespace.starts_with(SCRIPT_NAMESPACE_PREFIX) {
            warn!(
                "Cannot register the system set `{set:?}` into the script namespace `{namespace}`, script namespaces are reserved for sets registered by scripts"
            );
            return self;
        }
        let set = set.intern();
        let world = self.world_mut();
        world
            .get_resource_or_init::<ScriptSystemSetRegistry>()
            .register(script_path(namespace, name), set);
        world
            .get_resource_or_init::<AppScriptNamespaceMembers>()
            .write()
            .register_value(
                namespace,
                name,
                NamespaceValue {
                    maker: Arc::new(move |world| {
                        V(ReflectSystemSet::from_interned(set)).into_script(world)
                    }),
                    type_information: <V<ReflectSystemSet> as TypedThrough>::through_type_info(),
                    documentation: Some(format!("The `{set:?}` system set.").into()),
                },
            );
        self
    }
}
