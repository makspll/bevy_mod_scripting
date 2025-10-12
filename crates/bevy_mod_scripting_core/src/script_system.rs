//! everything to do with dynamically added script systems

use crate::{
    IntoScriptPluginParams, callbacks::ScriptCallbacks, event::CallbackLabel,
    extractors::get_all_access_ids, handler::ScriptingHandler, script::ScriptContext,
};

use ::{
    bevy_ecs::{
        archetype::{ArchetypeComponentId, ArchetypeGeneration},
        component::{ComponentId, Tick},
        entity::Entity,
        query::{Access, FilteredAccess, FilteredAccessSet, QueryState},
        reflect::AppTypeRegistry,
        schedule::SystemSet,
        system::{System, SystemParamValidationError},
        world::{World, unsafe_world_cell::UnsafeWorldCell},
    },
    bevy_reflect::Reflect,
};
use bevy_app::DynEq;
use bevy_ecs::{
    schedule::{InternedSystemSet, IntoScheduleConfigs, Schedule, Schedules},
    system::SystemIn,
    world::DeferredWorld,
};
use bevy_log::{debug, error, info, warn_once};
use bevy_mod_scripting_bindings::{
    AppReflectAllocator, AppScheduleRegistry, AppScriptComponentRegistry,
    AppScriptFunctionRegistry, InteropError, IntoScript, ReflectAccessId, ReflectReference,
    ScriptQueryBuilder, ScriptQueryResult, ScriptResourceRegistration, Val, WorldAccessGuard,
    WorldGuard,
};
use bevy_mod_scripting_script::ScriptAttachment;
use bevy_system_reflection::{ReflectSchedule, ReflectSystem};
use std::{
    any::TypeId, borrow::Cow, collections::HashSet, hash::Hash, marker::PhantomData, ops::Deref,
};
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

#[profiling::all_functions]
impl ScriptSystemSet {
    /// Creates a new script system set
    pub fn new(id: impl Into<Cow<'static, str>>) -> Self {
        Self(id.into())
    }
}

#[profiling::all_functions]
impl SystemSet for ScriptSystemSet {
    fn dyn_clone(&self) -> Box<dyn SystemSet> {
        Box::new(self.clone())
    }

    fn as_dyn_eq(&self) -> &dyn DynEq {
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
    pub(crate) attachment: ScriptAttachment,
    before: Vec<ReflectSystem>,
    after: Vec<ReflectSystem>,
    system_params: Vec<ScriptSystemParamDescriptor>,
    is_exclusive: bool,
}

#[profiling::all_functions]
impl ScriptSystemBuilder {
    /// Creates a new script system builder
    pub fn new(name: CallbackLabel, attachment: ScriptAttachment) -> Self {
        Self {
            before: vec![],
            after: vec![],
            name,
            attachment,
            system_params: vec![],
            is_exclusive: false,
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
        world.scope_schedule(schedule, |world, schedule| {
            // this is different to a normal event handler
            // the system doesn't listen to events
            // it immediately calls a singular script with a predefined payload
            let before_systems = self.before.clone();
            let after_systems = self.after.clone();
            let system_name = self.name.to_string();

            // this is quite important, by default systems are placed in a set defined by their TYPE, i.e. in this case
            // all script systems would be the same

            let system: DynamicScriptSystem<P> = bevy_ecs::system::IntoSystem::into_system(self);
            let mut system_config = system.into_configs();
            // let mut system_config = <ScriptSystemBuilder as IntoScheduleConfigs<Box<(dyn System<In = (), Out = Result<(), BevyError>> + 'static)>, (Infallible, IsDynamicScriptSystem<P>)>>::into_configs(self);            // apply ordering
            for (other, is_before) in before_systems
                .into_iter()
                .map(|b| (b, true))
                .chain(after_systems.into_iter().map(|a| (a, false)))
            {
                for default_set in other.default_system_sets() {
                    if is_before {
                        info!("before {default_set:?}");
                        system_config = system_config.before(*default_set);
                    } else {
                        info!("before {default_set:?}");
                        info!("after {default_set:?}");
                        system_config = system_config.after(*default_set);
                    }
                }
            }

            schedule.add_systems(system_config);
            // TODO: the node id seems to always be system.len()
            // if this is slow, we can always just get the node id that way
            // and let the schedule initialize itself right before it gets run
            // for now I want to avoid not having the right ID as that'd be a pain
            schedule.initialize(world).map_err(InteropError::external)?;
            // now find the system
            let (node_id, system) = schedule
                .systems()
                .map_err(InteropError::external)?
                .find(|(_, b)| b.name().deref() == system_name)
                .ok_or_else(|| InteropError::invariant("After adding the system, it was not found in the schedule, could not return a reference to it"))?;
            Ok(ReflectSystem::from_system(system.as_ref(), node_id))
        })?
    }
}

/// TODO: inline world guard into the system state, we should be able to re-use it
struct ScriptSystemState<P: IntoScriptPluginParams> {
    type_registry: AppTypeRegistry,
    function_registry: AppScriptFunctionRegistry,
    schedule_registry: AppScheduleRegistry,
    component_registry: AppScriptComponentRegistry,
    allocator: AppReflectAllocator,
    subset: HashSet<ReflectAccessId>,
    callback_label: CallbackLabel,
    system_params: Vec<ScriptSystemParam>,
    script_contexts: ScriptContext<P>,
    script_callbacks: ScriptCallbacks<P>,
}

/// Equivalent of [`bevy_ecs::system::SystemParam`] but for dynamic systems, these are the kinds of things
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
        /// the components in correct order describing the necessary references
        components: Vec<(ComponentId, TypeId)>,
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
    target_attachment: ScriptAttachment,
    archetype_generation: ArchetypeGeneration,
    system_param_descriptors: Vec<ScriptSystemParamDescriptor>,
    state: Option<ScriptSystemState<P>>,
    _marker: std::marker::PhantomData<fn() -> P>,
}

/// A marker type distinguishing between vanilla and script system types
pub struct IsDynamicScriptSystem<P>(PhantomData<fn() -> P>);

#[profiling::all_functions]
impl<P: IntoScriptPluginParams> bevy_ecs::system::IntoSystem<(), (), IsDynamicScriptSystem<P>>
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
            target_attachment: builder.attachment,
            state: None,
            component_access_set: Default::default(),
            archetype_component_access: Default::default(),
            _marker: Default::default(),
        }
    }
}

