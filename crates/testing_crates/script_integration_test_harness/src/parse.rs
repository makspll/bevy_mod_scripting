use std::path::PathBuf;

use anyhow::Error;
use bevy_mod_scripting_core::{
    asset::Language,
    callback_labels,
    event::{CallbackLabel, Recipients, ScriptCallbackEvent},
    script::{Domain, ScriptAttachment},
};

use crate::scenario::{ScenarioContext, ScenarioStep};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum ScenarioSchedule {
    Startup,
    Update,
    FixedUpdate,
    PostUpdate,
    Last,
}

callback_labels!(
    OnTest => "on_test",
    OnTestPostUpdate => "on_test_post_update",
    OnTestLast => "on_test_last",
    CallbackA => "callback_a",
    CallbackB => "callback_b",
    CallbackC => "callback_c",
);

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(tag = "step")]
pub enum ScenarioStepSerialized {
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
        #[serde(flatten)]
        schedule: ScenarioSchedule,
        #[serde(flatten)]
        label: ScenarioLabel,
    },
    /// Loads a script from the given path and assigns it a name,
    /// this handle can be used later when loaded.
    LoadScriptAs { path: PathBuf, as_name: String },
    /// Waits until the script with the given name is loaded.
    WaitForScriptLoaded { name: String },
    /// Spawns an entity with the given name and attaches the given script to it.
    SpawnEntityWithScript { name: String, script: String },

    /// Emits a ScriptCallbackEvent
    EmitScriptCallbackEvent {
        label: ScenarioLabel,
        #[serde(flatten)]
        recipients: ScenarioRecipients,
        language: Option<ScenarioLanguage>,
        emit_response: bool,
    },

    /// Run the app update loop once
    RunUpdateOnce,

    /// Asserts that a callback response was triggered for the given script attachment
    AssertCallbackSuccess {
        label: ScenarioLabel,
        #[serde(flatten)]
        attachment: ScenarioAttachment,
    },
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(tag = "attachment")]
pub enum ScenarioAttachment {
    EntityScript { entity: String, script: String },
    StaticScript { script: String },
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum ScenarioLanguage {
    Lua,
    Rhai,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum ScenarioLabel {
    OnTest,
    OnTestPostUpdate,
    OnTestLast,
    CallbackA,
    CallbackB,
    CallbackC,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(tag = "recipients")]
pub enum ScenarioRecipients {
    AllScripts,
    AllContexts,
    EntityScript { entity: String, script: String },
    StaticScript { script: String },
    Domain(String),
}

impl ScenarioStepSerialized {
    pub fn parse_language(language: ScenarioLanguage) -> Language {
        match language {
            ScenarioLanguage::Lua => Language::Lua,
            ScenarioLanguage::Rhai => Language::Rhai,
        }
    }

    pub fn resolve_attachment(
        context: &ScenarioContext,
        attachment: ScenarioAttachment,
    ) -> Result<ScriptAttachment, Error> {
        match attachment {
            ScenarioAttachment::EntityScript { entity, script } => {
                let entity = context.get_entity(&entity)?;
                let script = context.get_script_handle(&script)?;
                Ok(ScriptAttachment::EntityScript(entity, script, None))
            }
            ScenarioAttachment::StaticScript { script } => {
                let script = context.get_script_handle(&script)?;
                Ok(ScriptAttachment::StaticScript(script, None))
            }
        }
    }

    pub fn resolve_recipients(
        context: &ScenarioContext,
        recipients: ScenarioRecipients,
    ) -> Result<Recipients, Error> {
        Ok(match recipients {
            ScenarioRecipients::AllScripts => Recipients::AllScripts,
            ScenarioRecipients::AllContexts => Recipients::AllContexts,
            ScenarioRecipients::EntityScript { entity, script } => Recipients::ScriptEntity(
                context.get_script_handle(&script)?.id(),
                context.get_entity(&entity)?,
            ),
            ScenarioRecipients::StaticScript { script } => {
                Recipients::StaticScript(context.get_script_handle(&script)?.id())
            }
            ScenarioRecipients::Domain(domain) => Recipients::Domain(Domain::new(domain)),
        })
    }

    pub fn resolve_label(label: ScenarioLabel) -> CallbackLabel {
        match label {
            ScenarioLabel::OnTest => OnTest.into(),
            ScenarioLabel::OnTestPostUpdate => OnTestPostUpdate.into(),
            ScenarioLabel::OnTestLast => OnTestLast.into(),
            ScenarioLabel::CallbackA => CallbackA.into(),
            ScenarioLabel::CallbackB => CallbackB.into(),
            ScenarioLabel::CallbackC => CallbackC.into(),
        }
    }

    pub fn parse_and_resolve(self, context: &ScenarioContext) -> Result<ScenarioStep, Error> {
        Ok(match self {
            Self::RunUpdateOnce => ScenarioStep::RunUpdateOnce,
            Self::EmitScriptCallbackEvent {
                label,
                recipients,
                language,
                emit_response,
            } => {
                let label = Self::resolve_label(label.clone());
                let recipients = Self::resolve_recipients(context, recipients.clone())?;
                let language = language.map(Self::parse_language);
                let mut event = ScriptCallbackEvent::new(label, vec![], recipients, language);
                if emit_response {
                    event = event.with_response();
                }
                ScenarioStep::EmitScriptCallbackEvent { event }
            }
            Self::AssertCallbackSuccess { label, attachment } => {
                ScenarioStep::AssertCallbackSuccess {
                    label: Self::resolve_label(label.clone()),
                    script: Self::resolve_attachment(context, attachment)?,
                }
            }
            Self::SetupHandler { schedule, label } => ScenarioStep::SetupHandler {
                schedule,
                label: Self::resolve_label(label),
            },
            Self::LoadScriptAs { path, as_name } => ScenarioStep::LoadScriptAs {
                path,
                as_name: as_name.to_string(),
            },
            Self::WaitForScriptLoaded { name } => ScenarioStep::WaitForScriptLoaded {
                script: context.get_script_handle(&name)?,
            },
            Self::SpawnEntityWithScript { name, script } => ScenarioStep::SpawnEntityWithScript {
                script: context.get_script_handle(&script)?,
                entity: name,
            },
        })
    }

    pub fn to_json(&self) -> Result<String, Error> {
        Ok(serde_json::to_string(self)?)
    }

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

    pub fn from_flat_string(flat_string: &str) -> Result<Self, Error> {
        let mut parts = flat_string.split_whitespace();
        let step_name = parts.next().ok_or_else(|| {
            Error::msg("Flat string must start with the step name followed by key-value pairs")
        })?;
        let mut map = serde_json::Map::new();
        map.insert(
            "step".to_string(),
            serde_json::Value::String(step_name.to_string()),
        );

        let arg_part = parts.collect::<Vec<_>>().join(" ");
        let parts = arg_part.split(',').map(str::trim);

        for part in parts {
            let mut kv = part.split('=');
            let key = kv
                .next()
                .ok_or_else(|| Error::msg("Key-value pair must be in the format key=\"value\""))?;
            let value = kv
                .next()
                .ok_or_else(|| Error::msg("Key-value pair must be in the format key=\"value\""))?;
            map.insert(
                key.trim().to_string(),
                serde_json::Value::String(value.trim().trim_matches('"').to_string()),
            );
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
        };
        let flat_string = step.to_flat_string().unwrap();
        assert_eq!(flat_string, "AssertCallbackSuccess attachment=\"EntityScript\", entity=\"entity1\", label=\"OnTest\", script=\"script1\"");
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
            }
        );
    }
}
