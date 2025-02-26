//! Systems which are used to extract the various resources and components used by BMS.
//!
//! These are designed to be used to pipe inputs into other systems which require them, while handling any configuration erorrs nicely.
#![allow(deprecated)]
use std::ops::{Deref, DerefMut};

use bevy::ecs::{
    component::ComponentId,
    entity::Entity,
    event::{Event, EventCursor, EventIterator, Events},
    query::{Access, AccessConflicts},
    storage::SparseSetIndex,
    system::{Local, Resource, SystemParam, SystemState},
    world::World,
};
use fixedbitset::FixedBitSet;

use crate::{
    bindings::{
        access_map::ReflectAccessId, pretty_print::DisplayWithWorld, script_value::ScriptValue,
        WorldAccessGuard, WorldGuard,
    },
    context::{ContextLoadingSettings, ScriptContexts},
    error::{InteropError, ScriptError},
    event::IntoCallbackLabel,
    handler::CallbackSettings,
    runtime::RuntimeContainer,
    script::{ScriptId, Scripts, StaticScripts},
    IntoScriptPluginParams,
};

/// Executes `system_state.get_mut` followed by `system_state.apply` after running the given closure, makes sure state is correctly handled in the context of an exclusive system.
/// Using system state with a handler ctxt without applying the state after will leave the world in an inconsistent state.
pub fn with_handler_system_state<
    P: IntoScriptPluginParams,
    F: FnOnce(WorldGuard, &mut HandlerContext<P>) -> O,
    O,
>(
    world: &mut World,
    f: F,
) -> O {
    let mut system_state: SystemState<WithWorldGuard<HandlerContext<P>>> = SystemState::new(world);
    let mut with_guard = system_state.get_mut(world);
    let (guard, handler_ctxt) = with_guard.get_mut();
    let o = f(guard, handler_ctxt);
    system_state.apply(world);
    o
}

/// Semantics of [`bevy::ecs::change_detection::Res`] but doesn't claim read or write on the world by removing the resource from it ahead of time.
///
/// Similar to using [`World::resource_scope`].
///
/// This is useful for interacting with scripts, since [`WithWorldGuard`] will ensure scripts cannot gain exclusive access to the world if *any* reads or writes
/// are claimed on the world. Removing the resource from the world lets you access it in the context of running scripts without blocking exclusive world access.
///
/// # Safety
/// - Because the resource is removed during the `get_param` call, if there is a conflicting resource access, this will be unsafe
/// - You must ensure you're only using this in combination with system parameters which will not read or write to this resource in `get_param`
pub(crate) struct ResScope<'state, T: Resource + Default>(pub &'state mut T);

impl<T: Resource + Default> Deref for ResScope<'_, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        self.0
    }
}

impl<T: Resource + Default> DerefMut for ResScope<'_, T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0
    }
}

unsafe impl<T: Resource + Default> SystemParam for ResScope<'_, T> {
    type State = (T, bool);

    type Item<'world, 'state> = ResScope<'state, T>;

    fn init_state(
        _world: &mut World,
        system_meta: &mut bevy::ecs::system::SystemMeta,
    ) -> Self::State {
        system_meta.set_has_deferred();
        (T::default(), false)
    }

    unsafe fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        _system_meta: &bevy::ecs::system::SystemMeta,
        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'world>,
        _change_tick: bevy::ecs::component::Tick,
    ) -> Self::Item<'world, 'state> {
        state.1 = true;
        if let Some(mut r) = world.get_resource_mut::<T>() {
            std::mem::swap(&mut state.0, &mut r);
        }
        ResScope(&mut state.0)
    }

    fn apply(
        state: &mut Self::State,
        _system_meta: &bevy::ecs::system::SystemMeta,
        world: &mut bevy::ecs::world::World,
    ) {
        if state.1 {
            world.insert_resource(std::mem::take(&mut state.0));
            state.1 = false;
        }
    }
}

/// A version of [`bevy::ecs::event::EventReader`] which behaves just like [`ResScope`].
///
/// # Safety
/// - unsafe to use this in a way which violates the invariants on [`ResScope`].
/// - This is hidden from docs for a reason, rust doesn't allow expressing `type signature unsafety`
/// - It is only safe to use when other system parameters do not create aliasing references inside their `get_param` calls
#[derive(SystemParam)]
#[doc(hidden)]
#[deprecated(note = "This type is unsafe to use in systems")]
pub struct EventReaderScope<'s, T: Event> {
    events: ResScope<'s, Events<T>>,
    reader: Local<'s, EventCursor<T>>,
}

