//! A visualiser for bevy system schedules, as well as utilities for querying them via reflection
use std::{any::TypeId, borrow::Cow, ops::Deref};

use ::{
    bevy_ecs::{
        schedule::{
            InternedScheduleLabel, InternedSystemSet, NodeId, Schedule, ScheduleLabel, SystemSet,
        },
        system::{System, SystemInput},
    },
    bevy_platform::collections::{HashMap, HashSet},
    bevy_reflect::Reflect,
};
use bevy_log::warn;
use dot_writer::{Attributes, DotWriter};

#[derive(Reflect, Debug, Clone)]
#[reflect(opaque)]
/// A reflectable system.
pub struct ReflectSystem {
    pub(crate) name: Cow<'static, str>,
    pub(crate) type_id: TypeId,
    pub(crate) node_id: ReflectNodeId,
    pub(crate) default_system_sets: Vec<InternedSystemSet>,
}

impl ReflectSystem {
    /// Retrieves the name of the system.
    pub fn name(&self) -> &str {
        self.name.as_ref()
    }

    /// Retrieves the type id of the system.
    pub fn type_id(&self) -> TypeId {
        self.type_id
    }

    /// Retrieves the node id of the system.
    pub fn node_id(&self) -> NodeId {
        self.node_id.0
    }

    /// Retrieves the default system sets of the system.
    pub fn default_system_sets(&self) -> &[InternedSystemSet] {
        &self.default_system_sets
    }
    /// Creates a reflect system from a system specification
    pub fn from_system<In: SystemInput + 'static, Out: 'static>(
        system: &dyn System<In = In, Out = Out>,
        node_id: NodeId,
    ) -> Self {
        ReflectSystem {
            name: system.name().clone(),
            type_id: system.type_id(),
            node_id: ReflectNodeId(node_id),
            default_system_sets: system.default_system_sets(),
        }
    }

    /// gets the short identifier of the system, i.e. just the function name
    pub fn identifier(&self) -> &str {
        // if it contains generics it might contain more than
        if self.name.contains("<") {
            self.name
                .split("<")
                .next()
                .unwrap_or_default()
                .split("::")
                .last()
                .unwrap_or_default()
        } else {
            self.name.split("::").last().unwrap_or_default()
        }
    }

    /// gets the path of the system, i.e. the fully qualified function name
    pub fn path(&self) -> &str {
        self.name.as_ref()
    }
}

#[derive(Reflect, Clone, Debug, PartialEq, Eq, Hash, PartialOrd, Ord)]
#[reflect(opaque)]
pub(crate) struct ReflectNodeId(pub NodeId);

/// A reflectable schedule.
#[derive(Reflect, Clone, Debug)]
pub struct ReflectSchedule {
    /// The name of the schedule.
    type_path: &'static str,
    label: ReflectableScheduleLabel,
}

#[derive(Reflect, Clone, Debug)]
#[reflect(opaque)]
struct ReflectableScheduleLabel(InternedScheduleLabel);

impl Deref for ReflectableScheduleLabel {
    type Target = InternedScheduleLabel;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl From<InternedScheduleLabel> for ReflectableScheduleLabel {
    fn from(label: InternedScheduleLabel) -> Self {
        Self(label)
    }
}

impl ReflectSchedule {
    /// Retrieves the name of the schedule.
    pub fn type_path(&self) -> &'static str {
        self.type_path
    }

    /// Retrieves the short identifier of the schedule
    pub fn identifier(&self) -> &'static str {
        self.type_path.split("::").last().unwrap_or_default()
    }

    /// Retrieves the label of the schedule
    pub fn label(&self) -> &InternedScheduleLabel {
        &self.label
    }

    /// Creates a new reflect schedule from a schedule label
    pub fn from_label<T: ScheduleLabel + 'static>(label: T) -> Self {
        ReflectSchedule {
            type_path: std::any::type_name::<T>(),
            label: label.intern().into(),
        }
    }
}

#[derive(Reflect)]
/// A reflectable system set.
pub struct ReflectSystemSet {
    /// The node id of the system set.
    node_id: ReflectNodeId,

    /// The debug print of the system set
    debug: String,

    /// If this is a typed system set, the type id
    type_id: Option<TypeId>,
}

