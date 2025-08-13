use crate::{install_test_plugin, parse::*};
use anyhow::{anyhow, Context, Error};
use bevy::ecs::entity::Entity;
use bevy::ecs::system::Command;
use bevy::prelude::IntoSystem;
use bevy::{
    app::App,
    asset::{AssetEvent, Handle, LoadState},
    ecs::{
        event::{Event, EventCursor, Events},
        schedule::ScheduleLabel,
        world::World,
    },
};
use bevy_mod_scripting_core::asset::Language;
use bevy_mod_scripting_core::bindings::{DisplayWithWorld, ScriptValue, WorldGuard};
use bevy_mod_scripting_core::commands::{AddStaticScript, RemoveStaticScript};
use bevy_mod_scripting_core::event::ScriptEvent;
use bevy_mod_scripting_core::script::ContextPolicy;
use bevy_mod_scripting_core::script::ScriptContext;
use bevy_mod_scripting_core::{
    asset::ScriptAsset,
    event::{CallbackLabel, IntoCallbackLabel, ScriptCallbackEvent, ScriptCallbackResponseEvent},
    handler::event_handler,
    script::{ScriptAttachment, ScriptComponent},
};
use bevy_mod_scripting_core::{ConfigureScriptPlugin, LanguageExtensions};
use std::borrow::Cow;
use std::collections::VecDeque;
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::Instant,
};
use test_utils::test_data::setup_integration_test;

const TIMEOUT_SECONDS: u64 = 10;
pub const SCENARIO_SELF_SCRIPT_NAME: &str = "@this_script";
pub const SCENARIO_SELF_LANGUAGE_NAME: &str = "@this_language";

pub struct Scenario {
    pub steps: Vec<ScenarioStepSerialized>,
    pub context: ScenarioContext,
}

impl Scenario {
    /// Parses a scenario from a file.
    pub fn from_scenario_file(
        this_script_path: &Path,
        scenario_path: &Path,
    ) -> Result<Self, Error> {
        let content = std::fs::read_to_string(scenario_path).with_context(|| {
            format!("Failed to read scenario file: {}", scenario_path.display())
        })?;
        let lines = content
            .lines()
            .map(|line| line.trim())
            .filter(|line| !line.is_empty())
            .collect::<Vec<_>>();

        let steps = lines
            .into_iter()
            .map(|line| {
                ScenarioStepSerialized::from_flat_string(line)
                    .with_context(|| format!("Failed to parse scenario step: {line}"))
            })
            .collect::<Result<Vec<_>, Error>>()?;
        Ok(Self {
            steps,
            context: ScenarioContext::new(this_script_path.to_path_buf()),
        })
    }

    pub fn scenario_error(
        watcher: &InterestingEventWatcher,
        steps: &[ScenarioStepSerialized],
        error_step: (usize, Error),
    ) -> Error {
        let msg = steps
            .iter()
            .enumerate()
            .map(|(i, step)| {
                let step = format!("#{i}: {}", step.to_flat_string().unwrap_or_default());
                // print events, in blue
                let event_colour_start = "\x1b[34m";
                let event_colour_end = "\x1b[0m";
                let step_events = watcher
                    .events
                    .iter()
                    .filter(|(_, step_no)| *step_no == i)
                    .flat_map(|(event, _)| event.lines())
                    .map(|line| format!("\n\t{event_colour_start}{line}{event_colour_end}"))
                    .collect::<Vec<_>>()
                    .join("");

                if i == error_step.0 {
                    let anyhow_error_pretty = format!("{:?}", error_step.1);
                    let tabulated = anyhow_error_pretty
                        .lines()
                        .map(|line| format!("\t{line}"))
                        .collect::<Vec<_>>()
                        .join("\n");
                    let colour_start = "\x1b[31m";
                    let colour_end = "\x1b[0m";

                    format!("{colour_start}{step}\n{tabulated}{colour_end}{step_events}")
                } else {
                    format!("{step}{step_events}")
                }
            })
            .collect::<Vec<_>>()
            .join("\n");

        anyhow::anyhow!(
            "Error in scenario:\n{}\n\nWith steps:\n{}",
            error_step.1,
            msg
        )
    }

