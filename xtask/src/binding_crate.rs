use anyhow::Context;
use indexmap::IndexMap;
use serde::Serialize;

#[derive(Debug, Serialize)]
pub struct BindingCrate {
    pub crate_name: String,
    pub crate_version: String,
    pub features: Vec<String>,
    pub generated_from_version: String,
    pub enable_default_feature: bool,
    pub dependencies: IndexMap<String, crate::Dependency>,
    pub include_bevy_ecs_dep: bool,
    pub include_bevy_app_dep: bool,
}

const BINDING_CRATE_MANIFEST_TEMPLATE: &str = include_str!("../templates/bindings_crate.toml.tera");
const BINDING_CRATE_README_TEMPLATE: &str = include_str!("../templates/bindings_crate.readme.tera");

impl BindingCrate {
    pub fn new(
        crate_name: &str,
        crate_version: &str,
        mut features: Vec<String>,
        generated_from_version: String,
        mut dependencies: IndexMap<String, crate::Dependency>,
    ) -> Self {
        let enable_default_feature = features.contains(&"default".to_string());
        features.retain(|f| f != "default");

        // do the same for dependencies
        dependencies.iter_mut().for_each(|(_, dep)| {
            if dep.features.contains(&"default".to_string()) {
                dep.enable_default_feature = true;
                dep.features.retain(|f| f != "default");
            }
        });
        Self {
            crate_name: crate_name.to_string(),
            crate_version: crate_version.to_string(),
            features,
            generated_from_version,
            enable_default_feature,
            include_bevy_ecs_dep: crate_name != "bevy_ecs"
                && !dependencies.contains_key("bevy_ecs"),
            include_bevy_app_dep: crate_name != "bevy_app"
                && !dependencies.contains_key("bevy_app"),
            dependencies,
        }
    }

    pub fn generate_in_dir(&self, dir: &std::path::Path) -> anyhow::Result<()> {
        let mut tera = tera::Tera::default();
        let context = tera::Context::from_serialize(self).unwrap();
        let manifest_content = tera
            .render_str(BINDING_CRATE_MANIFEST_TEMPLATE, &context)
            .with_context(|| format!("Failed to render manifest for crate {}", self.crate_name))?;
        let readme_content = tera
            .render_str(BINDING_CRATE_README_TEMPLATE, &context)
            .with_context(|| format!("Failed to render README for crate {}", self.crate_name))?;

        std::fs::create_dir_all(dir)?;
        std::fs::write(dir.join("Cargo.toml"), manifest_content)?;
        std::fs::write(dir.join("README.md"), readme_content)?;
        Ok(())
    }
}
