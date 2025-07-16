//! Systems and resources for handling script assets and events

use crate::{
    StaticScripts,
    ScriptComponent,
    LanguageExtensions,
    commands::CreateOrUpdateScript,
    error::ScriptError,
    script::{DisplayProxy, ScriptDomain, ContextKey},
    IntoScriptPluginParams, ScriptingSystemSet,
};
use bevy::{
    app::{App, PreUpdate},
    asset::{Asset, AssetEvent, AssetLoader, Assets, LoadState},
    log::{info, trace, warn, error, warn_once},
    prelude::{
        Commands, EventReader, IntoSystemConfigs, IntoSystemSetConfigs, Res,
        ResMut, Added, Query, AssetServer, Entity, Resource, Deref, DerefMut,
    },
    reflect::TypePath,
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
}

impl From<String> for ScriptAsset {
    fn from(s: String) -> ScriptAsset {
        ScriptAsset {
            content: s.into_bytes().into_boxed_slice(),
            language: Language::default(),
        }
    }
}

impl ScriptAsset {
    /// Create a new script asset with an unknown language.
    pub fn new(s: impl Into<String>) -> Self {
        s.into().into()
    }
}

/// The queue that evaluates scripts.
#[derive(Resource, Deref, DerefMut, Default)]
pub(crate) struct ScriptQueue(VecDeque<ContextKey>);

/// Script settings
#[derive(Debug, Clone, Serialize, Deserialize, Default)]
pub struct ScriptSettings {
    /// Define the language for a script or use the extension if None.
    pub language: Option<Language>,
}

#[derive(Default)]
/// A loader for script assets
pub struct ScriptAssetLoader {
    /// The file extensions this loader should handle
    language_extensions: LanguageExtensions,
    extensions: &'static [&'static str],
    /// preprocessor to run on the script before saving the content to an asset
    pub preprocessor: Option<Box<dyn Fn(&mut [u8]) -> Result<(), ScriptError> + Send + Sync>>,
}

impl ScriptAssetLoader {
    /// Create a new script asset loader for the given extensions.
    pub fn new(language_extensions: LanguageExtensions) -> Self {
        let extensions: Vec<&'static str> = language_extensions.keys().copied()
                                                               .collect();
        let new_arr_static = Vec::leak(extensions);
        Self {
            language_extensions,
            extensions: new_arr_static,
            preprocessor: None,
        }
    }

    /// For testing purposes.
    #[allow(dead_code)]
    pub(crate) fn for_extension(extension: &'static str) -> Self {
        let mut language_extensions = LanguageExtensions::default();
        language_extensions.insert(extension, Language::Unknown);
        Self::new(language_extensions)
    }

