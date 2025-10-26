use std::collections::{HashMap, HashSet, VecDeque};

use indexmap::{IndexMap, IndexSet};
use log::error;
use petgraph::Directed;

use crate::{CrateName, DependencyKind, Feature, FeatureName, Workspace};

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DependsOn {
    Optionally,
    Unconditionally,
}

pub enum Node {
    WorkspaceCrate(CrateName),
    OtherCrate(CrateName),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum EnablesDependencyIf {
    Always,
    IfFeature(CrateName, FeatureName),
}

trait GraphExtensions<N, E, Directed> {
    fn find_node_by<F: Fn(&N) -> bool, L: Fn() -> String>(
        &self,
        find: F,
        log_if_missing: L,
    ) -> Option<petgraph::graph::NodeIndex>;

    fn find_node_by_opt<F: Fn(&N) -> bool>(&self, find: F) -> Option<petgraph::graph::NodeIndex>;
}
impl<N: PartialEq, E, Directed> GraphExtensions<N, E, Directed> for petgraph::Graph<N, E, Directed>
where
    Directed: petgraph::EdgeType,
{
    fn find_node_by<F: Fn(&N) -> bool, L: Fn() -> String>(
        &self,
        find: F,
        log_if_missing: L,
    ) -> Option<petgraph::graph::NodeIndex> {
        match self.node_indices().find(|i| find(&self[*i])) {
            Some(index) => Some(index),
            None => {
                error!("{}", log_if_missing());
                None
            }
        }
    }

    fn find_node_by_opt<F: Fn(&N) -> bool>(&self, find: F) -> Option<petgraph::graph::NodeIndex> {
        self.node_indices().find(|i| find(&self[*i]))
    }
}
#[derive(Default)]
pub struct WorkspaceGraph {
    pub dependency_conditions_graph: petgraph::Graph<CrateName, EnablesDependencyIf, Directed>,
    pub workspace: Workspace,
    pub input: HashMap<CrateName, (Vec<FeatureName>, bool)>,
}

impl WorkspaceGraph {
    #[cfg(feature = "serde")]
    /// Serializes the graph to a file
    pub fn serialize(self, path: &std::path::Path) -> std::io::Result<()> {
        // make dirs
        if let Some(parent) = path.parent() {
            std::fs::create_dir_all(parent)?
        }
        let serialized =
            serde_json::to_string_pretty(&self.workspace).map_err(std::io::Error::other)?;
        let input = serde_json::to_string_pretty(&self.input).map_err(std::io::Error::other)?;

        let concatenated = serialized + "\n\n" + &input;

        std::fs::write(path, concatenated)
    }

    #[cfg(feature = "serde")]
    /// Deserializes the graph from a file, the graph will be reconstructed
    pub fn deserialize(path: &std::path::Path) -> std::io::Result<Self> {
        let data = std::fs::read_to_string(path)?;
        let parts: Vec<&str> = data.split("\n\n").collect();
        let workspace: Workspace = serde_json::from_str(parts[0]).map_err(std::io::Error::other)?;
        let input: HashMap<CrateName, (Vec<FeatureName>, bool)> =
            serde_json::from_str(parts[1]).map_err(std::io::Error::other)?;

        let mut desered = Self {
            input,
            workspace,
            dependency_conditions_graph: petgraph::Graph::new(),
        };

        desered.calculate_enabled_features_and_dependencies(desered.input.clone(), false);

        Ok(desered)
    }

    /// Get all crates in the workspace which are enabled
    /// Should be called after `calculate_enabled_features_and_dependencies` or will return an empty list
    pub fn all_enabled_workspace_crates(&self) -> Vec<&CrateName> {
        self.workspace
            .workspace_crates
            .iter()
            .filter(|c| c.is_enabled.unwrap_or(false))
            .map(|c| &c.name)
            .collect()
    }

    #[cfg(feature = "dot_parser")]
    pub fn to_dot(&self) -> String {
        format!(
            "{:?}",
            petgraph::dot::Dot::with_config(&self.dependency_conditions_graph, &[])
        )
    }

