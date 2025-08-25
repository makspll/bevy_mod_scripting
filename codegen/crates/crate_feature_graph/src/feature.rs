use std::{
    borrow::{Borrow, Cow},
    collections::VecDeque,
    fmt::Display,
};

use cargo_metadata::{
    Metadata, Package,
    semver::{Version, VersionReq},
};
use indexmap::{IndexMap, IndexSet};
use itertools::{Either, Itertools};
use log::error;

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CrateName(pub(crate) String);

impl<'a, T: Borrow<Package>> From<&'a T> for CrateName {
    fn from(pkg: &'a T) -> Self {
        CrateName(pkg.borrow().name.clone())
    }
}

impl CrateName {
    pub fn new(name: impl Into<String>) -> Self {
        Self(name.into())
    }
}

impl Display for CrateName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

/// A feature name
#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FeatureName(Cow<'static, str>);

impl FeatureName {
    pub const DEFAULT_FEATURE: FeatureName = FeatureName(Cow::Borrowed("default"));

    pub fn feature_name(&self) -> &str {
        &self.0
    }

    pub fn new(name: impl Into<Cow<'static, str>>) -> Self {
        Self(name.into())
    }
}

impl Display for FeatureName {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub enum FeatureEffect {
    /// I.e. `foo=["feature"]` is represented as `EnableFeature("feature")`
    EnableFeature(FeatureName),
    /// I.e. `foo=["dep:optional"]` is represented as `EnableOptionalDependency("optional")`
    EnableOptionalDependency(CrateName),
    /// I.e. `foo=["optional(?)/feature"]` is represented as `EnableFeatureInDependency("feature", "optional")`
    /// this effect by itself does not enable the dependency, only the feature in it unless `enables_optionals` is true
    EnableFeatureInDependency(FeatureName, CrateName, bool),
}

impl FeatureEffect {
    /// Returns true if this effect enables the given crate when the feature is enabled
    pub fn enables_crate(&self, crate_name: &CrateName) -> bool {
        match self {
            FeatureEffect::EnableOptionalDependency(c) if c == crate_name => true,
            FeatureEffect::EnableFeatureInDependency(_, c, enables_optionals)
                if c == crate_name =>
            {
                *enables_optionals
            }
            _ => false,
        }
    }

