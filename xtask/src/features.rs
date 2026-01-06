use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use itertools::Itertools;
use strum::VariantNames;

#[derive(
    Clone,
    Copy,
    Debug,
    PartialEq,
    Eq,
    Hash,
    PartialOrd,
    Ord,
    strum::EnumString,
    strum::EnumIter,
    strum::Display,
    strum::VariantNames,
    strum::VariantArray,
)]
#[strum(serialize_all = "snake_case")]
pub enum Feature {
    // bindings
    CoreFunctions,
    BevyEcsBindings,
    BevyInputBindings,
    BevyMathBindings,
    BevyReflectBindings,
    BevyTimeBindings,
    BevyTransformBindings,
    BevyColorBindings,
    BevyCorePipelineBindings,
    BevyA11yBindings,
    BevyAnimationBindings,
    BevyAssetBindings,
    BevyGizmosBindings,
    BevyGltfBindings,
    BevyImageBindings,
    BevyInputFocusBindings,
    BevyMeshBindings,
    BevyPbrBindings,
    BevyPickingBindings,
    BevyRenderBindings,
    BevySceneBindings,
    BevySpriteBindings,
    BevyTextBindings,

    // Ladfile
    LuaLanguageServerFiles,

    // Lua
    Lua51,
    Lua52,
    Lua53,
    Lua54,
    Luajit,
    Luajit52,
    Luau,
    UnsafeLuaModules,
    MluaSerialize,
    MluaMacros,
    MluaAsync,
    // Rhai,
    Rhai,
    // Rune
    // Rune,

    // Profiling
    ProfileWithTracy,
}

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq, strum::EnumIter)]
pub enum FeatureGroup {
    LuaExclusive,
    RhaiExclusive,
    // RuneExclusive,
    ForExternalCrate,
    BMSFeature,
    BMSFeatureNotInPowerset,
}

impl FeatureGroup {
    pub fn default_feature(self) -> Feature {
        match self {
            FeatureGroup::LuaExclusive => Feature::Lua54,
            FeatureGroup::RhaiExclusive => Feature::Rhai,
            // FeatureGroup::RuneExclusive => Feature::Rune,
            _ => panic!("No default feature for non-exclusive group"),
        }
    }

    pub fn is_exclusive(self) -> bool {
        matches!(
            self,
            FeatureGroup::LuaExclusive | FeatureGroup::RhaiExclusive // | FeatureGroup::RuneExclusive
        )
    }
}

trait IntoFeatureGroup {
    fn to_feature_group(self) -> FeatureGroup;
}

