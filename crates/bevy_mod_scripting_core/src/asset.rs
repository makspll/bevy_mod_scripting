use crate::{
    commands::{CreateOrUpdateScript, DeleteScript},
    error::ScriptError,
    script::ScriptId,
    IntoScriptPluginParams, ScriptingSystemSet,
};
use bevy::{
    app::{App, PreUpdate},
    asset::{Asset, AssetEvent, AssetId, AssetLoader, Assets},
    ecs::system::Resource,
    log::{error, info, trace},
    prelude::{Commands, EventReader, IntoSystemConfigs, IntoSystemSetConfigs, Res, ResMut},
    reflect::TypePath,
    utils::HashMap,
};
use std::{
    borrow::Cow,
    path::{Path, PathBuf},
};

/// Represents a scripting language. Languages which compile into another language should use the target language as their language.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub enum Language {
    Rhai,
    Lua,
    Rune,
    External(Cow<'static, str>),
    /// Set if none of the asset path to language mappers match
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
    pub content: Box<[u8]>,
    /// The virtual filesystem path of the asset, used to map to the script Id for asset backed scripts
    pub asset_path: PathBuf,
}

#[derive(Default)]
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
            asset_path: load_context.path().to_owned(),
        };
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        self.extensions
    }
}

#[derive(Clone, Resource)]
pub struct ScriptAssetSettings {
    pub script_id_mapper: AssetPathToScriptIdMapper,
    pub script_language_mappers: Vec<AssetPathToLanguageMapper>,
}

impl ScriptAssetSettings {
    pub fn select_script_language(&self, path: &Path) -> Language {
        for mapper in &self.script_language_mappers {
            let language = (mapper.map)(path);
            match language {
                Language::Unknown => continue,
                _ => return language,
            }
        }

        Language::Unknown
    }
}

impl Default for ScriptAssetSettings {
    fn default() -> Self {
        Self {
            script_id_mapper: AssetPathToScriptIdMapper {
                map: (|path: &Path| path.to_string_lossy().into_owned().into()),
            },
            script_language_mappers: vec![],
        }
    }
}

/// Strategy for mapping asset paths to script ids, by default this is the identity function
#[derive(Clone, Copy)]
pub struct AssetPathToScriptIdMapper {
    pub map: fn(&Path) -> ScriptId,
}

#[derive(Clone, Copy)]
pub struct AssetPathToLanguageMapper {
    pub map: fn(&Path) -> Language,
}

/// A cache of asset id's to their script id's. Necessary since when we drop an asset we won't have the ability to get the path from the asset.
#[derive(Default, Debug, Resource)]
pub struct ScriptMetadataStore {
    pub map: HashMap<AssetId<ScriptAsset>, ScriptMetadata>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ScriptMetadata {
    pub script_id: ScriptId,
    pub language: Language,
}

impl ScriptMetadataStore {
    pub fn insert(&mut self, id: AssetId<ScriptAsset>, meta: ScriptMetadata) {
        // TODO: new generations of assets are not going to have the same ID as the old one
        self.map.insert(id, meta);
    }

    pub fn get(&self, id: AssetId<ScriptAsset>) -> Option<&ScriptMetadata> {
        self.map.get(&id)
    }

