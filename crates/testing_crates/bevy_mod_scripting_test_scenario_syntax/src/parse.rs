use std::path::PathBuf;

use anyhow::Error;
use bevy_app::{FixedUpdate, Last, PostUpdate, Startup, Update};
use bevy_ecs::schedule::ScheduleLabel;

/// the special name for referencing the current script
pub const SCENARIO_SELF_SCRIPT_NAME: &str = "@this_script";
/// the special name for referencing the current script's language
pub const SCENARIO_SELF_LANGUAGE_NAME: &str = "@this_language";

/// Maps to bevy schedules
#[derive(Debug, Clone, Hash, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum ScenarioSchedule {
    /// maps to the bevy schedule
    Startup,
    /// maps to the bevy schedule
    Update,
    /// maps to the bevy schedule
    FixedUpdate,
    /// maps to the bevy schedule
    PostUpdate,
    /// maps to the bevy schedule
    Last,
}

impl ScheduleLabel for ScenarioSchedule {
    fn dyn_clone(&self) -> Box<dyn ScheduleLabel> {
        match self {
            ScenarioSchedule::Startup => Startup.dyn_clone(),
            ScenarioSchedule::Update => Update.dyn_clone(),
            ScenarioSchedule::FixedUpdate => FixedUpdate.dyn_clone(),
            ScenarioSchedule::PostUpdate => PostUpdate.dyn_clone(),
            ScenarioSchedule::Last => Last.dyn_clone(),
        }
    }
}

/// The mode of context assignment for scripts in the scenario.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum ContextMode {
    /// maps to the BMS domain
    Global,
    /// maps to the BMS domain
    PerEntity,
    /// maps to the BMS domain
    PerEntityPerScript,
}

/// Describes the available scenario steps
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(tag = "step")]
pub enum ScenarioStepSerialized {
    /// arbitrary comment
    Comment {
        /// a comment
        comment: String,
    },
    /// Installs the scripting plugin with the given settings and intializes the app
    InstallPlugin {
        /// the context policy to use in the scenario
        context_policy: Option<ContextMode>,
        /// sets the emit_responses flag for core callbacks
        emit_responses: Option<bool>,
        /// the amount of nanoseconds to set the loading budget to
        nanoseconds_budget: Option<u64>,
    },
    /// Sets the pipeline processing budget
    SetNanosecondsBudget {
        /// the amount of nanoseconds to set the loading budget to
        nanoseconds_budget: Option<u64>,
    },
    /// Called after the app config is set up, but before we run anything
    FinalizeApp,
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
        /// the schedule in which to place this handler
        #[serde(flatten)]
        schedule: ScenarioSchedule,
        /// the callback label to setup this handler for
        #[serde(flatten)]
        label: ScenarioLabel,
    },
    /// Loads a script from the given path and assigns it a name,
    /// this handle can be used later when loaded.
    LoadScriptAs {
        /// the path to load the script from
        path: PathBuf,
        /// the name to give this script for future reference
        as_name: String,
    },
    /// Waits until the script with the given name is loaded.
    WaitForScriptAssetLoaded {
        /// the name of the script for which to wait
        name: String,
    },
    /// Spawns an entity with the given name and attaches the given script to it.
    SpawnEntityWithScript {
        /// the name to give this entity for future reference
        name: String,
        /// the script to spawn on the entity
        script: String,
    },
    /// Attaches a static script
    AttachStaticScript {
        /// the script to be attached
        script: String,
    },
    /// Detaches a static script
    DetachStaticScript {
        /// the script to be detached
        script: String,
    },
    /// Drops the script asset from the scenario context.
    DropScriptAsset {
        /// the script to drop the asset for
        script: String,
    },
    /// Despawns the entity with the given name.
    DespawnEntity {
        /// the entity to despawn
        entity: String,
    },
    /// Emits a ScriptCallbackEvent
    EmitScriptCallbackEvent {
        /// the label to fire the callback for
        label: ScenarioLabel,
        /// the recipients to set on the callback event
        #[serde(flatten)]
        recipients: ScenarioRecipients,
        /// the language to set on the callback event
        language: Option<ScenarioLanguage>,
        /// sets the emit_response flag on the callback event
        #[serde(default)]
        emit_response: bool,
        /// the string value to send as payload
        string_value: Option<String>,
    },

    /// Run the app update loop once
    RunUpdateOnce,

    /// Asserts that a callback response was triggered for the given script attachment
    AssertCallbackSuccess {
        /// the label for which to query
        label: ScenarioLabel,
        /// the attachment to query
        #[serde(flatten)]
        attachment: ScenarioAttachment,
        /// the string value to expect returned from the callback
        expect_string_value: Option<String>,
        /// the language to query
        language: Option<ScenarioLanguage>,
    },
    /// Assert that no callbacks were emitted this frame
    AssertNoCallbackResponsesEmitted,
    /// Asserts that the context for the given attachment is in a certain state
    AssertContextState {
        /// the attachment to query
        #[serde(flatten)]
        attachment: ScenarioAttachment,
        /// the context state to assert
        state: ScenarioContextState,
    },
    /// Assert that the context for the given attachment has a certain amount of residents
    AssertContextResidents {
        /// the attachment to query
        #[serde(flatten)]
        script: ScenarioAttachment,
        /// number of residents to expect
        residents_num: usize,
    },
    /// Modifies the existing script asset by reloading it from the given path.
    ReloadScriptFrom {
        /// the script to reload
        script: String,
        /// the path to reload the new content from
        path: PathBuf,
    },

    /// Sets the current script language context to be used untll this is changed
    SetCurrentLanguage {
        /// the language
        language: ScenarioLanguage,
    },
}

