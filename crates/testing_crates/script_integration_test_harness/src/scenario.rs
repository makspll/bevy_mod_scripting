use crate::parse::*;
use anyhow::{anyhow, Context, Error};
use bevy::prelude::IntoSystem;
use bevy::{
    app::App,
    asset::{AssetEvent, Handle, LoadState},
    ecs::{
        event::{Event, EventCursor, Events},
        schedule::ScheduleLabel,
        world::World,
    },
    log,
};
use bevy_mod_scripting_core::{
    asset::ScriptAsset,
    bindings::{DisplayWithWorld, WorldGuard},
    event::{CallbackLabel, IntoCallbackLabel, ScriptCallbackEvent, ScriptCallbackResponseEvent},
    handler::event_handler,
    script::{ScriptAttachment, ScriptComponent},
    IntoScriptPluginParams,
};
use std::{
    collections::HashMap,
    path::{Path, PathBuf},
    time::Instant,
};

const TIMEOUT_SECONDS: u64 = 10;
const SCENARIO_SELF_SCRIPT_NAME: &str = "@this_script";

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
            .filter(|line| !line.is_empty() && !line.starts_with('#') && !line.starts_with("//"))
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

    pub fn dump_steps(steps: &[ScenarioStepSerialized]) -> String {
        steps
            .iter()
            .enumerate()
            .map(|(i, step)| format!("#{i}: {}", step.to_flat_string().unwrap_or_default()))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn dump_events(watcher: &InterestingEventWatcher) -> String {
        watcher
            .events
            .iter()
            .map(|(event, step)| format!("#{step}: {event}"))
            .collect::<Vec<_>>()
            .join("\n")
    }

    pub fn execute<P: IntoScriptPluginParams>(mut self, mut app: App) -> Result<(), Error> {
        let original_steps = self.steps.clone();
        for (i, step) in self.steps.into_iter().enumerate() {
            self.context.current_step_no = i;
            let original_step = step.clone();
            let parsed_step = step.parse_and_resolve(&self.context)?;
            parsed_step
                .execute::<P>(&mut self.context, &mut app)
                .with_context(|| {
                    format!(
                        "Error in step #{i}: {}",
                        original_step.to_flat_string().unwrap_or_default()
                    )
                })
                .with_context(|| {
                    format!(
                        "While execution scenario:\n{}",
                        Self::dump_steps(&original_steps)
                    )
                })
                .with_context(|| {
                    format!(
                        "With events:\n{}",
                        Self::dump_events(&self.context.event_log)
                    )
                })?;
        }
        Ok(())
    }

    pub fn new_standard_scenario(this_script_path: PathBuf) -> Self {
        Self {
            steps: vec![
                ScenarioStepSerialized::SetupHandler {
                    schedule: ScenarioSchedule::Update,
                    label: ScenarioLabel::OnTest,
                },
                ScenarioStepSerialized::SetupHandler {
                    schedule: ScenarioSchedule::PostUpdate,
                    label: ScenarioLabel::OnTestPostUpdate,
                },
                ScenarioStepSerialized::SetupHandler {
                    schedule: ScenarioSchedule::Last,
                    label: ScenarioLabel::OnTestLast,
                },
                ScenarioStepSerialized::LoadScriptAs {
                    path: SCENARIO_SELF_SCRIPT_NAME.into(),
                    as_name: SCENARIO_SELF_SCRIPT_NAME.to_string(),
                },
                ScenarioStepSerialized::WaitForScriptLoaded {
                    name: SCENARIO_SELF_SCRIPT_NAME.to_string(),
                },
                ScenarioStepSerialized::SpawnEntityWithScript {
                    name: "test_entity".to_string(),
                    script: SCENARIO_SELF_SCRIPT_NAME.to_string(),
                },
                ScenarioStepSerialized::EmitScriptCallbackEvent {
                    label: ScenarioLabel::OnTest,
                    recipients: ScenarioRecipients::EntityScript {
                        script: SCENARIO_SELF_SCRIPT_NAME.to_string(),
                        entity: "test_entity".to_string(),
                    },
                    language: None,
                    emit_response: true,
                },
                ScenarioStepSerialized::RunUpdateOnce,
                ScenarioStepSerialized::AssertCallbackSuccess {
                    label: ScenarioLabel::OnTest,
                    attachment: ScenarioAttachment::EntityScript {
                        script: SCENARIO_SELF_SCRIPT_NAME.to_string(),
                        entity: "test_entity".to_string(),
                    },
                },
                ScenarioStepSerialized::EmitScriptCallbackEvent {
                    label: ScenarioLabel::OnTestPostUpdate,
                    recipients: ScenarioRecipients::EntityScript {
                        script: SCENARIO_SELF_SCRIPT_NAME.to_string(),
                        entity: "test_entity".to_string(),
                    },
                    language: None,
                    emit_response: true,
                },
                ScenarioStepSerialized::RunUpdateOnce,
                ScenarioStepSerialized::AssertCallbackSuccess {
                    label: ScenarioLabel::OnTestPostUpdate,
                    attachment: ScenarioAttachment::EntityScript {
                        script: SCENARIO_SELF_SCRIPT_NAME.to_string(),
                        entity: "test_entity".to_string(),
                    },
                },
                ScenarioStepSerialized::EmitScriptCallbackEvent {
                    label: ScenarioLabel::OnTestLast,
                    recipients: ScenarioRecipients::EntityScript {
                        script: SCENARIO_SELF_SCRIPT_NAME.to_string(),
                        entity: "test_entity".to_string(),
                    },
                    language: None,
                    emit_response: true,
                },
                ScenarioStepSerialized::RunUpdateOnce,
                ScenarioStepSerialized::AssertCallbackSuccess {
                    label: ScenarioLabel::OnTestLast,
                    attachment: ScenarioAttachment::EntityScript {
                        script: SCENARIO_SELF_SCRIPT_NAME.to_string(),
                        entity: "test_entity".to_string(),
                    },
                },
            ],
            context: ScenarioContext::new(this_script_path),
        }
    }
}

