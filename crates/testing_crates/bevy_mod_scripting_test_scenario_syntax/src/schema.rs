use std::collections::BTreeMap;

use serde::Serialize;

#[derive(Serialize)]
struct ScenarioSchema {
    pub steps: BTreeMap<String, StepSchema>,
}

#[derive(Serialize)]
struct StepSchema {
    pub fields: BTreeMap<String, FieldSchema>,
}
#[derive(Serialize)]
enum SchemaType {
    String,
    Number,
}

#[derive(Serialize)]
struct FieldSchema {
    pub ty: SchemaType,
    pub optional: bool,
    pub doc: String,
}

/// Serializes the scenario schema
pub fn serialize_schema() -> serde_json::Result<String> {
    let schema = get_schema();
    serde_json::to_string_pretty(&schema)
}

/// Produces schema as expected by scenario files
fn get_schema() -> ScenarioSchema {
    fn str_field(name: &str, optional: bool, doc: &str) -> (String, FieldSchema) {
        (
            name.to_string(),
            FieldSchema {
                ty: SchemaType::String,
                optional,
                doc: doc.to_string(),
            },
        )
    }

    fn num_field(name: &str, optional: bool, doc: &str) -> (String, FieldSchema) {
        (
            name.to_string(),
            FieldSchema {
                ty: SchemaType::Number,
                optional,
                doc: doc.to_string(),
            },
        )
    }

    ScenarioSchema {
        steps: vec![
            (
                "Comment".into(),
                StepSchema {
                    fields: vec![str_field("comment", false, "a comment")]
                        .into_iter()
                        .collect(),
                },
            ),
            (
                "InstallPlugin".into(),
                StepSchema {
                    fields: vec![
                        str_field(
                            "context_policy",
                            true,
                            "the context policy to use in the scenario",
                        ),
                        str_field(
                            "emit_responses",
                            true,
                            "sets the emit_responses flag for core callbacks",
                        ),
                        num_field(
                            "nanoseconds_budget",
                            true,
                            "the amount of nanoseconds to set the loading budget to",
                        ),
                    ]
                    .into_iter()
                    .collect(),
                },
            ),
            (
                "SetNanosecondsBudget".into(),
                StepSchema {
                    fields: vec![num_field(
                        "nanoseconds_budget",
                        true,
                        "the amount of nanoseconds to set the loading budget to",
                    )]
                    .into_iter()
                    .collect(),
                },
            ),
            (
                "FinalizeApp".into(),
                StepSchema {
                    fields: BTreeMap::new(),
                },
            ),
            (
                "SetupHandler".into(),
                StepSchema {
                    fields: vec![
                        str_field("schedule", false, "schedule in which to place this handler"),
                        str_field("label", false, "callback label to setup this handler for"),
                    ]
                    .into_iter()
                    .collect(),
                },
            ),
            (
                "LoadScriptAs".into(),
                StepSchema {
                    fields: vec![
                        str_field("path", false, "the path to load the script from"),
                        str_field(
                            "as_name",
                            false,
                            "the name to give this script for future reference",
                        ),
                    ]
                    .into_iter()
                    .collect(),
                },
            ),
            (
                "WaitForScriptAssetLoaded".into(),
                StepSchema {
                    fields: vec![str_field(
                        "name",
                        false,
                        "the name of the script for which to wait",
                    )]
                    .into_iter()
                    .collect(),
                },
            ),
            (
                "SpawnEntityWithScript".into(),
                StepSchema {
                    fields: vec![
                        str_field(
                            "name",
                            false,
                            "the name to give this entity for future reference",
                        ),
                        str_field("script", false, "the script to spawn on the entity"),
                    ]
                    .into_iter()
                    .collect(),
                },
            ),
            (
                "AttachStaticScript".into(),
                StepSchema {
                    fields: vec![str_field("script", false, "the script to be attached")]
                        .into_iter()
                        .collect(),
                },
            ),
            (
                "DetachStaticScript".into(),
                StepSchema {
                    fields: vec![str_field("script", false, "the script to be detached")]
                        .into_iter()
                        .collect(),
                },
            ),
            (
                "DropScriptAsset".into(),
                StepSchema {
                    fields: vec![str_field(
                        "script",
                        false,
                        "the script to drop the asset for",
                    )]
                    .into_iter()
                    .collect(),
                },
            ),
            (
                "DespawnEntity".into(),
                StepSchema {
                    fields: vec![str_field("entity", false, "the entity to despawn")]
                        .into_iter()
                        .collect(),
                },
            ),
            (
                "EmitScriptCallbackEvent".into(),
                StepSchema {
                    fields: vec![
                        str_field("label", false, "the label to fire the callback for"),
                        // flattened ScenarioRecipients
                        str_field("recipients", false, "recipient selection discriminator"),
                        str_field("entity", true, "target entity when recipients requires it"),
                        str_field("script", true, "target script when recipients requires it"),
                        str_field(
                            "language",
                            true,
                            "the language to set on the callback event",
                        ),
                        str_field(
                            "emit_response",
                            false,
                            "sets the emit_response flag on the callback event",
                        ),
                        str_field("string_value", true, "the string value to send as payload"),
                    ]
                    .into_iter()
                    .collect(),
                },
            ),
            (
                "RunUpdateOnce".into(),
                StepSchema {
                    fields: BTreeMap::new(),
                },
            ),
            (
                "AssertCallbackSuccess".into(),
                StepSchema {
                    fields: vec![
                        str_field("label", false, "the label for which to query"),
                        // flattened ScenarioAttachment
                        str_field("attachment", false, "attachment discriminator"),
                        str_field("entity", true, "entity when attachment == EntityScript"),
                        str_field("script", true, "script when attachment targets a script"),
                        str_field(
                            "expect_string_value",
                            true,
                            "the string value to expect returned from the callback",
                        ),
                        str_field("language", true, "the language to query"),
                    ]
                    .into_iter()
                    .collect(),
                },
            ),
            (
                "AssertNoCallbackResponsesEmitted".into(),
                StepSchema {
                    fields: BTreeMap::new(),
                },
            ),
            (
                "AssertContextState".into(),
                StepSchema {
                    fields: vec![
                        str_field("attachment", false, "attachment discriminator"),
                        str_field("entity", true, "entity when attachment == EntityScript"),
                        str_field("script", true, "script when attachment targets a script"),
                        str_field("state", false, "the context state to assert"),
                    ]
                    .into_iter()
                    .collect(),
                },
            ),
            (
                "AssertContextResidents".into(),
                StepSchema {
                    fields: vec![
                        str_field("attachment", false, "attachment discriminator"),
                        str_field("entity", true, "entity when attachment == EntityScript"),
                        str_field("script", true, "script when attachment targets a script"),
                        num_field("residents_num", false, "number of residents to expect"),
                    ]
                    .into_iter()
                    .collect(),
                },
            ),
            (
                "ReloadScriptFrom".into(),
                StepSchema {
                    fields: vec![
                        str_field("script", false, "the script to reload"),
                        str_field("path", false, "the path to reload the new content from"),
                    ]
                    .into_iter()
                    .collect(),
                },
            ),
            (
                "SetCurrentLanguage".into(),
                StepSchema {
                    fields: vec![str_field("language", false, "the language")]
                        .into_iter()
                        .collect(),
                },
            ),
        ]
        .into_iter()
        .collect(),
    }
}