    /// Add a preprocessor
    pub fn with_preprocessor(mut self, preprocessor: Box<dyn Fn(&mut [u8]) -> Result<(), ScriptError> + Send + Sync>) -> Self {
        self.preprocessor = Some(preprocessor);
        self
    }
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
            .unwrap_or_else(|| {
                let ext = load_context.path().extension().and_then(|e| e.to_str()).unwrap_or_default();
                self.language_extensions.get(ext).cloned().unwrap_or_else(|| {
                    warn!("Unknown language for {:?}", load_context.path().display());
                    Language::Unknown
                })
            });
        if language == Language::Lua && cfg!(not(feature = "mlua")) {
            warn_once!("Script {:?} is a Lua script but the {:?} feature is not enabled; the script will not be evaluated.",
                       load_context.path().display(), "mlua");
        }
        if language == Language::Rhai && cfg!(not(feature = "rhai")) {
            warn_once!("Script {:?} is a Rhai script but the {:?} feature is not enabled; the script will not be evaluated.",
                       load_context.path().display(), "rhai");
        }
        let asset = ScriptAsset {
            content: content.into_boxed_slice(),
            language,
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
    scripts: Query<(Entity, &ScriptComponent, Option<&ScriptDomain>)>,
    mut commands: Commands,
) {
    for event in events.read() {

        trace!("{}: Received script asset event: {:?}", P::LANGUAGE, event);
        match event {
            AssetEvent::Modified{ id } => {
                if let Some(asset) = script_assets.get(*id) {
                    if asset.language != P::LANGUAGE {
                        continue;
                    }
                    // We need to reload the script for any context its
                    // associated with. That could be static scripts, script
                    // components.
                    for (entity, script_component, script_domain_maybe) in &scripts
                    {
                        if let Some(handle) = script_component.0.iter().find(|handle| handle.id() == *id) {
                            commands.queue(CreateOrUpdateScript::<P>::new(ContextKey {
                                entity: Some(entity),
                                script: Some(handle.clone()),
                                domain: script_domain_maybe.map(|x| x.0),
                            }));
                        }
                    }

                    if let Some(handle) = static_scripts.scripts.iter().find(|s| s.id() == *id) {
                        commands.queue(CreateOrUpdateScript::<P>::new(handle.clone()));
                    }
                }
            }
            AssetEvent::Removed{ id } => {
                info!("{}: Asset removed {:?}", P::LANGUAGE, id);
                if static_scripts.remove(*id) {
                    info!("{}: Removing static script {:?}", P::LANGUAGE, id);
                }
                // We're removing a context because its handle was removed. This
                // makes sense specifically for ScriptIdContext. However, it
                // doesn't quite work for the other context providers, and it
                // requires we keep a script loaded in memory when technically
                // it needn't be.
                //
                // If we want this kind of behavior, it seems like we'd want to
                // have handles to contexts.
                //
                // if script_contexts.remove(None, id, &None) {
                //     info!("{}: Removed script context {:?}", P::LANGUAGE, id);
                // }
            }
            _ => ()
        };
    }
}

pub(crate) fn eval_script<P: IntoScriptPluginParams>(
    script_comps: Query<(Entity, &ScriptComponent, Option<&ScriptDomain>), Added<ScriptComponent>>,
    mut script_queue: ResMut<ScriptQueue>,
    script_assets: Res<Assets<ScriptAsset>>,
    asset_server: Res<AssetServer>,
    mut commands: Commands,
    ) {
    for (id, script_comp, domain_maybe) in &script_comps {
        for handle in &script_comp.0 {
            script_queue.push_back(ContextKey {
                entity: Some(id),
                script: Some(handle.clone_weak()),
                domain: domain_maybe.map(|x| x.0)
            });
        }
    }
    while ! script_queue.is_empty() {
        let mut script_failed = false;
        // NOTE: Maybe using pop_front_if once stabalized.
        let script_ready = script_queue
            .front()
            .map(|context_key| {

                // If there is a script, wait for the script to load.
                context_key.script.as_ref()
                                     .map(|script|
                script_assets.contains(script.id()) || match asset_server.load_state(script) {
                    LoadState::NotLoaded => false,
                    LoadState::Loading => false,
                    LoadState::Loaded => true,
                    LoadState::Failed(e) => {
                        script_failed = true;
                        error!("Failed to load script {} for eval: {e}.", script.display());
                        true
                    }
                }).unwrap_or(true)
            })
            .unwrap_or(false);
        if ! script_ready {
            break;
        }
        if let Some(context_key) = script_queue.pop_front() {
            if script_failed {
                continue;
            }
            let language = context_key.script.as_ref()
                                      .and_then(|script_id| script_assets.get(script_id))
                                      .map(|asset| asset.language.clone())
                                      .unwrap_or_default();
            if language == P::LANGUAGE {
                match context_key.entity {
                    Some(id) => {
                        commands.entity(id).queue(CreateOrUpdateScript::<P>::new(context_key));
                    },
                    None => {
                        commands.queue(CreateOrUpdateScript::<P>::new(context_key));
                    },
                }
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
) {
    app.add_systems(
        PreUpdate,
        (eval_script::<P>, sync_script_data::<P>).in_set(ScriptingSystemSet::ScriptCommandDispatch),
    );
}

#[cfg(test)]
mod tests {
    use std::path::{PathBuf};

    use bevy::{
        app::{App, Update},
        asset::{AssetApp, AssetPlugin, AssetServer, Assets, Handle, LoadState, AssetPath},
        prelude::Resource,
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
        let loader = ScriptAssetLoader::for_extension("script");
        let mut app = init_loader_test(loader);

        let handle = load_asset(&mut app, "test_assets/test_script.script");
        let asset = app
            .world()
            .get_resource::<Assets<ScriptAsset>>()
            .unwrap()
            .get(&handle)
            .unwrap();

        assert_eq!(
            String::from_utf8(asset.content.clone().to_vec()).unwrap(),
            "test script".to_string()
        );
    }

    #[test]
    fn test_asset_loader_applies_preprocessor() {
        let loader = ScriptAssetLoader::for_extension("script")
            .with_preprocessor(Box::new(|content| {
                content[0] = b'p';
                Ok(())
            }));
        let mut app = init_loader_test(loader);

        let handle = load_asset(&mut app, "test_assets/test_script.script");
        let asset = app
            .world()
            .get_resource::<Assets<ScriptAsset>>()
            .unwrap()
            .get(&handle)
            .unwrap();

        assert_eq!(
            handle.path().unwrap(),
            &AssetPath::from(PathBuf::from("test_assets/test_script.script"))
        );
        assert_eq!(
            String::from_utf8(asset.content.clone().to_vec()).unwrap(),
            "pest script".to_string()
        );
    }

    #[allow(dead_code)]
    fn run_app_until_asset_event(app: &mut App, event_kind: AssetEvent<ScriptAsset>) {
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


    // #[test]
    // fn test_syncing_assets() {
    //     todo!()
    // }
}
