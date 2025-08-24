use std::collections::{HashMap, HashSet, VecDeque};

use log::error;
use petgraph::Directed;

use crate::{CrateName, Dependency, Feature, FeatureName, Workspace};

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
}
pub struct WorkspaceGraph {
    pub dependency_conditions_graph: petgraph::Graph<CrateName, EnablesDependencyIf, Directed>,
    pub workspace: Workspace,
}

impl WorkspaceGraph {
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

    /// Calculate the enabled features and dependencies for the workspace
    /// The set of enabled feature/crate pairs is treated as the set of explicitly enabled crates
    /// For a workspace this should be the workspace root with the desired features enabled..
    ///
    /// For --package, behaviour this should be just the specified package with the desired features enabled.
    ///
    /// The result will be written into the workspace crates
    pub fn calculate_enabled_features_and_dependencies(
        &mut self,
        enabled_crates: &HashMap<CrateName, (Vec<FeatureName>, bool)>,
    ) {
        // now we process crates, one at a time
        // first process the explicitly enabled crates with features
        // we keep processing crates untill no features can be enabled
        let mut open_set = VecDeque::new();

        open_set.extend(enabled_crates.keys().cloned());

        while let Some(krate) = open_set.pop_front() {
            log::trace!("Processing crate `{krate}`");
            match enabled_crates.get(&krate) {
                Some((features, enable_default)) => {
                    // a top level enabled crate, process only explicitly enabled features, we know nothing else will be there to enable more
                    let features = HashSet::from_iter(features.iter().cloned());
                    if *enable_default
                        && let Some(c) = self.workspace.find_crate_mut(&krate, || format!(
                            "package from workspace manifest: `{krate}` was not found in the parsed workspace list. This might lead to missing default features."))
                    {
                        let hack = Feature::new_enabling_default_for(c.name.clone());
                        c.features.push(hack);
                    };
                    log::trace!(
                        "Crate `{krate}` is explicitly enabled with features: {features:?}, default: {enable_default}"
                    );
                    self.process_crate(&krate, features);
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
                            .collect::<HashSet<_>>();

                    log::trace!(
                        "Crate `{krate}` is being enabled with parent features: {parent_features:?}"
                    );
                    self.process_crate(&krate, parent_features);
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

        // finally remove all enable_default_for_ features not to pollute the output
        for krate in self.workspace.all_crates_mut() {
            if let Some(features) = krate.active_features.as_mut() {
                features.retain(|f| !f.feature_name().starts_with("enable_default_for_"));
            }
        }
    }

    /// process the open set and enable features and dependencies based on
    /// the current state of the workspace as well as any explicitly enabled features
    fn process_crate(&mut self, krate: &CrateName, enable_features: HashSet<FeatureName>) {
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
        krate.compute_active_features(&enable_features, false);
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
                    Dependency::Optional(name) => name,
                    Dependency::NonOptional(name) => name,
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
        }
    }
}
