//! everything to do with dynamically added script systems

use super::{
    access_map::ReflectAccessId,
    function::{from::Val, into::IntoScript, script_function::AppScriptFunctionRegistry},
    schedule::{AppScheduleRegistry},
    script_value::ScriptValue,
    AppReflectAllocator, ReflectReference, ScriptQueryBuilder, ScriptQueryResult,
    ScriptResourceRegistration, WorldAccessGuard, WorldGuard,
};
use crate::{
    bindings::pretty_print::DisplayWithWorld,
    context::ContextLoadingSettings,
    error::{InteropError, ScriptError},
    event::CallbackLabel,
    extractors::get_all_access_ids,
    handler::CallbackSettings,
    runtime::RuntimeContainer,
    script::{ScriptId, Scripts},
    IntoScriptPluginParams,
};
use bevy::{
    ecs::{
        archetype::{ArchetypeComponentId, ArchetypeGeneration},
        component::{ComponentId, Tick},
        entity::Entity,
        query::{Access, FilteredAccess, FilteredAccessSet, QueryState},
        reflect::AppTypeRegistry,
        schedule::{IntoSystemConfigs, SystemSet},
        system::{IntoSystem, System},
        world::{unsafe_world_cell::UnsafeWorldCell, World},
    },
    reflect::{OffsetAccess, ParsedPath, Reflect},
    utils::hashbrown::HashSet,
};
use std::{any::TypeId, borrow::Cow, hash::Hash, marker::PhantomData, ops::Deref};
use bevy_system_reflection::{ReflectSchedule, ReflectSystem};
#[derive(Clone, Hash, PartialEq, Eq)]
/// a system set for script systems.
pub struct ScriptSystemSet(Cow<'static, str>);

impl std::fmt::Debug for ScriptSystemSet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str("ScriptSystem(")?;
        f.write_str(self.0.as_ref())?;
        f.write_str(")")?;
        Ok(())
    }
}

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

#[derive(Clone)]
enum ScriptSystemParamDescriptor {
    Res(ScriptResourceRegistration),
    EntityQuery(ScriptQueryBuilder),
}

/// A builder for systems living in scripts
#[derive(Reflect, Clone)]
#[reflect(opaque)]
pub struct ScriptSystemBuilder {
    pub(crate) name: CallbackLabel,
    pub(crate) script_id: ScriptId,
    before: Vec<ReflectSystem>,
    after: Vec<ReflectSystem>,
    system_params: Vec<ScriptSystemParamDescriptor>,
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
            system_params: vec![],
            is_exclusive: true,
        }
    }

    /// Adds a component access to the system
    pub fn query(&mut self, query: ScriptQueryBuilder) -> &mut Self {
        self.system_params
            .push(ScriptSystemParamDescriptor::EntityQuery(query));
        self
    }

    /// Adds a resource access to the system
    pub fn resource(&mut self, resource: ScriptResourceRegistration) -> &mut Self {
        self.system_params
            .push(ScriptSystemParamDescriptor::Res(resource));
        self
    }

    /// Sets the system to be exclusive, i.e. it will be able to access everything but cannot be parallelized.
    pub fn exclusive(&mut self, exclusive: bool) -> &mut Self {
        self.is_exclusive = exclusive;
        self
    }

    /// Adds a system to run before the script system
    pub fn before_system(&mut self, system: ReflectSystem) -> &mut Self {
        self.before.push(system);
        self
    }

    /// Adds a system to run after the script system
    pub fn after_system(&mut self, system: ReflectSystem) -> &mut Self {
        self.after.push(system);
        self
    }

    /// Builds the system and inserts it into the given schedule
    #[allow(deprecated)]
    pub fn build<P: IntoScriptPluginParams>(
        self,
        world: WorldGuard,
        schedule: &ReflectSchedule,
    ) -> Result<ReflectSystem, InteropError> {
        let o = world.scope_schedule(schedule, |world, schedule| {
            // this is different to a normal event handler
            // the system doesn't listen to events
            // it immediately calls a singular script with a predefined payload
            let before_systems = self.before.clone();
            let after_systems = self.after.clone();
            let system_name = self.name.to_string();

            // let system: DynamicScriptSystem<P> =
            //     IntoSystem::<(), (), (IsDynamicScriptSystem<P>, ())>::into_system(self);

            // dummy node id for now
            // let mut reflect_system = ReflectSystem::from_system(&system, NodeId::System(0));

            // this is quite important, by default systems are placed in a set defined by their TYPE, i.e. in this case
            // all script systems would be the same
            // let set = ScriptSystemSet::next();
            let mut config = IntoSystemConfigs::<IsDynamicScriptSystem<P>>::into_configs(self);

            // apply ordering
            for (other, is_before) in before_systems
                .into_iter()
                .map(|b| (b, true))
                .chain(after_systems.into_iter().map(|a| (a, false)))
            {
                for default_set in other.default_system_sets() {
                    if is_before {
                        bevy::log::info!("before {default_set:?}");
                        config = config.before(*default_set);
                    } else {                        bevy::log::info!("before {default_set:?}");
                        bevy::log::info!("after {default_set:?}");
                        config = config.after(*default_set);
                    }
                }
            }

            schedule.add_systems(config);
            schedule.initialize(world)?;

            let dot = bevy_system_reflection::schedule_to_dot_graph(schedule);

            std::fs::write(format!("/home/makspll/git/bevy_mod_scripting/{system_name}_post_update.dot"), dot).expect("Unable to write file");

            // now find the system
            let (node_id, system) = schedule
                .systems()?
                .find(|(_, b)| b.name().deref() == system_name)
                .ok_or_else(|| InteropError::invariant("After adding the system, it was not found in the schedule, could not return a reference to it"))?;
            


            Ok(ReflectSystem::from_system(system.as_ref(), node_id))

        })?;

        // #[allow(clippy::expect_used)]
        // world.with_global_access(|world| {

        //     let mut app = App::new();
        //     std::mem::swap(app.world_mut(), world);

        //     let dot = bevy_mod_debugdump::schedule_graph_dot(&mut app, PostUpdate, &Settings{
        //         ..Default::default()
        //     });
        //     // save to ./graph.dot
        //     std::fs::write("/home/makspll/git/bevy_mod_scripting/post_update.dot", dot).expect("Unable to write file");

        //     // swap worlds back
        //     std::mem::swap(app.world_mut(), world);
        // }).expect("");

        o
    }
}

