//! Plugins for bevy which allow generating ladfiles at startup

use std::{any::TypeId, ops::Deref, path::PathBuf, sync::Arc};

use ::{
    bevy_app::{App, Plugin, Startup},
    bevy_ecs::{prelude::Resource, reflect::AppTypeRegistry, system::Res, world::World},
};
use bevy_log::{error, info};
use bevy_mod_scripting_bindings::{
    AppScriptNamespaceMembers, DummyScriptFunctionRegistry, DynamicScriptFunction, FunctionInfo,
    IntoFunctionScope, ScriptFunctionRegistry, ScriptNamespaceMembers, StaticTypeRef,
    docgen::typed_through::BoundPlaceholder,
    function::{
        bound::is_asset_type, namespace::FunctionScope, script_function::AppScriptFunctionRegistry,
    },
    into_through_type_info_with_registry,
};
use bevy_reflect::{TypeInfo, TypeRegistry};
use ladfile::{LadFile, LadFilePlugin, LadTypeKind, default_importance};

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

/// The function used to generate a ladfile from pre-populated type, function and namespace registries
pub fn generate_lad_file(
    type_registry: &AppTypeRegistry,
    function_registry: &AppScriptFunctionRegistry,
    dummy_function_registry: &DummyScriptFunctionRegistry,
    namespace_members: &AppScriptNamespaceMembers,
    settings: &LadFileSettings,
) {
    let type_registry = type_registry.read();
    let functions = Functions {
        registry: &function_registry.read(),
        dummy: &dummy_function_registry.0.read(),
    };
    let mut builder = LadFileBuilder::new(&type_registry);
    builder
        .set_description(settings.description)
        .set_exclude_including_unregistered(settings.exclude_types_containing_unregistered)
        .set_sorted(true);

    // reflection only tells us the arguments of a generic type, so the generic types
    // which scripts see have to be declared
    if let Err(error) = builder.add_generic_type(
        "bevy_asset::handle::Handle<$A>",
        "A handle to an asset of type `A`.",
    ) {
        error!("Could not declare `Handle` as a generic type: {error}");
    }

    add_world(&mut builder, &functions);
    for registration in type_registry.iter() {
        add_registered_type(
            &mut builder,
            &functions,
            &type_registry,
            registration.type_info(),
        );
    }
    add_builtins(&mut builder, &functions);
    add_namespace_members(&mut builder, &namespace_members.read());

    write_lad_file(&builder.build(), settings);
}

/// The functions to document, the ones scripts can call and the ones documented on their own (see [`DummyScriptFunctionRegistry`])
struct Functions<'a> {
    registry: &'a ScriptFunctionRegistry,
    dummy: &'a ScriptFunctionRegistry,
}

impl Functions<'_> {
    /// The functions in the given scope
    fn in_scope(&self, scope: FunctionScope) -> impl Iterator<Item = &DynamicScriptFunction> {
        self.registry
            .iter_namespace(scope)
            .chain(self.dummy.iter_namespace(scope))
            .map(|(_, function)| function)
    }
}

/// The world, which is not a reflected type, and the `world` builtin
fn add_world(builder: &mut LadFileBuilder, functions: &Functions) {
    builder.add_nonreflect_type::<World>(
        Some("bevy_ecs"),
        r#"The ECS world containing all Components, Resources and Systems. Main point of interaction with a Bevy App."#.trim(),
    );
    for function in functions.in_scope(World::into_function_scope()) {
        builder.add_function_info(&function.info);
    }
    builder.set_insignificance(
        std::any::TypeId::of::<World>(),
        (default_importance() / 2) - 1,
    );
    let world = LadTypeKind::Val(builder.lad_id_from_type_id(TypeId::of::<World>()));
    builder.add_instance_manually("world", true, world);
}