/// Serves as a chalkboard for the test scenario to write to and read from.
#[derive(Debug, Clone)]
pub struct ScenarioContext {
    pub script_handles: HashMap<String, Handle<ScriptAsset>>,
    pub entities: HashMap<String, bevy::ecs::entity::Entity>,
    pub scenario_time_started: Instant,
    pub this_script_asset_relative_path: PathBuf,
    pub callback_events_cursor: EventCursor<ScriptCallbackResponseEvent>,
    pub unmatched_callback_events: Vec<ScriptCallbackResponseEvent>,
    pub event_log: InterestingEventWatcher,
    pub current_step_no: usize,
}

#[derive(Debug, Clone, Default)]
pub struct InterestingEventWatcher {
    pub events: Vec<(String, usize)>,
    pub asset_event_cursor: EventCursor<AssetEvent<ScriptAsset>>,
    pub script_response_cursor: EventCursor<ScriptCallbackResponseEvent>,
}

impl InterestingEventWatcher {
    pub fn log_events(&mut self, step_no: usize, world: &bevy::ecs::world::World) {
        let asset_events = world.resource::<Events<AssetEvent<ScriptAsset>>>();
        let script_responses = world.resource::<Events<ScriptCallbackResponseEvent>>();
        for event in self.asset_event_cursor.read(asset_events) {
            self.events.push((format!("{event:?}"), step_no));
        }
        for event in self.script_response_cursor.read(script_responses) {
            self.events.push((format!("{event:?}"), step_no));
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
            callback_events_cursor: EventCursor::default(),
            unmatched_callback_events: Vec::new(),
            event_log: InterestingEventWatcher::default(),
            current_step_no: 0,
        }
    }

