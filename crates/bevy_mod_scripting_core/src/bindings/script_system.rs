//! everything to do with dynamically added script systems

use super::{
    function::{from::Val, into::IntoScript},
    schedule::{ReflectNodeId, ReflectSchedule, ReflectSystem},
    ReflectReference, ScriptQueryBuilder, ScriptQueryResult, ScriptResourceRegistration,
    WorldGuard,
};
use crate::{
    error::InteropError,
    event::CallbackLabel,
    extractors::{HandlerContext, WithWorldGuard},
    handler::handle_script_errors,
    script::ScriptId,
    IntoScriptPluginParams,
};
use bevy::{
    ecs::{
        archetype::ArchetypeGeneration,
        component::{ComponentId, Tick},
        entity::Entity,
        intern::Interned,
        query::{FilteredAccess, QueryState},
        schedule::{IntoSystemConfigs, NodeId, Schedule, SystemSet},
        system::{IntoSystem, System, SystemMeta, SystemState},
        world::{World, WorldId},
    },
    reflect::{OffsetAccess, ParsedPath, Reflect},
};
use std::{any::TypeId, borrow::Cow, hash::Hash};

#[derive(Debug, Clone, Hash, PartialEq, Eq)]
/// a system set for script systems.
pub struct ScriptSystemSet(Cow<'static, str>);

impl ScriptSystemSet {
    /// Creates a new script system set with a unique id
    pub fn new(id: impl Into<Cow<'static, str>>) -> Self {
        Self(id.into())
    }
}

impl SystemSet for ScriptSystemSet {
    fn dyn_clone(&self) -> bevy::ecs::label::Box<dyn SystemSet> {
        Box::new(self.clone())
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
    pub(crate) name: CallbackLabel,
    pub(crate) script_id: ScriptId,
    before: Vec<ReflectSystem>,
    after: Vec<ReflectSystem>,
    queries: Vec<ScriptQueryBuilder>,
    resources: Vec<ScriptResourceRegistration>,
    is_exclusive: bool,
}

impl ScriptSystemBuilder {
    /// Creates a new script system builder
    pub fn new(name: CallbackLabel, script_id: ScriptId) -> Self {
        Self {
            before: vec![],
            after: vec![],
            name,
            script_id,
            queries: vec![],
            resources: vec![],
            is_exclusive: true,
        }
    }

    /// Adds a component access to the system
    pub fn query(&mut self, query: ScriptQueryBuilder) -> &mut Self {
        self.queries.push(query);
        self
    }

    /// Adds a resource access to the system
    pub fn resource(&mut self, resource: ScriptResourceRegistration) -> &mut Self {
        self.resources.push(resource);
        self
    }

    /// Sets the system to be exclusive, i.e. it will be able to access everything but cannot be parallelized.
    pub fn exclusive(&mut self, exclusive: bool) -> &mut Self {
        self.is_exclusive = exclusive;
        self
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
    ) -> Option<Interned<dyn SystemSet>> {
        if let Ok(systems) = schedule.systems() {
            for (system_id, system) in systems {
                if system_id == node_id {
                    return system.default_system_sets().first().cloned();
                }
            }
        }

        None
    }

    /// Builds the system and inserts it into the given schedule
    #[allow(deprecated)]
    pub fn build<P: IntoScriptPluginParams>(
        self,
        world: WorldGuard,
        schedule: &ReflectSchedule,
    ) -> Result<ReflectSystem, InteropError> {
        world.scope_schedule(schedule, |world, schedule| {
            // this is different to a normal event handler
            // the system doesn't listen to events
            // it immediately calls a singular script with a predefined payload
            let system_name = format!("script_system_{}", &self.name);
            let _system = move |world: &mut World,
                                system_state: &mut SystemState<
                WithWorldGuard<HandlerContext<P>>,
            >| {
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
            let mut reflect_system =
                ReflectSystem::from_system(&function_system, NodeId::System(0));

            // this is quite important, by default systems are placed in a set defined by their TYPE, i.e. in this case
            // all script systems would be the same
            // let set = ScriptSystemSet::next();
            let mut config = IntoSystemConfigs::into_configs(function_system);

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
                            config = config.before(set);
                        } else {
                            config = config.after(set);
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

            // schedule.configure_sets(set);
            schedule.add_systems(config);

            schedule.initialize(world)?;

            let node_id = NodeId::System(schedule.systems_len());

            reflect_system.node_id = ReflectNodeId(node_id);

            Ok(reflect_system)
        })?
    }
}

/// A system specified, created, and added by a script
pub struct DynamicScriptSystem {
    name: Cow<'static, str>,
    exclusive: bool,
    resources: Vec<(ComponentId, TypeId)>,
    query_states: Vec<QueryState<Entity, ()>>,
    // pub(crate) param_state: Option<<F::Param as SystemParam>::State>,
    // pub(crate) state: Option<Vec<Res<>>>
    pub(crate) system_meta: SystemMeta,
    pub(crate) last_run: Tick,
    world_id: Option<WorldId>,
    target_script: ScriptId,
    archetype_generation: ArchetypeGeneration,
}

/// A marker type distinguishing between vanilla and script system types
pub struct IsDynamicScriptSystem;

impl<M> IntoSystem<(), (), (IsDynamicScriptSystem, M)> for ScriptSystemBuilder {
    type System = DynamicScriptSystem;

    fn into_system(builder: Self) -> Self::System {
        #[allow(clippy::todo, unreachable_code, reason = "temporary, due to WIP")]
        Self::System {
            name: builder.name.to_string().into(),
            exclusive: builder.is_exclusive,
            system_meta: todo!(),
            world_id: todo!(),
            archetype_generation: todo!(),
            resources: todo!(),
            query_states: todo!(),
            last_run: todo!(),
            target_script: todo!(),
        }
    }
}

impl System for DynamicScriptSystem {
    type In = ();

    type Out = ();

    fn name(&self) -> std::borrow::Cow<'static, str> {
        self.name.clone()
    }

    fn component_access(&self) -> &bevy::ecs::query::Access<bevy::ecs::component::ComponentId> {
        self.system_meta.component_access_set().combined_access()
    }

    fn archetype_component_access(
        &self,
    ) -> &bevy::ecs::query::Access<bevy::ecs::archetype::ArchetypeComponentId> {
        self.system_meta.archetype_component_access()
    }

    fn is_send(&self) -> bool {
        true
    }

    fn is_exclusive(&self) -> bool {
        self.exclusive
    }

    fn has_deferred(&self) -> bool {
        false
    }

    unsafe fn run_unsafe(
        &mut self,
        _input: bevy::ecs::system::SystemIn<'_, Self>,
        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell,
    ) -> Self::Out {
        let _change_tick = world.increment_change_tick();

        let mut payload = Vec::with_capacity(self.resources.len() + self.query_states.len());

        //TODO: this is unsafe, the guard must only be able to access what this system can
        // and also we cannot have a &mut World around in here, we're not an exclusive system
        let guard = WorldGuard::new(world.world_mut());

        for (comp_id, type_id) in self.resources.iter() {
            // make refs to these resources
            let res_ref = ReflectReference {
                base: super::ReflectBaseType {
                    type_id: *type_id,
                    base_id: super::ReflectBase::Resource(*comp_id),
                },
                reflect_path: ParsedPath::from(Vec::<OffsetAccess>::default()),
            };
            payload.push(res_ref.into_script(guard.clone()).unwrap());
        }

        for q in self.query_states.iter_mut() {
            // TODO: is this the right way to use this world cell for queries?
            let entities = q.iter_unchecked(world).collect::<Vec<_>>();
            let results = entities
                .into_iter()
                .map(|entity| {
                    Val(ScriptQueryResult {
                        entity,
                        // TODO: components
                        components: vec![],
                    })
                })
                .collect::<Vec<_>>();

            payload.push(results.into_script(guard.clone()).unwrap())
        }

        // now that we have everything ready, we need to run the callback on the targetted scripts
        // let's start with just calling the one targetted script
    }

    fn initialize(&mut self, _world: &mut bevy::ecs::world::World) {
        // we need to register all the:
        // - resources, simple just need the component ID's
        // - queries, more difficult the queries need to be built, and archetype access registered on top of component access

        // start with resources
        for (resource, _) in &self.resources {
            let mut access = FilteredAccess::<ComponentId>::matches_nothing();
            access.add_resource_write(*resource);
            // Safety: we are not removing
            unsafe { self.system_meta.component_access_set_mut() }.add(access);
        }

        // then go over the queries
        for query_state in &self.query_states {
            // Safety: we are not removing
            unsafe { self.system_meta.component_access_set_mut() }
                .add(query_state.component_access().clone());
        }

        // TODO: access to internal resources, i.e. handler state
    }

    fn update_archetype_component_access(
        &mut self,
        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell,
    ) {
        let archetypes = world.archetypes();

        let old_generation =
            std::mem::replace(&mut self.archetype_generation, archetypes.generation());

        for archetype in &archetypes[old_generation..] {
            for query_state in self.query_states.iter_mut() {
                // SAFETY: The assertion above ensures that the param_state was initialized from `world`.
                unsafe {
                    query_state
                        .new_archetype(archetype, self.system_meta.archetype_component_access_mut())
                };
            }
        }
    }

    fn check_change_tick(&mut self, change_tick: bevy::ecs::component::Tick) {
        let last_run = &mut self.last_run;
        let this_run = change_tick;

        let age = this_run.get().wrapping_sub(last_run.get());
        if age > Tick::MAX.get() {
            *last_run = Tick::new(this_run.get().wrapping_sub(Tick::MAX.get()));
        }
    }

    fn get_last_run(&self) -> bevy::ecs::component::Tick {
        self.last_run
    }

    fn set_last_run(&mut self, last_run: bevy::ecs::component::Tick) {
        self.last_run = last_run;
    }

    fn apply_deferred(&mut self, _world: &mut World) {}

    fn queue_deferred(&mut self, _world: bevy::ecs::world::DeferredWorld) {}

    unsafe fn validate_param_unsafe(
        &mut self,
        _world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell,
    ) -> bool {
        true
    }

    fn default_system_sets(&self) -> Vec<bevy::ecs::schedule::InternedSystemSet> {
        vec![ScriptSystemSet::new(self.name.clone()).intern()]
    }
}
