//! Systems and resources for handling script assets and events

use crate::{
    commands::{CreateOrUpdateScript, DeleteScript},
    error::ScriptError,
    script::ScriptId,
    IntoScriptPluginParams, ScriptingSystemSet,
};
use bevy::{
    app::{App, PreUpdate},
    asset::{Asset, AssetEvent, AssetId, AssetLoader, AssetPath, Assets},
    ecs::system::Resource,
    log::{debug, info, trace, warn},
    prelude::{
        Commands, Event, EventReader, EventWriter, IntoSystemConfigs, IntoSystemSetConfigs, Res,
        ResMut,
    },
    reflect::TypePath,
    utils::hashbrown::HashMap,
};
use std::borrow::Cow;

/// Represents a scripting language. Languages which compile into another language should use the target language as their language.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default)]
pub enum Language {
    /// The Rhai scripting language
    Rhai,
    /// The Lua scripting language
    Lua,
    /// The Rune scripting language
    Rune,
    /// An external scripting language
    External(Cow<'static, str>),
    /// Set if none of the asset path to language mappers match
    #[default]
    Unknown,
}

impl std::fmt::Display for Language {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Language::Rhai => "Rhai".fmt(f),
            Language::Lua => "Lua".fmt(f),
            Language::Rune => "Rune".fmt(f),
            Language::External(cow) => cow.fmt(f),
            Language::Unknown => "Unknown".fmt(f),
        }
    }
}

/// Represents a script loaded into memory as an asset
#[derive(Asset, TypePath, Clone)]
pub struct ScriptAsset {
    /// The body of the script
    pub content: Box<[u8]>,
    /// The virtual filesystem path of the asset, used to map to the script Id for asset backed scripts
    pub asset_path: AssetPath<'static>,
}

#[derive(Event, Debug, Clone)]
pub(crate) enum ScriptAssetEvent {
    Added(ScriptMetadata),
    Removed(ScriptMetadata),
    Modified(ScriptMetadata),
}

#[derive(Default)]
/// A loader for script assets
pub struct ScriptAssetLoader {
    /// The file extensions this loader should handle
    pub extensions: &'static [&'static str],
    /// preprocessor to run on the script before saving the content to an asset
    pub preprocessor: Option<Box<dyn Fn(&mut [u8]) -> Result<(), ScriptError> + Send + Sync>>,
}

impl AssetLoader for ScriptAssetLoader {
    type Asset = ScriptAsset;

    type Settings = ();

    type Error = ScriptError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        _settings: &Self::Settings,
        load_context: &mut bevy::asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut content = Vec::new();
        reader
            .read_to_end(&mut content)
            .await
            .map_err(|e| ScriptError::new_external(e).with_context(load_context.asset_path()))?;
        if let Some(processor) = &self.preprocessor {
            processor(&mut content)?;
        }
        let asset = ScriptAsset {
            content: content.into_boxed_slice(),
            asset_path: load_context.asset_path().to_owned(),
        };
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        self.extensions
    }
}

#[derive(Clone, Resource)]
/// Settings to do with script assets and how they are handled
pub struct ScriptAssetSettings {
    /// Strategy for mapping asset paths to script ids, by default this is the identity function
    pub script_id_mapper: AssetPathToScriptIdMapper,
    /// Mapping from extension to script language
    pub extension_to_language_map: HashMap<&'static str, Language>,

    /// The currently supported asset extensions
    /// Should be updated by each scripting plugin to include the extensions it supports.
    ///
    /// Will be used to populate the script asset loader with the supported extensions
    pub supported_extensions: &'static [&'static str],
}

impl ScriptAssetSettings {
    /// Selects the language for a given asset path
    pub fn select_script_language(&self, path: &AssetPath) -> Language {
        let extension = path
            .path()
            .extension()
            .and_then(|ext| ext.to_str())
            .unwrap_or_default();
        self.extension_to_language_map
            .get(extension)
            .cloned()
            .unwrap_or_default()
    }
}