impl IntoFeatureGroup for Feature {
    fn to_feature_group(self) -> FeatureGroup {
        match self {
            Feature::Lua51
            | Feature::Lua52
            | Feature::Lua53
            | Feature::Lua54
            | Feature::Luajit
            | Feature::Luajit52
            | Feature::Luau => FeatureGroup::LuaExclusive,
            Feature::Rhai => FeatureGroup::RhaiExclusive,
            // Feature::Rune => FeatureGroup::RuneExclusive,
            Feature::MluaAsync
            | Feature::MluaMacros
            | Feature::MluaSerialize
            | Feature::UnsafeLuaModules => FeatureGroup::ForExternalCrate,
            Feature::BevyEcsBindings
            | Feature::BevyInputBindings
            | Feature::BevyMathBindings
            | Feature::BevyReflectBindings
            | Feature::BevyTimeBindings
            | Feature::BevyTransformBindings
            | Feature::BevyColorBindings
            | Feature::BevyCorePipelineBindings
            | Feature::BevyA11yBindings
            | Feature::BevyAnimationBindings
            | Feature::BevyAssetBindings
            | Feature::BevyGizmosBindings
            | Feature::BevyGltfBindings
            | Feature::BevyImageBindings
            | Feature::BevyInputFocusBindings
            | Feature::BevyMeshBindings
            | Feature::BevyPbrBindings
            | Feature::BevyPickingBindings
            | Feature::BevyRenderBindings
            | Feature::BevySceneBindings
            | Feature::BevySpriteBindings
            | Feature::BevyTextBindings => FeatureGroup::BMSFeatureNotInPowerset,
            Feature::CoreFunctions
            | Feature::ProfileWithTracy
            | Feature::LuaLanguageServerFiles => FeatureGroup::BMSFeature, // don't use wildcard here, we want to be explicit
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Features(pub HashSet<Feature>);

impl Default for Features {
    fn default() -> Self {
        // should be kept up to date with the default feature + lua54 on top of anything that is handy to run locally every time
        Features::new(vec![
            Feature::Lua54,
            Feature::Rhai,
            Feature::CoreFunctions,
            Feature::BevyEcsBindings,
            Feature::BevyInputBindings,
            Feature::BevyMathBindings,
            Feature::BevyReflectBindings,
            Feature::BevyTimeBindings,
            Feature::BevyTransformBindings,
            Feature::BevyColorBindings,
            Feature::BevyCorePipelineBindings,
            Feature::LuaLanguageServerFiles,
        ])
    }
}

impl Features {
    pub fn new<I: IntoIterator<Item = Feature>>(features: I) -> Self {
        Self(features.into_iter().collect())
    }

    /// Returns all features except the exclusive ones which are not the default
    pub fn all_features() -> Self {
        // remove exclusive features which are not the default
        Self(
            <Feature as strum::VariantArray>::VARIANTS
                .iter()
                .filter(|f| {
                    let group = f.to_feature_group();
                    (!group.is_exclusive()) || (**f == group.default_feature())
                })
                .cloned()
                .collect(),
        )
    }

    pub fn display_no_default(self) -> String {
        let default_features = Self::default().0;

        let excluded_default_features = default_features
            .difference(&self.0)
            .map(|f| format!("-{f}"))
            .collect::<Vec<_>>();

        let excluded_non_powerset_features = self
            .0
            .iter()
            .filter(|f| matches!(f.to_feature_group(), FeatureGroup::BMSFeatureNotInPowerset))
            .map(|f| f.to_string())
            .collect::<Vec<_>>();

        let mut features = self
            .0
            .into_iter()
            .filter(|f| {
                !default_features.contains(f)
                    && !excluded_non_powerset_features.contains(&f.to_string())
            })
            .map(|f| f.to_string())
            .collect::<Vec<_>>();

        features.sort();
        excluded_default_features
            .into_iter()
            .chain(features)
            .chain(std::iter::once(format!(
                "+{} bindings",
                excluded_non_powerset_features.len()
            )))
            .collect::<Vec<_>>()
            .join(",")
    }

    pub fn without(self, feature: Feature) -> Self {
        Self(self.0.into_iter().filter(|f| *f != feature).collect())
    }

    pub fn to_cargo_args(&self) -> Vec<String> {
        if self.0.is_empty() {
            vec![]
        } else {
            vec![
                "--no-default-features".to_owned(),
                "--features".to_owned(),
                self.to_string(),
            ]
        }
    }

    pub fn to_placeholder() -> clap::builder::Str {
        format!("[{}]", Feature::VARIANTS.join("|")).into()
    }

    pub fn split_by_group(&self) -> HashMap<FeatureGroup, Vec<Feature>> {
        let mut groups = HashMap::new();
        for feature in &self.0 {
            let group = feature.to_feature_group();
            groups.entry(group).or_insert_with(Vec::new).push(*feature);
        }
        groups
    }
}

impl std::fmt::Display for Features {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // exclude default features
        for (i, feature) in self.0.iter().sorted().enumerate() {
            if i > 0 {
                write!(f, ",")?;
            }
            write!(f, "{feature}")?;
        }
        std::result::Result::Ok(())
    }
}

impl From<String> for Features {
    fn from(s: String) -> Self {
        if s.is_empty() {
            return Self::new(vec![]);
        }

        let features = s
            .trim()
            .split(',')
            .map(|f| {
                Feature::from_str(f).unwrap_or_else(|_| {
                    eprintln!("Unknown feature: '{f}'");
                    std::process::exit(1);
                })
            })
            .collect();
        Self(features)
    }
}