    pub fn execute(mut self, mut app: App) -> Result<(), Error> {
        let original_steps = self.steps.clone();
        for (i, step) in self.steps.into_iter().enumerate() {
            bevy::log::info!(
                "Executing step #{i}: {}",
                step.to_flat_string().unwrap_or_default()
            );
            self.context.current_step_no = i;
            let parsed_step = step.parse_and_resolve(&self.context)?;
            if let Err(err) = parsed_step.execute(&mut self.context, &mut app) {
                let error =
                    Scenario::scenario_error(&self.context.event_log, &original_steps, (i, err));
                return Err(error);
            }
        }
        Ok(())
    }
}

/// Serves as a chalkboard for the test scenario to write to and read from.
#[derive(Debug, Clone)]
pub struct ScenarioContext {
    pub script_handles: HashMap<String, Handle<ScriptAsset>>,
    pub entities: HashMap<String, bevy::ecs::entity::Entity>,
    pub scenario_time_started: Instant,
    pub this_script_asset_relative_path: PathBuf,
    pub event_log: InterestingEventWatcher,
    pub current_step_no: usize,
    pub current_script_language: Option<Language>,
    pub initialized_app: bool,
}

#[derive(Debug, Clone, Default)]
pub struct InterestingEventWatcher {
    pub events: Vec<(String, usize)>,
    pub asset_event_cursor: EventCursor<AssetEvent<ScriptAsset>>,
    pub script_events_cursor: EventCursor<ScriptEvent>,
    pub script_response_cursor: EventCursor<ScriptCallbackResponseEvent>,
    pub script_responses_queue: VecDeque<ScriptCallbackResponseEvent>,
}

impl InterestingEventWatcher {
    pub fn log_events(&mut self, step_no: usize, world: &bevy::ecs::world::World) {
        let asset_events = world.resource::<Events<AssetEvent<ScriptAsset>>>();
        let script_events = world.resource::<Events<ScriptEvent>>();
        let script_responses = world.resource::<Events<ScriptCallbackResponseEvent>>();
        let mut tracked_with_id = Vec::default();
        for (event, id) in self.asset_event_cursor.read_with_id(asset_events) {
            tracked_with_id.push((id.id, format!("AssetEvent : {event:?}")));
        }
        for (event, id) in self.script_events_cursor.read_with_id(script_events) {
            tracked_with_id.push((id.id, format!("ScriptEvent: {event:?}")));
        }
        let mut script_responses_by_id = Vec::default();
        for (event, id) in self.script_response_cursor.read_with_id(script_responses) {
            script_responses_by_id.push((id.id, event.clone()));
            tracked_with_id.push((id.id, format!("ScriptResponse: {event:?}")));
        }

        script_responses_by_id.sort_by_key(|(id, _)| *id);
        tracked_with_id.sort_by_key(|(id, _)| *id);
        for (_, event) in tracked_with_id {
            self.events.push((event, step_no));
        }
        for event in script_responses_by_id {
            self.script_responses_queue.push_back(event.1);
        }
    }
}

impl ScenarioContext {
    pub fn new(script_asset_path: PathBuf) -> Self {
        Self {
            scenario_time_started: Instant::now(),
            script_handles: HashMap::new(),
            this_script_asset_relative_path: script_asset_path,
            entities: HashMap::new(),
            event_log: InterestingEventWatcher::default(),
            current_step_no: 0,
            current_script_language: None,
            initialized_app: false,
        }
    }

    /// Returns a path relative to the parent of the current script in the "assets" frame of reference.
    pub fn scenario_path(&self, path: &PathBuf) -> PathBuf {
        self.this_script_asset_relative_path
            .parent()
            .unwrap_or(&PathBuf::new())
            .join(path)
    }

    /// Returns the absolute path to the assets directory
    pub fn assets_path(&self) -> PathBuf {
        PathBuf::from(std::env::var("BEVY_ASSET_ROOT").ok().unwrap()).join("assets")
    }

    /// Resolves an assset relative scenario path to an absolute path using the assets manifest path.
    pub fn absolute_scenario_path(&self, path: &PathBuf) -> PathBuf {
        self.assets_path().join(self.scenario_path(path))
    }

    pub fn get_script_handle(&self, name: &str) -> Result<Handle<ScriptAsset>, Error> {
        self
            .script_handles
            .get(name)
            .cloned()
            .ok_or_else(|| anyhow!("Script with name '{name}' not found in context. Did you miss a `LoadScriptAs` step?"))
    }

    pub fn get_entity(&self, name: &str) -> Result<bevy::ecs::entity::Entity, Error> {
        self
            .entities
            .get(name)
            .cloned()
            .ok_or_else(|| anyhow!("Entity with name '{name}' not found in context. Did you miss a `SpawnEntityWithScript` step?"))
    }
}