/// Maps to supported context states
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum ScenarioContextState {
    /// Maps to equivalent in BMS domain
    LoadedAndActive,
    /// Maps to equivalent in BMS domain
    Loading,
    /// Maps to equivalent in BMS domain
    Reloading,
    /// Maps to equivalent in BMS domain[]
    Unloading,
}

/// Maps to supported attachments
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(tag = "attachment")]
pub enum ScenarioAttachment {
    /// Maps to equivalent in BMS domain
    EntityScript {
        /// the entity of the attachment
        entity: String,
        /// the script of the attachment
        script: String,
    },
    /// Maps to equivalent in BMS domain
    StaticScript {
        /// the script of the attachment
        script: String,
    },
}

/// Maps to supported languages
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum ScenarioLanguage {
    /// Lua
    Lua,
    /// Rhai
    Rhai,
    /// The language of the script bound to this scenario
    #[serde(rename = "@this_script_language")]
    ThisScriptLanguage,
}

/// Maps to special / core callback labels
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum ScenarioLabel {
    /// An arbitrary callback
    OnTest,
    /// An arbitrary callback
    OnTestPostUpdate,
    /// An arbitrary callback
    OnTestLast,
    /// An arbitrary callback
    CallbackA,
    /// An arbitrary callback
    CallbackB,
    /// An arbitrary callback
    CallbackC,
    /// Maps to the on_script_loaded callback
    OnScriptLoaded,
    /// Maps to the on_script_unloaded callback
    OnScriptUnloaded,
    /// Maps to the on_script_reloaded callback
    OnScriptReloaded,
}

/// Maps to the BMS domain
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(tag = "recipients")]
pub enum ScenarioRecipients {
    /// Maps to the equivalent in BMS domain
    AllScripts,
    /// Maps to the equivalent in BMS domain
    AllContexts,
    /// Maps to the equivalent in BMS domain
    EntityScript {
        /// the entity to target
        entity: String,
        /// the script to target
        script: String,
    },
    /// Maps to the equivalent in BMS domain
    StaticScript {
        ///the script to target
        script: String,
    },
}

impl ScenarioStepSerialized {
    /// serialize a single scenario step to a json representation
    pub fn to_json(&self) -> Result<String, Error> {
        Ok(serde_json::to_string(self)?)
    }

    /// deserialize a single scenario step from a json representation
    pub fn from_json(json: &str) -> Result<Self, Error> {
        Ok(serde_json::from_str(json)?)
    }

