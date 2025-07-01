//! Systems and resources for handling script assets and events

use crate::{
    StaticScripts,
    ScriptComponent,
    commands::{CreateOrUpdateScript, DeleteScript},
    error::ScriptError,
    script::{DisplayProxy, ScriptId, Domain},
    IntoScriptPluginParams, ScriptingSystemSet,
};
use bevy::{
    app::{App, PreUpdate},
    asset::{Asset, AssetEvent, AssetId, AssetLoader, AssetPath, Assets, LoadState},
    ecs::system::Resource,
    log::{debug, info, trace, warn},
    prelude::{
        Commands, Event, EventReader, EventWriter, IntoSystemConfigs, IntoSystemSetConfigs, Res,
        ResMut, Added, Query, Local, Handle, AssetServer, Entity,
    },
    reflect::TypePath,
    utils::hashbrown::HashMap,
};
use std::{borrow::Cow, collections::VecDeque};
use serde::{Deserialize, Serialize};

/// Represents a scripting language. Languages which compile into another language should use the target language as their language.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Default, Serialize, Deserialize)]
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
    pub content: Box<[u8]>, // Any chance a Cow<'static, ?> could work here?
    /// The language of the script
    pub language: Language,
    /// The domain if any the script will run in
    pub domain: Option<Domain>,
}

impl From<String> for ScriptAsset {
    fn from(s: String) -> ScriptAsset {
        ScriptAsset {
            content: s.into_bytes().into_boxed_slice(),
            language: Language::default(),
            domain: None,
        }
    }
}

/// Script settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScriptSettings {
    /// Define the language for a script or use the extension if None.
    pub language: Option<Language>,
    /// Specify the domain for the script.
    pub domain: Option<Domain>,
}

#[derive(Default)]
/// A loader for script assets
pub struct ScriptAssetLoader {
    /// The file extensions this loader should handle
    pub extensions: &'static [&'static str],
    /// preprocessor to run on the script before saving the content to an asset
    pub preprocessor: Option<Box<dyn Fn(&mut [u8]) -> Result<(), ScriptError> + Send + Sync>>,
}

#[profiling::all_functions]
impl AssetLoader for ScriptAssetLoader {
    type Asset = ScriptAsset;

    type Settings = ScriptSettings;

    type Error = ScriptError;

    async fn load(
        &self,
        reader: &mut dyn bevy::asset::io::Reader,
        settings: &Self::Settings,
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
        let language = settings
            .language
            .clone()
            .unwrap_or_else(||
                match load_context.path().extension().and_then(|e| e.to_str()).unwrap_or_default() {
                    "lua" => Language::Lua,
                    "rhai" => Language::Rhai,
                    "rn" => Language::Rune,
                    x => {
                        warn!("Unknown language for {:?}", load_context.path().display());
                        Language::Unknown
                    }
                });
        let asset = ScriptAsset {
            content: content.into_boxed_slice(),
            language,
            domain: settings.domain.clone(),
        };
        Ok(asset)
    }

    fn extensions(&self) -> &[&str] {
        self.extensions
    }
}

/// Listens to [`AssetEvent`] events and dispatches [`CreateOrUpdateScript`] and [`DeleteScript`] commands accordingly.
///
/// Allows for hot-reloading of scripts.
#[profiling::function]
pub(crate) fn sync_script_data<P: IntoScriptPluginParams>(
    mut events: EventReader<AssetEvent<ScriptAsset>>,
    script_assets: Res<Assets<ScriptAsset>>,
    mut static_scripts: ResMut<StaticScripts>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
) {
    for event in events.read() {

        trace!("{}: Received script asset event: {:?}", P::LANGUAGE, event);
        match event {
            // emitted when a new script asset is loaded for the first time
            AssetEvent::LoadedWithDependencies { id } | AssetEvent::Added { id } | AssetEvent::Modified{ id } => {
                if let Some(asset) = script_assets.get(*id) {
                    if asset.language != P::LANGUAGE {
                        match asset_server.get_path(*id) {
                            Some(path) => {
                                trace!(
                                    "{}: Script path {} is for a different language than this sync system. Skipping.",
                                    P::LANGUAGE,
                                    path);
                            }
                            None => {
                                trace!(
                                    "{}: Script id {} is for a different language than this sync system. Skipping.",
                                    P::LANGUAGE,
                                    id);
                            }
                        }
                        continue;
                    }

                    if static_scripts.iter().any(|handle| handle.id() == *id) {
                        info!("{}: Loading static script: {:?}", P::LANGUAGE, id);
                        commands.queue(CreateOrUpdateScript::<P>::new(
                            Handle::Weak(*id),
                            asset.content.clone(), // Cloning seems bad!
                            asset.domain.clone(),
                        ));
                    }
                }
            }
            AssetEvent::Removed{ id } => {
                if static_scripts.remove(id) {
                    info!("{}: Removing static script {:?}", P::LANGUAGE, id);
                }
            }
            AssetEvent::Unused { id } => {
            }
        };
    }
}

