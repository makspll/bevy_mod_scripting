use indexmap::IndexMap;

#[derive(Debug, serde::Deserialize)]
pub struct Meta {
    pub features: Vec<String>,
    pub version: String,
    pub dependencies: IndexMap<String, Dependency>,
}

#[derive(Debug, serde::Deserialize, serde::Serialize)]
pub struct Dependency {
    pub version: String,
    pub features: Vec<String>,
    #[serde(default)]
    pub enable_default_feature: bool,
}