#[allow(deprecated)]
impl<T: Event> EventReaderScope<'_, T> {
    /// Read all events that happened since the last read
    pub fn read(&mut self) -> EventIterator<'_, T> {
        self.reader.read(&self.events)
    }
}

/// Context for systems which handle events for scripts
#[derive(SystemParam)]
pub struct HandlerContext<'s, P: IntoScriptPluginParams> {
    /// Settings for callbacks
    pub(crate) callback_settings: ResScope<'s, CallbackSettings<P>>,
    /// Settings for loading contexts
    pub(crate) context_loading_settings: ResScope<'s, ContextLoadingSettings<P>>,
    /// Scripts
    pub(crate) scripts: ResScope<'s, Scripts>,
    /// The runtime container
    pub(crate) runtime_container: ResScope<'s, RuntimeContainer<P>>,
    /// The script contexts
    pub(crate) script_contexts: ResScope<'s, ScriptContexts<P>>,
    /// List of static scripts
    pub(crate) static_scripts: ResScope<'s, StaticScripts>,
}

impl<P: IntoScriptPluginParams> HandlerContext<'_, P> {
    /// Get the callback settings
    pub fn callback_settings(&mut self) -> &mut CallbackSettings<P> {
        &mut self.callback_settings
    }

    /// Get the context loading settings
    pub fn context_loading_settings(&mut self) -> &mut ContextLoadingSettings<P> {
        &mut self.context_loading_settings
    }

    /// Get the scripts
    pub fn scripts(&mut self) -> &mut Scripts {
        &mut self.scripts
    }

    /// Get the runtime container
    pub fn runtime_container(&mut self) -> &mut RuntimeContainer<P> {
        &mut self.runtime_container
    }

    /// Get the script contexts
    pub fn script_contexts(&mut self) -> &mut ScriptContexts<P> {
        &mut self.script_contexts
    }

    /// Get the static scripts
    pub fn static_scripts(&mut self) -> &mut StaticScripts {
        &mut self.static_scripts
    }

    /// checks if the script is loaded such that it can be executed.
    pub fn is_script_fully_loaded(&self, script_id: ScriptId) -> bool {
        // check script exists in scripts and contexts
        let script = match self.scripts.scripts.get(&script_id) {
            Some(script) => script,
            None => {
                return false;
            }
        };

        self.script_contexts
            .contexts
            .contains_key(&script.context_id)
    }

    /// Invoke a callback in a script immediately.
    ///
    /// This will return [`crate::error::InteropErrorInner::MissingScript`] or [`crate::error::InteropErrorInner::MissingContext`] errors while the script is loading.
    /// Run [`Self::is_script_fully_loaded`] before calling the script to ensure the script and context were loaded ahead of time.
    pub fn call<C: IntoCallbackLabel>(
        &mut self,
        script_id: ScriptId,
        entity: Entity,
        payload: Vec<ScriptValue>,
        guard: WorldGuard<'_>,
    ) -> Result<ScriptValue, ScriptError> {
        // find script
        let script = match self.scripts.scripts.get(&script_id) {
            Some(script) => script,
            None => return Err(InteropError::missing_script(script_id).into()),
        };

        // find context
        let context = match self.script_contexts.contexts.get_mut(&script.context_id) {
            Some(context) => context,
            None => return Err(InteropError::missing_context(script.context_id, script_id).into()),
        };

        // call the script
        let handler = self.callback_settings.callback_handler;
        let pre_handling_initializers = &self
            .context_loading_settings
            .context_pre_handling_initializers;
        let runtime = &mut self.runtime_container.runtime;
        CallbackSettings::<P>::call(
            handler,
            payload,
            entity,
            &script_id,
            &C::into_callback_label(),
            context,
            pre_handling_initializers,
            runtime,
            guard,
        )
    }
}

