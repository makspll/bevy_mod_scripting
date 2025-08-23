//! Dynamic scheduling from scripts

use super::{WorldAccessGuard, script_system::ScriptSystemBuilder};
use crate::{IntoScriptPluginParams, error::InteropError};
use ::{
    bevy_app::{
        First, FixedFirst, FixedLast, FixedMain, FixedPostUpdate, FixedPreUpdate, FixedUpdate,
        Last, PostStartup, PostUpdate, PreStartup, PreUpdate, RunFixedMainLoop, Startup, Update,
    },
    bevy_ecs::{
        schedule::{Schedule, ScheduleLabel, Schedules},
        world::World,
    },
};
use bevy_ecs::resource::Resource;
use bevy_log::debug;
use bevy_platform::collections::HashMap;
use bevy_system_reflection::{ReflectSchedule, ReflectSystem};
use parking_lot::RwLock;
use std::{any::TypeId, sync::Arc};
#[derive(Default, Clone, Resource)]
/// A Send + Sync registry of bevy schedules.
pub struct AppScheduleRegistry(Arc<RwLock<ScheduleRegistry>>);

impl AppScheduleRegistry {
    /// Reads the schedule registry.
    pub fn read(&self) -> parking_lot::RwLockReadGuard<'_, ScheduleRegistry> {
        self.0.read()
    }

    /// Writes to the schedule registry.
    pub fn write(&self) -> parking_lot::RwLockWriteGuard<'_, ScheduleRegistry> {
        self.0.write()
    }

    /// Creates a new schedule registry pre-populated with default bevy schedules.
    pub fn new() -> Self {
        Self(Arc::new(RwLock::new(ScheduleRegistry::new())))
    }
}

#[derive(Default)]
/// A registry of bevy schedules.
pub struct ScheduleRegistry {
    schedules: HashMap<TypeId, ReflectSchedule>,
}

#[profiling::all_functions]
impl ScheduleRegistry {
    /// Creates a new schedule registry containing all default bevy schedules.
    pub fn new() -> Self {
        let mut self_ = Self::default();
        self_
            .register(Update)
            .register(First)
            .register(PreUpdate)
            .register(RunFixedMainLoop)
            .register(PostUpdate)
            .register(Last)
            .register(PreStartup)
            .register(Startup)
            .register(PostStartup)
            .register(FixedMain)
            .register(FixedFirst)
            .register(FixedPreUpdate)
            .register(FixedUpdate)
            .register(FixedPostUpdate)
            .register(FixedLast);
        self_
    }

    /// Retrieves a schedule by name
    pub fn get_schedule_by_name(&self, name: &str) -> Option<&ReflectSchedule> {
        self.schedules.iter().find_map(|(_, schedule)| {
            (schedule.identifier() == name || schedule.type_path() == name).then_some(schedule)
        })
    }

    /// Registers a schedule
    pub fn register<T: ScheduleLabel + 'static>(&mut self, label: T) -> &mut Self {
        let schedule = ReflectSchedule::from_label(label);
        self.schedules.insert(TypeId::of::<T>(), schedule);
        self
    }

    /// Retrieves the given schedule
    pub fn get(&self, type_id: TypeId) -> Option<&ReflectSchedule> {
        self.schedules.get(&type_id)
    }

    /// Retrieves the given schedule mutably
    pub fn get_mut(&mut self, type_id: TypeId) -> Option<&mut ReflectSchedule> {
        self.schedules.get_mut(&type_id)
    }

    /// Checks if the given schedule is contained
    pub fn contains(&self, type_id: TypeId) -> bool {
        self.schedules.contains_key(&type_id)
    }

    /// Creates an iterator over all schedules
    pub fn iter(&self) -> impl Iterator<Item = (&TypeId, &ReflectSchedule)> {
        self.schedules.iter()
    }

    /// Creates an iterator over all schedules mutably
    pub fn iter_mut(&mut self) -> impl Iterator<Item = (&TypeId, &mut ReflectSchedule)> {
        self.schedules.iter_mut()
    }
}

