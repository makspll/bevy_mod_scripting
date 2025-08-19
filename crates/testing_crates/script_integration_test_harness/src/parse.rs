use std::path::PathBuf;

use anyhow::Error;
use bevy_mod_scripting_core::{
    asset::Language,
    bindings::ScriptValue,
    callback_labels,
    event::{
        CallbackLabel, OnScriptLoaded, OnScriptReloaded, OnScriptUnloaded, Recipients,
        ScriptCallbackEvent,
    },
    script::{ContextPolicy, ScriptAttachment},
};

use crate::scenario::{SCENARIO_SELF_LANGUAGE_NAME, ScenarioContext, ScenarioStep};

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum ScenarioSchedule {
    Startup,
    Update,
    FixedUpdate,
    PostUpdate,
    Last,
}

/// The mode of context assignment for scripts in the scenario.
#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum ContextMode {
    Global,
    PerEntity,
    PerEntityPerScript,
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
    Comment {
        comment: String,
    },
    /// Installs the scripting plugin with the given settings and intializes the app
    InstallPlugin {
        context_policy: Option<ContextMode>,
        emit_responses: Option<bool>,
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
        #[serde(flatten)]
        schedule: ScenarioSchedule,
        #[serde(flatten)]
        label: ScenarioLabel,
    },
    /// Loads a script from the given path and assigns it a name,
    /// this handle can be used later when loaded.
    LoadScriptAs {
        path: PathBuf,
        as_name: String,
    },
    /// Waits until the script with the given name is loaded.
    WaitForScriptLoaded {
        name: String,
    },
    /// Spawns an entity with the given name and attaches the given script to it.
    SpawnEntityWithScript {
        name: String,
        script: String,
    },
    AttachStaticScript {
        script: String,
    },
    DetachStaticScript {
        script: String,
    },
    /// Drops the script asset from the scenario context.
    DropScriptAsset {
        script: String,
    },
    /// Despawns the entity with the given name.
    DespawnEntity {
        entity: String,
    },
    /// Emits a ScriptCallbackEvent
    EmitScriptCallbackEvent {
        label: ScenarioLabel,
        #[serde(flatten)]
        recipients: ScenarioRecipients,
        language: Option<ScenarioLanguage>,
        #[serde(default)]
        emit_response: bool,
        string_value: Option<String>,
    },

    /// Run the app update loop once
    RunUpdateOnce,

    /// Asserts that a callback response was triggered for the given script attachment
    AssertCallbackSuccess {
        label: ScenarioLabel,
        #[serde(flatten)]
        attachment: ScenarioAttachment,
        expect_string_value: Option<String>,
        language: Option<ScenarioLanguage>,
    },
    AssertNoCallbackResponsesEmitted,
    AssertContextResidents {
        #[serde(flatten)]
        script: ScenarioAttachment,
        residents_num: usize,
    },
    /// Modifies the existing script asset by reloading it from the given path.
    ReloadScriptFrom {
        script: String,
        path: PathBuf,
    },

    /// Sets the current script language context to be used untll this is changed
    SetCurrentLanguage {
        language: ScenarioLanguage,
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
    #[serde(rename = "@this_script_language")]
    ThisScriptLanguage,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub enum ScenarioLabel {
    OnTest,
    OnTestPostUpdate,
    OnTestLast,
    CallbackA,
    CallbackB,
    CallbackC,
    OnScriptLoaded,
    OnScriptUnloaded,
    OnScriptReloaded,
}

#[derive(Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
#[serde(tag = "recipients")]
pub enum ScenarioRecipients {
    AllScripts,
    AllContexts,
    EntityScript { entity: String, script: String },
    StaticScript { script: String },
}

impl ScenarioStepSerialized {
    pub fn parse_language(language: ScenarioLanguage) -> Language {
        match language {
            ScenarioLanguage::Lua => Language::Lua,
            ScenarioLanguage::Rhai => Language::Rhai,
            ScenarioLanguage::ThisScriptLanguage => {
                Language::External(SCENARIO_SELF_LANGUAGE_NAME.into())
            }
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
                Ok(ScriptAttachment::EntityScript(entity, script))
            }
            ScenarioAttachment::StaticScript { script } => {
                let script = context.get_script_handle(&script)?;
                Ok(ScriptAttachment::StaticScript(script))
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
            ScenarioLabel::OnScriptLoaded => OnScriptLoaded.into(),
            ScenarioLabel::OnScriptUnloaded => OnScriptUnloaded.into(),
            ScenarioLabel::OnScriptReloaded => OnScriptReloaded.into(),
        }
    }

    pub fn resolve_context_policy(context_policy: Option<ContextMode>) -> ContextPolicy {
        match context_policy {
            Some(ContextMode::Global) => ContextPolicy::shared(),
            Some(ContextMode::PerEntity) => ContextPolicy::per_entity(),
            Some(ContextMode::PerEntityPerScript) => ContextPolicy::per_entity_and_script(),
            None => ContextPolicy::default(),
        }
    }

    pub fn parse_and_resolve(self, context: &ScenarioContext) -> Result<ScenarioStep, Error> {
        Ok(match self {
            Self::FinalizeApp => ScenarioStep::FinalizeApp,
            Self::AssertContextResidents {
                script,
                residents_num,
            } => ScenarioStep::AssertContextResidents {
                script: Self::resolve_attachment(context, script)?,
                residents_num,
            },
            Self::AttachStaticScript { script } => ScenarioStep::AttachStaticScript {
                script: context.get_script_handle(&script)?,
            },
            Self::DetachStaticScript { script } => ScenarioStep::DetachStaticScript {
                script: context.get_script_handle(&script)?,
            },
            Self::SetCurrentLanguage { language } => ScenarioStep::SetCurrentLanguage {
                language: Self::parse_language(language),
            },
            Self::InstallPlugin {
                context_policy,
                emit_responses,
            } => ScenarioStep::InstallPlugin {
                context_policy: Self::resolve_context_policy(context_policy),
                emit_responses: emit_responses.unwrap_or(false),
            },
            Self::DropScriptAsset { script } => ScenarioStep::DropScriptAsset {
                script: context.get_script_handle(&script)?,
            },
            Self::RunUpdateOnce => ScenarioStep::RunUpdateOnce,
            Self::EmitScriptCallbackEvent {
                label,
                recipients,
                language,
                emit_response,
                string_value,
            } => {
                let label = Self::resolve_label(label.clone());
                let recipients = Self::resolve_recipients(context, recipients.clone())?;
                let language = language.map(Self::parse_language);
                let payload = string_value
                    .map(|s| vec![ScriptValue::String(s.into())])
                    .unwrap_or(vec![]);
                let mut event = ScriptCallbackEvent::new(label, payload, recipients, language);
                if emit_response {
                    event = event.with_response();
                }
                ScenarioStep::EmitScriptCallbackEvent { event }
            }
            Self::AssertCallbackSuccess {
                label,
                attachment,
                expect_string_value,
                language,
            } => ScenarioStep::AssertCallbackSuccess {
                label: Self::resolve_label(label.clone()),
                script: Self::resolve_attachment(context, attachment)?,
                expect_string_value,
                language: language.map(Self::parse_language),
            },
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
            Self::ReloadScriptFrom { script, path } => ScenarioStep::ReloadScriptFrom {
                script: context.get_script_handle(&script)?,
                path,
            },
            Self::AssertNoCallbackResponsesEmitted => {
                ScenarioStep::AssertNoCallbackResponsesEmitted
            }
            Self::DespawnEntity { entity } => ScenarioStep::DespawnEntity {
                entity: context.get_entity(&entity)?,
            },
            Self::Comment { comment } => ScenarioStep::Comment { comment },
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
            .ok_or_else(|| anyhow::anyhow!("Invalid flat string step: `{}`", flat_string))?;
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
