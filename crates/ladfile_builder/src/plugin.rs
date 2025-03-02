//! Plugins for bevy which allow generating ladfiles at startup

use std::path::PathBuf;

use bevy::{
    app::{App, Plugin, Startup},
    ecs::{
        reflect::AppTypeRegistry,
        system::{Res, Resource},
    },
};
use bevy_mod_scripting_core::bindings::{
    function::{namespace::Namespace, script_function::AppScriptFunctionRegistry},
    globals::AppScriptGlobalsRegistry,
};

use crate::LadFileBuilder;

/// Plugin which enables the generation of LAD files at runtime for the purposes of creating documentation and other goodies.
///
/// When added, will automatically generate a LAD file on the Startup schedule
pub struct ScriptingDocgenPlugin(LadFileSettings);

#[derive(Resource, Clone)]
/// Stores the settings for the generated Ladfile
pub struct LadFileSettings {
    /// The path at which to generate the LAD file. If relative, will be relative from the assets directory
    /// The extension should be `json.lad`
    ///
    /// By default this will be `assets/bindings.lad.json`
    pub path: PathBuf,
    /// The description to use for the LAD file, by default it's empty
    pub description: &'static str,

    /// Whether to pretty print the output JSON. By default this is true (slay)
    pub pretty: bool,
}

impl Default for ScriptingDocgenPlugin {
    fn default() -> Self {
        Self(LadFileSettings {
            path: PathBuf::from("bindings.lad.json"),
            description: "",
            pretty: true,
        })
    }
}

impl ScriptingDocgenPlugin {
    /// Create a new instance of the plugin with the given path
    pub fn new(path: PathBuf, description: &'static str, pretty: bool) -> Self {
        Self(LadFileSettings {
            path,
            description,
            pretty,
        })
    }
}

fn generate_lad_file(
    type_registry: Res<AppTypeRegistry>,
    function_registry: Res<AppScriptFunctionRegistry>,
    global_registry: Res<AppScriptGlobalsRegistry>,
    settings: Res<LadFileSettings>,
) {
    let type_registry = type_registry.read();
    let function_registry = function_registry.read();
    let global_registry = global_registry.read();
    let mut builder = LadFileBuilder::new(&type_registry);
    builder
        .set_description(settings.description)
        .set_sorted(true);

    // first of all, iterate over all the types and register them
    for registration in type_registry.iter() {
        let type_info = registration.type_info();

        // ignore things without an identifier
        if type_info.type_path_table().ident().is_none() {
            continue;
        }

        builder.add_type_info(type_info);

        // find functions on the namespace
        for (_, function) in
            function_registry.iter_namespace(Namespace::OnType(type_info.type_id()))
        {
            builder.add_function_info(function.info.clone());
        }
    }

    // find functions on the global namespace
    for (_, function) in function_registry.iter_namespace(Namespace::Global) {
        builder.add_function_info(function.info.clone());
    }

    // find global instances

    for (key, global) in global_registry.iter() {
        builder.add_instance_dynamic(key.to_string(), global.maker.is_none(), global.type_id);
    }

    let file = builder.build();

    let mut path = PathBuf::from("assets");
    path.push(settings.path.clone());

    // generate
    let file = match ladfile::serialize_lad_file(&file, settings.pretty) {
        Ok(file) => file,
        Err(e) => {
            bevy::log::error!("Error serializing LAD file: {}", e);
            return;
        }
    };

    // save
    match std::fs::write(&path, file) {
        Ok(_) => {
            bevy::log::info!("Successfully generated LAD file at {:?}", path);
        }
        Err(e) => {
            bevy::log::error!("Error saving LAD file to {:?}: {}", path, e);
        }
    }
}

impl Plugin for ScriptingDocgenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.0.clone());
        app.add_systems(Startup, generate_lad_file);
    }
}