#[profiling::all_functions]
/// Impls to do with dynamically querying systems and schedules
impl WorldAccessGuard<'_> {
    /// Temporarilly removes the given schedule from the world, and calls the given function on it, then re-inserts it.
    ///
    /// Useful for initializing schedules, or modifying systems
    pub fn scope_schedule<O, F: FnOnce(&mut World, &mut Schedule) -> O>(
        &self,
        label: &ReflectSchedule,
        f: F,
    ) -> Result<O, InteropError> {
        self.with_global_access(|world| {
            let mut schedules = world.get_resource_mut::<Schedules>().ok_or_else(|| {
                InteropError::unsupported_operation(
                    None,
                    None,
                    "accessing schedules in a world with no schedules",
                )
            })?;

            let mut removed_schedule = schedules
                .remove(*label.label())
                .ok_or_else(|| InteropError::missing_schedule(label.identifier()))?;

            let result = f(world, &mut removed_schedule);

            let mut schedules = world.get_resource_mut::<Schedules>().ok_or_else(|| {
                InteropError::unsupported_operation(
                    None,
                    None,
                    "removing `Schedules` resource within a schedule scope",
                )
            })?;

            assert!(
                removed_schedule.label() == *label.label(),
                "removed schedule label doesn't match the original"
            );
            schedules.insert(removed_schedule);

            Ok(result)
        })?
    }

    /// Retrieves all the systems in a schedule
    pub fn systems(&self, schedule: &ReflectSchedule) -> Result<Vec<ReflectSystem>, InteropError> {
        self.with_resource(|schedules: &Schedules| {
            let schedule = schedules
                .get(*schedule.label())
                .ok_or_else(|| InteropError::missing_schedule(schedule.identifier()))?;

            let systems = schedule.systems()?;

            Ok(systems
                .map(|(node_id, system)| ReflectSystem::from_system(system.as_ref(), node_id))
                .collect())
        })?
    }

    /// Creates a system from a system builder and inserts it into the given schedule
    pub fn add_system<P: IntoScriptPluginParams>(
        &self,
        schedule: &ReflectSchedule,
        builder: ScriptSystemBuilder,
    ) -> Result<ReflectSystem, InteropError> {
        debug!(
            "Adding script system '{}' for script '{}' to schedule '{}'",
            builder.name,
            builder.attachment,
            schedule.identifier()
        );

        builder.build::<P>(self.clone(), schedule)
    }
}

#[cfg(test)]
#[allow(
    dead_code,
    unused_imports,
    reason = "tests are there but not working currently"
)]
mod tests {
    use ::{
        bevy_app::{App, Plugin, Update},
        bevy_ecs::{
            entity::Entity,
            schedule::{NodeId, Schedules},
            system::IntoSystem,
        },
    };
    use test_utils::make_test_plugin;

    use super::*;

    #[test]
    fn test_schedule_registry() {
        let mut registry = ScheduleRegistry::default();
        registry.register(Update);

        assert!(registry.contains(TypeId::of::<Update>()));

        let schedule = registry.get(TypeId::of::<Update>()).unwrap();
        assert_eq!(schedule.identifier(), "Update");
        assert_eq!(schedule.type_path(), std::any::type_name::<Update>());
        assert_eq!(
            registry
                .get_schedule_by_name("Update")
                .unwrap()
                .identifier(),
            "Update"
        );
    }

    fn test_system_generic<T>() {}
    fn test_system() {}

    #[test]
    fn test_reflect_system_names() {
        let system = IntoSystem::into_system(test_system_generic::<String>);
        let system = ReflectSystem::from_system(&system, NodeId::Set(0));

        assert_eq!(system.identifier(), "test_system_generic");
        assert_eq!(
            system.path(),
            "bevy_mod_scripting_core::bindings::schedule::tests::test_system_generic<alloc::string::String>"
        );

        let system = IntoSystem::into_system(test_system);
        let system = ReflectSystem::from_system(&system, NodeId::Set(0));

        assert_eq!(system.identifier(), "test_system");
        assert_eq!(
            system.path(),
            "bevy_mod_scripting_core::bindings::schedule::tests::test_system"
        );
    }