    #[cfg(feature = "dot_parser")]
    /// Generates a dot file visualising how the feature flags flow through the workspace
    /// showing the feature effects between dependencies, and showing which are enabled using green and gray colours
    pub fn visualise_feature_flow(&self, only_show_crates: Vec<String>) -> std::io::Result<String> {
        use petgraph::visit::EdgeRef;

        let mut visualiser_graph = petgraph::Graph::<String, String, Directed>::new();

        let mut already_added_crates = HashSet::<CrateName>::new();
        let mut already_added_edges = HashSet::<String>::new();
        // the nodes are crates as well as dependencies
        for krate in self.workspace.all_crates() {
            let krate_with_attrs = if krate.is_enabled.unwrap_or(false) {
                format!("{} (enabled)", krate.name)
            } else {
                format!("{} (disabled)", krate.name)
            };
            if already_added_crates.insert(krate.name.clone()) {
                visualiser_graph.add_node(krate_with_attrs);
            }
        }

        for dependency in self.workspace.all_crates().flat_map(|c| c.dependencies()) {
            let krate = dependency.crate_name();
            if already_added_crates.insert(krate.clone()) {
                visualiser_graph.add_node(krate.to_string());
            }
        }

        // now add edges, we connect each feature effect to the neighbours
        for krate in self.workspace.all_crates() {
            let from_node =
                match visualiser_graph.find_node_by_opt(|n| n.contains(&krate.name.to_string())) {
                    Some(n) => n,
                    None => continue,
                };

            for feature in &krate.features {
                let is_active = krate
                    .active_features
                    .as_ref()
                    .is_some_and(|f| f.contains(&feature.name));

                // to make the graph less cluttered
                if !is_active {
                    continue;
                }

                for effect in feature.effects() {
                    use crate::FeatureEffect;
                    let effect_descriptor = match effect {
                        FeatureEffect::EnableFeature(feature_name) => {
                            format!("enables feature `{feature_name}`")
                        }
                        FeatureEffect::EnableOptionalDependency(crate_name) => {
                            format!("enables optional dependency `{crate_name}`")
                        }
                        FeatureEffect::EnableFeatureInDependency(
                            feature_name,
                            crate_name,
                            enables_optionals,
                        ) => {
                            let connector = if *enables_optionals {
                                "and"
                            } else {
                                "if enabled"
                            };
                            format!(
                                "enables feature `{feature_name}` {connector} dependency `{crate_name}`"
                            )
                        }
                    };
                    let label = if is_active {
                        format!("{effect_descriptor} (enabled)")
                    } else {
                        format!("{effect_descriptor} (disabled)")
                    };
                    let (from, to) = match effect {
                        FeatureEffect::EnableFeature(_) => {
                            continue; // this makes the graph too cluttered
                            // (from_node, from_node)
                        }
                        FeatureEffect::EnableOptionalDependency(crate_name) => {
                            let to_node = match visualiser_graph
                                .find_node_by_opt(|n| n.contains(&crate_name.to_string()))
                            {
                                Some(n) => n,
                                None => continue,
                            };
                            (from_node, to_node)
                        }
                        FeatureEffect::EnableFeatureInDependency(_, crate_name, _) => {
                            let to_node = match visualiser_graph
                                .find_node_by_opt(|n| n == &crate_name.to_string())
                            {
                                Some(n) => n,
                                None => continue,
                            };
                            (from_node, to_node)
                        }
                    };
                    if already_added_edges.insert(label.clone()) {
                        visualiser_graph.add_edge(from, to, label);
                    }
                }
            }
        }

        // finally filter out nodes we don't want to show but connect edges
        if !only_show_crates.is_empty() {
            log::info!("Filtering to only show crates: {only_show_crates:?}");
            let only_show_crates: HashSet<_> = only_show_crates
                .into_iter()
                .map(|s| s.to_string())
                .collect();
            let to_remove: Vec<_> = already_added_crates
                .into_iter()
                .filter(|c| !only_show_crates.contains(&c.to_string()))
                .collect();

            log::info!(
                "Removing and interconnecting {} nodes, out of {}",
                to_remove.len(),
                visualiser_graph.node_count()
            );

            for n in to_remove {
                log::info!("Removing node {n}");
                let node_index =
                    match visualiser_graph.find_node_by_opt(|node| node.contains(&n.to_string())) {
                        Some(n) => n,
                        None => continue,
                    };

                // outgoing and incoming edges need to be reconnected to all neighbours correctly
                let outgoing_edges = visualiser_graph
                    .edges_directed(node_index, petgraph::Direction::Outgoing)
                    .map(|e| (e.target(), e.weight().clone()))
                    .collect::<Vec<_>>();
                let incoming_edges = visualiser_graph
                    .edges_directed(node_index, petgraph::Direction::Incoming)
                    .map(|e| (e.source(), e.weight().clone()))
                    .collect::<Vec<_>>();

                for (source, incoming) in &incoming_edges {
                    for (target, outgoing) in &outgoing_edges {
                        if source != target {
                            // compute the edge label, we concatenate with a comma if the edge already
                            // got merged earlier, we want to extract the "middle" edges, which contain crate
                            // tracing
                            let mut new_label = format!("{incoming},{outgoing}");
                            let parts = new_label.split(',').collect::<Vec<&str>>();
                            if parts.len() > 2 {
                                // collapse the middle parts into a "number"
                                new_label = parts.first().unwrap().to_string()
                                    + &format!("-> {} crates ->", parts.len() - 2)
                                    + parts.last().unwrap();
                            }
                            visualiser_graph.add_edge(*source, *target, new_label);
                        }
                    }
                }

                // remove the node and edges

                visualiser_graph.retain_edges(|a, e| {
                    let (source, target) = a.edge_endpoints(e).unwrap();
                    source != node_index && target != node_index
                });
                visualiser_graph.remove_node(node_index);
            }
        }

        // visualise
        let dot = petgraph::dot::Dot::with_attr_getters(
            &visualiser_graph,
            &[],
            &|_, e| {
                // if contains (enabled) make green, else gray
                if e.weight().contains("(enabled)") {
                    "color=green"
                } else {
                    "color=gray"
                }
                .to_string()
            },
            &|_, (_, node)| {
                let color = if node.contains("(enabled)") {
                    "color=green"
                } else {
                    "color=gray"
                }
                .to_string();

                // active features
                let matching_crate = self
                    .workspace
                    .find_crate_opt(&CrateName(node.split(' ').next().unwrap_or("").into()));

                let (active_features, active_dependencies) = match matching_crate {
                    Some(krate) => (
                        krate
                            .active_features
                            .as_ref()
                            .map(|f| f.iter().map(|f| f.to_string()).collect())
                            .unwrap_or_default(),
                        krate
                            .active_dependency_features
                            .as_ref()
                            .map(|m| {
                                m.iter()
                                    .map(|(k, v)| {
                                        (
                                            k.to_string(),
                                            v.iter().map(|f| f.to_string()).collect::<Vec<_>>(),
                                        )
                                    })
                                    .collect()
                            })
                            .unwrap_or_default(),
                    ),
                    None => (vec![], HashMap::new()),
                };
                let tooltip = format!(
                    "Active features: [\\n{}\\n]\\n\\nActive dependency features: [\\n{}\\n]",
                    active_features.join("\\n"),
                    active_dependencies
                        .iter()
                        .map(|(k, v)| format!("{}: [{}]", k, v.join(", ")))
                        .collect::<Vec<_>>()
                        .join("\\n")
                );

                format!("{color}, tooltip=\"{tooltip}\"")
            },
        )
        .to_string();
        Ok(dot)
    }