impl ScenarioSchedule {
    pub fn add_handler<T: IntoCallbackLabel + 'static>(
        &self,
        language: Option<Language>,
        app: &mut App,
    ) {
        let language = language.unwrap_or(Language::External("Unset language".into()));
        match language {
            #[cfg(feature = "lua")]
            Language::Lua => {
                let system = IntoSystem::into_system(
                    event_handler::<T, bevy_mod_scripting_lua::LuaScriptingPlugin>,
                )
                .with_name(T::into_callback_label().to_string());
                app.add_systems(self.clone(), system);
            }
            #[cfg(feature = "rhai")]
            Language::Rhai => {
                let system = IntoSystem::into_system(
                    event_handler::<T, bevy_mod_scripting_rhai::RhaiScriptingPlugin>,
                )
                .with_name(T::into_callback_label().to_string());
                app.add_systems(self.clone(), system);
            }
            _ => {
                panic!("Unsupported language for scenario schedule: {language:?}");
            }
        }
    }
}
impl ScheduleLabel for ScenarioSchedule {
    fn dyn_clone(&self) -> Box<dyn ScheduleLabel> {
        match self {
            ScenarioSchedule::Startup => bevy::app::Startup.dyn_clone(),
            ScenarioSchedule::Update => bevy::app::Update.dyn_clone(),
            ScenarioSchedule::FixedUpdate => bevy::app::FixedUpdate.dyn_clone(),
            ScenarioSchedule::PostUpdate => bevy::app::PostUpdate.dyn_clone(),
            ScenarioSchedule::Last => bevy::app::Last.dyn_clone(),
        }
    }

    fn as_dyn_eq(&self) -> &dyn bevy::ecs::label::DynEq {
        match self {
            ScenarioSchedule::Startup => bevy::app::Startup.as_dyn_eq(),
            ScenarioSchedule::Update => bevy::app::Update.as_dyn_eq(),
            ScenarioSchedule::FixedUpdate => bevy::app::FixedUpdate.as_dyn_eq(),
            ScenarioSchedule::PostUpdate => bevy::app::PostUpdate.as_dyn_eq(),
            ScenarioSchedule::Last => bevy::app::Last.as_dyn_eq(),
        }
    }

    fn dyn_hash(&self, state: &mut dyn ::core::hash::Hasher) {
        match self {
            ScenarioSchedule::Startup => bevy::app::Startup.dyn_hash(state),
            ScenarioSchedule::Update => bevy::app::Update.dyn_hash(state),
            ScenarioSchedule::FixedUpdate => bevy::app::FixedUpdate.dyn_hash(state),
            ScenarioSchedule::PostUpdate => bevy::app::PostUpdate.dyn_hash(state),
            ScenarioSchedule::Last => bevy::app::Last.dyn_hash(state),
        }
    }
}

#[derive(Debug)]
pub enum ScenarioStep {
    /// A comment in the scenario, ignored during execution.
    Comment {
        comment: String,
    },
    /// Installs the scripting plugin with the given context policy and whether to emit responses.
    InstallPlugin {
        context_policy: ContextPolicy,
        emit_responses: bool,
    },
    /// Finalizes the app, cleaning up resources and preparing for the next steps.
    FinalizeApp,
    SetCurrentLanguage {
        language: Language,
    },

    /// Sets up a handler for the given schedule and label.
    /// You can onle use one of the following callbacks:
    /// - `on_test`
    /// - `on_test_post_update`
    /// - `on_test_last`
    /// - `callback_a`
    /// - `callback_b`
    /// - `callback_c`
    ///
    /// and main bevy schedule labels.
    SetupHandler {
        schedule: ScenarioSchedule,
        label: CallbackLabel,
    },
    /// Loads a script from the given path and assigns it a name,
    /// this handle can be used later when loaded.
    LoadScriptAs {
        path: PathBuf,
        as_name: String,
    },
    /// Waits until the script with the given name is loaded.
    WaitForScriptLoaded {
        script: Handle<ScriptAsset>,
    },
    /// Spawns an entity with the given name and attaches the given script to it.
    SpawnEntityWithScript {
        script: Handle<ScriptAsset>,
        entity: String,
    },
    AttachStaticScript {
        script: Handle<ScriptAsset>,
    },
    DetachStaticScript {
        script: Handle<ScriptAsset>,
    },
    /// Drops the named script asset from the scenario context.
    DropScriptAsset {
        script: Handle<ScriptAsset>,
    },