impl Default for ScriptAssetSettings {
    fn default() -> Self {
        Self {
            script_id_mapper: AssetPathToScriptIdMapper {
                map: (|path: &AssetPath| path.path().to_string_lossy().into_owned().into()),
            },
            extension_to_language_map: HashMap::from_iter(vec![
                ("lua", Language::Lua),
                ("luau", Language::Lua),
                ("rhai", Language::Rhai),
                ("rn", Language::Rune),
            ]),
            supported_extensions: &["lua", "luau", "rhai", "rn"],
        }
    }
}

/// Strategy for mapping asset paths to script ids, by default this is the identity function
#[derive(Clone, Copy)]
pub struct AssetPathToScriptIdMapper {
    /// The mapping function
    pub map: fn(&AssetPath) -> ScriptId,
}

/// A cache of asset id's to their script id's. Necessary since when we drop an asset we won't have the ability to get the path from the asset.
#[derive(Default, Debug, Resource)]
pub struct ScriptMetadataStore {
    /// The map of asset id's to their metadata
    pub map: HashMap<AssetId<ScriptAsset>, ScriptMetadata>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
/// Metadata for a script asset
pub struct ScriptMetadata {
    /// The asset id of the script
    pub asset_id: AssetId<ScriptAsset>,
    /// The script id of the script
    pub script_id: ScriptId,
    /// The language of the script
    pub language: Language,
}

impl ScriptMetadataStore {
    /// Inserts a new metadata entry
    pub fn insert(&mut self, id: AssetId<ScriptAsset>, meta: ScriptMetadata) {
        // TODO: new generations of assets are not going to have the same ID as the old one
        self.map.insert(id, meta);
    }

    /// Gets a metadata entry
    pub fn get(&self, id: AssetId<ScriptAsset>) -> Option<&ScriptMetadata> {
        self.map.get(&id)
    }

    /// Removes a metadata entry
    pub fn remove(&mut self, id: AssetId<ScriptAsset>) -> Option<ScriptMetadata> {
        self.map.remove(&id)
    }