struct DynamicHandlerContext<'w, P: IntoScriptPluginParams> {
    scripts: &'w Scripts<P>,
    callback_settings: &'w CallbackSettings<P>,
    context_loading_settings: &'w ContextLoadingSettings<P>,
    runtime_container: &'w RuntimeContainer<P>,
}

impl<'w, P: IntoScriptPluginParams> DynamicHandlerContext<'w, P> {
    #[allow(
        clippy::expect_used,
        reason = "cannot avoid panicking inside init_param due to Bevy API structure"
    )]
    pub fn init_param(world: &mut World, system: &mut FilteredAccessSet<ComponentId>) {
        let mut access = FilteredAccess::<ComponentId>::matches_nothing();
        let scripts_res_id = world
            .resource_id::<Scripts<P>>()
            .expect("Scripts resource not found");
        let callback_settings_res_id = world
            .resource_id::<CallbackSettings<P>>()
            .expect("CallbackSettings resource not found");
        let context_loading_settings_res_id = world
            .resource_id::<ContextLoadingSettings<P>>()
            .expect("ContextLoadingSettings resource not found");
        let runtime_container_res_id = world
            .resource_id::<RuntimeContainer<P>>()
            .expect("RuntimeContainer resource not found");

        access.add_resource_read(scripts_res_id);
        access.add_resource_read(callback_settings_res_id);
        access.add_resource_read(context_loading_settings_res_id);
        access.add_resource_read(runtime_container_res_id);

        system.add(access);
    }

    #[allow(
        clippy::expect_used,
        reason = "cannot avoid panicking inside get_param due to Bevy API structure"
    )]
    pub fn get_param(system: &UnsafeWorldCell<'w>) -> Self {
        unsafe {
            Self {
                scripts: system.get_resource().expect("Scripts resource not found"),
                callback_settings: system
                    .get_resource()
                    .expect("CallbackSettings resource not found"),
                context_loading_settings: system
                    .get_resource()
                    .expect("ContextLoadingSettings resource not found"),
                runtime_container: system
                    .get_resource()
                    .expect("RuntimeContainer resource not found"),
            }
        }
    }

    /// Call a dynamic label on a script
    pub fn call_dynamic_label(
        &self,
        label: &CallbackLabel,
        script_id: &ScriptId,
        entity: Entity,
        payload: Vec<ScriptValue>,
        guard: WorldGuard<'_>,
    ) -> Result<ScriptValue, ScriptError> {
        // find script
        let script = match self.scripts.scripts.get(script_id) {
            Some(script) => script,
            None => return Err(InteropError::missing_script(script_id.clone()).into()),
        };

        // call the script
        let handler = self.callback_settings.callback_handler;
        let pre_handling_initializers = &self
            .context_loading_settings
            .context_pre_handling_initializers;
        let runtime = &self.runtime_container.runtime;

        let mut context = script.context.lock();

        CallbackSettings::<P>::call(
            handler,
            payload,
            entity,
            script_id,
            label,
            &mut context,
            pre_handling_initializers,
            runtime,
            guard,
        )
    }
}