    /// Emits a ScriptCallbackEvent
    EmitScriptCallbackEvent {
        event: ScriptCallbackEvent,
    },

    /// Run the app update loop once
    RunUpdateOnce,

    /// Asserts that a callback response was triggered for the given label and from the given recipient
    AssertCallbackSuccess {
        label: CallbackLabel,
        script: ScriptAttachment,
        expect_string_value: Option<String>,
        language: Option<Language>,
    },

    /// Asserts that no more callback events are left to process.
    AssertNoCallbackResponsesEmitted,

    /// Reloads script with the given name from the specified path.
    ReloadScriptFrom {
        script: Handle<ScriptAsset>,
        path: PathBuf,
    },
    /// Asserts that a context for the given attachment does not exist
    AssertContextResidents {
        script: ScriptAttachment,
        residents_num: usize,
    },

    /// Despawns the entity with the given name.
    DespawnEntity {
        entity: Entity,
    },
}

/// Execution
impl ScenarioStep {
    pub fn run_update_catching_error_events(
        context: &mut ScenarioContext,
        app: &mut App,
    ) -> Result<(), Error> {
        app.update();

        // add watched events
        let world = app.world_mut();
        context.event_log.log_events(context.current_step_no, world);
        if context.scenario_time_started.elapsed().as_secs() > TIMEOUT_SECONDS {
            return Err(anyhow!(
                "Test scenario timed out after {} seconds",
                TIMEOUT_SECONDS
            ));
        }
        Ok(())
    }

    /// Will execute the app update loop until an event of type `T` is received or we timeout.
    pub fn execute_until_event<
        T: Event + Clone,
        E,
        F: Fn(&T) -> bool,
        G: Fn(&World) -> Option<E>,
    >(
        context: &mut ScenarioContext,
        app: &mut App,
        filter: F,
        early_exit: G,
    ) -> Result<Result<Vec<T>, E>, Error> {
        let mut event_cursor = EventCursor::<T>::default();
        loop {
            {
                let world = app.world_mut();
                let events = world.resource::<bevy::ecs::event::Events<T>>();

                let events = event_cursor
                    .read(events)
                    .filter(|&e| filter(e))
                    .cloned()
                    .collect::<Vec<_>>();
                if !events.is_empty() {
                    return Ok(Ok(events));
                }
                if let Some(e) = early_exit(world) {
                    return Ok(Err(e));
                }
            }
            Self::run_update_catching_error_events(context, app)
                .with_context(|| format!("timed out waiting for event {}", stringify!(T)))?;
        }
    }