    pub fn remove(&mut self, id: AssetId<ScriptAsset>) -> Option<ScriptMetadata> {
        self.map.remove(&id)
    }
}

/// Listens to `AssetEvent<ScriptAsset>::Added` events and populates the script metadata store
pub fn insert_script_metadata(
    mut events: EventReader<AssetEvent<ScriptAsset>>,
    script_assets: Res<Assets<ScriptAsset>>,
    mut asset_path_map: ResMut<ScriptMetadataStore>,
    settings: Res<ScriptAssetSettings>,
) {
    for event in events.read() {
        if let AssetEvent::LoadedWithDependencies { id } = event {
            let asset = script_assets.get(*id);
            if let Some(asset) = asset {
                let path = &asset.asset_path;
                let converter = settings.script_id_mapper.map;
                let script_id = converter(path);

                let language = settings.select_script_language(path);
                let metadata = ScriptMetadata {
                    script_id,
                    language,
                };
                info!("Populating script metadata for script: {:?}:", metadata);
                asset_path_map.insert(*id, metadata);
            } else {
                error!("A script was added but it's asset was not found, failed to compute metadata. This script will not be loaded. {}", id);
            }
        }
    }
}

/// Listens to [`AssetEvent<ScriptAsset>::Removed`] events and removes the corresponding script metadata
pub fn remove_script_metadata(
    mut events: EventReader<AssetEvent<ScriptAsset>>,
    mut asset_path_map: ResMut<ScriptMetadataStore>,
) {
    for event in events.read() {
        if let AssetEvent::Removed { id } = event {
            let previous = asset_path_map.remove(*id);
            if let Some(previous) = previous {
                info!("Removed script metadata for removed script: {:?}", previous);
            }
        }
    }
}

/// Listens to [`AssetEvent<ScriptAsset>`] events and dispatches [`CreateOrUpdateScript`] and [`DeleteScript`] commands accordingly.
///
/// Allows for hot-reloading of scripts.
pub fn sync_script_data<P: IntoScriptPluginParams>(
    mut events: EventReader<AssetEvent<ScriptAsset>>,
    script_assets: Res<Assets<ScriptAsset>>,
    script_metadata: Res<ScriptMetadataStore>,
    mut commands: Commands,
) {
    for event in events.read() {
        trace!("{}: Received script asset event: {:?}", P::LANGUAGE, event);
        match event {
            // emitted when a new script asset is loaded for the first time
            AssetEvent::LoadedWithDependencies { id } | AssetEvent::Modified { id } => {
                let metadata = match script_metadata.get(*id) {
                    Some(m) => m,
                    None => {
                        error!(
                            "{}: Script metadata not found for script asset with id: {}. Cannot load script.",
                            P::LANGUAGE,
                            id
                        );
                        continue;
                    }
                };

                if metadata.language != P::LANGUAGE {
                    trace!(
                        "{}: Script asset with id: {} is for a different langauge than this sync system. Skipping.",
                        P::LANGUAGE,
                        metadata.script_id
                    );
                    continue;
                }

                info!(
                    "{}: Dispatching Creation/Modification command for script: {:?}. Asset Id: {}",
                    P::LANGUAGE,
                    metadata,
                    id
                );

                if let Some(asset) = script_assets.get(*id) {
                    commands.queue(CreateOrUpdateScript::<P>::new(
                        metadata.script_id.clone(),
                        asset.content.clone(),
                        Some(script_assets.reserve_handle().clone_weak()),
                    ));
                }
            }
            AssetEvent::Removed { id } => {
                let metadata = match script_metadata.get(*id) {
                    Some(m) => m,
                    None => {
                        error!(
                            "{}: Script metadata not found for script asset with id: {}. Cannot delete script.",
                            P::LANGUAGE,
                            id
                        );
                        return;
                    }
                };

                info!(
                    "{}: Dispatching Deletion command for script: {:?}. Asset Id: {}",
                    P::LANGUAGE,
                    metadata,
                    id
                );
                commands.queue(DeleteScript::<P>::new(metadata.script_id.clone()));
            }
            _ => return,
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
            insert_script_metadata.in_set(ScriptingSystemSet::ScriptMetadataInsertion),
            remove_script_metadata.in_set(ScriptingSystemSet::ScriptMetadataRemoval),
        ),
    )
    .configure_sets(
        PreUpdate,
        (
            ScriptingSystemSet::ScriptMetadataInsertion.after(bevy::asset::TrackAssets),
            ScriptingSystemSet::ScriptCommandDispatch
                .after(ScriptingSystemSet::ScriptMetadataInsertion)
                .before(ScriptingSystemSet::ScriptMetadataRemoval),
        ),
    )
    .init_resource::<ScriptMetadataStore>()
    .init_resource::<ScriptAssetSettings>();

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
            script_id_mapper: AssetPathToScriptIdMapper {
                map: |path| path.to_string_lossy().into_owned().into(),
            },
            script_language_mappers: vec![
                AssetPathToLanguageMapper {
                    map: |path| {
                        if path.extension().unwrap() == "lua" {
                            Language::Lua
                        } else {
                            Language::Unknown
                        }
                    },
                },
                AssetPathToLanguageMapper {
                    map: |path| {
                        if path.extension().unwrap() == "rhai" {
                            Language::Rhai
                        } else {
                            Language::Unknown
                        }
                    },
                },
            ],
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
            PathBuf::from("test_assets/test_script.script")
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
            PathBuf::from("test_assets/test_script.script")
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

        let path = Path::new("test.lua");
        assert_eq!(settings.select_script_language(path), Language::Lua);
        assert_eq!(
            settings.select_script_language(Path::new("test.rhai")),
            Language::Rhai
        );
        assert_eq!(
            settings.select_script_language(Path::new("test.blob")),
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

        fn build_runtime() -> Self::R {
            todo!()
        }
    }

    #[test]
    fn test_asset_metadata_insert_remove_systems() {
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

    #[test]
    fn test_syncing_assets() {
        todo!()
    }
}
