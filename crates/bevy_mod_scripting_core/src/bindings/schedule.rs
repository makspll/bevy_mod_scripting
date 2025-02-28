//! Dynamic scheduling from scripts

use std::{
    any::TypeId,
    borrow::Cow,
    collections::HashMap,
    hash::Hash,
    ops::Deref,
    sync::{
        atomic::{AtomicUsize, Ordering},
        Arc,
    },
};

use bevy::{
    app::{
        First, FixedFirst, FixedLast, FixedMain, FixedPostUpdate, FixedPreUpdate, FixedUpdate,
        Last, PostStartup, PostUpdate, PreStartup, PreUpdate, RunFixedMainLoop, Startup, Update,
    },
    ecs::{
        entity::Entity,
        intern::Interned,
        schedule::{
            InternedScheduleLabel, IntoSystemConfigs, NodeId, Schedule, ScheduleLabel, Schedules,
            SystemSet,
        },
        system::{IntoSystem, Resource, System, SystemInput, SystemState},
        world::World,
    },
    reflect::Reflect,
};
use parking_lot::RwLock;

use crate::{
    error::InteropError,
    event::CallbackLabel,
    extractors::{HandlerContext, WithWorldGuard},
    handler::handle_script_errors,
    script::ScriptId,
    IntoScriptPluginParams,
};

use super::WorldAccessGuard;

#[derive(Reflect, Debug, Clone)]
/// A reflectable system.
pub struct ReflectSystem {
    name: Cow<'static, str>,
    type_id: TypeId,
    node_id: ReflectNodeId,
}

#[derive(Reflect, Clone, Debug)]
#[reflect(opaque)]
pub(crate) struct ReflectNodeId(pub(crate) NodeId);

