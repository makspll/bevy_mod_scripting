//! Plugins for bevy which allow generating ladfiles at startup

use std::{any::TypeId, ops::Deref, path::PathBuf, sync::Arc};

use ::{
    bevy_app::{App, Plugin, Startup},
    bevy_ecs::{prelude::Resource, reflect::AppTypeRegistry, system::Res, world::World},
};
use bevy_log::{error, info};
use bevy_mod_scripting_bindings::{
    DummyScriptFunctionRegistry, EnumVariantRef, IntoNamespace, StaticTypeRef,
    function::{namespace::Namespace, script_function::AppScriptFunctionRegistry},
    globals::AppScriptGlobalsRegistry,
    into_through_type_info,
};
use bevy_reflect::TypeInfo;
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

        // functions on the static type reference are available on every type's own static reference,
        // bound to the type, unless the type has a function of its own with the same name.
        // This is the same signature scripts see at runtime, see `WorldExtensions::lookup_type_function`
        if type_info.type_id() != TypeId::of::<StaticTypeRef>() {
            let namespace = Namespace::OnType(type_info.type_id());
            for (key, function) in
                function_registry.iter_namespace(Namespace::OnType(TypeId::of::<StaticTypeRef>()))
            {
                if function_registry.contains(namespace, key.name.clone()) {
                    continue;
                }
                builder.add_function_info(&function.info.bind_to_type(type_info));
            }
        }

        // functions on enum variants whose signatures depend on the variant, i.e. its constructor, are bound to each variant
        if let TypeInfo::Enum(enum_info) = type_info {
            for (_, function) in
                function_registry.iter_namespace(Namespace::OnType(TypeId::of::<EnumVariantRef>()))
            {
                if !function.info.has_placeholders() {
                    continue;
                }
                for variant in enum_info.iter() {
                    let bound = (*function.info)
                        .clone()
                        .bind_placeholders(type_info, Some(variant.name()));
                    builder.add_variant_function_info(&bound, type_info.type_id(), variant.name());
                }
            }
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
            bevy_log::error!("Error in running ladfile processor: {e:#}")
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

#[cfg(test)]
mod test {
    use super::*;
    use bevy_app::App;
    use bevy_mod_scripting_bindings::{
        EnumVariantRef,
        function::{
            bound::{BoundFields, BoundValue},
            from::V,
            namespace::NamespaceBuilder,
        },
    };
    use bevy_reflect::{Reflect, std_traits::ReflectDefault};
    use ladfile::{LadFile, LadFunctionNamespace, ReflectionPrimitiveKind};
    use std::{error::Error, path::Path, sync::Mutex};

    #[derive(Reflect)]
    enum TestEnum {
        A,
    }

    #[derive(Reflect)]
    struct TestStruct;

    #[derive(Reflect, Default)]
    #[reflect(Default)]
    enum DefaultEnum {
        A(TestStruct),
        #[default]
        B,
    }

    #[derive(Reflect)]
    struct MixedFields {
        defaulted: usize,
        required: TestStruct,
    }

    struct Capture(Arc<Mutex<Option<LadFile>>>);

    impl LadFilePlugin for Capture {
        fn name(&self) -> &'static str {
            "capture"
        }

        fn run(&self, ladfile: &LadFile, _: &Path) -> Result<(), Box<dyn Error>> {
            *self.0.lock().unwrap() = Some(ladfile.clone());
            Ok(())
        }
    }

    /// Registers a constructor like the one from `ConstructorsPlugin`, which lives in a crate we don't depend on
    fn generate(constructor_name: &'static str) -> LadFile {
        let mut app = App::new();
        NamespaceBuilder::<StaticTypeRef>::new(app.world_mut()).register_documented(
            constructor_name,
            |_: V<StaticTypeRef>, _: BoundFields| -> BoundValue { unreachable!() },
            "",
            &["type", "payload"],
        );
        NamespaceBuilder::<EnumVariantRef>::new(app.world_mut()).register_documented(
            constructor_name,
            |_: V<EnumVariantRef>, _: BoundFields| -> BoundValue { unreachable!() },
            "",
            &["variant", "payload"],
        );
        NamespaceBuilder::<StaticTypeRef>::new(app.world_mut())
            .register("describe", |_: V<StaticTypeRef>| -> BoundValue {
                unreachable!()
            });
        NamespaceBuilder::<TestStruct>::new(app.world_mut()).register("describe", || 1usize);
        app.register_type::<TestEnum>()
            .register_type::<TestStruct>()
            .register_type::<DefaultEnum>()
            .register_type::<MixedFields>()
            .register_type::<usize>();

        let captured = Arc::new(Mutex::new(None));
        let settings = LadFileSettings {
            processors: vec![Box::new(Capture(captured.clone()))],
            ..Default::default()
        };
        let world = app.world_mut();
        generate_lad_file(
            &world.get_resource_or_init::<AppTypeRegistry>().clone(),
            &world
                .get_resource_or_init::<AppScriptFunctionRegistry>()
                .clone(),
            &world
                .get_resource_or_init::<DummyScriptFunctionRegistry>()
                .clone(),
            &world
                .get_resource_or_init::<AppScriptGlobalsRegistry>()
                .clone(),
            &settings,
        );
        captured
            .lock()
            .unwrap()
            .take()
            .expect("no ladfile generated")
    }

    /// The function with the given name on the type with the given identifier
    fn function<'a>(
        file: &'a LadFile,
        type_name: &str,
        function: &str,
    ) -> Option<&'a ladfile::LadFunction> {
        let (type_id, _) = file.types.iter().find(|(_, t)| t.identifier == type_name)?;
        file.functions.values().find(|f| {
            f.identifier == function
                && matches!(&f.namespace, ladfile::LadFunctionNamespace::Type(id) if id == type_id)
        })
    }

    fn type_id(file: &LadFile, type_name: &str) -> ladfile::LadTypeId {
        file.types
            .iter()
            .find(|(_, t)| t.identifier == type_name)
            .map(|(id, _)| id.clone())
            .unwrap()
    }

    #[test]
    fn static_type_functions_are_bound_to_every_type() {
        let file = generate("new");

        for type_name in ["TestEnum", "TestStruct"] {
            let new = function(&file, type_name, "new")
                .unwrap_or_else(|| panic!("`new` missing on {type_name}"));
            let type_id = type_id(&file, type_name);

            // the type is bound, so it is not an argument, and the placeholders are filled in
            assert_eq!(new.arguments.len(), 1, "{:?}", new.arguments);
            assert_eq!(new.arguments[0].name.as_deref(), Some("payload"));
            assert_eq!(
                new.arguments[0].kind,
                LadFieldOrVariableKind::Fields {
                    type_id: type_id.clone(),
                    variant: None
                }
            );
            assert_eq!(new.return_type.kind, LadFieldOrVariableKind::Val(type_id));
        }

        // the original keeps its receiver, and is documented as it is at runtime
        let generic_fields = LadFieldOrVariableKind::HashMap(
            Box::new(LadFieldOrVariableKind::Primitive(
                ReflectionPrimitiveKind::String,
            )),
            Box::new(LadFieldOrVariableKind::Primitive(
                ReflectionPrimitiveKind::ScriptValue,
            )),
        );
        let generic_value =
            LadFieldOrVariableKind::Primitive(ReflectionPrimitiveKind::ReflectReference);
        let new = function(&file, "StaticTypeRef", "new").expect("`new` missing on StaticTypeRef");
        assert_eq!(new.arguments[0].name.as_deref(), Some("type"));
        assert_eq!(new.arguments[1].kind, generic_fields);
        let new =
            function(&file, "EnumVariantRef", "new").expect("`new` missing on EnumVariantRef");
        assert_eq!(new.arguments[0].name.as_deref(), Some("variant"));
        assert_eq!(new.return_type.kind, generic_value);
    }

    #[test]
    fn variant_functions_are_bound_to_every_variant() {
        let file = generate("new");
        let enum_id = type_id(&file, "DefaultEnum");

        for variant in ["A", "B"] {
            let new = file
                .functions
                .values()
                .find(|f| {
                    f.identifier == "new"
                        && f.namespace
                            == LadFunctionNamespace::Variant {
                                type_id: enum_id.clone(),
                                variant: variant.into(),
                            }
                })
                .unwrap_or_else(|| panic!("`new` missing on variant {variant}"));

            // the receiver is the variant, which is not bound
            assert_eq!(new.arguments[0].name.as_deref(), Some("variant"));
            assert_eq!(
                new.arguments[1].kind,
                LadFieldOrVariableKind::Fields {
                    type_id: enum_id.clone(),
                    variant: Some(variant.into())
                }
            );
            assert_eq!(
                new.return_type.kind,
                LadFieldOrVariableKind::Val(enum_id.clone())
            );
        }

        // variants are not types, so their functions are not associated with the enum
        assert!(file.types[&enum_id].associated_functions.iter().all(|id| {
            !matches!(
                file.functions[id].namespace,
                LadFunctionNamespace::Variant { .. }
            )
        }));
    }

    #[test]
    fn functions_on_the_type_take_priority() {
        let file = generate("new");

        // `TestStruct` has its own `describe`, `TestEnum` gets the shared one
        let own = function(&file, "TestStruct", "describe").unwrap();
        assert_eq!(
            own.return_type.kind,
            LadFieldOrVariableKind::Primitive(ladfile::ReflectionPrimitiveKind::Usize)
        );
        let shared = function(&file, "TestEnum", "describe").unwrap();
        assert_eq!(
            shared.return_type.kind,
            LadFieldOrVariableKind::Val(type_id(&file, "TestEnum"))
        );
    }

    #[test]
    fn constructor_name_is_configurable() {
        let file = generate("create");
        assert!(function(&file, "TestStruct", "create").is_some());
        assert!(function(&file, "TestStruct", "new").is_none());
    }

    #[test]
    fn defaults_are_recorded() {
        let file = generate("new");
        let definition = |name: &str| &file.types[&type_id(&file, name)];

        // enums record which variant their default is in
        assert_eq!(
            definition("DefaultEnum").metadata.default,
            Some(ladfile::LadDefault {
                variant: Some("B".into())
            })
        );
        assert_eq!(definition("MixedFields").metadata.default, None);

        // fields record whether their type has a default
        let ladfile::LadTypeLayout::MonoVariant(ladfile::LadVariant::Struct { fields, .. }) =
            &definition("MixedFields").layout
        else {
            panic!("expected a struct layout");
        };
        let has_default = fields
            .iter()
            .map(|f| (f.name.as_str(), f.has_default))
            .collect::<Vec<_>>();
        assert_eq!(has_default, [("defaulted", true), ("required", false)]);
    }
}
