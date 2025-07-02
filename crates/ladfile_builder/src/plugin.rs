//! Plugins for bevy which allow generating ladfiles at startup

use std::path::PathBuf;

use bevy::{
	app::{App, Plugin, Startup},
	ecs::{prelude::Resource, reflect::AppTypeRegistry, system::Res, world::World},
};
use bevy_mod_scripting_core::bindings::{
	function::{namespace::Namespace, script_function::AppScriptFunctionRegistry},
	globals::AppScriptGlobalsRegistry,
	IntoNamespace, MarkAsCore, MarkAsGenerated, MarkAsSignificant,
};
use ladfile::{default_importance, LadTypeKind};

use crate::LadFileBuilder;

/// Plugin which enables the generation of LAD files at runtime for the purposes of creating documentation and other goodies.
///
/// When added, will automatically generate a LAD file on the Startup schedule
#[derive(Default)]
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

    /// Whether to exclude types which are not registered.
    ///
    /// i.e. `HashMap<T,V>` where `T` or `V` are not registered types
    pub exclude_types_containing_unregistered: bool,

    /// Whether to pretty print the output JSON. By default this is true (slay)
    pub pretty: bool,
}

impl Default for LadFileSettings {
    fn default() -> Self {
        Self {
            path: PathBuf::from("bindings.lad.json"),
            description: "",
            pretty: true,
            exclude_types_containing_unregistered: true,
        }
    }
}

impl ScriptingDocgenPlugin {
    /// Create a new instance of the plugin with the given path
    pub fn new(
        path: PathBuf,
        description: &'static str,
        exclude_types_containing_unregistered: bool,
        pretty: bool,
    ) -> Self {
        Self(LadFileSettings {
            path,
            description,
            pretty,
            exclude_types_containing_unregistered,
        })
    }
}

/// The function used to generate a ladfile from pre-populated type, function and global registries
pub fn generate_lad_file(
    type_registry: &AppTypeRegistry,
    function_registry: &AppScriptFunctionRegistry,
    global_registry: &AppScriptGlobalsRegistry,
    settings: &LadFileSettings,
) {
    let type_registry = type_registry.read();
    let function_registry = function_registry.read();
    let global_registry = global_registry.read();
    let mut builder = LadFileBuilder::new(&type_registry);
    builder
        .set_description(settings.description)
        .set_exclude_including_unregistered(settings.exclude_types_containing_unregistered)
        .set_sorted(true);

    // process world as a special value
    builder.add_nonreflect_type::<World>(
        Some("bevy_ecs"),
        r#"The ECS world containing all Components, Resources and Systems. Main point of interaction with a Bevy App."#.trim(),
    );

    for (_, function) in function_registry.iter_namespace(World::into_namespace()) {
        builder.add_function_info(&function.info);
    }

    builder.set_insignificance(
        std::any::TypeId::of::<World>(),
        (default_importance() / 2) - 1,
    );

    // first of all, iterate over all the types and register them
    for registration in type_registry.iter() {
        let type_info = registration.type_info();

        // ignore things without an identifier
        if type_info.type_path_table().ident().is_none() {
            continue;
        }

        builder.add_type_info(type_info);

        if registration.contains::<MarkAsGenerated>() {
            builder.mark_generated(registration.type_id());
        }

        if registration.contains::<MarkAsCore>() {
            builder.set_insignificance(registration.type_id(), default_importance() / 2);
        }

        if registration.contains::<MarkAsSignificant>() {
            builder.set_insignificance(registration.type_id(), default_importance() / 4);
        }

        // find functions on the namespace
        for (_, function) in
            function_registry.iter_namespace(Namespace::OnType(type_info.type_id()))
        {
            builder.add_function_info(&function.info);
        }
    }

    // find functions on the global namespace
    for (_, function) in function_registry.iter_namespace(Namespace::Global) {
        builder.add_function_info(&function.info);
    }

    // find global instances
    for (key, global) in global_registry.iter() {
        let type_info = global.type_information.clone();
        builder.add_instance_dynamic(key.to_string(), global.maker.is_none(), type_info);
    }

    // find global dummies
    for (key, global) in global_registry.iter_dummies() {
        let lad_type_id = builder.lad_id_from_type_id(global.type_id);
        builder.add_instance_manually(key.to_string(), false, LadTypeKind::Val(lad_type_id));
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

fn generate_lad_file_system(
    type_registry: Res<AppTypeRegistry>,
    function_registry: Res<AppScriptFunctionRegistry>,
    global_registry: Res<AppScriptGlobalsRegistry>,
    settings: Res<LadFileSettings>,
) {
    generate_lad_file(
        &type_registry,
        &function_registry,
        &global_registry,
        &settings,
    );
}

impl Plugin for ScriptingDocgenPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(self.0.clone());
        app.add_systems(Startup, generate_lad_file_system);
    }
}
