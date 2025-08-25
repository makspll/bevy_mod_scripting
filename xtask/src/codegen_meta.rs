use std::collections::HashMap;

#[derive(Debug, serde::Deserialize)]
pub struct Meta {
    pub features: Vec<String>,
    pub version: String,
    pub dependencies: HashMap<String, Dependency>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Dependency {
    pub version: String,
    pub features: Vec<String>,
    #[serde(default)]
    pub enable_default_feature: bool,
}