    /// Equivalent to [`crate::calculate_enabled_features_and_dependencies_parse`] but
    /// additionally parses features as if coming from the command line,
    ///
    /// the normal syntax applies with the small exception that "default", features get converted to  "enable-default-features"
    pub fn calculate_enabled_features_and_dependencies_parse(
        &mut self,
        features: Vec<String>,
        root: Option<CrateName>,
    ) {
        // parse both the crate if the feature contains a slash
        // and the feature name
        // extract "default" features from the list and convert to bool instantiations
        let mut features_map = HashMap::<CrateName, (Vec<FeatureName>, bool)>::new();

        for feature in features {
            let parts: Vec<&str> = feature.split('/').collect();
            let len_parts = parts.len();
            let mut parts = parts.into_iter();
            let crate_name = if len_parts > 1
                && let Some(part) = parts.next()
            {
                CrateName(part.to_string())
            } else {
                root.as_ref().cloned().unwrap_or_else(|| {
                    self.workspace
                        .root
                        .as_ref()
                        .expect("Workspace must contain a root crate")
                        .clone()
                })
            };

            let feature = parts
                .next()
                .map(|s| s.to_string())
                .expect("malformed feature");

            let feature_entry = features_map.entry(crate_name).or_default();
            if feature == "default" {
                feature_entry.1 = true;
            } else {
                feature_entry.0.push(FeatureName::new(feature.clone()));
            }
        }

        log::info!("Parsed features map: {features_map:?}");
        self.calculate_enabled_features_and_dependencies(features_map, false);
    }