    pub fn scenario_path(&self, path: &PathBuf) -> PathBuf {
        self.this_script_asset_relative_path
            .parent()
            .unwrap_or(&PathBuf::new())
            .join(path)
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
    pub fn add_handler<P: IntoScriptPluginParams, T: IntoCallbackLabel + 'static>(
        &self,
        app: &mut App,
    ) {
        let system = IntoSystem::into_system(event_handler::<T, P>);
        let system = system.with_name(T::into_callback_label().to_string());
        app.add_systems(self.clone(), system);
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

#[derive(Debug, Clone)]
pub enum ScenarioStep {
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
    LoadScriptAs { path: PathBuf, as_name: String },
    /// Waits until the script with the given name is loaded.
    WaitForScriptLoaded { script: Handle<ScriptAsset> },
    /// Spawns an entity with the given name and attaches the given script to it.
    SpawnEntityWithScript {
        script: Handle<ScriptAsset>,
        entity: String,
    },

    /// Emits a ScriptCallbackEvent
    EmitScriptCallbackEvent { event: ScriptCallbackEvent },

    /// Run the app update loop once
    RunUpdateOnce,

    /// Asserts that a callback response was triggered for the given label and from the given recipient
    AssertCallbackSuccess {
        label: CallbackLabel,
        script: ScriptAttachment,
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

    pub fn execute<P: IntoScriptPluginParams>(
        self,
        context: &mut ScenarioContext,
        app: &mut App,
    ) -> Result<(), Error> {
        match self {
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

                log::info!(
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

                log::info!("Script '{}' loaded successfully", script.id());
            }
            ScenarioStep::SetupHandler { schedule, label } => match label.to_string().as_str() {
                "on_test" => schedule.add_handler::<P, OnTest>(app),
                "on_test_post_update" => schedule.add_handler::<P, OnTestPostUpdate>(app),
                "on_test_last" => schedule.add_handler::<P, OnTestLast>(app),
                "callback_a" => schedule.add_handler::<P, CallbackA>(app),
                "callback_b" => schedule.add_handler::<P, CallbackB>(app),
                "callback_c" => schedule.add_handler::<P, CallbackC>(app),
                _ => {
                    return Err(anyhow!(
                    "callback label: {} is not allowed, you can only use one of a set of labels",
                    label
                ))
                }
            },
            ScenarioStep::SpawnEntityWithScript {
                entity: name,
                script,
            } => {
                let entity = app
                    .world_mut()
                    .spawn(ScriptComponent::new([script.clone()]))
                    .id();

                context.entities.insert(name.to_string(), entity);
                log::info!("Spawned entity '{}' with script '{}'", entity, script.id());
            }
            ScenarioStep::EmitScriptCallbackEvent { event } => {
                app.world_mut().send_event(event.clone());
            }
            ScenarioStep::AssertCallbackSuccess { label, script } => {
                let mut event_cursor = context.callback_events_cursor.clone();
                let unprocessed_and_new_events = event_cursor
                    .read(
                        app.world_mut()
                            .resource::<Events<ScriptCallbackResponseEvent>>(),
                    )
                    .map(|e| (e, true))
                    .chain(context.unmatched_callback_events.iter().map(|e| (e, false)));
                let mut to_add = Vec::default();
                let mut errors = Vec::default();

                let mut found_match = false;
                for (event, is_new) in unprocessed_and_new_events {
                    if event.label == label && event.context_key == script {
                        found_match = true;
                        if let Err(e) = &event.response {
                            errors.push(e.clone());
                        } else {
                            log::info!(
                                "Callback '{}' for attachment: '{}' succeeded",
                                label,
                                script.to_string()
                            );
                        }
                    } else if is_new {
                        to_add.push(event.clone());
                    }
                }

                if !found_match {
                    return Err(anyhow!(
                        "Callback '{}' for attachment: '{}' not found",
                        label,
                        script.to_string()
                    ));
                }

                if let Some(first_error) = errors.first() {
                    let guard = WorldGuard::new_exclusive(app.world_mut());
                    let err = first_error.display_with_world(guard);
                    return Err(anyhow!(
                        "Callback '{}' for attachment: '{}' failed with error:\n{err}",
                        label,
                        script.to_string(),
                    ));
                }

                context.unmatched_callback_events.extend(to_add);
            }
            ScenarioStep::RunUpdateOnce => {
                Self::run_update_catching_error_events(context, app)?;
            }
        }
        Ok(())
    }
}