impl ReflectSystemSet {
    /// Creates a reflect system set from a system set
    pub fn from_set(set: &dyn SystemSet, node_id: NodeId) -> Self {
        ReflectSystemSet {
            node_id: ReflectNodeId(node_id),
            debug: format!("{set:?}"),
            type_id: set.system_type(),
        }
    }
}

/// Renders a schedule to a dot graph using the optimized schedule.
pub fn schedule_to_dot_graph(schedule: &Schedule) -> String {
    let graph = schedule_to_reflect_graph(schedule);
    reflect_graph_to_dot(graph)
}

/// Renders a reflectable system graph to a dot graph
pub fn reflect_graph_to_dot(graph: ReflectSystemGraph) -> String {
    // create a dot graph with:
    // - hierarchy represented by red composition arrows
    // - dependencies represented by blue arrows
    let mut output_bytes = Vec::new();
    let mut writer = DotWriter::from(&mut output_bytes);
    {
        let mut writer = writer.digraph();

        let mut node_id_map = HashMap::new();
        for node in graph.nodes {
            match node {
                ReflectSystemGraphNode::System(reflect_system) => {
                    let mut node = writer.node_auto();

                    node.set_label(&reflect_system.name);
                    node_id_map.insert(reflect_system.node_id, node.id());
                }
                ReflectSystemGraphNode::SystemSet(reflect_system_set) => {
                    let name = if reflect_system_set.type_id.is_some() {
                        // special sets, that pollute the graph, summize them as "system type set", each system gets one
                        "SystemTypeSet".to_owned()
                    } else {
                        format!("SystemSet {}", reflect_system_set.debug)
                    };

                    let mut node = writer.node_auto();
                    node.set_label(&name);
                    node_id_map.insert(reflect_system_set.node_id, node.id());
                }
            }
        }

        // go through hierarchy edges
        for edge in graph.hierarchy {
            let from = node_id_map.get(&edge.from).cloned().unwrap_or_else(|| {
                let mut unknown = writer.node_auto();
                unknown.set_label(&format!("unknown_parent {:?}", edge.from.0));
                let id = unknown.id();
                node_id_map.insert(edge.from, id.clone());
                id
            });
            let to = node_id_map.get(&edge.to).cloned().unwrap_or_else(|| {
                let mut unknown = writer.node_auto();
                unknown.set_label(&format!("unknown_child {:?}", edge.to.0));
                let id = unknown.id();
                node_id_map.insert(edge.to, id.clone());
                id
            });
            writer
                .edge(to, from)
                .attributes()
                .set_color(dot_writer::Color::Red)
                .set_label("child of")
                .set_arrow_head(dot_writer::ArrowType::Diamond);
        }
        // go through dependency edges
        for edge in graph.dependencies {
            let from = node_id_map.get(&edge.from).cloned().unwrap_or_else(|| {
                let mut unknown = writer.node_auto();
                unknown.set_label(&format!("unknown_dependant {:?}", edge.from.0));
                let id = unknown.id();
                node_id_map.insert(edge.from, id.clone());
                id
            });
            let to = node_id_map.get(&edge.to).cloned().unwrap_or_else(|| {
                let mut unknown = writer.node_auto();
                unknown.set_label(&format!("unknown_dependency {:?}", edge.to.0));
                let id = unknown.id();
                node_id_map.insert(edge.to, id.clone());
                id
            });
            writer
                .edge(from, to)
                .attributes()
                .set_color(dot_writer::Color::Blue)
                .set_label("runs before")
                .set_arrow_head(dot_writer::ArrowType::Normal);
        }
    }

    String::from_utf8(output_bytes).unwrap_or_default()
}