    pub fn execute(self, context: &mut ScenarioContext, app: &mut App) -> Result<(), Error> {
        match self {
            ScenarioStep::SetCurrentLanguage { language } => {
                let language = if language == Language::External(SCENARIO_SELF_LANGUAGE_NAME.into())
                {
                    // main script language can be gotten from the "this_script_asset_relative_path"
                    let extension = context
                        .this_script_asset_relative_path
                        .extension()
                        .and_then(|ext| ext.to_str())
                        .unwrap_or_default();
                    let extensions = LanguageExtensions::default();
                    match extensions.get(extension) {
                        Some(language) => language.clone(),
                        None => {
                            return Err(anyhow!(
                                "Unknown script language for extension: {}",
                                extension
                            ));
                        }
                    }
                } else {
                    language
                };

                context.current_script_language = Some(language);
                bevy::log::info!(
                    "Set current script language to: {:?}",
                    context.current_script_language
                );
            }
            ScenarioStep::FinalizeApp => {
                app.finish();
                app.cleanup();
                bevy::log::info!("App finalized and cleaned up");
            }
            ScenarioStep::InstallPlugin {
                context_policy,
                emit_responses,
            } => {
                if !context.initialized_app {
                    *app = setup_integration_test(|_, _| {});
                    install_test_plugin(app, true);
                    context.initialized_app = true;
                }

                match context.current_script_language {
                    #[cfg(feature = "lua")]
                    Some(Language::Lua) => {
                        let plugin = crate::make_test_lua_plugin();
                        let plugin = plugin
                            .set_context_policy(context_policy)
                            .emit_core_callback_responses(emit_responses);
                        app.add_plugins(plugin);
                    }
                    #[cfg(feature = "rhai")]
                    Some(Language::Rhai) => {
                        let plugin = crate::make_test_rhai_plugin();
                        let plugin = plugin
                            .set_context_policy(context_policy)
                            .emit_core_callback_responses(emit_responses);
                        app.add_plugins(plugin);
                    }
                    _ => {
                        return Err(anyhow!(
                                                            "Scenario step InstallPlugin is not supported for the current plugin type: '{}'",
                                                            context.current_script_language
                                                                .as_ref()
                                                                .map(|l| l.to_string())
                                                                .unwrap_or_else(|| "None".to_string())
                                                        ));
                    }
                }
                return Ok(());
            }
            ScenarioStep::LoadScriptAs { path, as_name } => {
                let path = if path.ends_with(SCENARIO_SELF_SCRIPT_NAME) {
                    context
                        .this_script_asset_relative_path
                        .file_name()
                        .unwrap_or_default()
                        .into()
                } else {
                    path.clone()
                };
                let asset_server = app.world_mut().resource::<bevy::asset::AssetServer>();
                let script_handle = asset_server.load(context.scenario_path(&path));
                context
                    .script_handles
                    .insert(as_name.to_string(), script_handle);

                bevy::log::info!(
                    "Script '{}' marked for loading from path '{}'",
                    as_name,
                    path.display()
                );
            }
            ScenarioStep::WaitForScriptLoaded { script } => {
                let res = Self::execute_until_event::<AssetEvent<ScriptAsset>, _, _, _>(
                    context,
                    app,
                    |e| e.is_added(script.id()),
                    |w| {
                        let server = w.resource::<bevy::asset::AssetServer>();
                        if let LoadState::Failed(r) = server.load_state(script.id()) {
                            Some(r)
                        } else {
                            None
                        }
                    },
                )?;

                if let Err(e) = res {
                    return Err(anyhow!("Failed to load script: {e}"));
                }

                bevy::log::info!("Script '{}' loaded successfully", script.id());
            }
            ScenarioStep::SetupHandler { schedule, label } => {
                match label.to_string().as_str() {
                    "on_test" => {
                        schedule.add_handler::<OnTest>(context.current_script_language.clone(), app)
                    }
                    "on_test_post_update" => schedule.add_handler::<OnTestPostUpdate>(
                        context.current_script_language.clone(),
                        app,
                    ),
                    "on_test_last" => schedule
                        .add_handler::<OnTestLast>(context.current_script_language.clone(), app),
                    "callback_a" => schedule
                        .add_handler::<CallbackA>(context.current_script_language.clone(), app),
                    "callback_b" => schedule
                        .add_handler::<CallbackB>(context.current_script_language.clone(), app),
                    "callback_c" => schedule
                        .add_handler::<CallbackC>(context.current_script_language.clone(), app),
                    _ => {
                        return Err(anyhow!(
                    "callback label: {} is not allowed, you can only use one of a set of labels",
                    label
                ))
                    }
                }
            }
            ScenarioStep::SpawnEntityWithScript {
                entity: name,
                script,
            } => {
                let entity = app
                    .world_mut()
                    .spawn(ScriptComponent::new([script.clone()]))
                    .id();

                context.entities.insert(name.to_string(), entity);
                bevy::log::info!("Spawned entity '{}' with script '{}'", entity, script.id());
            }
            ScenarioStep::EmitScriptCallbackEvent { event } => {
                app.world_mut().send_event(event.clone());
            }
            ScenarioStep::AssertCallbackSuccess {
                label,
                script,
                expect_string_value,
                language,
            } => {
                let next_event = context.event_log.script_responses_queue.pop_front();

                if let Some(event) = next_event {
                    let language_correct = language.is_none_or(|l| l == event.language);
                    if event.label != label || event.context_key != script || !language_correct {
                        return Err(anyhow!(
                            "Callback '{}' for attachment: '{}' was not the next event, found: {:?}. Order of events was incorrect.",
                            label,
                            script.to_string(),
                            event
                        ));
                    }

                    match &event.response {
                        Ok(val) => {
                            bevy::log::info!(
                                "Callback '{}' for attachment: '{}' succeeded, with value: {:?}",
                                label,
                                script.to_string(),
                                &val
                            );

                            if let Some(expected_string) = expect_string_value.as_ref() {
                                if ScriptValue::String(Cow::Owned(expected_string.clone())) != *val
                                {
                                    return Err(anyhow!(
                                                                                                "Callback '{}' for attachment: '{}' expected: {}, but got: {}",
                                                                                                label,
                                                                                                script.to_string(),
                                                                                                expected_string,
                                                                                                val.display_with_world(WorldGuard::new_exclusive(app.world_mut()))
                                                                                            ));
                                }
                            }
                        }
                        Err(e) => {
                            return Err(anyhow!(
                                "Callback '{}' for attachment: '{}' failed with error: {}",
                                label,
                                script.to_string(),
                                e.display_with_world(WorldGuard::new_exclusive(app.world_mut()))
                            ));
                        }
                    }
                } else {
                    return Err(anyhow!(
                        "No callback response event found for label: {} and attachment: {}",
                        label,
                        script.to_string()
                    ));
                }
            }
            ScenarioStep::RunUpdateOnce => {
                Self::run_update_catching_error_events(context, app)?;
            }
            ScenarioStep::DropScriptAsset { script } => {
                let name = context
                    .script_handles
                    .iter_mut()
                    .find_map(|(name, handle)| {
                        if handle.id() == script.id() {
                            *handle = handle.clone_weak();
                            Some(name.clone())
                        } else {
                            None
                        }
                    })
                    .ok_or_else(|| {
                        anyhow!(
                            "Script asset with id '{}' not found in context",
                            script.id()
                        )
                    })?;
                bevy::log::info!("Dropped script asset '{}' from context", name);
            }
            ScenarioStep::ReloadScriptFrom { script, path } => {
                let mut assets = app
                    .world_mut()
                    .resource_mut::<bevy::asset::Assets<ScriptAsset>>();

                let absolute_path = context.absolute_scenario_path(&path);

                if let Some(existing) = assets.get_mut(&script) {
                    let content = std::fs::read_to_string(&absolute_path).with_context(|| {
                        format!("Failed to read script file: {}", absolute_path.display())
                    })?;
                    let boxed_byte_arr = content.into_bytes().into_boxed_slice();
                    *existing = ScriptAsset {
                        content: boxed_byte_arr,
                        asset_path: path.into(),
                        language: existing.language.clone(),
                    };
                } else {
                    return Err(anyhow!(
                                        "Script asset with id '{}' not found in context. Tried reloading from path: {}",
                                        script.id(),
                                        path.display()
                                    ));
                }
            }
            ScenarioStep::AssertNoCallbackResponsesEmitted => {
                let next_event = context.event_log.script_responses_queue.pop_front();
                if next_event.is_some() {
                    return Err(anyhow!(
                        "Expected no callback responses to be emitted, but found: {:?}",
                        next_event
                    ));
                } else {
                    bevy::log::info!("No callback responses emitted as expected");
                }
            }
            ScenarioStep::DespawnEntity { entity } => {
                let success = app.world_mut().despawn(entity);
                if !success {
                    return Err(anyhow!(
                        "Failed to despawn entity with name '{}'. It may not exist.",
                        entity
                    ));
                } else {
                    bevy::log::info!("Despawning entity with name '{}'", entity);
                }
            }
            ScenarioStep::AttachStaticScript { script } => {
                AddStaticScript::new(script.clone()).apply(app.world_mut());
                bevy::log::info!("Attached static script with handle: {}", script.id());
            }
            ScenarioStep::DetachStaticScript { script } => {
                RemoveStaticScript::new(script.clone()).apply(app.world_mut());
                bevy::log::info!("Detached static script with handle: {}", script.id());
            }
            ScenarioStep::AssertContextResidents {
                script,
                residents_num,
            } => {
                let world = app.world_mut();
                let residents = match context.current_script_language {
                    #[cfg(feature = "lua")]
                    Some(Language::Lua) => world
                        .resource::<ScriptContext<bevy_mod_scripting_lua::LuaScriptingPlugin>>()
                        .residents_len(&script),
                    #[cfg(feature = "rhai")]
                    Some(Language::Rhai) => world
                        .resource::<ScriptContext<bevy_mod_scripting_rhai::RhaiScriptingPlugin>>()
                        .residents_len(&script),
                    _ => {
                        return Err(anyhow!(
                                    "Scenario step AssertContextRemoved is not supported for the current plugin type: '{:?}'",
                                    context.current_script_language
                                ));
                    }
                };

                if residents != residents_num {
                    return Err(anyhow!(
                        "Expected {} residents for script attachment: {}, but found {}",
                        residents_num,
                        script.to_string(),
                        residents
                    ));
                } else {
                    bevy::log::info!(
                        "Script attachment: {} has {} residents as expected",
                        script.to_string(),
                        residents
                    );
                }
            }
            ScenarioStep::Comment { comment } => {
                // Comments are ignored, do nothing, log it though for debugging
                bevy::log::info!("Comment: {}", comment);
            }
        }
        Ok(())
    }
}