/// A reflected type, with its functions, the functions shared by all types bound to it, and the functions of its variants
fn add_registered_type(
    builder: &mut LadFileBuilder,
    functions: &Functions,
    type_registry: &TypeRegistry,
    type_info: &'static TypeInfo,
) {
    // ignore things without an identifier
    if type_info.type_path_table().ident().is_none() {
        return;
    }

    // sets only tell us the type id of the types they hold so need the registry
    let through_type_info = into_through_type_info_with_registry(type_info, type_registry);
    builder.add_through_type_info(&through_type_info);

    let scope = FunctionScope::OnType(type_info.type_id());
    for function in functions.in_scope(scope) {
        builder.add_function_info(&function.info);
    }

    // functions on the static type reference are available on every type's own static reference,
    // bound to the type, unless the type has a function of its own with the same name.
    // This is the same signature scripts see at runtime, see `WorldExtensions::lookup_static_member`
    // Functions on asset handles are only documented on asset types.
    if type_info.type_id() != TypeId::of::<StaticTypeRef>() {
        let is_asset = is_asset_type(type_registry, type_info.type_id());
        for function in functions
            .registry
            .iter_namespace(FunctionScope::all_types())
            .map(|(_, f)| f)
        {
            if functions.registry.contains(scope, function.name().clone()) {
                continue;
            }
            if !is_asset && function.info.has_placeholder(BoundPlaceholder::Handle) {
                continue;
            }
            builder.add_function_info(&function.info.bind_to_type(type_info));
        }
    }

    // functions on enum variants whose signatures depend on the variant, i.e. its constructor, are bound to each variant
    if let TypeInfo::Enum(enum_info) = type_info {
        for (_, function) in functions
            .registry
            .iter_namespace(FunctionScope::all_variants())
        {
            if !function.info.has_placeholders() {
                continue;
            }
            // the documentation is the same for every variant, and stays on the function on `EnumVariantRef`.
            // Copying it would multiply it by the number of variants of all enums, a large part of the file
            let undocumented = FunctionInfo {
                docs: None,
                ..(*function.info).clone()
            };
            for variant in enum_info.iter() {
                let bound = undocumented
                    .clone()
                    .bind_placeholders(type_info, Some(variant.name()));
                builder.add_variant_function_info(&bound, type_info.type_id(), variant.name());
            }
        }
    }
}

/// The builtin functions, the ones in the global scope
fn add_builtins(builder: &mut LadFileBuilder, functions: &Functions) {
    for function in functions.in_scope(FunctionScope::Global) {
        builder.add_function_info(&function.info);
    }
}

/// The functions and values registered into namespaces
fn add_namespace_members(builder: &mut LadFileBuilder, members: &ScriptNamespaceMembers) {
    for (namespace, contents) in members.iter() {
        for function in contents.functions.values() {
            builder.add_namespace_function(namespace.clone(), &function.info);
        }
        for (name, value) in &contents.values {
            builder.add_namespace_value(namespace.clone(), name.clone(), &value.type_information);
        }
    }
}