/// Converts a schedule to a reflectable system graph
pub fn schedule_to_reflect_graph(schedule: &Schedule) -> ReflectSystemGraph {
    let graph = schedule.graph();
    let hierarchy = graph.hierarchy().graph();
    let dependency = graph.dependency().graph();

    let mut nodes = Vec::new();
    let mut covered_nodes = HashSet::new();
    for (node_id, system_set, _) in graph.system_sets() {
        covered_nodes.insert(node_id);
        nodes.push(ReflectSystemGraphNode::SystemSet(
            ReflectSystemSet::from_set(system_set, node_id),
        ));
    }

    // for some reason the graph doesn't contain these
    if let Ok(systems) = schedule.systems() {
        for (node_id, system) in systems {
            covered_nodes.insert(node_id);
            nodes.push(ReflectSystemGraphNode::System(ReflectSystem::from_system(
                system.as_ref(),
                node_id,
            )));
        }
    }

    // try find all uncovered nodes, and warn about them, or do something about them
    // for now we just warn
    for node_id in hierarchy.nodes() {
        if covered_nodes.contains(&node_id) {
            continue;
        }

        warn!("Found uncovered node {node_id:?}");
    }

    let dependencies = dependency
        .all_edges()
        .map(|(from, to)| Edge {
            from: ReflectNodeId(from),
            to: ReflectNodeId(to),
        })
        .collect();

    let hierarchy = hierarchy
        .all_edges()
        .map(|(from, to)| Edge {
            from: ReflectNodeId(from),
            to: ReflectNodeId(to),
        })
        .collect();

    ReflectSystemGraph {
        schedule: ReflectSchedule::from_label(schedule.label()),
        nodes,
        dependencies,
        hierarchy,
    }
}

/// A graph of systems and system sets for a single schedule
#[derive(Reflect)]
pub struct ReflectSystemGraph {
    /// The schedule that this graph represents
    schedule: ReflectSchedule,
    /// All of the included nodes
    nodes: Vec<ReflectSystemGraphNode>,

    /// The edges signifying the dependency relationship between each node.
    ///
    /// I.e. if there is an edge from A -> B, then A depends on B
    dependencies: Vec<Edge>,

    /// The edges signifying the hierarchy relationship between each node.
    /// I.e. if there is an edge from A -> B, then A is a child of B
    hierarchy: Vec<Edge>,
}

impl ReflectSystemGraph {
    /// Sorts the graph nodes and edges.
    ///
    /// Useful if you want a stable order and deterministic graphs.
    pub fn sort(&mut self) {
        // sort everything in this graph
        self.nodes.sort_by_key(|node| match node {
            ReflectSystemGraphNode::System(system) => system.node_id.0,
            ReflectSystemGraphNode::SystemSet(system_set) => system_set.node_id.0,
        });

        self.dependencies.sort();

        self.hierarchy.sort();
    }

    /// removes the set and bridges the edges connecting to it
    fn absorb_set(&mut self, node_id: NodeId) {
        // collect hierarchy parents and children in one pass
        let mut hierarchy_parents = Vec::new();
        let mut hierarchy_children = Vec::new();
        // the relation ship expressed by edges is "is child of"
        for edge in &self.hierarchy {
            // these are children
            if edge.to.0 == node_id {
                hierarchy_children.push(edge.from.clone());
            }
            // these are parents
            if edge.from.0 == node_id {
                hierarchy_parents.push(edge.to.clone());
            }
        }

        //
        let mut dependencies = Vec::new();
        let mut dependents = Vec::new();
        // the relationship expressed is "runs before" i.e. "is depended on by"
        for edge in &self.dependencies {
            if edge.to.0 == node_id {
                dependencies.push(edge.from.clone());
            }
            if edge.from.0 == node_id {
                dependents.push(edge.to.clone());
            }
        }

        let mut new_hierarchy_edges =
            HashSet::with_capacity(hierarchy_parents.len() * hierarchy_children.len());
        let mut new_dependency_edges =
            HashSet::with_capacity(dependencies.len() * dependents.len());

        // each parent, becomes a parent to the sets children
        for parent in hierarchy_parents.iter() {
            for child in hierarchy_children.iter() {
                new_hierarchy_edges.insert(Edge {
                    from: child.clone(),
                    to: parent.clone(),
                });
            }
        }

        for child in hierarchy_parents.iter() {
            // bridge dependencies
            for dependency in dependencies.iter() {
                new_dependency_edges.insert(Edge {
                    from: dependency.clone(),
                    to: child.clone(),
                });
            }

            for dependent in dependents.iter() {
                new_dependency_edges.insert(Edge {
                    from: child.clone(),
                    to: dependent.clone(),
                });
            }
        }

        // remove any edges involving the set to absorb
        self.hierarchy
            .retain(|edge| edge.from.0 != node_id && edge.to.0 != node_id);
        self.dependencies
            .retain(|edge| edge.from.0 != node_id && edge.to.0 != node_id);

        // remove the set from nodes
        self.nodes.retain(|node| match node {
            ReflectSystemGraphNode::SystemSet(system_set) => system_set.node_id.0 != node_id,
            _ => true,
        });

        // add new bridging edges
        self.hierarchy.extend(new_hierarchy_edges);
        self.dependencies.extend(new_dependency_edges);
    }