    /// Returns Some(feature) if this effect enables the given feature in the given crate when the feature is enabled
    pub fn enables_feature_in_crate(&self, crate_name: &CrateName) -> Option<&FeatureName> {
        match self {
            FeatureEffect::EnableFeatureInDependency(f, c, _) if c == crate_name => Some(f),
            _ => None,
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Feature {
    pub name: FeatureName,
    pub effects: Vec<FeatureEffect>,
    pub enabled_by_trail: VecDeque<(CrateName, FeatureName)>, // (crate, feature) that enables this feature, if any
}

impl Feature {
    pub fn new_enabling_default_for(enables_default_in: CrateName) -> Self {
        Self {
            name: FeatureName(format!("enable_default_for_{enables_default_in}").into()),
            effects: vec![FeatureEffect::EnableFeatureInDependency(
                FeatureName::DEFAULT_FEATURE,
                enables_default_in,
                false,
            )],
            enabled_by_trail: VecDeque::new(),
        }
    }

    pub fn effects(&self) -> impl Iterator<Item = &FeatureEffect> {
        self.effects.iter()
    }

    pub fn parse(name: String, features: Vec<String>) -> Self {
        let effects = features.into_iter().map(|feature| {
            if let Some(crate_name) = feature.strip_prefix("dep:") {
                return vec![FeatureEffect::EnableOptionalDependency(CrateName(
                    crate_name.to_string(),
                ))];
            }
            // leftover patterns are: "feature", "optional?/feature", "non_optional/default"
            let mut chars = feature.chars();
            let part_before_slash = chars
                .by_ref()
                .peeking_take_while(|&c| c != '/')
                .collect::<String>();

            let (optional, crate_name) =
                if let Some(crate_name) = part_before_slash.strip_suffix('?') {
                    (true, crate_name)
                } else {
                    (false, part_before_slash.as_str())
                };

            if let Some(_slash) = chars.next() {
                // slash present
                let feature_name = chars.collect::<String>();
                if optional {
                    vec![FeatureEffect::EnableFeatureInDependency(
                        FeatureName(feature_name.into()),
                        CrateName(crate_name.to_string()),
                        false,
                    )]
                } else {
                    vec![
                        FeatureEffect::EnableOptionalDependency(CrateName(crate_name.to_string())),
                        FeatureEffect::EnableFeatureInDependency(
                            FeatureName(feature_name.into()),
                            CrateName(crate_name.to_string()),
                            true,
                        ),
                    ]
                }
            } else {
                if optional {
                    // no slash, but optional marker present -> invalid
                    // but we can parse ignoring it, let the user know though
                    error!("Invalid feature specification: {feature}. Continuing...");
                }

                // no slash
                vec![FeatureEffect::EnableFeature(FeatureName(
                    part_before_slash.clone().into(),
                ))]
            }
        });

        Self {
            name: FeatureName(name.into()),
            effects: effects.flatten().collect(),
            enabled_by_trail: VecDeque::new(),
        }
    }
}

/// Describes a dependency relationship
#[derive(Clone, Debug, PartialEq, Eq, Hash)]
pub enum DependencyKind {
    Optional(CrateName),
    NonOptional(CrateName),
}

impl DependencyKind {
    pub fn crate_name(&self) -> &CrateName {
        match self {
            DependencyKind::Optional(name) => name,
            DependencyKind::NonOptional(name) => name,
        }
    }

    pub fn is_optional(&self) -> bool {
        matches!(self, DependencyKind::Optional(_))
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct CrateDependency {
    pub name: CrateName,
    pub version: VersionReq,
}

impl PartialOrd for CrateDependency {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for CrateDependency {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Crate {
    pub name: CrateName,
    pub features: Vec<Feature>,
    pub dependencies: Vec<CrateDependency>,
    pub optional_dependencies: Vec<CrateDependency>,
    pub version: Version,
    pub in_workspace: Option<bool>,
    pub active_features: Option<IndexSet<FeatureName>>,
    pub active_dependency_features: Option<IndexMap<CrateName, Vec<FeatureName>>>,
    pub is_enabled: Option<bool>,
}

impl PartialOrd for Crate {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Crate {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.name.cmp(&other.name)
    }
}

struct DependencyIter<'a> {
    krate: &'a Crate,
    index: usize,
}

impl<'a> Iterator for DependencyIter<'a> {
    type Item = DependencyKind;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.krate.dependencies.len() {
            let dep = &self.krate.dependencies[self.index];
            self.index += 1;
            Some(DependencyKind::NonOptional(dep.name.clone()))
        } else if self.index
            < self.krate.dependencies.len() + self.krate.optional_dependencies.len()
        {
            let opt_dep =
                &self.krate.optional_dependencies[self.index - self.krate.dependencies.len()];
            self.index += 1;
            Some(DependencyKind::Optional(opt_dep.name.clone()))
        } else {
            None
        }
    }
}
impl Crate {
    pub fn dependencies(&self) -> impl Iterator<Item = DependencyKind> {
        DependencyIter {
            krate: self,
            index: 0,
        }
    }

    pub fn mark_enabled_by_if_exists(
        &mut self,
        enabled: FeatureName,
        by_crate: CrateName,
        by_feature: FeatureName,
    ) {
        if let Some(feat) = self.features.iter_mut().find(|f| f.name == enabled) {
            feat.enabled_by_trail.push_back((by_crate, by_feature));
        }
    }

    /// Checks if a dependency of this crate is enabled via one of its active features
    /// The active features must be computed first
    pub fn optional_dependency_is_enabled(&self, dep: &CrateName) -> bool {
        self.optional_dependencies.iter().any(|d| {
            &d.name == dep
                && self.active_features.as_ref().is_some_and(|f| {
                    f.iter().any(|f| {
                        self.features.iter().any(|feat| {
                            feat.name == *f && feat.effects().any(|e| e.enables_crate(dep))
                        })
                    })
                })
        })
    }

    /// processes the feature set, given the initially enabled features, and returns the full set of enabled features.
    /// if `enable_default` is true, the "default" feature is considered enabled initially if it exists.
    pub fn compute_active_features(
        &mut self,
        enabled_features: &IndexSet<FeatureName>,
        keep_trails: bool,
    ) {
        let mut all_features = IndexSet::default();
        let mut to_process = enabled_features
            .clone()
            .into_iter()
            .collect::<VecDeque<_>>();

        log::trace!(
            "Computing active features for crate `{}` with initial features: {:?}",
            self.name,
            enabled_features
        );

        // enable all enable_default_for_{crate} features
        // as those only exist when --default-features is true for the dependency crate
        for feature in self
            .features
            .iter()
            .filter(|f| f.name.0.starts_with("enable_default_for_"))
        {
            to_process.push_back(feature.name.clone());
        }

        while let Some(next) = to_process.pop_front() {
            if let Some(feature) = self.features.iter().find(|f| f.name == next).cloned() {
                for effect in feature.effects() {
                    let enables_feature = match effect {
                        FeatureEffect::EnableFeature(f) => f,
                        FeatureEffect::EnableFeatureInDependency(f, d, _) if d == &self.name => f,
                        _ => continue,
                    };

                    log::trace!(
                        "Crate `{}`: feature `{}` enables feature `{}`",
                        self.name,
                        next,
                        enables_feature
                    );

                    if all_features.insert(enables_feature.clone()) {
                        to_process.push_back(enables_feature.clone());
                        if keep_trails {
                            self.mark_enabled_by_if_exists(
                                enables_feature.clone(),
                                self.name.clone(),
                                next.clone(),
                            );
                        }
                    }
                }
                all_features.insert(next);
            } else if next.feature_name() == "default" {
                // for default do still include the feature even if it doesn't exist
                all_features.insert(next);
            } else {
                // error!(
                //     "Crate `{}` does not have feature `{}`. Are you using the right features?",
                //     self.name, next
                // );
                // too noisy for now, figure out what's going on TODO
            }
        }

        if let Some(active_features) = &self.active_features {
            // merge, features are always additive in a workspace
            if self.in_workspace.is_some_and(|a| a) {
                log::trace!(
                    "Merging active features for crate `{}`, as new path is being computed: {:?} + {:?}",
                    self.name,
                    active_features,
                    all_features
                );
            }
            self.active_features = Some(active_features.union(&all_features).cloned().collect());
        } else {
            self.active_features = Some(all_features);
        }
    }
}
impl From<Package> for Crate {
    fn from(meta: Package) -> Self {
        let (optional_dependencies, dependencies): (Vec<_>, Vec<_>) = meta
            .dependencies
            .iter()
            .filter(|d| d.kind == cargo_metadata::DependencyKind::Normal) // dev dependencies can introduce weird cycles, and we don't care about them anyway
            .map(|f| {
                (
                    f.name.clone(),
                    f.optional,
                    f.uses_default_features,
                    f.req.clone(),
                )
            })
            .partition_map(|(name, opt, enable_default, req)| {
                if opt {
                    Either::Left((CrateName(name), enable_default, req))
                } else {
                    Either::Right((CrateName(name), enable_default, req))
                }
            });

        let mut features = meta
            .features
            .iter()
            .map(|(name, effects)| Feature::parse(name.clone(), effects.clone()))
            .collect::<Vec<_>>();

        // for each dependency that enables default features, we add a feature named after the dependency that enables the "default" feature
        for (dep_name, enable_default, _) in dependencies.iter().chain(optional_dependencies.iter())
        {
            if *enable_default {
                features.push(Feature::new_enabling_default_for(dep_name.clone()));
            }
        }

        Self {
            name: CrateName(meta.name.clone()),
            features,
            dependencies: dependencies
                .into_iter()
                .map(|(n, _, req)| CrateDependency {
                    name: n,
                    version: req,
                })
                .collect(),
            optional_dependencies: optional_dependencies
                .into_iter()
                .map(|(n, _, req)| CrateDependency {
                    name: n,
                    version: req,
                })
                .collect(),
            version: meta.version,
            in_workspace: Some(true),
            active_features: None,
            active_dependency_features: None,
            is_enabled: None,
        }
    }
}

#[derive(Clone, Debug, Default)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct Workspace {
    pub workspace_crates: Vec<Crate>,
    pub other_crates: Vec<Crate>,
    pub root: Option<CrateName>,
}

pub struct WorkspaceIter<'a> {
    workspace: &'a Workspace,
    index: usize,
}

pub struct WorkspaceIterMut<'a> {
    workspace: &'a mut Workspace,
    index: usize,
}

impl<'a> Iterator for WorkspaceIter<'a> {
    type Item = &'a Crate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.workspace.workspace_crates.len() {
            let krate = &self.workspace.workspace_crates[self.index];
            self.index += 1;
            Some(krate)
        } else if self.index
            < self.workspace.workspace_crates.len() + self.workspace.other_crates.len()
        {
            let krate =
                &self.workspace.other_crates[self.index - self.workspace.workspace_crates.len()];
            self.index += 1;
            Some(krate)
        } else {
            None
        }
    }
}

impl<'a> Iterator for WorkspaceIterMut<'a> {
    type Item = &'a mut Crate;

