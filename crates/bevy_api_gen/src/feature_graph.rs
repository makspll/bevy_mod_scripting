use std::collections::{HashMap, HashSet};

use cargo_metadata::{DependencyKind, Metadata, Package};
use itertools::{Either, Itertools};
use log::debug;

#[derive(Clone, Debug)]
pub enum FeatureEffect {
    /// A feature which enables another feature
    /// in a dependency
    /// if enable_optional is true, the dependency may not itself be enabled in which case the feature is not enabled
    EnableDepFeature {
        feature: String,
        dependency: String,
        enable_optional: bool,
    },

    /// A feature which enables another feature
    EnableFeature(String),

    /// A feature which enables an optional dependency
    EnableOptionalDep(String),
}

#[derive(Clone, Debug)]
pub struct Feature {
    name: String,
    effects: Vec<FeatureEffect>,
}

#[derive(Clone, Debug)]
pub struct Crate {
    pub name: String,
    pub features: Vec<Feature>,
    pub optional_dependencies: Vec<String>,
    pub other_dependencies: Vec<String>,
}

impl Eq for Crate {}
impl PartialEq for Crate {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}
impl std::hash::Hash for Crate {
    fn hash<H: std::hash::Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

pub enum Depdenency {
    Optional(String),
    Required(String),
}

#[derive(Debug)]
pub struct FeatureGraph {
    pub workspace_root: String,
    pub crates: Vec<Crate>,
}

impl FeatureGraph {
    /// Works out which dependencies are enabled by the given feature, including transitive relations between crates
    /// includes normal dependencies as well
    pub fn dependencies_for_features(
        &self,
        features: &[String],
        include_default: bool,
    ) -> Vec<&str> {
        // start off in workspace root
        let root = self
            .crates
            .iter()
            .find(|c| c.name == self.workspace_root)
            .unwrap();
        let mut buffer = Default::default();
        self.dependencies_for_features_on_crate(root, features, include_default, &mut buffer, 0);

        buffer.iter().map(|c| c.name.as_str()).collect()
    }

    fn dependencies_for_features_on_crate<'a>(
        &'a self,
        crate_: &'a Crate,
        features: &[String],
        include_default: bool,
        buffer: &mut HashSet<&'a Crate>,
        depth: usize,
    ) {
        let log_prefix = "|".to_owned() + &"-".repeat(depth);
        debug!(
            "{log_prefix}Processing dependencies for crate: `{}` with features: {}",
            crate_.name,
            features.join(", ")
        );
        if depth > 30 {
            panic!("Recursion depth exceeded");
        }

        let active_features = features
            .iter()
            .map(|f| {
                crate_
                    .features
                    .iter()
                    .find(|cf| cf.name == *f || (include_default && cf.name == "default"))
                    .unwrap_or_else(|| panic!("Feature '{}' not found in crate {}", f, crate_.name))
            })
            .collect::<Vec<_>>();

        let effects = Self::normalize_features(active_features.as_slice(), &crate_.features);

        // find which dependencies are brought in and with which features
        let mut deps: HashMap<&Crate, Vec<String>> = Default::default();
        for e in &effects {
            match e {
                FeatureEffect::EnableDepFeature {
                    feature,
                    dependency,
                    enable_optional,
                } => {
                    // we ignore optional dependencies's features, is this what we want to do?
                    if *enable_optional {
                        deps.entry(self.crates.iter().find(|c| c.name == *dependency).unwrap())
                            .or_default()
                            .push(feature.to_owned());
                    }
                }
                FeatureEffect::EnableOptionalDep(d) => {
                    _ = deps
                        .entry(self.crates.iter().find(|c| c.name == *d).unwrap())
                        .or_default()
                }
                _ => unreachable!(),
            };
        }

        deps.extend(crate_.other_dependencies.iter().filter_map(|d| {
            self.crates
                .iter()
                .find(|c| c.name == *d)
                .map(|c| (c, vec![]))
        }));

        // repeat for all dependencies recursively
        // we also ignore optional dependencies here, again is this what we want to do? I can't remember
        for (dep, features) in deps.iter() {
            debug!(
                "{log_prefix}Adding dependency: {} with features {:?}",
                dep.name, features
            );
            buffer.insert(dep);
            self.dependencies_for_features_on_crate(
                dep,
                features,
                include_default,
                buffer,
                depth + 1,
            );
        }

        debug!(
            "{log_prefix}Finished processing dependencies for crate: `{}`",
            crate_.name
        );
    }

    /// "flattens" feature effects to a list of effects based on the selected active features, features which enable other features are expanded until
    /// only dependencies and "pure" features are left or those which enable dependency features
    fn normalize_features(
        active_features: &[&Feature],
        features: &[Feature],
    ) -> Vec<FeatureEffect> {
        let mut stack = active_features.to_vec();
        let mut result = Vec::new();

        // guaranteed to be a DAG so no need to worry about cycles
        // at least I believe so
        while let Some(feature) = stack.pop() {
            for effect in &feature.effects {
                match effect {
                    FeatureEffect::EnableFeature(name) => {
                        stack.push(
                            features
                                .iter()
                                .chain(features.iter())
                                .find(|f| f.name == *name)
                                .unwrap(),
                        );
                    }
                    _ => result.push(effect.clone()),
                }
            }
        }
        result
    }

    pub fn from_metadata(meta: &Metadata, workspace_root: &str) -> Self {
        let crates = meta
            .workspace_packages()
            .iter()
            .map(|p| Self::process_crate(p))
            .collect::<Vec<_>>();

        // log the workspace crate dependencies
        debug!(
            "Workspace root: {}, Crates: {}",
            workspace_root,
            crates.iter().map(|c| format!("{c:?}")).join("\n\n ")
        );

        Self {
            workspace_root: workspace_root.to_owned(),
            crates,
        }
    }

    fn process_crate(meta: &Package) -> Crate {
        let (optional_dependencies, other_dependencies) = meta
            .dependencies
            .iter()
            .filter(|d| d.kind == DependencyKind::Normal) // dev dependencies can introduce weird cycles, and we don't care about them anyway
            .map(|f| (f.name.clone(), f.optional))
            .partition_map(|(name, opt)| {
                if opt {
                    Either::Left(name)
                } else {
                    Either::Right(name)
                }
            });

        let features = meta
            .features
            .iter()
            .map(|(name, effects)| {
                let effects = effects
                    .iter()
                    .map(|effect| {
                        if let Some(name) = effect.strip_prefix("dep:") {
                            FeatureEffect::EnableOptionalDep(name.to_string())
                        } else {
                            let parts = effect.split('/').collect::<Vec<_>>();
                            if parts.len() > 2 {
                                panic!("Invalid feature effect: {}", effect);
                            } else if parts.len() == 1 {
                                return FeatureEffect::EnableFeature(parts[0].to_owned());
                            } else {
                                return FeatureEffect::EnableDepFeature {
                                    feature: parts[1].to_owned(),
                                    dependency: parts[0]
                                        .strip_suffix('?')
                                        .unwrap_or(parts[0])
                                        .to_owned(),
                                    enable_optional: !parts[0].ends_with('?'),
                                };
                            }
                        }
                    })
                    .collect();
                Feature {
                    name: name.clone(),
                    effects,
                }
            })
            .collect();
        Crate {
            name: meta.name.clone(),
            features,
            optional_dependencies,
            other_dependencies,
        }
    }
}