    make_test_plugin!(crate);

    // #[test]
    // fn test_into_system_set_identical_for_real_and_reflect_set() {
    //     let root_system = || {};
    //     let as_system = IntoSystem::into_system(root_system);
    //     let as_reflect_system = ReflectSystem::from_system(&as_system);

    //     let set1 = Box::new(IntoSystemSet::into_system_set(root_system)) as Box<dyn SystemSet>;
    //     let set2 =
    //         Box::new(IntoSystemSet::into_system_set(as_reflect_system)) as Box<dyn SystemSet>;

    //     let mut hasher1 = std::collections::hash_map::DefaultHasher::new();
    //     set1.dyn_hash(&mut hasher1);
    //     let mut hasher2 = std::collections::hash_map::DefaultHasher::new();
    //     set2.dyn_hash(&mut hasher2);
    //     pretty_assertions::assert_eq!(hasher1.finish(), hasher2.finish());

    //     pretty_assertions::assert_eq!(set1.system_type(), set2.system_type());
    //     assert!(set1.dyn_eq(&set2));
    // }

    #[derive(ScheduleLabel, Hash, PartialEq, Eq, Debug, Clone)]
    struct TestSchedule;

    fn test_system_a() {}
    fn test_system_b() {}

    /// Verifies that the given schedule graph contains the expected node names and edges.
    ///
    /// # Arguments
    ///
    /// * `app` - A mutable reference to the Bevy App.
    /// * `schedule_label` - The schedule label to locate the schedule.
    /// * `expected_nodes` - A slice of node names expected to be present.
    /// * `expected_edges` - A slice of tuples representing expected edges (from, to).
    pub fn verify_schedule_graph<T>(
        app: &mut App,
        schedule_label: T,
        expected_nodes: &[&str],
        expected_edges: &[(&str, &str)],
    ) where
        T: ScheduleLabel + std::hash::Hash + Eq + Clone + Send + Sync + 'static,
    {
        // Remove schedules, then remove the schedule to verify.
        let mut schedules = app
            .world_mut()
            .remove_resource::<Schedules>()
            .expect("Schedules resource not found");
        let mut schedule = schedules
            .remove(schedule_label.clone())
            .expect("Schedule not found");

        schedule.initialize(app.world_mut()).unwrap();
        let graph = schedule.graph();

        // Build a mapping from system name to its node id.

        let resolve_name = |node_id: NodeId| {
            let out = {
                // try systems
                if let Some(system) = graph.get_system_at(node_id) {
                    system.name().clone().to_string()
                } else if let Some(system_set) = graph.get_set_at(node_id) {
                    format!("{system_set:?}").to_string()
                } else {
                    // try schedule systems
                    let mut default = format!("{node_id:?}").to_string();
                    for (system_node, system) in schedule.systems().unwrap() {
                        if node_id == system_node {
                            default = system.name().clone().to_string();
                        }
                    }
                    default
                }
            };

            // trim module path
            let trim = "bevy_mod_scripting_core::bindings::schedule::tests::";
            out.replace(trim, "")
        };

        let all_nodes = graph
            .dependency()
            .graph()
            .nodes()
            .map(&resolve_name)
            .collect::<Vec<_>>();

        // Assert expected nodes exist.
        for &node in expected_nodes {
            assert!(
                all_nodes.contains(&node.to_owned()),
                "Graph does not contain expected node '{node}' nodes: {all_nodes:?}"
            );
        }

        // Collect all edges as (from, to) name pairs.
        let mut found_edges = Vec::new();
        for (from, to) in graph.dependency().graph().all_edges() {
            let name_from = resolve_name(from);
            let name_to = resolve_name(to);
            found_edges.push((name_from, name_to));
        }

        // Assert each expected edge exists.
        for &(exp_from, exp_to) in expected_edges {
            assert!(
                found_edges.contains(&(exp_from.to_owned(), exp_to.to_owned())),
                "Expected edge ({exp_from} -> {exp_to}) not found. Found edges: {found_edges:?}"
            );
        }

        // Optionally, reinsert the schedule back into the schedules resource.
        schedules.insert(schedule);
        app.world_mut().insert_resource(schedules);
    }