    /// type system sets, are not really important to us, for all intents and purposes
    /// they are one and the same as the underlying systems
    /// Adapter and pipe systems might have multiple default system sets attached, but we want all them gone from the graph.
    ///
    /// Inlines every type system set into its children, replacing anything pointing to those sets by edges to every system contained in the set
    pub fn absorb_type_system_sets(&mut self) {
        let type_sets = self
            .nodes
            .iter()
            .filter_map(|node| match node {
                ReflectSystemGraphNode::SystemSet(system_set) => {
                    if system_set.type_id.is_some() {
                        Some(system_set.node_id.0)
                    } else {
                        None
                    }
                }
                _ => None,
            })
            .collect::<Vec<_>>();

        // yes this is very inefficient, no this isn't a big hot path, these graphs mostly serve a debugging role.
        for node_id in type_sets {
            self.absorb_set(node_id);
        }
    }
}

/// A node in the reflectable system graph
#[derive(Reflect)]
pub enum ReflectSystemGraphNode {
    /// A system node
    System(ReflectSystem),
    /// A system set node
    SystemSet(ReflectSystemSet),
}

/// An edge in the graph
#[derive(Reflect, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Edge {
    /// The id of the node this edge is coming from
    from: ReflectNodeId,
    /// The id of the node this edge is going to
    to: ReflectNodeId,
}

#[cfg(test)]
mod test {
    use ::{
        bevy_app::Update,
        bevy_ecs::{schedule::IntoScheduleConfigs, world::World},
    };

    use super::*;

    fn system_a() {}

    fn system_b() {}

    fn system_c() {}

    fn system_d() {}

    fn system_e() {}

    const BLESS_MODE: bool = false;

    #[test]
    fn test_graph_is_as_expected() {
        // create empty schedule and graph it

        let mut schedule = Schedule::new(Update);

        #[derive(SystemSet, Hash, PartialEq, Eq, PartialOrd, Ord, Debug, Clone, Copy)]
        enum SystemSet {
            SystemSetG,
            SystemSetH,
        }

        // add a few systems that depend on each other, some via system sets

        schedule
            .add_systems(system_a)
            .add_systems(system_b.before(system_a))
            .add_systems(system_c.after(system_b).before(SystemSet::SystemSetH))
            .add_systems(system_d.in_set(SystemSet::SystemSetG))
            .add_systems(system_e.in_set(SystemSet::SystemSetH))
            .configure_sets(SystemSet::SystemSetG.after(SystemSet::SystemSetH));
        let mut world = World::new();
        schedule.initialize(&mut world).unwrap();

        let mut graph = schedule_to_reflect_graph(&schedule);
        graph.absorb_type_system_sets();
        graph.sort();
        let dot = reflect_graph_to_dot(graph);

        let normalize = |s: &str| {
            // trim each line individually from the start, and replace " = " with "=" to deal with formatters
            let lines: Vec<&str> = s.lines().map(|line| line.trim_start()).collect();
            lines
                .join("\n")
                .replace(" = ", "")
                .replace(";", "")
                .replace(",", "")
                .trim()
                .to_string()
        };

        // check that the dot graph is as expected
        // the expected file is found next to the src/lib.rs folder
        let expected = include_str!("../test_graph.dot");
        let expected_path = manifest_dir_macros::file_path!("test_graph.dot");

        if BLESS_MODE {
            std::fs::write(expected_path, normalize(&dot)).unwrap();
            panic!("Bless mode is active");
        } else {
            pretty_assertions::assert_eq!(normalize(&dot), normalize(expected));
        }
    }
}