#[profiling::all_functions]
impl<P: IntoScriptPluginParams> System for DynamicScriptSystem<P> {
    type In = ();

    type Out = ();

    fn name(&self) -> std::borrow::Cow<'static, str> {
        self.name.clone()
    }

    fn component_access(&self) -> &Access<ComponentId> {
        self.component_access_set.combined_access()
    }

    fn archetype_component_access(&self) -> &Access<ArchetypeComponentId> {
        &self.archetype_component_access
    }

    fn is_send(&self) -> bool {
        !self.is_exclusive()
    }

    fn is_exclusive(&self) -> bool {
        self.exclusive
    }

    fn has_deferred(&self) -> bool {
        false
    }

    unsafe fn run_unsafe(
        &mut self,
        _input: SystemIn<'_, Self>,
        world: UnsafeWorldCell,
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

        let guard = if self.exclusive {
            // safety: we are an exclusive system, therefore the cell allows us to do this
            let world = unsafe { world.world_mut() };
            WorldAccessGuard::new_exclusive(world)
        } else {
            unsafe {
                WorldAccessGuard::new_non_exclusive(
                    world,
                    state.subset.clone(),
                    state.type_registry.clone(),
                    state.allocator.clone(),
                    state.function_registry.clone(),
                    state.schedule_registry.clone(),
                    state.component_registry.clone(),
                )
            }
        };

        // TODO: cache references which don't change once we have benchmarks
        for param in &mut state.system_params {
            match param {
                ScriptSystemParam::Res {
                    component_id,
                    type_id,
                } => {
                    let res_ref = ReflectReference::new_resource_ref_by_id(*component_id, *type_id);
                    payload.push(res_ref.into_script_inline_error(guard.clone()));
                }
                ScriptSystemParam::EntityQuery { query, components } => {
                    // TODO: is this the right way to use this world cell for queries?
                    let entities = unsafe { query.iter_unchecked(world) }.collect::<Vec<_>>();
                    let results = entities
                        .into_iter()
                        .map(|entity| {
                            Val(ScriptQueryResult {
                                entity,
                                components: components
                                    .iter()
                                    .map(|(component_id, type_id)| {
                                        ReflectReference::new_component_ref_by_id(
                                            entity,
                                            *component_id,
                                            *type_id,
                                        )
                                    })
                                    .collect(),
                            })
                        })
                        .collect::<Vec<_>>();

                    payload.push(results.into_script_inline_error(guard.clone()))
                }
            }
        }

        // Now that we have everything ready, we need to run the callback on the
        // targetted scripts. Let's start with just calling the one targetted
        // script.

        let script_context = &state.script_contexts.read();

        if let Some(context) = script_context.get_context(&self.target_attachment) {
            let mut context = context.lock();
            let result = P::handle(
                payload,
                &self.target_attachment,
                &state.callback_label,
                &mut context,
                state.script_callbacks.clone(),
                guard.clone(),
            );
            drop(context);
            // TODO: Emit error events via commands, maybe accumulate in state
            // instead and use apply.
            match result {
                Ok(_) => {}
                Err(err) => {
                    error!("Error in dynamic script system `{}`: {:#?}", self.name, err)
                }
            }
        } else {
            warn_once!(
                "Dynamic script system `{}` could not find script for attachment: {}. It will not run until it's loaded.",
                self.name,
                self.target_attachment
            );
        }
    }

    fn initialize(&mut self, world: &mut World) {
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
                        panic!("Duplicate resource access in system: {raid:?}.");
                    }
                    subset.insert(raid);
                }
                ScriptSystemParamDescriptor::EntityQuery(query) => {
                    let components: Vec<_> = query
                        .components
                        .iter()
                        .map(|c| (c.component_id, c.type_registration().type_id()))
                        .collect();
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
                        components,
                    });
                    subset.extend(new_raids);
                }
            }
        }

        self.state = Some(ScriptSystemState {
            type_registry: world.get_resource_or_init::<AppTypeRegistry>().clone(),
            function_registry: world
                .get_resource_or_init::<AppScriptFunctionRegistry>()
                .clone(),
            schedule_registry: world.get_resource_or_init::<AppScheduleRegistry>().clone(),
            allocator: world.get_resource_or_init::<AppReflectAllocator>().clone(),
            component_registry: world
                .get_resource_or_init::<AppScriptComponentRegistry>()
                .clone(),
            subset,
            callback_label: self.name.to_string().into(),
            system_params,
            script_contexts: world.get_resource_or_init::<ScriptContext<P>>().clone(),
            script_callbacks: world.get_resource_or_init::<ScriptCallbacks<P>>().clone(),
        })
    }

    fn update_archetype_component_access(&mut self, world: UnsafeWorldCell) {
        let archetypes = world.archetypes();

        let old_generation =
            std::mem::replace(&mut self.archetype_generation, archetypes.generation());

        if let Some(state) = &mut self.state {
            for archetype in &archetypes[old_generation..] {
                for param in &mut state.system_params {
                    if let ScriptSystemParam::EntityQuery { query, .. } = param {
                        // SAFETY: The assertion above ensures that the param_state was initialized from `world`.
                        unsafe {
                            query.new_archetype(archetype, &mut self.archetype_component_access)
                        };
                    }
                }
            }
        }
    }

    fn check_change_tick(&mut self, change_tick: Tick) {
        let last_run = &mut self.last_run;
        let this_run = change_tick;

        let age = this_run.get().wrapping_sub(last_run.get());
        if age > Tick::MAX.get() {
            *last_run = Tick::new(this_run.get().wrapping_sub(Tick::MAX.get()));
        }
    }

    fn get_last_run(&self) -> Tick {
        self.last_run
    }

    fn set_last_run(&mut self, last_run: Tick) {
        self.last_run = last_run;
    }

    fn apply_deferred(&mut self, _world: &mut World) {}

    fn queue_deferred(&mut self, _world: DeferredWorld) {}

    unsafe fn validate_param_unsafe(
        &mut self,
        _world: UnsafeWorldCell,
    ) -> Result<(), SystemParamValidationError> {
        Ok(())
    }

    fn default_system_sets(&self) -> Vec<InternedSystemSet> {
        vec![ScriptSystemSet::new(self.name.clone()).intern()]
    }

    fn type_id(&self) -> TypeId {
        TypeId::of::<Self>()
    }

    fn validate_param(&mut self, world: &World) -> Result<(), SystemParamValidationError> {
        let world_cell = world.as_unsafe_world_cell_readonly();
        self.update_archetype_component_access(world_cell);
        // SAFETY:
        // - We have exclusive access to the entire world.
        // - `update_archetype_component_access` has been called.
        unsafe { self.validate_param_unsafe(world_cell) }
    }
}