/// A wrapper around a world which pre-populates access, to safely co-exist with other system params,
/// acts exactly like `&mut World` so this should be your only top-level system param
///
/// The reason is the guard needs to know the underlying access that
pub struct WithWorldGuard<'w, 's, T: SystemParam> {
    world_guard: WorldGuard<'w>,
    param: T::Item<'w, 's>,
}

impl<'w, 's, T: SystemParam> WithWorldGuard<'w, 's, T> {
    /// Get the world guard and the inner system param
    pub fn get(&self) -> (WorldGuard<'w>, &T::Item<'w, 's>) {
        (self.world_guard.clone(), &self.param)
    }

    /// Get the world guard and the inner system param mutably
    pub fn get_mut(&mut self) -> (WorldGuard<'w>, &mut T::Item<'w, 's>) {
        (self.world_guard.clone(), &mut self.param)
    }
}

unsafe impl<T: SystemParam> SystemParam for WithWorldGuard<'_, '_, T> {
    type State = (T::State, Vec<(ReflectAccessId, bool)>);

    type Item<'world, 'state> = WithWorldGuard<'world, 'state, T>;

    fn init_state(
        world: &mut bevy::ecs::world::World,
        system_meta: &mut bevy::ecs::system::SystemMeta,
    ) -> Self::State {
        // verify there are no accesses previously
        let other_accessed_components =
            system_meta.component_access_set().combined_access().clone();

        let inner_state = T::init_state(world, system_meta);

        let accessed_components = system_meta.component_access_set().combined_access();
        let access_ids = get_all_access_ids(accessed_components);
        let other_access_ids = get_all_access_ids(&other_accessed_components);

        // reason: we can't handle this error nicely, and continuing is a safety issue
        #[allow(clippy::panic)]
        if !other_access_ids.is_empty() {
            panic!(
                "WithWorldGuard must be the only top-level system param, cannot run system: `{}`",
                system_meta.name()
            );
        }

        // Safety: not removing any accesses
        unsafe { system_meta.component_access_set_mut().write_all() }
        unsafe { system_meta.archetype_component_access_mut().write_all() }
        (inner_state, access_ids)
    }

    unsafe fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        system_meta: &bevy::ecs::system::SystemMeta,
        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'world>,
        change_tick: bevy::ecs::component::Tick,
    ) -> Self::Item<'world, 'state> {
        // read components being read in the state, and lock those accesses in the new world access guard
        let guard = WorldAccessGuard::new(world.world_mut());

        #[allow(
            clippy::panic,
            reason = "This API does not allow us to handle this error nicely, and continuing is a safety issue."
        )]
        for (raid, is_write) in &state.1 {
            if *is_write {
                if !guard.claim_write_access(*raid) {
                    panic!("System tried to access set of system params which break rust aliasing rules. Aliasing access: {}", (*raid).display_with_world(guard.clone()));
                }
            } else if !guard.claim_read_access(*raid) {
                panic!("System tried to access set of system params which break rust aliasing rules. Aliasing access: {}", (*raid).display_with_world(guard.clone()));
            }
        }

        WithWorldGuard {
            world_guard: guard,
            param: T::get_param(&mut state.0, system_meta, world, change_tick),
        }
    }

    unsafe fn new_archetype(
        state: &mut Self::State,
        archetype: &bevy::ecs::archetype::Archetype,
        system_meta: &mut bevy::ecs::system::SystemMeta,
    ) {
        T::new_archetype(&mut state.0, archetype, system_meta)
    }

    fn apply(
        state: &mut Self::State,
        system_meta: &bevy::ecs::system::SystemMeta,
        world: &mut World,
    ) {
        T::apply(&mut state.0, system_meta, world)
    }

    fn queue(
        state: &mut Self::State,
        system_meta: &bevy::ecs::system::SystemMeta,
        world: bevy::ecs::world::DeferredWorld,
    ) {
        T::queue(&mut state.0, system_meta, world)
    }

    unsafe fn validate_param(
        state: &Self::State,
        system_meta: &bevy::ecs::system::SystemMeta,
        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell,
    ) -> bool {
        T::validate_param(&state.0, system_meta, world)
    }
}

fn individual_conflicts(conflicts: AccessConflicts) -> FixedBitSet {
    match conflicts {
        // todo, not sure what to do here
        AccessConflicts::All => FixedBitSet::new(),
        AccessConflicts::Individual(fixed_bit_set) => fixed_bit_set,
    }
}

