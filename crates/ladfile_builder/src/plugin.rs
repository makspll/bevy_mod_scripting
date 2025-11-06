//! Plugins for bevy which allow generating ladfiles at startup

use std::{ops::Deref, path::PathBuf, sync::Arc};

use ::{
    bevy_app::{App, Plugin, Startup},
    bevy_ecs::{prelude::Resource, reflect::AppTypeRegistry, system::Res, world::World},
};
use bevy_log::{error, info};
use bevy_mod_scripting_bindings::{
    DummyScriptFunctionRegistry, IntoNamespace,
    function::{namespace::Namespace, script_function::AppScriptFunctionRegistry},
    globals::AppScriptGlobalsRegistry,
    into_through_type_info,
};
use ladfile::{LadFieldOrVariableKind, LadFilePlugin, default_importance};

use crate::LadFileBuilder;

/// Plugin which enables the generation of LAD files at runtime for the purposes of creating documentation and other goodies.
///
/// When added, will automatically generate a LAD file on the Startup schedule
#[derive(Default, Clone)]
pub struct ScriptingFilesGenerationPlugin(LadFileSettingsArc);

/// Stores the settings for the generated Ladfile
#[derive(Resource, Default, Clone)]
pub struct LadFileSettingsArc(pub Arc<LadFileSettings>);

impl Deref for LadFileSettingsArc {
    type Target = LadFileSettings;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

/// Stores the settings for the generated Ladfile
pub struct LadFileSettings {
    /// Can be used to decide whether or not to generate ladfiles and associated files at runtime
    /// by default enabled.
    pub enabled: bool,

    /// if set the file name in the output directory to write the ladfile. Not saved by default
    pub ladfile_filename: Option<PathBuf>,

    /// The path at which to generate the LAD file and other outputs from each processor.
    ///
    /// By default this will be the current working directory.
    pub output_directory: PathBuf,

    /// The description to use for the LAD file, by default it's empty
    pub description: &'static str,

    /// Whether to exclude types which are not registered.
    ///
    /// i.e. `HashMap<T,V>` where `T` or `V` are not registered types
    pub exclude_types_containing_unregistered: bool,

    /// Whether to pretty print the output JSON. By default this is true (slay)
    pub pretty: bool,

    /// Processors to apply to the generated ladfile
    pub processors: Vec<Box<dyn LadFilePlugin + Send + Sync + 'static>>,
}

impl Default for LadFileSettings {
    fn default() -> Self {
        Self {
            enabled: true,
            ladfile_filename: None,
            output_directory: PathBuf::from("."),
            description: "",
            pretty: true,
            exclude_types_containing_unregistered: true,
            processors: ScriptingFilesGenerationPlugin::default_processors(),
        }
    }
}

impl ScriptingFilesGenerationPlugin {
    /// The default processors according to currently set features
    #[allow(unused_mut, clippy::vec_init_then_push)]
    pub fn default_processors() -> Vec<Box<dyn LadFilePlugin + Send + Sync + 'static>> {
        let mut processors = Vec::default();

        #[cfg(feature = "lua_language_server_files")]
        processors.push(Box::new(
            lua_language_server_lad_backend::LuaLanguageServerLadPlugin::default(),
        ) as Box<dyn LadFilePlugin + Send + Sync + 'static>);
        processors
    }

    /// Create a new instance of the plugin responsible for setting up the ladfile processing pipeline
    pub fn new(
        enabled: bool,
        output_directory: PathBuf,
        ladfile_filename: Option<PathBuf>,
        description: &'static str,
        exclude_types_containing_unregistered: bool,
        pretty: bool,
    ) -> Self {
        Self(LadFileSettingsArc(Arc::new(LadFileSettings {
            enabled,
            output_directory,
            ladfile_filename,
            description,
            pretty,
            exclude_types_containing_unregistered,
            processors: Self::default_processors(),
        })))
    }
}

/// The function used to generate a ladfile from pre-populated type, function and global registries
pub fn generate_lad_file(
    type_registry: &AppTypeRegistry,
    function_registry: &AppScriptFunctionRegistry,
    dummy_function_registry: &DummyScriptFunctionRegistry,
    global_registry: &AppScriptGlobalsRegistry,
    settings: &LadFileSettings,
) {
    let type_registry = type_registry.read();
    let function_registry = function_registry.read();
    let dummy_function_registry = dummy_function_registry.0.read();
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

    for (_, function) in function_registry
        .iter_namespace(World::into_namespace())
        .chain(dummy_function_registry.iter_namespace(World::into_namespace()))
    {
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

        // we don't really care about the Option in Option<T> as that is magically worked around in the entire API
        // get the "pure" types
        let through_type_info = into_through_type_info(type_info);
        builder.add_through_type_info(&through_type_info);

        // find functions on the namespace
        for (_, function) in function_registry
            .iter_namespace(Namespace::OnType(type_info.type_id()))
            .chain(dummy_function_registry.iter_namespace(Namespace::OnType(type_info.type_id())))
        {
            builder.add_function_info(&function.info);
        }
    }

    // find functions on the global namespace
    for (_, function) in function_registry
        .iter_namespace(Namespace::Global)
        .chain(dummy_function_registry.iter_namespace(Namespace::Global))
    {
        builder.add_function_info(&function.info);
    }

    // find global instances
    for (key, global) in global_registry.iter() {
        let type_info = global.type_information.clone();
        builder.add_instance_dynamic(key.to_string(), global.maker.is_none(), type_info);
    }

    // find global dummies
    for (key, global) in global_registry.iter_dummies() {
        let kind = if let Some(type_info) = &global.type_information {
            builder.add_through_type_info(type_info);
            builder.lad_type_kind_from_through_type(type_info)
        } else {
            LadFieldOrVariableKind::Val(builder.lad_id_from_type_id(global.type_id))
        };

        builder.add_instance_manually(key.to_string(), false, kind);
    }

    let file = builder.build();
    let directory = &settings.output_directory;

    for processor in settings.processors.iter() {
        bevy_log::info!("Running ladfile processor: '{}'", processor.name());
        if let Err(e) = processor.run(&file, directory) {
            bevy_log::error!("Error in running ladfile processor: {e}")
        }
    }

    if let Some(filename) = &settings.ladfile_filename {
        let path = directory.join(filename);
        // generate
        let file = match ladfile::serialize_lad_file(&file, settings.pretty) {
            Ok(file) => file,
            Err(e) => {
                error!("Error serializing LAD file: {}", e);
                return;
            }
        };

        // save
        match std::fs::write(&path, file) {
            Ok(_) => {
                info!("Successfully generated LAD file at {:?}", path);
            }
            Err(e) => {
                error!("Error saving LAD file to {:?}: {}", path, e);
            }
        }
    }
}

fn generate_lad_file_system(
    type_registry: Res<AppTypeRegistry>,
    function_registry: Res<AppScriptFunctionRegistry>,
    dummy_function_registry: Res<DummyScriptFunctionRegistry>,
    global_registry: Res<AppScriptGlobalsRegistry>,
    settings: Res<LadFileSettingsArc>,
) {
    generate_lad_file(
        &type_registry,
        &function_registry,
        &dummy_function_registry,
        &global_registry,
        &settings.0,
    );
}

impl Plugin for ScriptingFilesGenerationPlugin {
    fn build(&self, app: &mut App) {
        if self.0.0.enabled {
            app.insert_resource(self.0.clone());
            app.add_systems(Startup, generate_lad_file_system);
        }
    }
}