/// Runs the processors on the file, and writes it if the settings say so
fn write_lad_file(file: &LadFile, settings: &LadFileSettings) {
    let directory = &settings.output_directory;

    for processor in settings.processors.iter() {
        bevy_log::info!("Running ladfile processor: '{}'", processor.name());
        if let Err(e) = processor.run(file, directory) {
            bevy_log::error!("Error in running ladfile processor: {e:#}")
        }
    }

    if let Some(filename) = &settings.ladfile_filename {
        let path = directory.join(filename);
        let file = match ladfile::serialize_lad_file(file, settings.pretty) {
            Ok(file) => file,
            Err(e) => {
                error!("Error serializing LAD file: {}", e);
                return;
            }
        };

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
    namespace_members: Option<Res<AppScriptNamespaceMembers>>,
    settings: Res<LadFileSettingsArc>,
) {
    generate_lad_file(
        &type_registry,
        &function_registry,
        &dummy_function_registry,
        &namespace_members.map(|m| m.clone()).unwrap_or_default(),
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
            |_: StaticTypeRef, _: BoundFields| -> BoundValue { unreachable!() },
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
            .register("describe", |_: StaticTypeRef| -> BoundValue {
                unreachable!()
            });
        NamespaceBuilder::<TestStruct>::new(app.world_mut()).register("describe", || 1usize);
        app.register_type::<TestEnum>()
            .register_type::<TestStruct>()
            .register_type::<DefaultEnum>()
            .register_type::<MixedFields>()
            .register_type::<usize>();

        generate_from(app)
    }

    /// Generates the ladfile of the app
    fn generate_from(mut app: App) -> LadFile {
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
                .get_resource_or_init::<AppScriptNamespaceMembers>()
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
                LadTypeKind::Fields {
                    type_id: type_id.clone(),
                    variant: None
                }
            );
            assert_eq!(new.return_type.kind, LadTypeKind::Val(type_id));
        }

        // the original keeps its receiver, and is documented as it is at runtime
        let generic_fields = LadTypeKind::HashMap(
            Box::new(LadTypeKind::Primitive(ReflectionPrimitiveKind::String)),
            Box::new(LadTypeKind::Primitive(ReflectionPrimitiveKind::ScriptValue)),
        );
        let generic_value = LadTypeKind::Primitive(ReflectionPrimitiveKind::ReflectReference);
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
                LadTypeKind::Fields {
                    type_id: enum_id.clone(),
                    variant: Some(variant.into())
                }
            );
            assert_eq!(new.return_type.kind, LadTypeKind::Val(enum_id.clone()));
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
            LadTypeKind::Primitive(ladfile::ReflectionPrimitiveKind::Usize)
        );
        let shared = function(&file, "TestEnum", "describe").unwrap();
        assert_eq!(
            shared.return_type.kind,
            LadTypeKind::Val(type_id(&file, "TestEnum"))
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

    #[derive(bevy_asset::Asset, Reflect)]
    struct TestAsset;

    /// A struct with a field for each container, which scripts see as the values they hold
    #[derive(Reflect)]
    struct Containers {
        vec: Vec<TestStruct>,
        option: Option<TestStruct>,
        map: bevy_platform::collections::HashMap<String, TestStruct>,
        tuple: (TestStruct, usize),
        array: [TestStruct; 2],
        set: bevy_platform::collections::HashSet<usize>,
    }

    #[derive(Reflect)]
    #[reflect(no_field_bounds, from_reflect = false)]
    struct UndeclaredGeneric<T> {
        #[reflect(ignore)]
        phantom: std::marker::PhantomData<T>,
    }

    #[test]
    fn handles_are_a_declared_generic_type() {
        let mut app = App::new();
        app.register_type::<TestAsset>()
            .register_type::<bevy_asset::Handle<TestAsset>>()
            .register_type_data::<TestAsset, bevy_asset::ReflectAsset>();
        let file = generate_from(app);

        assert_eq!(file.generic_types.len(), 1, "{:?}", file.generic_types);
        let (generic_id, generic_type) = file.generic_types.iter().next().unwrap();
        assert_eq!(generic_type.identifier, "Handle");
        assert_eq!(generic_type.path, "bevy_asset::handle::Handle");
        assert_eq!(generic_type.params, ["A"]);
        assert_eq!(generic_type.name_with_params(), "Handle<A>");
        assert!(generic_type.documentation.is_some());

        // the instantiation points at it, and its argument is the asset
        let instantiations = file
            .types
            .values()
            .filter(|ty| ty.generic_type.as_ref() == Some(generic_id))
            .collect::<Vec<_>>();
        assert_eq!(instantiations.len(), 1);
        let argument = &instantiations[0].generics[0];
        assert_eq!(argument.name, generic_type.params[0]);
        assert_eq!(argument.type_id, type_id(&file, "TestAsset"));
    }

    #[test]
    fn containers_are_not_types_or_generic_types() {
        let mut app = App::new();
        app.register_type::<Containers>();
        let file = generate_from(app);

        assert!(file.generic_types.is_empty(), "{:?}", file.generic_types);
        // the registry has all of them, but they are described as what they hold
        let containers = file
            .types
            .values()
            .filter(|ty| {
                ["Vec<", "Option<", "HashMap<", "HashSet<", "(", "["]
                    .iter()
                    .any(|c| ty.path.contains(c))
            })
            .map(|ty| ty.path.as_str())
            .collect::<Vec<_>>();
        assert!(containers.is_empty(), "{containers:?}");
        type_id(&file, "TestStruct");
    }

    #[test]
    fn undeclared_generic_types_are_only_known_by_their_instantiations() {
        let mut app = App::new();
        app.register_type::<UndeclaredGeneric<usize>>();
        let file = generate_from(app);

        let instantiation = &file.types[&type_id(&file, "UndeclaredGeneric")];
        assert_eq!(instantiation.generics.len(), 1);
        assert!(instantiation.generic_type.is_none());
        assert!(file.generic_types.is_empty());
    }
}
