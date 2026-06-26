//! Systems which are used to extract the various resources and components used by BMS.
//!
//! These are designed to be used to pipe inputs into other systems which require them, while handling any configuration erorrs nicely.

use bevy_ecs::{
    component::ComponentId,
    query::{Access, AccessConflicts, ComponentIdSet},
};

// /// A wrapper around a world which pre-populates access, to safely co-exist with other system params,
// /// acts exactly like `&mut World` so this should be your only top-level system param
// ///
// /// The reason is the guard needs to know the underlying access that
// pub struct WithWorldGuard<'w, 's, T: SystemParam> {
//     world_guard: WorldGuard<'w>,
//     param: T::Item<'w, 's>,
// }

// impl<'w, 's, T: SystemParam> WithWorldGuard<'w, 's, T> {
//     /// Get the world guard and the inner system param
//     pub fn get(&self) -> (WorldGuard<'w>, &T::Item<'w, 's>) {
//         (self.world_guard.clone(), &self.param)
//     }

//     /// Get the world guard and the inner system param mutably
//     pub fn get_mut(&mut self) -> (WorldGuard<'w>, &mut T::Item<'w, 's>) {
//         (self.world_guard.clone(), &mut self.param)
//     }
// }

// unsafe impl<T: SystemParam> SystemParam for WithWorldGuard<'_, '_, T> {
//     type State = (T::State, Vec<(ReflectAccessId, bool)>);

//     type Item<'world, 'state> = WithWorldGuard<'world, 'state, T>;

//     fn init_access(
//         _state: &Self::State,
//         _system_meta: &mut SystemMeta,
//         component_access_set: &mut bevy_ecs::query::FilteredAccessSet,
//         _world: &mut World,
//     ) {
//         // verify there are no accesses previously
//         // let other_accessed_components = component_access_set.combined_access().clone();

//         // let accessed_components = component_access_set.combined_access();
//         // let access_ids = get_all_access_ids(accessed_components);
//         // let other_access_ids = get_all_access_ids(&other_accessed_components);

//         // reason: we can't handle this error nicely, and continuing is a safety issue
//         // #[allow(clippy::panic)]
//         // if !other_access_ids.is_empty() {
//         //     panic!(
//         //         "WithWorldGuard must be the only top-level system param, cannot run system: `{}`",
//         //         system_meta.name()
//         //     );
//         // }

//         // Safety: not removing any accesses
//         component_access_set.write_all()
//     }

//     fn init_state(world: &mut World) -> Self::State {
//         // // verify there are no accesses previously
//         // let other_accessed_components =
//         //     system_meta.component_access_set().combined_access().clone();

//         // let inner_state = T::init_state(world);

//         // let accessed_components = system_meta.component_access_set().combined_access();
//         // let inner_state = T::init_access(T::init_state(world));
//         // let access_ids = get_all_access_ids(accessed_components);
//         // let other_access_ids = get_all_access_ids(&other_accessed_components);

//         (T::init_state(world), vec![])
//     }

//     unsafe fn get_param<'world, 'state>(
//         state: &'state mut (T::State, Vec<(ReflectAccessId, bool)>),
//         system_meta: &SystemMeta,
//         world: UnsafeWorldCell<'world>,
//         change_tick: Tick,
//     ) -> Self::Item<'world, 'state> {
//         if state.1.is_empty() {
//             T::init_access()
//         }

//         // create a guard which can only access the resources/components specified by the system.
//         let guard = WorldAccessGuard::new_exclusive(unsafe { world.world_mut() });

//         #[allow(
//             clippy::panic,
//             reason = "This API does not allow us to handle this error nicely, and continuing is a safety issue."
//         )]
//         for (raid, is_write) in &state.1 {
//             if *is_write {
//                 if !guard.claim_write_access(*raid) {
//                     panic!(
//                         "System tried to access set of system params which break rust aliasing rules. Aliasing access: {raid:#?}",
//                     );
//                 }
//             } else if !guard.claim_read_access(*raid) {
//                 panic!(
//                     "System tried to access set of system params which break rust aliasing rules. Aliasing access: {raid:#?}",
//                 );
//             }
//         }

//         WithWorldGuard {
//             world_guard: guard,
//             param: unsafe { T::get_param(&mut state.0, system_meta, world, change_tick) },
//         }
//     }

//     fn apply(state: &mut Self::State, system_meta: &SystemMeta, world: &mut World) {
//         T::apply(&mut state.0, system_meta, world)
//     }

//     fn queue(state: &mut Self::State, system_meta: &SystemMeta, world: DeferredWorld) {
//         T::queue(&mut state.0, system_meta, world)
//     }

//     unsafe fn validate_param(
//         state: &mut Self::State,
//         system_meta: &SystemMeta,
//         world: UnsafeWorldCell,
//     ) -> Result<(), SystemParamValidationError> {
//         unsafe { T::validate_param(&mut state.0, system_meta, world) }
//     }
// }

fn individual_conflicts(conflicts: AccessConflicts) -> ComponentIdSet {
    match conflicts {
        // todo, not sure what to do here
        AccessConflicts::All => ComponentIdSet::new(),
        AccessConflicts::Individual(fixed_bit_set) => fixed_bit_set,
    }
}

pub(crate) fn get_all_access_ids(access: &Access) -> Vec<(ComponentId, bool)> {
    let mut access_all_read = Access::default();
    access_all_read.read_all();

    let mut access_all_write = Access::default();
    access_all_write.write_all();

    // read conflicts with each set to figure out the necessary locks

    let mut read = individual_conflicts(access.get_conflicts(&access_all_read));
    let written = individual_conflicts(access.get_conflicts(&access_all_write));

    // remove reads from writes
    read.difference_with(&written);

    let mut result = Vec::new();
    for c in read.iter() {
        result.push((c, false));
    }
    for c in written.iter() {
        result.push((c, true));
    }

    result
}