    /// converts to json object then converts to a format like:
    /// StepName key="value", key2=value2
    pub fn to_flat_string(&self) -> Result<String, Error> {
        let json = self.to_json()?;
        let json_obj: serde_json::Value = serde_json::from_str(&json)?;
        let mut flat_string = String::new();
        if let serde_json::Value::Object(map) = json_obj {
            // the `step` key is the name of the step
            if let Some(step_name) = map.get("step") {
                flat_string.push_str(&format!("{} ", step_name.as_str().unwrap_or("")));
            }
            let non_step_keys: Vec<_> = map.into_iter().filter(|(k, _)| k != "step").collect();
            for (index, (key, value)) in non_step_keys.iter().enumerate() {
                if key != "step" {
                    flat_string.push_str(&format!(
                        "{}=\"{}\"",
                        key,
                        value.as_str().unwrap_or(value.to_string().as_str())
                    ));
                }
                if index + 1 != non_step_keys.len() {
                    flat_string.push_str(", ");
                }
            }
        }
        Ok(flat_string.trim().to_string())
    }

    /// deserialize a single scenario step from a flattened text representation
    pub fn from_flat_string(flat_string: &str) -> Result<Self, Error> {
        let flat_string = flat_string.trim();
        if flat_string.starts_with("//") {
            // This is a comment, ignore it
            return Ok(ScenarioStepSerialized::Comment {
                comment: flat_string.trim_start_matches("//").trim().to_string(),
            });
        }

        let mut parts = flat_string.split_whitespace();
        let step_name = parts
            .next()
            .ok_or_else(|| anyhow::anyhow!("Invalid flat string step: `{flat_string}`"))?;
        let mut map = serde_json::Map::new();
        map.insert(
            "step".to_string(),
            serde_json::Value::String(step_name.to_string()),
        );

        let arg_part = parts.collect::<Vec<_>>().join(" ");

        let args = arg_part
            .split(',')
            .map(str::trim)
            .filter(|p| !p.is_empty())
            .collect::<Vec<_>>();

        for part in args {
            let mut kv = part.split('=');
            let key = kv.next().ok_or_else(|| {
                anyhow::anyhow!(
                    "Key-value pair must be in the format key=\"value\" in part: `{part}`"
                )
            })?;
            let value = kv
                .next()
                .ok_or_else(|| {
                    anyhow::anyhow!(
                        "Key-value pair must be in the format key=\"value\" in part: `{part}`"
                    )
                })?
                .trim();
            let parsed_value = if !value.starts_with('"') && !value.ends_with('"') {
                serde_json::from_str::<serde_json::Value>(value)
                    .map_err(|e| Error::msg(format!("Failed to parse value: {e}")))?
            } else {
                serde_json::Value::String(value.trim_matches('"').to_string())
            };
            map.insert(key.trim().to_string(), parsed_value);
        }

        Ok(serde_json::from_value(serde_json::Value::Object(map))?)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_scenario_step_serialized_to_flat_string() {
        let step = ScenarioStepSerialized::AssertCallbackSuccess {
            label: ScenarioLabel::OnTest,
            attachment: ScenarioAttachment::EntityScript {
                entity: "entity1".to_string(),
                script: "script1".to_string(),
            },
            expect_string_value: None,
            language: None,
        };
        let flat_string = step.to_flat_string().unwrap();
        assert_eq!(
            flat_string,
            "AssertCallbackSuccess attachment=\"EntityScript\", entity=\"entity1\", expect_string_value=\"null\", label=\"OnTest\", language=\"null\", script=\"script1\""
        );
    }

    #[test]
    fn test_scenario_step_serialized_from_flat_string() {
        let flat_string = "AssertCallbackSuccess attachment=\"EntityScript\", entity=\"entity1\", label=\"OnTest\", script=\"script1\"";
        let step = ScenarioStepSerialized::from_flat_string(flat_string).unwrap();
        assert_eq!(
            step,
            ScenarioStepSerialized::AssertCallbackSuccess {
                label: ScenarioLabel::OnTest,
                attachment: ScenarioAttachment::EntityScript {
                    entity: "entity1".to_string(),
                    script: "script1".to_string(),
                },
                expect_string_value: None,
                language: None,
            }
        );
    }
}