/// A trait for managing script systems in schedules dynamically
pub trait ManageScriptSystems {
    /// Temporarilly removes the given schedule from the world, and calls the given function on it, then re-inserts it.
    ///
    /// Useful for initializing schedules, or modifying systems
    fn scope_schedule<O, F: FnOnce(&mut World, &mut Schedule) -> O>(
        &self,
        label: &ReflectSchedule,
        f: F,
    ) -> Result<O, InteropError>;

    /// Retrieves all the systems in a schedule
    fn systems(&self, schedule: &ReflectSchedule) -> Result<Vec<ReflectSystem>, InteropError>;

    /// Creates a system from a system builder and inserts it into the given schedule
    fn add_system<P: IntoScriptPluginParams>(
        &self,
        schedule: &ReflectSchedule,
        builder: ScriptSystemBuilder,
    ) -> Result<ReflectSystem, InteropError>;
}

impl ManageScriptSystems for WorldGuard<'_> {
    /// Temporarilly removes the given schedule from the world, and calls the given function on it, then re-inserts it.
    ///
    /// Useful for initializing schedules, or modifying systems
    fn scope_schedule<O, F: FnOnce(&mut World, &mut Schedule) -> O>(
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
    fn systems(&self, schedule: &ReflectSchedule) -> Result<Vec<ReflectSystem>, InteropError> {
        self.with_resource(|schedules: &Schedules| {
            let schedule = schedules
                .get(*schedule.label())
                .ok_or_else(|| InteropError::missing_schedule(schedule.identifier()))?;

            let systems = schedule.systems().map_err(InteropError::external)?;

            Ok(systems
                .map(|(node_id, system)| ReflectSystem::from_system(system.as_ref(), node_id))
                .collect())
        })?
    }

    /// Creates a system from a system builder and inserts it into the given schedule
    fn add_system<P: IntoScriptPluginParams>(
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
mod test {
    use ::{
        bevy_app::{App, MainScheduleOrder, Plugin, Update},
        bevy_asset::{AssetId, AssetPlugin, Handle},
        bevy_diagnostic::DiagnosticsPlugin,
        bevy_ecs::{
            entity::Entity,
            schedule::{ScheduleLabel, Schedules},
        },
    };
    use bevy_mod_scripting_bindings::ScriptValue;
    use test_utils::make_test_plugin;

    use crate::{
        BMSScriptingInfrastructurePlugin,
        config::{GetPluginThreadConfig, ScriptingPluginConfiguration},
    };

    use super::*;

    make_test_plugin!(crate);

    fn test_system_rust(_world: &mut World) {}

    #[test]
    fn test_script_system_with_existing_system_dependency_can_execute() {
        let mut app = App::new();
        #[derive(ScheduleLabel, Clone, Debug, Hash, PartialEq, Eq)]
        struct TestSchedule;

        app.add_plugins((
            AssetPlugin::default(),
            DiagnosticsPlugin,
            TestPlugin::default(),
            BMSScriptingInfrastructurePlugin::default(),
        ));
        app.init_schedule(TestSchedule);
        let mut main_schedule_order = app.world_mut().resource_mut::<MainScheduleOrder>();
        main_schedule_order.insert_after(Update, TestSchedule);
        app.add_systems(TestSchedule, test_system_rust);

        // run the app once
        app.finish();
        app.cleanup();
        app.update();

        // find existing rust system
        let test_system = app
            .world_mut()
            .resource_scope::<Schedules, _>(|_, schedules| {
                let (node_id, system) = schedules
                    .get(TestSchedule)
                    .unwrap()
                    .systems()
                    .unwrap()
                    .find(|(_, system)| system.name().contains("test_system_rust"))
                    .unwrap();

                ReflectSystem::from_system(system.as_ref(), node_id)
            });

        // now dynamically add script system via builder, without a matching script
        let mut builder = ScriptSystemBuilder::new(
            "test".into(),
            ScriptAttachment::StaticScript(Handle::Weak(AssetId::invalid())),
        );
        builder.before_system(test_system);

        let _ = builder
            .build::<TestPlugin>(
                WorldAccessGuard::new_exclusive(app.world_mut()),
                &ReflectSchedule::from_label(TestSchedule),
            )
            .unwrap();

        // now re-run app, expect no panicks
        app.update();
    }
}