impl ReflectSystem {
    /// Creates a reflect system from a system specification
    pub fn from_system<In: SystemInput + 'static, Out: 'static>(
        system: &dyn System<In = In, Out = Out>,
        node_id: NodeId,
    ) -> Self {
        ReflectSystem {
            name: system.name().clone(),
            type_id: system.type_id(),
            node_id: ReflectNodeId(node_id),
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

#[derive(Default, Clone, Resource)]
/// A Send + Sync registry of bevy schedules.
pub struct AppScheduleRegistry(Arc<RwLock<ScheduleRegistry>>);

impl AppScheduleRegistry {
    /// Reads the schedule registry.
    pub fn read(&self) -> parking_lot::RwLockReadGuard<ScheduleRegistry> {
        self.0.read()
    }

    /// Writes to the schedule registry.
    pub fn write(&self) -> parking_lot::RwLockWriteGuard<ScheduleRegistry> {
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

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
/// a system set for script systems.
pub struct ScriptSystemSet(usize);

impl ScriptSystemSet {
    /// Creates a new script system set with a unique id
    pub fn next() -> Self {
        static CURRENT_ID: AtomicUsize = AtomicUsize::new(0);
        Self(CURRENT_ID.fetch_add(1, Ordering::Relaxed))
    }
}

impl SystemSet for ScriptSystemSet {
    fn dyn_clone(&self) -> bevy::ecs::label::Box<dyn SystemSet> {
        Box::new(*self)
    }

    fn as_dyn_eq(&self) -> &dyn bevy::ecs::label::DynEq {
        self
    }

    fn dyn_hash(&self, mut state: &mut dyn ::core::hash::Hasher) {
        self.hash(&mut state);
    }
}

/// A builder for systems living in scripts
#[derive(Reflect)]
pub struct ScriptSystemBuilder {
    name: CallbackLabel,
    script_id: ScriptId,
    before: Vec<ReflectSystem>,
    after: Vec<ReflectSystem>,
}

impl ScriptSystemBuilder {
    /// Creates a new script system builder
    pub fn new(name: CallbackLabel, script_id: ScriptId) -> Self {
        Self {
            before: vec![],
            after: vec![],
            name,
            script_id,
        }
    }

    /// Adds a system to run before the script system
    pub fn before(&mut self, system: ReflectSystem) -> &mut Self {
        self.before.push(system);
        self
    }

    /// Adds a system to run after the script system
    pub fn after(&mut self, system: ReflectSystem) -> &mut Self {
        self.after.push(system);
        self
    }

    /// Selects the most granual system set it can for the given system node id or None
    fn get_individual_system_system_set(
        node_id: NodeId,
        schedule: &Schedule,
    ) -> Option<Box<dyn SystemSet>> {
        if let Some(system) = schedule.graph().get_system_at(node_id) {
            if let Some(system_set) = system.default_system_sets().first() {
                return Some(system_set.dyn_clone());
            }
        }

        if let Some(system_set) = schedule.graph().get_set_at(node_id) {
            bevy::log::debug!("!!!Found system set for system {:?}", node_id);
            return Some(system_set.dyn_clone());
        }

        None
    }

    /// Builds the system and inserts it into the given schedule
    #[allow(deprecated)]
    pub fn build<P: IntoScriptPluginParams>(self, schedule: &mut Schedule) -> ReflectSystem {
        // this is different to a normal event handler
        // the system doesn't listen to events
        // it immediately calls a singular script with a predefined payload
        let system_name = format!("script_system_{}", &self.name);
        let _system =
            move |world: &mut World,
                  system_state: &mut SystemState<WithWorldGuard<HandlerContext<P>>>| {
                let mut with_guard = system_state.get_mut(world);

                {
                    let (guard, handler_ctxt) = with_guard.get_mut();
                    let name = self.name.clone();
                    bevy::log::debug_once!("First call to script system {}", name);
                    match handler_ctxt.call_dynamic_label(
                        &name,
                        self.script_id.clone(),
                        Entity::from_raw(0),
                        vec![],
                        guard.clone(),
                    ) {
                        Ok(_) => {}
                        Err(err) => {
                            handle_script_errors(
                                guard,
                                vec![err.with_script(self.script_id.clone())].into_iter(),
                            );
                        }
                    };
                }

                system_state.apply(world);
            };

        let function_system = IntoSystem::into_system(_system.clone()).with_name(system_name);

        // dummy node id for now
        let reflect_system = ReflectSystem::from_system(&function_system, NodeId::System(0));

        // this is quite important, by default systems are placed in a set defined by their TYPE, i.e. in this case
        // all script systems would be the same
        let set = ScriptSystemSet::next();
        let mut config = IntoSystemConfigs::into_configs(function_system.in_set(set));

        // apply ordering
        for (other, is_before) in self
            .before
            .iter()
            .map(|b| (b, true))
            .chain(self.after.iter().map(|a| (a, false)))
        {
            match Self::get_individual_system_system_set(other.node_id.0, schedule) {
                Some(set) => {
                    if is_before {
                        config = config.before(Interned::<dyn SystemSet>(Box::leak(set)));
                    } else {
                        config = config.after(Interned::<dyn SystemSet>(Box::leak(set)));
                    }
                }
                None => {
                    bevy::log::warn!(
                        "Could not find system set for system {:?}",
                        other.identifier()
                    );
                }
            }
        }

        // const DEFAULT_ID: usize = 99999;
        // let node_id = NodeId::System(schedule.systems_len());

        // if matches!(node_id, NodeId::Set(DEFAULT_ID)) {
        //     bevy::log::warn!("Could not find node id for system {}", system_name);
        // }

        // schedule.configure_sets(AnonymousSet::new(9999 + schedule.systems_len()));
        schedule.configure_sets(set);
        schedule.add_systems(config);

        // reflect_system.node_id = ReflectNodeId(node_id);

        reflect_system
    }
}

#[profiling::all_functions]
/// Impls to do with dynamically querying systems and schedules
impl WorldAccessGuard<'_> {
    /// Retrieves all the systems in a schedule
    pub fn systems(&self, schedule: &ReflectSchedule) -> Result<Vec<ReflectSystem>, InteropError> {
        self.with_resource(|schedules: &Schedules| {
            let schedule = match schedules.get(*schedule.label) {
                Some(schedule) => schedule,
                None => return vec![],
            };

            let systems = match schedule.systems() {
                Ok(systems) => systems,
                Err(_) => return vec![],
            };

            systems
                .map(|(node_id, system)| ReflectSystem::from_system(system.as_ref(), node_id))
                .collect()
        })
    }

    // separates the borrows of schedules and an individual schedule
    fn scope_schedules<O, F: FnOnce(&mut World, &mut Schedule) -> O>(
        world: &mut World,
        label: &ReflectSchedule,
        f: F,
    ) -> Result<O, InteropError> {
        let mut schedules = world.get_resource_mut::<Schedules>().ok_or_else(|| {
            InteropError::unsupported_operation(
                None,
                None,
                "accessing schedules in a world with no schedules",
            )
        })?;

        let mut removed_schedule = schedules
            .remove(*label.label)
            .ok_or_else(|| InteropError::missing_schedule(label.identifier()))?;

        let result = f(world, &mut removed_schedule);

        let mut schedules = world.get_resource_mut::<Schedules>().ok_or_else(|| {
            InteropError::unsupported_operation(
                None,
                None,
                "re-inserting a schedule into a world with no schedules",
            )
        })?;

        schedules.insert(removed_schedule);

        Ok(result)
    }

    /// Creates a system from a system builder and inserts it into the given schedule
    pub fn add_system<P: IntoScriptPluginParams>(
        &self,
        schedule: &ReflectSchedule,
        builder: ScriptSystemBuilder,
    ) -> Result<ReflectSystem, InteropError> {
        self.with_global_access(|world| {
            bevy::log::debug!(
                "Adding script system '{}' for script '{}' to schedule '{}'",
                builder.name,
                builder.script_id,
                schedule.identifier()
            );

            Self::scope_schedules(world, schedule, |world, schedule| {
                let mut reflect_system = builder.build::<P>(schedule);
                schedule
                    .initialize(world)
                    .map_err(|e| InteropError::external_error(Box::new(e)))?;
                let node_id = schedule
                    .systems()
                    .unwrap()
                    .find_map(|(node_id, system)| {
                        (system.name() == reflect_system.path()).then_some(node_id)
                    })
                    .unwrap();
                reflect_system.node_id = ReflectNodeId(node_id);
                Ok(reflect_system)
            })?
        })?
    }
}

#[cfg(test)]
mod tests {

    use bevy::{
        app::{App, Update},
        asset::AssetPlugin,
        diagnostic::DiagnosticsPlugin,
        ecs::system::IntoSystem,
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
        assert_eq!(system.path(), "bevy_mod_scripting_core::bindings::schedule::tests::test_system_generic<alloc::string::String>");

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

    #[test]
    fn test_builder_creates_correctly_ordered_systems() {
        let mut app = App::new();
        app.add_plugins((
            AssetPlugin::default(),
            DiagnosticsPlugin,
            TestPlugin::default(),
        ));

        #[derive(ScheduleLabel, Hash, PartialEq, Eq, Debug, Clone)]
        struct TestSchedule;

        let root_system = || {};
        let as_system = IntoSystem::into_system(root_system).with_name("root1");
        let as_reflect_system = ReflectSystem::from_system(&as_system, NodeId::System(0));

        let root_system_2 = || {};
        let as_system_2 = IntoSystem::into_system(root_system_2).with_name("root2");
        let as_reflect_system_2 = ReflectSystem::from_system(&as_system_2, NodeId::System(1));

        let mut system_builder = ScriptSystemBuilder::new("test".into(), ScriptId::from("test"));

        system_builder
            .after(as_reflect_system)
            .before(as_reflect_system_2);

        app.init_schedule(TestSchedule);
        app.add_systems(TestSchedule, (as_system, as_system_2));
        let _ = system_builder.build::<TestPlugin>(app.get_schedule_mut(TestSchedule).unwrap());

        // read the graph of systems
        let mut schedules = app.world_mut().remove_resource::<Schedules>().unwrap();
        let mut test_schedule = schedules.remove(TestSchedule).unwrap();
        test_schedule.initialize(app.world_mut()).unwrap();
        let dag = test_schedule.graph().dependency();
        let graph = dag.graph();

        let mut name_to_node_id_map = HashMap::new();

        for (node_id, system) in test_schedule.systems().unwrap() {
            name_to_node_id_map.insert(system.name().clone(), node_id);
        }

        // make assertions on node id's

        assert!(graph.contains_node(name_to_node_id_map["root1"]));
        assert!(graph.contains_node(name_to_node_id_map["root2"]));
        assert!(graph.contains_node(name_to_node_id_map["script_system_test"]));

        let mut edges = Vec::default();
        for (from, to, _) in graph.all_edges() {
            let name_from = name_to_node_id_map
                .iter()
                .find_map(|(n, &id)| (id == from).then_some(n.to_string()))
                .unwrap_or_default();
            let name_to = name_to_node_id_map
                .iter()
                .find_map(|(n, &id)| (id == to).then_some(n.to_string()))
                .unwrap_or_default();
            edges.push((name_from, name_to));
        }

        // make assertions on edges
        assert!(
            edges.contains(&("root1".to_owned(), "script_system_test".to_owned())),
            "edge not found in: '{:?}'",
            edges
        );

        assert!(
            edges.contains(&("script_system_test".to_owned(), "root2".to_owned())),
            "edge not found in: '{:?}'",
            edges
        );
    }
}