    /// Checks if the store contains a metadata entry
    pub fn contains(&self, id: AssetId<ScriptAsset>) -> bool {
        self.map.contains_key(&id)
    }
}

/// Converts incoming asset events, into internal script asset events, also loads and inserts metadata for newly added scripts
pub(crate) fn dispatch_script_asset_events(
    mut events: EventReader<AssetEvent<ScriptAsset>>,
    mut script_asset_events: EventWriter<ScriptAssetEvent>,
    assets: Res<Assets<ScriptAsset>>,
    mut metadata_store: ResMut<ScriptMetadataStore>,
    settings: Res<ScriptAssetSettings>,
) {
    for event in events.read() {
        match event {
            AssetEvent::LoadedWithDependencies { id } | AssetEvent::Added { id } => {
                // these can occur multiple times, we only send one added event though
                if !metadata_store.contains(*id) {
                    let asset = assets.get(*id);
                    if let Some(asset) = asset {
                        let path = &asset.asset_path;
                        let converter = settings.script_id_mapper.map;
                        let script_id = converter(path);

                        let language = settings.select_script_language(path);
                        let metadata = ScriptMetadata {
                            asset_id: *id,
                            script_id,
                            language,
                        };
                        debug!("Script loaded, populating metadata: {:?}:", metadata);
                        script_asset_events.send(ScriptAssetEvent::Added(metadata.clone()));
                        metadata_store.insert(*id, metadata);
                    } else {
                        warn!("A script was added but it's asset was not found, failed to compute metadata. This script will not be loaded. Did you forget to store `Handle<ScriptAsset>` somewhere?. {}", id);
                    }
                }
            }
            AssetEvent::Removed { id } => {
                if let Some(metadata) = metadata_store.get(*id) {
                    debug!("Script removed: {:?}", metadata);
                    script_asset_events.send(ScriptAssetEvent::Removed(metadata.clone()));
                } else {
                    warn!("Script metadata not found for removed script asset: {}. Cannot properly clean up script", id);
                }
            }
            AssetEvent::Modified { id } => {
                if let Some(metadata) = metadata_store.get(*id) {
                    debug!("Script modified: {:?}", metadata);
                    script_asset_events.send(ScriptAssetEvent::Modified(metadata.clone()));
                } else {
                    warn!("Script metadata not found for modified script asset: {}. Cannot properly update script", id);
                }
            }
            _ => {}
        }
    }
}

/// Listens to [`ScriptAssetEvent::Removed`] events and removes the corresponding script metadata.
pub(crate) fn remove_script_metadata(
    mut events: EventReader<ScriptAssetEvent>,
    mut asset_path_map: ResMut<ScriptMetadataStore>,
) {
    for event in events.read() {
        if let ScriptAssetEvent::Removed(metadata) = event {
            let previous = asset_path_map.remove(metadata.asset_id);
            if let Some(previous) = previous {
                debug!("Removed script metadata: {:?}", previous);
            }
        }
    }
}

/// Listens to [`ScriptAssetEvent`] events and dispatches [`CreateOrUpdateScript`] and [`DeleteScript`] commands accordingly.
///
/// Allows for hot-reloading of scripts.
pub(crate) fn sync_script_data<P: IntoScriptPluginParams>(
    mut events: EventReader<ScriptAssetEvent>,
    script_assets: Res<Assets<ScriptAsset>>,
    mut commands: Commands,
) {
    for event in events.read() {
        let metadata = match event {
            ScriptAssetEvent::Added(script_metadata)
            | ScriptAssetEvent::Removed(script_metadata)
            | ScriptAssetEvent::Modified(script_metadata) => script_metadata,
        };

        if metadata.language != P::LANGUAGE {
            continue;
        }

        trace!("{}: Received script asset event: {:?}", P::LANGUAGE, event);
        match event {
            // emitted when a new script asset is loaded for the first time
            ScriptAssetEvent::Added(_) | ScriptAssetEvent::Modified(_) => {
                if metadata.language != P::LANGUAGE {
                    trace!(
                        "{}: Script asset with id: {} is for a different langauge than this sync system. Skipping.",
                        P::LANGUAGE,
                        metadata.script_id
                    );
                    continue;
                }

                info!("{}: Loading Script: {:?}", P::LANGUAGE, metadata.script_id,);

                if let Some(asset) = script_assets.get(metadata.asset_id) {
                    commands.queue(CreateOrUpdateScript::<P>::new(
                        metadata.script_id.clone(),
                        asset.content.clone(),
                        Some(script_assets.reserve_handle().clone_weak()),
                    ));
                }
            }
            ScriptAssetEvent::Removed(_) => {
                info!("{}: Deleting Script: {:?}", P::LANGUAGE, metadata.script_id,);
                commands.queue(DeleteScript::<P>::new(metadata.script_id.clone()));
            }
        };
    }
}

/// Setup all the asset systems for the scripting plugin and the dependencies
pub(crate) fn configure_asset_systems(app: &mut App) -> &mut App {
    // these should be in the same set as bevy's asset systems
    // currently this is in the PreUpdate set
    app.add_systems(
        PreUpdate,
        (
            dispatch_script_asset_events.in_set(ScriptingSystemSet::ScriptAssetDispatch),
            remove_script_metadata.in_set(ScriptingSystemSet::ScriptMetadataRemoval),
        ),
    )
    .configure_sets(
        PreUpdate,
        (
            ScriptingSystemSet::ScriptAssetDispatch.after(bevy::asset::TrackAssets),
            ScriptingSystemSet::ScriptCommandDispatch
                .after(ScriptingSystemSet::ScriptAssetDispatch)
                .before(ScriptingSystemSet::ScriptMetadataRemoval),
        ),
    )
    .init_resource::<ScriptMetadataStore>()
    .init_resource::<ScriptAssetSettings>()
    .add_event::<ScriptAssetEvent>();

    app
}

/// Setup all the asset systems for the scripting plugin and the dependencies
pub(crate) fn configure_asset_systems_for_plugin<P: IntoScriptPluginParams>(
    app: &mut App,
) -> &mut App {
    app.add_systems(
        PreUpdate,
        sync_script_data::<P>.in_set(ScriptingSystemSet::ScriptCommandDispatch),
    );
    app
}

#[cfg(test)]
mod tests {
    use std::path::{Path, PathBuf};