    pub fn stable_sort(&mut self) {
        // sort everything
        self.workspace.other_crates.sort();
        self.workspace.workspace_crates.sort();
        for krate in self.workspace.all_crates_mut() {
            krate.features.sort();
            krate.dependencies.sort_by(|a, b| a.name.cmp(&b.name));
            krate
                .optional_dependencies
                .sort_by(|a, b| a.name.cmp(&b.name));
            if let Some(f) = krate.active_features.as_mut() {
                f.sort()
            }
            if let Some(m) = krate.active_dependency_features.as_mut() {
                for v in m.values_mut() {
                    v.sort();
                }
            }
        }
    }

    /// Calculate the enabled features and dependencies for the workspace
    /// The set of enabled feature/crate pairs is treated as the set of explicitly enabled crates
    /// For a workspace this should be the workspace root with the desired features enabled..
    ///
    /// For --package, behaviour this should be just the specified package with the desired features enabled.
    ///
    /// The result will be written into the workspace crates
    pub fn calculate_enabled_features_and_dependencies(
        &mut self,
        mut enabled_crates: HashMap<CrateName, (Vec<FeatureName>, bool)>,
        trace_features: bool,
    ) {
        self.input = enabled_crates.clone();
        // now we process crates, one at a time
        // first process the explicitly enabled crates with features
        // we keep processing crates untill no features can be enabled
        let mut open_set = VecDeque::new();

        open_set.extend(enabled_crates.keys().cloned());

        log::info!("Starting feature calculation with open set: {open_set:?}");

        while let Some(krate) = open_set.pop_front() {
            log::trace!("Processing crate `{krate}`");
            // remove so if we reprocess it later, from a subsequent dependency, we can still
            // compute the full feature set
            match enabled_crates.remove(&krate) {
                Some((features, enable_default)) => {
                    // a top level enabled crate, process only explicitly enabled features, we know nothing else will be there to enable more
                    let features = IndexSet::from_iter(features.iter().cloned());
                    if enable_default
                        && let Some(c) = self.workspace.find_crate_mut(&krate, || format!(
                            "package from workspace manifest: `{krate}` was not found in the parsed workspace list. This might lead to missing default features."))
                    {
                        let hack = Feature::new_enabling_default_for(c.name.clone());
                        c.features.push(hack);
                    };
                    log::trace!(
                        "Crate `{krate}` is explicitly enabled with features: {features:?}, default: {enable_default}"
                    );
                    self.process_crate(&krate, features, trace_features);
                }
                None => {
                    let dependents = self.dependency_conditions_graph
                        .neighbors_directed(
                            self.dependency_conditions_graph.find_node_by(|n| n == &krate, || format!(
                                "package from workspace manifest: `{krate}` was not found in the graph. When calculating dependant crate features."
                            )).unwrap(),
                            petgraph::Direction::Incoming,
                        )
                        .map(|i| &self.dependency_conditions_graph[i]);

                    // enable default will be handled, via the features we inserted when loading the workspace.
                    let parent_features =
                        dependents
                            .filter_map(|d| self.workspace.find_crate(d, || format!(
                                "package from workspace manifest: `{d}` was not found in the parsed workspace list. When calculating dependant crate features, the crate was not found."
                            )))
                            .filter_map(|dependent| {
                                // if not computed, it's possible the crate is not included, in which case it won't provide any features
                                let active_features = match dependent.active_features.as_ref() {
                                    Some(f) => f,
                                    None => return None,
                                };
                                log::trace!("Considering features from dependant crate `{}`: {:?}", dependent.name, active_features);

                                let active_features = dependent.features.iter().filter(|f| active_features.contains(&f.name));

                                Some(active_features
                                    .filter_map(|f| f.effects().find_map(|e| e.enables_feature_in_crate(&krate).cloned())))
                            })
                            .flatten()
                            .collect::<IndexSet<_>>();

                    log::trace!(
                        "Crate `{krate}` is being enabled with parent features: {parent_features:?}"
                    );
                    self.process_crate(&krate, parent_features, trace_features);
                }
            };

            // now we need to add further dependencies to the open set
            let from_node = match self.dependency_conditions_graph.find_node_by(|n| n == &krate, || format!(
                        "package from workspace manifest: `{krate}` was not found in the graph. While processing the next open set item. It was missing from the workspace."
                    )) {
                         Some(n) => n, None => continue };

            let (dependencies, nodes): (Vec<_>, Vec<_>) = self
                .dependency_conditions_graph
                .neighbors_directed(from_node, petgraph::Direction::Outgoing)
                .map(|i| (&self.dependency_conditions_graph[i], i))
                .unzip();

            log::trace!("Considering extended dependencies: {dependencies:?}");
            for (i, &dependency) in dependencies.iter().enumerate() {
                let to_node = nodes[i];
                let edge = self
                    .dependency_conditions_graph
                    .find_edge(from_node, to_node)
                    .expect("edge must exist");

                let edge = &self.dependency_conditions_graph[edge];

                if let EnablesDependencyIf::IfFeature(required_crate, required_feature) = edge {
                    // check if the required feature is active
                    let enabled = self.workspace.find_crate(required_crate, || format!(
                        "package from workspace manifest: `{required_crate}` was not found in the parsed workspace list. While looking for required feature in enabling dependency."
                    )).map(|c| {
                        // again if not computed, it's possible the crate is not included, in which case we can't possibly enable this dependency
                        c.active_features.as_ref().is_some_and(|f| {
                            f.contains(required_feature)
                        })
                    }).unwrap_or(false);

                    if !enabled {
                        log::trace!(
                            "Not enabling dependency `{dependency}` from `{krate}` because it requires feature `{required_feature}` in crate `{required_crate}` which is not enabled"
                        );
                        continue;
                    }
                };

                // add this to the open set
                open_set.push_back(dependency.clone());
            }
        }

        // then compute the active dependency features for each crate
        let mut active_dependency_features = HashMap::<_, Vec<_>>::new();
        for krate in self
            .workspace
            .all_crates()
            .filter(|c| c.is_enabled.unwrap_or(false))
        {
            let all_active_dependencies = krate.dependencies.iter().chain(
                krate
                    .optional_dependencies
                    .iter()
                    .filter(|d| krate.optional_dependency_is_enabled(&d.name)),
            );
            // we can compute from the dependencies we have enabled on this crate
            for dependency in all_active_dependencies {
                for feature in krate.active_features.as_ref().unwrap() {
                    let feature = match krate.features.iter().find(|f| &f.name == feature) {
                        Some(f) => f,
                        None => {
                            error!(
                                "Active feature computed `{}` not found in crate `{}`",
                                feature, krate.name
                            );
                            continue;
                        }
                    };
                    for effect in feature.effects() {
                        if let Some(feature) = effect.enables_feature_in_crate(&dependency.name) {
                            active_dependency_features
                                .entry((krate.name.clone(), dependency.name.clone()))
                                .or_default()
                                .push(feature.clone());
                        }
                    }
                }
            }
        }
        // finally remove all enable_default_for_ features not to pollute the output
        // and insert the previously computed active dependency features
        for krate in self.workspace.all_crates_mut() {
            if let Some(features) = krate.active_features.as_mut() {
                features.retain(|f| !f.feature_name().starts_with("enable_default_for_"));
            }
        }

        for ((in_crate, dependency), features) in active_dependency_features {
            if let Some(krate) = self.workspace.find_crate_mut(&in_crate, || format!(
                "package from workspace manifest: `{in_crate}` was not found in the parsed workspace list. While setting active dependency features."
            )) {
                match krate.active_dependency_features.as_mut() {
                    Some(map) => {
                        map.insert(dependency, features);
                    }
                    None => {
                        let mut map = IndexMap::new();
                        map.insert(dependency, features);
                        krate.active_dependency_features = Some(map);
                    }
                }
            }
        }
    }

