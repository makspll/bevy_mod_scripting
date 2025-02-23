//! Systems which are used to extract the various resources and components used by BMS.
//!
//! These are designed to be used to pipe inputs into other systems which require them, while handling any configuration erorrs nicely.

use bevy::ecs::{
    component::ComponentId,
    entity::Entity,
    query::{Access, AccessConflicts, FilteredAccessSet},
    storage::SparseSetIndex,
    system::{ResMut, SystemParam},
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

/// Context for systems which handle events for scripts
#[derive(SystemParam)]
pub struct HandlerContext<'w, P: IntoScriptPluginParams> {
    /// Settings for callbacks
    pub callback_settings: ResMut<'w, CallbackSettings<P>>,
    /// Settings for loading contexts
    pub context_loading_settings: ResMut<'w, ContextLoadingSettings<P>>,
    /// Scripts
    pub scripts: ResMut<'w, Scripts>,
    /// The runtime container
    pub runtime_container: ResMut<'w, RuntimeContainer<P>>,
    /// The script contexts
    pub script_contexts: ResMut<'w, ScriptContexts<P>>,
    /// List of static scripts
    pub static_scripts: ResMut<'w, StaticScripts>,
}

impl<'w, P: IntoScriptPluginParams> HandlerContext<'w, P> {
    /// Invoke a callback in a script immediately.
    pub fn call<C: IntoCallbackLabel>(
        &mut self,
        script_id: ScriptId,
        entity: Entity,
        payload: Vec<ScriptValue>,
        guard: WorldGuard<'w>,
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
    type State = (T::State, FilteredAccessSet<ComponentId>);

    type Item<'world, 'state> = WithWorldGuard<'world, 'state, T>;

    fn init_state(
        world: &mut bevy::ecs::world::World,
        system_meta: &mut bevy::ecs::system::SystemMeta,
    ) -> Self::State {
        let inner_state = T::init_state(world, system_meta);

        let accessed_components = system_meta.component_access_set().clone();
        unsafe { system_meta.component_access_set_mut().write_all() }
        unsafe { system_meta.archetype_component_access_mut().write_all() }
        (inner_state, accessed_components)
    }

    unsafe fn get_param<'world, 'state>(
        state: &'state mut Self::State,
        system_meta: &bevy::ecs::system::SystemMeta,
        world: bevy::ecs::world::unsafe_world_cell::UnsafeWorldCell<'world>,
        change_tick: bevy::ecs::component::Tick,
    ) -> Self::Item<'world, 'state> {
        // read components being read in the state, and lock those accesses in the new world access guard
        let guard = WorldAccessGuard::new(world.world_mut());
        let combined_access: Access<ComponentId> = state.1.combined_access().clone();

        #[allow(
            clippy::panic,
            reason = "This API does not allow us to handle this error nicely, and continuing is a safety issue."
        )]
        for (raid, is_write) in get_all_access_ids(&combined_access) {
            if is_write {
                if !guard.claim_write_access(raid) {
                    panic!("System tried to access set of system params which break rust aliasing rules. Aliasing access: {}", raid.display_with_world(guard.clone()));
                }
            } else if !guard.claim_read_access(raid) {
                panic!("System tried to access set of system params which break rust aliasing rules. Aliasing access: {}", raid.display_with_world(guard.clone()));
            }
        }

        WithWorldGuard {
            world_guard: guard,
            param: T::get_param(&mut state.0, system_meta, world, change_tick),
        }
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
        app::Update,
        ecs::{
            component::Component,
            system::{Query, Resource},
        },
    };

    use super::*;
    #[derive(Component)]
    struct Comp;

    #[derive(Resource)]
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
}
