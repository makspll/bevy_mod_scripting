//! Systems and resources for handling script assets and events

use std::{borrow::Cow, collections::VecDeque};

use ::{
    bevy_app::{App, Last},
    bevy_asset::{Asset, AssetEvent, AssetLoader, Assets, LoadState},
    bevy_log::{error, trace, warn, warn_once},
    // prelude::{
    //     AssetServer, Commands, Entity, EventReader, EventWriter, IntoScheduleConfigs, Local, Query,
    //     Res,
    // },
    bevy_reflect::Reflect,
};
use bevy_asset::AssetServer;
use bevy_ecs::{
    entity::Entity,
    event::{EventReader, EventWriter},
    schedule::IntoScheduleConfigs,
    system::{Commands, Local, Query, Res},
};
use serde::{Deserialize, Serialize};

use crate::{
    IntoScriptPluginParams, LanguageExtensions, ScriptComponent, ScriptingSystemSet, StaticScripts,
    commands::{CreateOrUpdateScript, DeleteScript},
    context::ContextLoadingSettings,
    error::ScriptError,
    event::ScriptEvent,
    script::{ContextKey, DisplayProxy, ScriptAttachment},
};

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
#[derive(Asset, Clone, Reflect)]
#[reflect(opaque)]
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
type ScriptQueue = VecDeque<ScriptAttachment>;
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
        let extensions: Vec<&'static str> = language_extensions.keys().copied().collect();
        let new_arr_static = Vec::leak(extensions);
        Self {
            language_extensions,
            extensions: new_arr_static,
            preprocessor: None,
        }
    }

    /// Add a preprocessor
    pub fn with_preprocessor(
        mut self,
        preprocessor: Box<dyn Fn(&mut [u8]) -> Result<(), ScriptError> + Send + Sync>,
    ) -> Self {
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
        reader: &mut dyn bevy_asset::io::Reader,
        settings: &Self::Settings,
        load_context: &mut bevy_asset::LoadContext<'_>,
    ) -> Result<Self::Asset, Self::Error> {
        let mut content = Vec::new();
        reader
            .read_to_end(&mut content)
            .await
            .map_err(|e| ScriptError::new_external(e).with_context(load_context.asset_path()))?;
        if let Some(processor) = &self.preprocessor {
            processor(&mut content)?;
        }
        let language = settings.language.clone().unwrap_or_else(|| {
            let ext = load_context
                .path()
                .extension()
                .and_then(|e| e.to_str())
                .unwrap_or_default();
            self.language_extensions
                .0
                .get(ext)
                .cloned()
                .unwrap_or_else(|| {
                    warn!("Unknown language for {:?}", load_context.path().display());
                    Language::Unknown
                })
        });
        if language == Language::Lua && cfg!(not(feature = "mlua")) {
            warn_once!(
                "Script {:?} is a Lua script but the {:?} feature is not enabled; the script will not be evaluated.",
                load_context.path().display(),
                "mlua"
            );
        }
        if language == Language::Rhai && cfg!(not(feature = "rhai")) {
            warn_once!(
                "Script {:?} is a Rhai script but the {:?} feature is not enabled; the script will not be evaluated.",
                load_context.path().display(),
                "rhai"
            );
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

fn sync_assets(
    mut events: EventReader<AssetEvent<ScriptAsset>>,
    mut script_events: EventWriter<ScriptEvent>,
) {
    for event in events.read() {
        match event {
            AssetEvent::Modified { id } => {
                script_events.write(ScriptEvent::Modified { script: *id });
            }
            AssetEvent::Added { id } => {
                script_events.write(ScriptEvent::Added { script: *id });
            }
            AssetEvent::Removed { id } => {
                script_events.write(ScriptEvent::Removed { script: *id });
            }
            _ => (),
        }
    }
}

/// Listens to [`ScriptEvent`] events and dispatches [`CreateOrUpdateScript`] and [`DeleteScript`] commands accordingly.
///
/// Allows for hot-reloading of scripts.
#[profiling::function]
fn handle_script_events<P: IntoScriptPluginParams>(
    mut events: EventReader<ScriptEvent>,
    script_assets: Res<Assets<ScriptAsset>>,
    static_scripts: Res<StaticScripts>,
    scripts: Query<(Entity, &ScriptComponent)>,
    asset_server: Res<AssetServer>,
    mut script_queue: Local<ScriptQueue>,
    mut commands: Commands,
    context_loading_settings: Res<ContextLoadingSettings<P>>,
) {
    for event in events.read() {
        trace!("{}: Received script event: {:?}", P::LANGUAGE, event);
        match event {
            ScriptEvent::Modified { script: id } => {
                if let Some(asset) = script_assets.get(*id) {
                    if asset.language != P::LANGUAGE {
                        continue;
                    }
                    // We need to reload the script for any context it's
                    // associated with. That could be static scripts, script
                    // components.
                    for (entity, script_component) in &scripts {
                        if let Some(handle) =
                            script_component.0.iter().find(|handle| handle.id() == *id)
                        {
                            commands.queue(
                                CreateOrUpdateScript::<P>::new(ScriptAttachment::EntityScript(
                                    entity,
                                    handle.clone(),
                                ))
                                .with_responses(context_loading_settings.emit_responses),
                            );
                        }
                    }

                    if let Some(handle) = static_scripts.scripts.iter().find(|s| s.id() == *id) {
                        commands.queue(
                            CreateOrUpdateScript::<P>::new(ScriptAttachment::StaticScript(
                                handle.clone(),
                            ))
                            .with_responses(context_loading_settings.emit_responses),
                        );
                    }
                }
            }
            ScriptEvent::Detached { key } => {
                commands.queue(
                    DeleteScript::<P>::new(key.clone())
                        .with_responses(context_loading_settings.emit_responses),
                );
            }
            ScriptEvent::Attached { key } => {
                script_queue.push_back(key.clone());
            }
            _ => (),
        }
    }

    // Evalute the scripts in the order they were attached.
    //
    // If a script is not loaded yet, we stop evaluation and try again on the
    // next call.
    while !script_queue.is_empty() {
        let mut script_failed = false;
        // NOTE: Maybe using pop_front_if once stabalized.
        let script_ready = script_queue
            .front()
            .map(|context_key| context_key.clone().into())
            .map(|context_key: ContextKey| {
                // If there is a script, wait for the script to load.
                context_key
                    .script
                    .as_ref()
                    .map(|script| {
                        script_assets.contains(script.id())
                            || match asset_server.load_state(script) {
                                LoadState::NotLoaded => false,
                                LoadState::Loading => false,
                                LoadState::Loaded => true,
                                LoadState::Failed(e) => {
                                    script_failed = true;
                                    error!(
                                        "Failed to load script {} for eval: {e}.",
                                        script.display()
                                    );
                                    true
                                }
                            }
                    })
                    .unwrap_or(true)
            })
            .unwrap_or(false);
        if !script_ready {
            // We can't evaluate it yet. It's still loading.
            break;
        }

        if let Some(context_key) = script_queue.pop_front() {
            if script_failed {
                continue;
            }

            let language = script_assets
                .get(&context_key.script())
                .map(|asset| asset.language.clone())
                .unwrap_or_default();

            if language == P::LANGUAGE {
                commands.queue(
                    CreateOrUpdateScript::<P>::new(context_key)
                        .with_responses(context_loading_settings.emit_responses),
                );
            }
        }
    }
}

/// Setup all the asset systems for the scripting plugin and the dependencies
#[profiling::function]
pub(crate) fn configure_asset_systems(app: &mut App) {
    // these should be in the same set as bevy's asset systems
    // currently this is in the PreUpdate set
    app.add_systems(
        Last,
        (sync_assets).in_set(ScriptingSystemSet::ScriptAssetDispatch),
    )
    .configure_sets(
        Last,
        (
            ScriptingSystemSet::ScriptAssetDispatch.after(bevy_asset::AssetEvents),
            ScriptingSystemSet::ScriptCommandDispatch
                .after(ScriptingSystemSet::ScriptAssetDispatch),
        ),
    );
}

/// Setup all the asset systems for the scripting plugin and the dependencies
#[profiling::function]
pub(crate) fn configure_asset_systems_for_plugin<P: IntoScriptPluginParams>(app: &mut App) {
    app.add_systems(
        Last,
        handle_script_events::<P>.in_set(ScriptingSystemSet::ScriptCommandDispatch),
    );
}

// #[cfg(test)]
// mod tests {

//     use ::{
//         bevy_app::App,
//         bevy_asset::{AssetServer, Handle, LoadState},
//     };

//     use super::*;

//     // fn init_loader_test(loader: ScriptAssetLoader) -> App {
//     //     let mut app = App::new();
//     //     app.add_plugins((MinimalPlugins, AssetPlugin::default()));
//     //     app.init_asset::<ScriptAsset>();
//     //     app.register_asset_loader(loader);
//     //     app
//     // }

//     // fn for_extension(extension: &'static str) -> ScriptAssetLoader {
//     //     let mut language_extensions = LanguageExtensions::default();
//     //     language_extensions.insert(extension, Language::Unknown);
//     //     ScriptAssetLoader::new(language_extensions)
//     // }

//     // fn load_asset(app: &mut App, path: &str) -> Handle<ScriptAsset> {
//     //     let handle = app.world_mut().resource::<AssetServer>().load(path);

//     //     loop {
//     //         let state = app
//     //             .world()
//     //             .resource::<AssetServer>()
//     //             .get_load_state(&handle)
//     //             .unwrap();
//     //         if !matches!(state, LoadState::Loading) {
//     //             break;
//     //         }
//     //         app.update();
//     //     }

//     //     match app
//     //         .world()
//     //         .resource::<AssetServer>()
//     //         .get_load_state(&handle)
//     //         .unwrap()
//     //     {
//     //         LoadState::NotLoaded => panic!("Asset not loaded"),
//     //         LoadState::Loaded => {}
//     //         LoadState::Failed(asset_load_error) => {
//     //             panic!("Asset load failed: {asset_load_error:?}")
//     //         }
//     //         _ => panic!("Unexpected load state"),
//     //     }

//     //     handle
//     // }

//     // #[test]
//     // fn test_asset_loader_loads() {
//     //     let loader = for_extension("script");
//     //     let mut app = init_loader_test(loader);

//     //     let handle = load_asset(&mut app, "test_assets/test_script.script");
//     //     let asset = app
//     //         .world()
//     //         .get_resource::<Assets<ScriptAsset>>()
//     //         .unwrap()
//     //         .get(&handle)
//     //         .unwrap();

//     //     assert_eq!(
//     //         String::from_utf8(asset.content.clone().to_vec()).unwrap(),
//     //         "test script".to_string()
//     //     );
//     // }

//     // #[test]
//     // fn test_asset_loader_applies_preprocessor() {
//     //     let loader = for_extension("script").with_preprocessor(Box::new(|content| {
//     //         content[0] = b'p';
//     //         Ok(())
//     //     }));
//     //     let mut app = init_loader_test(loader);

//     //     let handle = load_asset(&mut app, "test_assets/test_script.script");
//     //     let asset = app
//     //         .world()
//     //         .get_resource::<Assets<ScriptAsset>>()
//     //         .unwrap()
//     //         .get(&handle)
//     //         .unwrap();

//     //     assert_eq!(
//     //         handle.path().unwrap(),
//     //         &AssetPath::from(PathBuf::from("test_assets/test_script.script"))
//     //     );
//     //     assert_eq!(
//     //         String::from_utf8(asset.content.clone().to_vec()).unwrap(),
//     //         "pest script".to_string()
//     //     );
//     // }
// }