    fn next(&mut self) -> Option<Self::Item> {
        if self.index < self.workspace.workspace_crates.len() {
            let krate_ptr = &mut self.workspace.workspace_crates[self.index] as *mut Crate;
            self.index += 1;
            // SAFETY: we ensure that we only yield one mutable reference at a time, so this is safe
            Some(unsafe { &mut *krate_ptr })
        } else if self.index
            < self.workspace.workspace_crates.len() + self.workspace.other_crates.len()
        {
            let krate = &mut self.workspace.other_crates
                [self.index - self.workspace.workspace_crates.len()]
                as *mut Crate;
            self.index += 1;
            // SAFETY: we ensure that we only yield one mutable reference at a time, so this is safe
            Some(unsafe { &mut *krate })
        } else {
            None
        }
    }
}

impl Workspace {
    pub fn all_crates(&self) -> impl Iterator<Item = &Crate> {
        WorkspaceIter {
            workspace: self,
            index: 0,
        }
    }

    pub fn all_crates_mut(&mut self) -> impl Iterator<Item = &mut Crate> {
        WorkspaceIterMut {
            workspace: self,
            index: 0,
        }
    }

    pub fn find_crate<L: Fn() -> String>(
        &self,
        name: &CrateName,
        log_if_missing: L,
    ) -> Option<&Crate> {
        self.all_crates().find(|c| &c.name == name).or_else(|| {
            error!("{}", log_if_missing());
            None
        })
    }

