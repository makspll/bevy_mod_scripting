//! A visualiser for bevy system schedules, as well as utilities for querying them via reflection
use std::ops::Deref;
use std::{any::TypeId, borrow::Cow};

use bevy::ecs::schedule::{
    InternedScheduleLabel, InternedSystemSet, NodeId, Schedule, ScheduleLabel, SystemSet,
};
use bevy::ecs::system::{System, SystemInput};
use bevy::reflect::Reflect;
use bevy::utils::HashSet;
use bevy::utils::hashbrown::HashMap;
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

#[derive(Reflect, Clone, Debug, PartialEq, Eq, Hash)]
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
            debug: format!("{:?}", set),
            type_id: set.system_type(),
        }
    }
}

/// Renders a schedule to a dot graph using the optimized schedule.
pub fn schedule_to_dot_graph(schedule: &Schedule) -> String {
    let graph = schedule_to_reflect_graph(schedule);

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
        // go through hierarchy edges
        for edge in graph.hierarchy {
            let from = node_id_map.get(&edge.from).cloned().unwrap_or_else(|| {
                let mut unknown = writer.node_auto();
                unknown.set_label(&format!("unknown_child {:?}", edge.from.0));
                let id = unknown.id();
                node_id_map.insert(edge.from, id.clone());
                id
            });
            let to = node_id_map.get(&edge.to).cloned().unwrap_or_else(|| {
                let mut unknown = writer.node_auto();
                unknown.set_label(&format!("unknown_parent {:?}", edge.to.0));
                let id = unknown.id();
                node_id_map.insert(edge.to, id.clone());
                id
            });
            writer
                .edge(from, to)
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
                .set_label("depends on")
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

        bevy::log::warn!("Found uncovered node {node_id:?}");
    }

    let dependencies = dependency
        .all_edges()
        .map(|(from, to, _)| Edge {
            from: ReflectNodeId(from),
            to: ReflectNodeId(to),
        })
        .collect();

    let hierarchy = hierarchy
        .all_edges()
        .map(|(from, to, _)| Edge {
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

/// A node in the reflectable system graph
#[derive(Reflect)]
pub enum ReflectSystemGraphNode {
    /// A system node
    System(ReflectSystem),
    /// A system set node
    SystemSet(ReflectSystemSet),
}

/// An edge in the graph
#[derive(Reflect)]
pub struct Edge {
    /// The id of the node this edge is coming from
    from: ReflectNodeId,
    /// The id of the node this edge is going to
    to: ReflectNodeId,
}
