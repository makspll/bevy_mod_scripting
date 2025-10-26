//! Systems which are used to extract the various resources and components used by BMS.
//!
//! These are designed to be used to pipe inputs into other systems which require them, while handling any configuration erorrs nicely.

use bevy_ecs::{
    archetype::Archetype,
    component::{ComponentId, Tick},
    query::{Access, AccessConflicts},
    storage::SparseSetIndex,
    system::{SystemMeta, SystemParam, SystemParamValidationError},
    world::{DeferredWorld, World, unsafe_world_cell::UnsafeWorldCell},
};
use bevy_mod_scripting_bindings::{WorldAccessGuard, WorldGuard, access_map::ReflectAccessId};

use fixedbitset::FixedBitSet;

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

    fn init_state(world: &mut World, system_meta: &mut SystemMeta) -> Self::State {
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
        system_meta: &SystemMeta,
        world: UnsafeWorldCell<'world>,
        change_tick: Tick,
    ) -> Self::Item<'world, 'state> {
        // create a guard which can only access the resources/components specified by the system.
        let guard = WorldAccessGuard::new_exclusive(unsafe { world.world_mut() });

        #[allow(
            clippy::panic,
            reason = "This API does not allow us to handle this error nicely, and continuing is a safety issue."
        )]
        for (raid, is_write) in &state.1 {
            if *is_write {
                if !guard.claim_write_access(*raid) {
                    panic!(
                        "System tried to access set of system params which break rust aliasing rules. Aliasing access: {raid:#?}",
                    );
                }
            } else if !guard.claim_read_access(*raid) {
                panic!(
                    "System tried to access set of system params which break rust aliasing rules. Aliasing access: {raid:#?}",
                );
            }
        }

        WithWorldGuard {
            world_guard: guard,
            param: unsafe { T::get_param(&mut state.0, system_meta, world, change_tick) },
        }
    }

    unsafe fn new_archetype(
        state: &mut Self::State,
        archetype: &Archetype,
        system_meta: &mut SystemMeta,
    ) {
        unsafe { T::new_archetype(&mut state.0, archetype, system_meta) }
    }

    fn apply(state: &mut Self::State, system_meta: &SystemMeta, world: &mut World) {
        T::apply(&mut state.0, system_meta, world)
    }

    fn queue(state: &mut Self::State, system_meta: &SystemMeta, world: DeferredWorld) {
        T::queue(&mut state.0, system_meta, world)
    }

    unsafe fn validate_param(
        state: &Self::State,
        system_meta: &SystemMeta,
        world: UnsafeWorldCell,
    ) -> Result<(), SystemParamValidationError> {
        unsafe { T::validate_param(&state.0, system_meta, world) }
    }
}

fn individual_conflicts(conflicts: AccessConflicts) -> FixedBitSet {
    match conflicts {
        // todo, not sure what to do here
        AccessConflicts::All => FixedBitSet::new(),
        AccessConflicts::Individual(fixed_bit_set) => fixed_bit_set,
    }
}

pub(crate) fn get_all_access_ids(access: &Access<ComponentId>) -> Vec<(ReflectAccessId, bool)> {
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
    use crate::config::{GetPluginThreadConfig, ScriptingPluginConfiguration};
    use bevy_ecs::resource::Resource;
    use bevy_mod_scripting_bindings::ScriptValue;
    use test_utils::make_test_plugin;

    use {
        bevy_app::{App, Plugin, Update},
        bevy_ecs::{
            component::Component,
            entity::Entity,
            event::{Event, EventReader},
            system::{Query, ResMut},
        },
    };

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
            assert!(
                !guard.claim_read_access(
                    ReflectAccessId::for_resource::<Res>(&guard.as_unsafe_world_cell().unwrap())
                        .unwrap()
                )
            );
            assert!(
                !guard.claim_write_access(
                    ReflectAccessId::for_resource::<Res>(&guard.as_unsafe_world_cell().unwrap())
                        .unwrap()
                )
            );
        };

        let mut app = App::new();
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

        let mut app = App::new();
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

        let mut app = App::new();
        app.add_systems(Update, system_fn);
        app.insert_resource(Res);
        app.world_mut().spawn(Comp);

        app.cleanup();
        app.finish();
        app.update();
    }
}