/// TODO: inline world guard into the system state, we should be able to re-use it
struct ScriptSystemState {
    type_registry: AppTypeRegistry,
    function_registry: AppScriptFunctionRegistry,
    schedule_registry: AppScheduleRegistry,
    allocator: AppReflectAllocator,
    subset: HashSet<ReflectAccessId>,
    callback_label: CallbackLabel,
    system_params: Vec<ScriptSystemParam>,
}

/// Equivalent of [`SystemParam`] but for dynamic systems, these are the kinds of things
/// that scripts can ask for access to and get passed in through dynamic script systems.
pub enum ScriptSystemParam {
    /// An exclusive resource access
    Res {
        /// The component ID of the resource
        component_id: ComponentId,
        /// The type ID of the resource
        type_id: TypeId,
    },
    /// A query which returns entities
    /// Boxed to reduce stack size
    EntityQuery {
        /// The internal state of the query
        query: Box<QueryState<Entity, ()>>,
    },
}

/// A system specified, created, and added by a script
pub struct DynamicScriptSystem<P: IntoScriptPluginParams> {
    name: Cow<'static, str>,
    exclusive: bool,
    /// The set of component accesses for this system. This is used to determine
    /// - soundness issues (e.g. multiple [`SystemParam`]s mutably accessing the same component)
    /// - ambiguities in the schedule (e.g. two systems that have some sort of conflicting access)
    pub(crate) component_access_set: FilteredAccessSet<ComponentId>,
    /// This [`Access`] is used to determine which systems can run in parallel with each other
    /// in the multithreaded executor.
    ///
    /// We use a [`ArchetypeComponentId`] as it is more precise than just checking [`ComponentId`]:
    /// for example if you have one system with `Query<&mut T, With<A>>` and one system with `Query<&mut T, With<B>>`
    /// they conflict if you just look at the [`ComponentId`] of `T`; but if there are no archetypes with
    /// both `A`, `B` and `T` then in practice there's no risk of conflict. By using [`ArchetypeComponentId`]
    /// we can be more precise because we can check if the existing archetypes of the [`World`]
    /// cause a conflict
    pub(crate) archetype_component_access: Access<ArchetypeComponentId>,
    pub(crate) last_run: Tick,
    target_script: ScriptId,
    archetype_generation: ArchetypeGeneration,
    system_param_descriptors: Vec<ScriptSystemParamDescriptor>,
    state: Option<ScriptSystemState>,
    _marker: std::marker::PhantomData<fn() -> P>,
}

/// A marker type distinguishing between vanilla and script system types
pub struct IsDynamicScriptSystem<P>(PhantomData<fn() -> P>);

impl<P: IntoScriptPluginParams> IntoSystem<(), (), IsDynamicScriptSystem<P>>
    for ScriptSystemBuilder
{
    type System = DynamicScriptSystem<P>;

    fn into_system(builder: Self) -> Self::System {
        Self::System {
            name: builder.name.to_string().into(),
            exclusive: builder.is_exclusive,
            archetype_generation: ArchetypeGeneration::initial(),
            system_param_descriptors: builder.system_params,
            last_run: Default::default(),
            target_script: builder.script_id,
            state: None,
            component_access_set: Default::default(),
            archetype_component_access: Default::default(),
            _marker: Default::default(),
        }
    }
}

impl<P: IntoScriptPluginParams> System for DynamicScriptSystem<P> {
    type In = ();

    type Out = ();