    pub fn find_crate_opt(&self, name: &CrateName) -> Option<&Crate> {
        self.all_crates().find(|c| &c.name == name).or(None)
    }

    pub fn find_crate_mut<L: Fn() -> String>(
        &mut self,
        name: &CrateName,
        log_if_missing: L,
    ) -> Option<&mut Crate> {
        self.all_crates_mut().find(|c| &c.name == name).or_else(|| {
            error!("{}", log_if_missing());
            None
        })
    }
}

impl From<&Metadata> for Workspace {
    fn from(meta: &Metadata) -> Self {
        let workspace_packages = meta
            .workspace_packages()
            .iter()
            .map(CrateName::from)
            .collect::<IndexSet<_>>();

        let (mut workspace_crates, mut other_crates): (Vec<_>, Vec<_>) = meta
            .packages
            .iter()
            .map(|p| {
                (
                    Crate::from(p.clone()),
                    workspace_packages.contains(&CrateName::from(p)),
                )
            })
            .partition_map(|(p, is_workspace_member)| {
                if is_workspace_member {
                    Either::Left(p)
                } else {
                    Either::Right(p)
                }
            });

        for krate in workspace_crates.iter_mut() {
            krate.in_workspace = Some(true);
        }
        for krate in other_crates.iter_mut() {
            krate.in_workspace = Some(false);
        }

        let root = meta.workspace_packages().iter().find_map(|p| {
            if meta.workspace_root.join("Cargo.toml") == p.manifest_path {
                Some(CrateName::from(p))
            } else {
                None
            }
        });
        Self {
            workspace_crates,
            other_crates,
            root,
        }
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_feature_parsing() {
        assert_eq!(
            Feature::parse(
                "feature".to_string(),
                vec![
                    String::from("a"),
                    String::from("dep:b"),
                    String::from("c?/c_feature"),
                    String::from("d/d_feature"),
                ]
            ),
            Feature {
                name: FeatureName("feature".into()),
                effects: vec![
                    FeatureEffect::EnableFeature(FeatureName("a".into())),
                    FeatureEffect::EnableOptionalDependency(CrateName("b".into())),
                    FeatureEffect::EnableFeatureInDependency(
                        FeatureName("c_feature".into()),
                        CrateName("c".into()),
                        false
                    ),
                    FeatureEffect::EnableOptionalDependency(CrateName("d".into())),
                    FeatureEffect::EnableFeatureInDependency(
                        FeatureName("d_feature".into()),
                        CrateName("d".into()),
                        true
                    ),
                ],
                enabled_by_trail: VecDeque::new(),
            }
        );
    }

    #[test]
    fn test_compute_active_features() {
        let mut krate = Crate {
            name: CrateName("my_crate".into()),
            features: vec![
                Feature::parse(
                    "default".into(),
                    vec![String::from("feat_a"), String::from("feat_b")],
                ),
                Feature::parse("feat_a".into(), vec![String::from("feat_c")]),
                Feature::parse("feat_b".into(), vec![]),
                Feature::parse("feat_c".into(), vec![]),
            ],
            dependencies: vec![],
            optional_dependencies: vec![],
            version: Version::new(0, 1, 0),
            in_workspace: Some(true),
            active_features: None,
            is_enabled: None,
            active_dependency_features: None,
        };

        krate.compute_active_features(&IndexSet::from([FeatureName("default".into())]), true);

        let active_features = krate.active_features.unwrap();
        assert!(active_features.contains(&FeatureName("default".into())));
        assert!(active_features.contains(&FeatureName("feat_a".into())));
        assert!(active_features.contains(&FeatureName("feat_b".into())));
        assert!(active_features.contains(&FeatureName("feat_c".into())));
        assert_eq!(active_features.len(), 4);

        // verify trail
        let feat_a = krate
            .features
            .iter()
            .find(|f| f.name == FeatureName("feat_a".into()))
            .unwrap();
        assert_eq!(feat_a.enabled_by_trail.len(), 1);
        assert_eq!(
            feat_a.enabled_by_trail[0],
            (CrateName("my_crate".into()), FeatureName("default".into()))
        );
    }

    #[test]
    fn test_workspace_iteration() {
        let mut workspace = Workspace {
            workspace_crates: vec![
                Crate {
                    name: CrateName("crate_a".into()),
                    features: vec![],
                    dependencies: vec![],
                    optional_dependencies: vec![],
                    version: Version::new(0, 1, 0),
                    in_workspace: Some(true),
                    active_features: None,
                    is_enabled: None,
                    active_dependency_features: None,
                },
                Crate {
                    name: CrateName("crate_b".into()),
                    features: vec![],
                    dependencies: vec![],
                    optional_dependencies: vec![],
                    version: Version::new(0, 1, 0),
                    in_workspace: Some(true),
                    active_features: None,
                    is_enabled: None,
                    active_dependency_features: None,
                },
            ],
            other_crates: vec![Crate {
                name: CrateName("crate_c".into()),
                features: vec![],
                dependencies: vec![],
                optional_dependencies: vec![],
                version: Version::new(0, 1, 0),
                in_workspace: Some(false),
                active_features: None,
                is_enabled: None,
                active_dependency_features: None,
            }],
            root: None,
        };

        let mut iter = workspace.all_crates_mut();
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        let third = iter.next().unwrap();
        assert_eq!(first.name.0, "crate_a");
        assert_eq!(second.name.0, "crate_b");
        assert_eq!(third.name.0, "crate_c");
        assert!(iter.next().is_none());
        drop(iter);

        let mut iter = workspace.all_crates();
        let first = iter.next().unwrap();
        let second = iter.next().unwrap();
        let third = iter.next().unwrap();
        assert_eq!(first.name.0, "crate_a");
        assert_eq!(second.name.0, "crate_b");
        assert_eq!(third.name.0, "crate_c");
        assert!(iter.next().is_none());
    }
}