pub(crate) fn eval_script<P: IntoScriptPluginParams>(
    script_comps: Query<(Entity, &ScriptComponent), Added<ScriptComponent>>,
    mut script_queue: Local<VecDeque<(Entity, Handle<ScriptAsset>)>>,
    script_assets: Res<Assets<ScriptAsset>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    ) {
    for (id, script_comp) in &script_comps {
        for handle in &script_comp.0 {
            script_queue.push_back((id, handle.clone_weak()));
        }
    }
    while ! script_queue.is_empty() {
        let script_ready = script_queue.front().map(|(_, script_id)| match asset_server.load_state(&*script_id) {
            LoadState::Failed(e) => {
                warn!("Failed to load script {}", script_id.display());
                true
            }
            LoadState::Loaded => true,
            _ => false
        }).unwrap_or(false);
        if ! script_ready {
            break;
        }
        // NOTE: Maybe once pop_front_if is stabalized.
        // if let Some(script_id) = script_queue.pop_front_if(|script_id| match asset_server.load_state(script_id) {
        //     LoadState::Failed(e) => {
        //         warn!("Failed to load script {}", &script_id);
        //         true
        //     }
        //     LoadState::Loaded => true,
        //     _ => false
        // }) {
        if let Some((id, script_id)) = script_queue.pop_front() {
            if let Some(asset) = script_assets.get(&script_id) {
                if asset.language == P::LANGUAGE {
                    commands.entity(id).queue(CreateOrUpdateScript::<P>::new(
                        script_id,
                        asset.content.clone(),
                        asset.domain.clone(),
                    ));
                }
            } else {
                // This is probably a load failure. What to do? We've already
                // provided a warning on failure. Doing nothing is fine then we
                // process the next one.
            }
        } else {
            break;
        }
    }
}

/// Setup all the asset systems for the scripting plugin and the dependencies
#[profiling::function]
pub(crate) fn configure_asset_systems(app: &mut App) -> &mut App {
    // these should be in the same set as bevy's asset systems
    // currently this is in the PreUpdate set
    app
        // .add_systems(
        //     PreUpdate,
        //     (
        //         dispatch_script_asset_events.in_set(ScriptingSystemSet::ScriptAssetDispatch),
        //         remove_script_metadata.in_set(ScriptingSystemSet::ScriptMetadataRemoval),
        //     ),
        // )
    .configure_sets(
        PreUpdate,
        (
            ScriptingSystemSet::ScriptAssetDispatch.after(bevy::asset::TrackAssets),
            ScriptingSystemSet::ScriptCommandDispatch
                .after(ScriptingSystemSet::ScriptAssetDispatch)
                .before(ScriptingSystemSet::ScriptMetadataRemoval),
        ),
    );

    app
}

/// Setup all the asset systems for the scripting plugin and the dependencies
#[profiling::function]
pub(crate) fn configure_asset_systems_for_plugin<P: IntoScriptPluginParams>(
    app: &mut App,
) -> &mut App {
    app.add_systems(
        PreUpdate,
        (eval_script::<P>, sync_script_data::<P>).in_set(ScriptingSystemSet::ScriptCommandDispatch),
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
                panic!("Asset load failed: {asset_load_error:?}")
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
                println!("{event:?}");
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