    /// process the open set and enable features and dependencies based on
    /// the current state of the workspace as well as any explicitly enabled features
    fn process_crate(
        &mut self,
        krate: &CrateName,
        enable_features: IndexSet<FeatureName>,
        trace_features: bool,
    ) {
        // find all conditional features in this crate, only consider the crates in the open set.
        // for example, if we start with the root workspace crate, we will eventually process the workspace
        // if we however start with a dependency crate, we will only process the features in that crate and down
        // this allows us to process the --workspace and --package flags fairly easilly
        // flatten the features by repeatedly activating features depending on the settings
        // consider default features appropriately

        let krate = match self.workspace.find_crate_mut(krate, || format!(
            "package from workspace manifest: `{krate}` was not found in the parsed workspace list. While computing active features."
        )) {
            Some(c) => c,
            None => return,
        };

        krate.is_enabled = Some(true);
        krate.compute_active_features(&enable_features, trace_features);
    }
}

impl From<Workspace> for WorkspaceGraph {
    fn from(workspace: Workspace) -> Self {
        // the graph describes the "enables" relasionship between crates and features
        // - a crate node can point to itself if it enables one of its own features
        // - it can point to other crates if it enables features in dependencies
        // - it can also point to other crates if it enables them
        let mut dependency_conditions_graph = petgraph::Graph::new();

        // add all crates as nodes
        for krate in workspace.all_crates() {
            dependency_conditions_graph.add_node(krate.name.clone());
        }

        // add all dependencies as edges
        for krate in workspace.all_crates() {
            for dependency in krate.dependencies() {
                let to_name = match &dependency {
                    DependencyKind::Optional(name) => name,
                    DependencyKind::NonOptional(name) => name,
                };

                let _ = (|| {
                    // some dependencies which are not part of the workspace will not be found here
                    // we just skip them
                    let to_node = workspace.find_crate_opt(to_name)?;

                    let from_index = dependency_conditions_graph.find_node_by(|n| n == &krate.name, || format!(
                            "package from workspace manifest: `{}` was not found in the graph. While initializing the graph, the current crate was not found.",
                            krate.name
                        ))?;

                    let to_index = dependency_conditions_graph.find_node_by(|n| n == &to_node.name, || format!(
                            "package from workspace manifest: `{}` was not found in the graph. While initializing the graph, the dependency crate was not found in the graph.",
                            to_node.name
                        ))?;

                    // non optional dependencies are always enabled
                    match dependency.is_optional() {
                        true => {
                            // find feature effects which enable this dependency
                            for feature in &krate.features {
                                for _ in feature.effects().filter(|e| e.enables_crate(to_name)) {
                                    dependency_conditions_graph.add_edge(
                                        from_index,
                                        to_index,
                                        EnablesDependencyIf::IfFeature(
                                            krate.name.clone(),
                                            feature.name.clone(),
                                        ),
                                    );
                                }
                            }
                        }
                        false => {
                            dependency_conditions_graph.add_edge(
                                from_index,
                                to_index,
                                EnablesDependencyIf::Always,
                            );
                        }
                    }

                    Some(())
                })();
            }
        }

        WorkspaceGraph {
            dependency_conditions_graph,
            workspace,
            input: HashMap::new(),
        }
    }
}