fn get_all_access_ids(access: &Access<ComponentId>) -> Vec<(ReflectAccessId, bool)> {
    let mut access_all_read = Access::<ComponentId>::default();
    access_all_read.read_all();

    let mut access_all_write = Access::<ComponentId>::default();
    access_all_write.write_all();

    // read conflicts with each set to figure out the necessary locks

    let mut read = individual_conflicts(access.get_conflicts(&access_all_read));
    let written = individual_conflicts(access.get_conflicts(&access_all_write));

    // remove reads from writes
    read.difference_with(&written);

    let mut result = Vec::new();
    for c in read.ones() {
        result.push((
            ReflectAccessId::for_component_id(ComponentId::get_sparse_set_index(c)),
            false,
        ));
    }
    for c in written.ones() {
        result.push((
            ReflectAccessId::for_component_id(ComponentId::get_sparse_set_index(c)),
            true,
        ));
    }

    result
}

#[cfg(test)]
mod test {
    use bevy::{
        app::{App, Update},
        ecs::{
            component::Component,
            event::{Event, EventReader},
            system::{Query, ResMut, Resource},
            world::FromWorld,
        },
    };
    use test_utils::make_test_plugin;

    use super::*;

    make_test_plugin!(crate);

    #[derive(Component)]
    struct Comp;

    #[derive(Resource, Default)]
    struct Res;

    #[test]
    pub fn check_with_world_correctly_locks_resource_and_component() {
        let system_fn = |mut guard: WithWorldGuard<(ResMut<Res>, Query<&'static Comp>)>| {
            let (guard, (_res, _entity)) = guard.get_mut();
            assert_eq!(guard.list_accesses().len(), 2, "Expected 2 accesses");
            assert!(!guard.claim_read_access(
                ReflectAccessId::for_resource::<Res>(&guard.as_unsafe_world_cell().unwrap())
                    .unwrap()
            ));
            assert!(!guard.claim_write_access(
                ReflectAccessId::for_resource::<Res>(&guard.as_unsafe_world_cell().unwrap())
                    .unwrap()
            ));
        };

        let mut app = bevy::app::App::new();
        app.add_systems(Update, system_fn);
        app.insert_resource(Res);
        app.world_mut().spawn(Comp);

        app.cleanup();
        app.finish();
        app.update();
    }

    #[test]
    #[should_panic(
        expected = "WithWorldGuard must be the only top-level system param, cannot run system"
    )]
    pub fn check_with_world_panics_when_used_with_resource_top_level() {
        let system_fn = |_res: ResMut<Res>, mut _guard: WithWorldGuard<Query<&'static Comp>>| {};

        let mut app = bevy::app::App::new();
        app.add_systems(Update, system_fn);
        app.insert_resource(Res);
        app.world_mut().spawn(Comp);

        app.cleanup();
        app.finish();
        app.update();
    }

    #[test]
    #[should_panic(
        expected = "WithWorldGuard must be the only top-level system param, cannot run system"
    )]
    pub fn check_with_world_panics_when_used_with_event_reader_top_level() {
        #[derive(Event)]
        struct TestEvent;
        let system_fn =
            |_res: EventReader<TestEvent>, mut _guard: WithWorldGuard<Query<&'static Comp>>| {};

        let mut app = bevy::app::App::new();
        app.add_systems(Update, system_fn);
        app.insert_resource(Res);
        app.world_mut().spawn(Comp);

        app.cleanup();
        app.finish();
        app.update();
    }

    #[test]
    pub fn resscope_reinserts_resource() {
        // apply deffered system should be inserted after the system automatically
        let mut app = App::new();

        app.insert_resource(Res);
        app.add_systems(Update, |_: ResScope<Res>| {});

        app.update();

        // check the resources are re-inserted
        assert!(app.world().contains_resource::<Res>());
    }

    #[test]
    pub fn rescope_does_not_remove_until_system_call() {
        let mut world = World::new();
        world.insert_resource(Res);

        // this will call init, and that should't remove the resource
        assert!(world.contains_resource::<Res>());
        SystemState::<ResScope<Res>>::from_world(&mut world);
        assert!(world.contains_resource::<Res>());
    }
}