    fn name(&self) -> std::borrow::Cow<'static, str> {
        self.name.clone()
    }

    fn component_access(&self) -> &bevy::ecs::query::Access<bevy::ecs::component::ComponentId> {
        self.component_access_set.combined_access()
    }

    fn archetype_component_access(
        &self,
    ) -> &bevy::ecs::query::Access<bevy::ecs::archetype::ArchetypeComponentId> {
        &self.archetype_component_access
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

        #[allow(
            clippy::panic,
            reason = "cannot avoid panicking inside run_unsafe due to Bevy API structure"
        )]
        let state = match &mut self.state {
            Some(state) => state,
            None => panic!("System state not initialized!"),
        };

        let mut payload = Vec::with_capacity(state.system_params.len());

        let guard = WorldAccessGuard::new_non_exclusive(
            world,
            state.subset.clone(),
            state.type_registry.clone(),
            state.allocator.clone(),
            state.function_registry.clone(),
            state.schedule_registry.clone(),
        );

        // TODO: cache references which don't change once we have benchmarks
        for param in &mut state.system_params {
            match param {
                ScriptSystemParam::Res {
                    component_id,
                    type_id,
                } => {
                    let res_ref = ReflectReference {
                        base: super::ReflectBaseType {
                            type_id: *type_id,
                            base_id: super::ReflectBase::Resource(*component_id),
                        },
                        reflect_path: ParsedPath::from(Vec::<OffsetAccess>::default()),
                    };
                    payload.push(res_ref.into_script_inline_error(guard.clone()));
                }
                ScriptSystemParam::EntityQuery { query } => {
                    // TODO: is this the right way to use this world cell for queries?
                    let entities = query.iter_unchecked(world).collect::<Vec<_>>();
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

                    payload.push(results.into_script_inline_error(guard.clone()))
                }
            }
        }

        // now that we have everything ready, we need to run the callback on the targetted scripts
        // let's start with just calling the one targetted script

        let handler_ctxt = DynamicHandlerContext::<P>::get_param(&world);

        let result = handler_ctxt.call_dynamic_label(
            &state.callback_label,
            &self.target_script,
            Entity::from_raw(0),
            payload,
            guard.clone(),
        );

        // TODO: emit error events via commands, maybe accumulate in state instead and use apply
        match result {
            Ok(_) => {}
            Err(err) => {
                bevy::log::error!(
                    "Error in dynamic script system `{}`: {}",
                    self.name,
                    err.display_with_world(guard)
                )
            }
        }
    }

    fn initialize(&mut self, world: &mut bevy::ecs::world::World) {
        // we need to register all the:
        // - resources, simple just need the component ID's
        // - queries, more difficult the queries need to be built, and archetype access registered on top of component access

        // start with resources
        let mut subset = HashSet::default();
        let mut system_params = Vec::with_capacity(self.system_param_descriptors.len());
        for param in &self.system_param_descriptors {
            match param {
                ScriptSystemParamDescriptor::Res(res) => {
                    let component_id = res.resource_id;
                    let type_id = res.type_registration().type_id();

                    let system_param = ScriptSystemParam::Res {
                        component_id,
                        type_id,
                    };
                    system_params.push(system_param);

                    let mut access = FilteredAccess::<ComponentId>::matches_nothing();

                    access.add_resource_write(component_id);
                    self.component_access_set.add(access);
                    let raid = ReflectAccessId::for_component_id(component_id);
                    #[allow(
                        clippy::panic,
                        reason = "WIP, to be dealt with in validate params better, but panic will still remain"
                    )]
                    if subset.contains(&raid) {
                        panic!("Duplicate resource access in system: {:?}.", raid);
                    }
                    subset.insert(raid);
                }
                ScriptSystemParamDescriptor::EntityQuery(query) => {
                    let query = query.as_query_state::<Entity>(world);

                    // Safety: we are not removing
                    self.component_access_set
                        .add(query.component_access().clone());

                    let new_raids = get_all_access_ids(query.component_access().access())
                        .into_iter()
                        .map(|(a, _)| a)
                        .collect::<HashSet<_>>();

                    #[allow(
                        clippy::panic,
                        reason = "WIP, to be dealt with in validate params better, but panic will still remain"
                    )]
                    if !subset.is_disjoint(&new_raids) {
                        panic!("Non-disjoint query in dynamic system parameters.");
                    }

                    system_params.push(ScriptSystemParam::EntityQuery {
                        query: query.into(),
                    });
                    subset.extend(new_raids);
                }
            }
        }

        // TODO: access to internal resources, i.e. handler state
        DynamicHandlerContext::<P>::init_param(world, &mut self.component_access_set);

        self.state = Some(ScriptSystemState {
            type_registry: world.get_resource_or_init::<AppTypeRegistry>().clone(),
            function_registry: world
                .get_resource_or_init::<AppScriptFunctionRegistry>()
                .clone(),
            schedule_registry: world.get_resource_or_init::<AppScheduleRegistry>().clone(),
            allocator: world.get_resource_or_init::<AppReflectAllocator>().clone(),
            subset,
            callback_label: self.name.to_string().into(),
            system_params,
        })
    }

    fn update_archetype_component_access(
        &mut self,
        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell,
    ) {
        let archetypes = world.archetypes();

        let old_generation =
            std::mem::replace(&mut self.archetype_generation, archetypes.generation());

        if let Some(state) = &mut self.state {
            for archetype in &archetypes[old_generation..] {
                for param in &mut state.system_params {
                    if let ScriptSystemParam::EntityQuery { query } = param {
                        // SAFETY: The assertion above ensures that the param_state was initialized from `world`.
                        unsafe {
                            query.new_archetype(archetype, &mut self.archetype_component_access)
                        };
                    }
                }
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