    use bevy::{
        app::{App, Update},
        asset::{AssetApp, AssetPlugin, AssetServer, Assets, Handle, LoadState},
        MinimalPlugins,
    };

    use super::*;

    fn init_loader_test(loader: ScriptAssetLoader) -> App {
        let mut app = App::new();
        app.add_plugins((MinimalPlugins, AssetPlugin::default()));
        app.init_asset::<ScriptAsset>();
        app.register_asset_loader(loader);
        app
    }

    fn make_test_settings() -> ScriptAssetSettings {
        ScriptAssetSettings {
            supported_extensions: &[],
            script_id_mapper: AssetPathToScriptIdMapper {
                map: |path| path.path().to_string_lossy().into_owned().into(),
            },
            extension_to_language_map: HashMap::from_iter(vec![
                ("lua", Language::Lua),
                ("rhai", Language::Rhai),
            ]),
        }
    }

    fn load_asset(app: &mut App, path: &str) -> Handle<ScriptAsset> {
        let handle = app.world_mut().resource::<AssetServer>().load(path);

        loop {
            let state = app
                .world()
                .resource::<AssetServer>()
                .get_load_state(&handle)
                .unwrap();
            if !matches!(state, LoadState::Loading) {
                break;
            }
            app.update();
        }

        match app
            .world()
            .resource::<AssetServer>()
            .get_load_state(&handle)
            .unwrap()
        {
            LoadState::NotLoaded => panic!("Asset not loaded"),
            LoadState::Loaded => {}
            LoadState::Failed(asset_load_error) => {
                panic!("Asset load failed: {:?}", asset_load_error)
            }
            _ => panic!("Unexpected load state"),
        }

        handle
    }

    #[test]
    fn test_asset_loader_loads() {
        let loader = ScriptAssetLoader {
            extensions: &["script"],
            preprocessor: None,
        };
        let mut app = init_loader_test(loader);

        let handle = load_asset(&mut app, "test_assets/test_script.script");
        let asset = app
            .world()
            .get_resource::<Assets<ScriptAsset>>()
            .unwrap()
            .get(&handle)
            .unwrap();

        assert_eq!(
            asset.asset_path,
            AssetPath::from_path(&PathBuf::from("test_assets/test_script.script"))
        );

        assert_eq!(
            String::from_utf8(asset.content.clone().to_vec()).unwrap(),
            "test script".to_string()
        );
    }

    #[test]
    fn test_asset_loader_applies_preprocessor() {
        let loader = ScriptAssetLoader {
            extensions: &["script"],
            preprocessor: Some(Box::new(|content| {
                content[0] = b'p';
                Ok(())
            })),
        };
        let mut app = init_loader_test(loader);

        let handle = load_asset(&mut app, "test_assets/test_script.script");
        let asset = app
            .world()
            .get_resource::<Assets<ScriptAsset>>()
            .unwrap()
            .get(&handle)
            .unwrap();

        assert_eq!(
            asset.asset_path,
            AssetPath::from(PathBuf::from("test_assets/test_script.script"))
        );
        assert_eq!(
            String::from_utf8(asset.content.clone().to_vec()).unwrap(),
            "pest script".to_string()
        );
    }

    #[test]
    fn test_metadata_store() {
        let mut store = ScriptMetadataStore::default();
        let id = AssetId::invalid();
        let meta = ScriptMetadata {
            asset_id: AssetId::invalid(),
            script_id: "test".into(),
            language: Language::Lua,
        };

        store.insert(id, meta.clone());
        assert_eq!(store.get(id), Some(&meta));

        assert_eq!(store.remove(id), Some(meta));
    }