    // #[test]
    // fn test_builder_creates_correct_system_graph_against_rust_systems() {
    //     let mut app = App::new();
    //     app.add_plugins((
    //         bevy::asset::AssetPlugin::default(),
    //         bevy::diagnostic::DiagnosticsPlugin,
    //         TestPlugin::default(), // assuming TestPlugin is defined appropriately
    //     ));

    //     let system_a = IntoSystem::into_system(test_system_a);

    //     let system_b = IntoSystem::into_system(test_system_b);

    //     let mut system_builder = ScriptSystemBuilder::new("test".into(), ScriptId::from("test"));
    //     // Set ordering: script system runs after "root1" and before "root2".
    //     system_builder
    //         .after(ReflectSystem::from_system(&system_a, NodeId::System(0)))
    //         .before(ReflectSystem::from_system(&system_b, NodeId::System(1)));

    //     app.init_schedule(TestSchedule);
    //     app.add_systems(TestSchedule, system_a);
    //     app.add_systems(TestSchedule, system_b);
    //     let _ = system_builder.build::<TestPlugin>(
    //         WorldGuard::new(app.world_mut()),
    //         &ReflectSchedule::from_label(TestSchedule),
    //     );

    //     verify_schedule_graph(
    //         &mut app,
    //         TestSchedule,
    //         // expected nodes
    //         &["test_system_a", "test_system_b", "script_system_test"],
    //         // expected edges (from, to), i.e. before, after, relationships
    //         &[
    //             ("SystemTypeSet(fn bevy_ecs::system::function_system::FunctionSystem<fn(), test_system_a>())", "script_system_test"),
    //             ("script_system_test", "SystemTypeSet(fn bevy_ecs::system::function_system::FunctionSystem<fn(), test_system_b>())"),
    //         ],
    //     );
    // }

    // #[test]
    // fn test_builder_creates_correct_system_graph_against_script_systems() {
    //     let mut app = App::new();
    //     app.add_plugins((
    //         bevy::asset::AssetPlugin::default(),
    //         bevy::diagnostic::DiagnosticsPlugin,
    //         TestPlugin::default(), // assuming TestPlugin is defined appropriately
    //     ));
    //     app.init_schedule(TestSchedule);
    //     let reflect_schedule = ReflectSchedule::from_label(TestSchedule);

    //     let system_root = ScriptSystemBuilder::new("root".into(), "script_root.lua".into())
    //         .build::<TestPlugin>(WorldGuard::new(app.world_mut()), &reflect_schedule)
    //         .unwrap();

    //     let mut system_a = ScriptSystemBuilder::new("a".into(), "script_a.lua".into());
    //     system_a.before(system_root.clone());
    //     system_a
    //         .build::<TestPlugin>(WorldGuard::new(app.world_mut()), &reflect_schedule)
    //         .unwrap();

    //     let mut system_b = ScriptSystemBuilder::new("b".into(), "script_b.lua".into());
    //     system_b.after(system_root.clone());
    //     system_b
    //         .build::<TestPlugin>(WorldGuard::new(app.world_mut()), &reflect_schedule)
    //         .unwrap();

    //     verify_schedule_graph(
    //         &mut app,
    //         TestSchedule,
    //         // expected nodes
    //         &["script_system_root", "script_system_a", "script_system_b"],
    //         // expected edges (from, to), i.e. before, after, relationships
    //         &[
    //             // this doesn't work currently TODO: fix this, i.e. we inject the systems but not the ordering constraints
    //             // ("SystemTypeSet(fn bevy_ecs::system::function_system::FunctionSystem<fn(), test_system_a>())", "script_system_test"),
    //             // ("script_system_test", "SystemTypeSet(fn bevy_ecs::system::function_system::FunctionSystem<fn(), test_system_b>())"),
    //         ],
    //     );
    // }
}
