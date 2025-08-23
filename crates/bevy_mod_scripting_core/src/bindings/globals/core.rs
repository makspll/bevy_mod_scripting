//! Core globals exposed by the BMS framework

use ::{
    bevy_app::Plugin,
    bevy_asset::Handle,
    bevy_ecs::{entity::Entity, reflect::AppTypeRegistry, world::World},
    bevy_reflect::TypeRegistration,
};
use bevy_app::App;
use bevy_mod_scripting_derive::script_globals;
use bevy_platform::collections::HashMap;
use std::{cell::RefCell, sync::Arc};

use crate::{
    asset::ScriptAsset,
    bindings::{
        ScriptComponentRegistration, ScriptResourceRegistration, ScriptTypeRegistration,
        WorldGuard,
        function::from::{Union, Val},
    },
    docgen::into_through_type_info,
    error::InteropError,
};

use super::AppScriptGlobalsRegistry;

/// A plugin introducing core globals for the BMS framework.
///
/// By default all types added to the type registry are present as globals, you can customize this behavior
/// by providing a filter function
pub struct CoreScriptGlobalsPlugin {
    /// the filter function used to determine which types are registered as globals
    /// When `true` for the given type registration, the type will be registered as a global.
    pub filter: fn(&TypeRegistration) -> bool,

    /// Whether to register static references to types
    /// By default static type references such as `Vec3` or `Mat3` are accessible directly from the global namespace.
    pub register_static_references: bool,
}

impl Default for CoreScriptGlobalsPlugin {
    fn default() -> Self {
        Self {
            filter: |_| true,
            register_static_references: true,
        }
    }
}

thread_local! {
    static GLOBAL_OPTS: RefCell<fn(&TypeRegistration) -> bool> = RefCell::new(|_| true);
}

impl Plugin for CoreScriptGlobalsPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AppScriptGlobalsRegistry>();
    }
    fn finish(&self, app: &mut App) {
        // profiling::function_scope!("app finish");

        if self.register_static_references {
            register_static_core_globals(app.world_mut(), self.filter);
        }

        // TODO: add ability to make the generated function receive generic payload
        GLOBAL_OPTS.replace(self.filter);
        register_core_globals(app.world_mut());
    }
}

#[profiling::function]
fn register_static_core_globals(world: &mut World, filter: fn(&TypeRegistration) -> bool) {
    let global_registry = world
        .get_resource_or_init::<AppScriptGlobalsRegistry>()
        .clone();
    let type_registry = world.get_resource_or_init::<AppTypeRegistry>().clone();
    let mut global_registry = global_registry.write();
    let type_registry = type_registry.read();

    // find all reflectable types without generics
    for registration in type_registry.iter().filter(|r| filter(r)) {
        if !registration.type_info().generics().is_empty() {
            continue;
        }

        if let Some(global_name) = registration.type_info().type_path_table().ident() {
            let documentation = "A reference to the type, allowing you to call static methods.";
            let type_info = registration.type_info();
            global_registry.register_static_documented_dynamic(
                registration.type_id(),
                into_through_type_info(type_info),
                global_name.into(),
                documentation.into(),
            );
        }
    }

    // register basic globals
    global_registry.register_dummy::<World>("world", "The current ECS world.");
    global_registry
        .register_dummy::<Entity>("entity", "The entity this script is attached to if any.");
    global_registry.register_dummy_typed::<Val<Handle<ScriptAsset>>>("script_asset", "the asset handle for this script. If the asset is ever unloaded, the handle will be less useful.");
}

#[script_globals(bms_core_path = "crate", name = "core_globals")]
impl CoreGlobals {
    /// A cache of types normally available through the `world.get_type_by_name` function.
    ///
    /// You can use this to avoid having to store type references.
    ///
    /// Note that this cache will NOT contain types manually registered by scripts via `register_new_component`.
    fn types(
        guard: WorldGuard,
    ) -> Result<
        HashMap<
            String,
            Union<
                Val<ScriptTypeRegistration>,
                Union<Val<ScriptComponentRegistration>, Val<ScriptResourceRegistration>>,
            >,
        >,
        InteropError,
    > {
        // profiling::function_scope!("registering core globals");
        let type_registry = guard.type_registry();
        let type_registry = type_registry.read();
        let mut type_cache = HashMap::<String, _>::default();
        let filter = GLOBAL_OPTS.with(|opts| *opts.borrow());
        for registration in type_registry.iter().filter(|r| filter(r)) {
            let type_path = registration.type_info().type_path_table().short_path();
            let registration = ScriptTypeRegistration::new(Arc::new(registration.clone()));
            let registration = guard.clone().get_type_registration(registration)?;
            let registration =
                registration.map_both(Val::from, |u| u.map_both(Val::from, Val::from));
            type_cache.insert(type_path.to_owned(), registration);
        }

        Ok(type_cache)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use ::{bevy_app::App, bevy_reflect::Reflect};

    #[test]
    fn test_register_globals() {
        let mut app = App::new();

        // register a type
        #[derive(Debug, Clone, Reflect)]
        struct TestType;
        app.register_type::<TestType>();
        let plugin = CoreScriptGlobalsPlugin::default();
        plugin.build(&mut app);
        plugin.finish(&mut app);
        let globals = app
            .world()
            .get_resource::<AppScriptGlobalsRegistry>()
            .unwrap()
            .read();
        assert!(globals.get("TestType").is_some());

        // now do the same but with a filter
        let mut app = App::new();
        let plugin = CoreScriptGlobalsPlugin {
            filter: |_| false,
            register_static_references: true,
        };
        plugin.build(&mut app);
        plugin.finish(&mut app);

        let globals = app
            .world()
            .get_resource::<AppScriptGlobalsRegistry>()
            .unwrap()
            .read();

        // check that the type is not registered
        assert!(globals.len() == 1);
        assert!(globals.get("types").is_some());
    }
}