    #[test]
    fn test_script_asset_settings_select_language() {
        let settings = make_test_settings();

        let path = AssetPath::from(Path::new("test.lua"));
        assert_eq!(settings.select_script_language(&path), Language::Lua);
        assert_eq!(
            settings.select_script_language(&AssetPath::from(Path::new("test.rhai"))),
            Language::Rhai
        );
        assert_eq!(
            settings.select_script_language(&AssetPath::from(Path::new("test.blob"))),
            Language::Unknown
        );
    }

    fn run_app_untill_asset_event(app: &mut App, event_kind: AssetEvent<ScriptAsset>) {
        let checker_system = |mut reader: EventReader<AssetEvent<ScriptAsset>>,
                              mut event_target: ResMut<EventTarget>| {
            println!("Reading asset events this frame");
            for event in reader.read() {
                println!("{:?}", event);
                if matches!(
                    (event_target.event, event),
                    (AssetEvent::Added { .. }, AssetEvent::Added { .. })
                        | (AssetEvent::Modified { .. }, AssetEvent::Modified { .. })
                        | (AssetEvent::Removed { .. }, AssetEvent::Removed { .. })
                        | (AssetEvent::Unused { .. }, AssetEvent::Unused { .. })
                        | (
                            AssetEvent::LoadedWithDependencies { .. },
                            AssetEvent::LoadedWithDependencies { .. },
                        )
                ) {
                    println!("Event matched");
                    event_target.happened = true;
                }
            }
        };

        if !app.world().contains_resource::<EventTarget>() {
            // for when we run this multiple times in a test
            app.add_systems(Update, checker_system);
        }

        #[derive(Resource)]
        struct EventTarget {
            event: AssetEvent<ScriptAsset>,
            happened: bool,
        }
        app.world_mut().insert_resource(EventTarget {
            event: event_kind,
            happened: false,
        });

        loop {
            println!("Checking if asset event was dispatched");
            if app.world().get_resource::<EventTarget>().unwrap().happened {
                println!("Stopping loop");
                break;
            }
            println!("Running app");

            app.update();
        }
    }

    struct DummyPlugin;

    impl IntoScriptPluginParams for DummyPlugin {
        type R = ();
        type C = ();
        const LANGUAGE: Language = Language::Lua;

        fn build_runtime() -> Self::R {}
    }

    #[test]
    fn test_asset_metadata_systems() {
        // test metadata flow
        let mut app = init_loader_test(ScriptAssetLoader {
            extensions: &[],
            preprocessor: None,
        });
        app.world_mut().insert_resource(make_test_settings());
        configure_asset_systems(&mut app);

        // update untill the asset event gets dispatched
        let asset_server: &AssetServer = app.world().resource::<AssetServer>();
        let handle = asset_server.load("test_assets/test_script.lua");
        run_app_untill_asset_event(
            &mut app,
            AssetEvent::LoadedWithDependencies {
                id: AssetId::invalid(),
            },
        );
        let asset_id = handle.id();

        // we expect the metadata to be inserted now, in the same frame as the asset is loaded
        let metadata = app
            .world()
            .get_resource::<ScriptMetadataStore>()
            .unwrap()
            .get(asset_id)
            .expect("Metadata not found");

        assert_eq!(metadata.script_id, "test_assets/test_script.lua");
        assert_eq!(metadata.language, Language::Lua);

        // ----------------- REMOVING -----------------

        // we drop the handle and wait untill the first asset event is dispatched
        drop(handle);

        run_app_untill_asset_event(
            &mut app,
            AssetEvent::Removed {
                id: AssetId::invalid(),
            },
        );

        // we expect the metadata to be removed now, in the same frame as the asset is removed
        let metadata_len = app
            .world()
            .get_resource::<ScriptMetadataStore>()
            .unwrap()
            .map
            .len();

        assert_eq!(metadata_len, 0);
    }

    // #[test]
    // fn test_syncing_assets() {
    //     todo!()
    // }
}
