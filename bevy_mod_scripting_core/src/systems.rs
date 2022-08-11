use std::collections::HashSet;

use bevy::{prelude::{Query, Entity, ChangeTrackers, Changed, ResMut, Assets, Res, debug, RemovedComponents, EventReader, AssetEvent, Mut, World, EventWriter, FromWorld}, ecs::system::SystemState};
use bevy_event_priority::PriorityEventReader;

use crate::{prelude::{ScriptHost, ScriptCollection, APIProviders, ScriptContexts, Script, ScriptData}, ScriptErrorEvent};


/// Handles creating contexts for new/modified scripts
/// Scripts are likely not loaded instantly at this point, so most of the time
/// this system simply inserts an empty context
pub fn script_add_synchronizer<H: ScriptHost + 'static>(
    query: Query<
        (
            Entity,
            &ScriptCollection<H::ScriptAsset>,
            ChangeTrackers<ScriptCollection<H::ScriptAsset>>,
        ),
        Changed<ScriptCollection<H::ScriptAsset>>,
    >,
    mut host: ResMut<H>,
    mut providers: ResMut<APIProviders<H>>,
    script_assets: Res<Assets<H::ScriptAsset>>,
    mut contexts: ResMut<ScriptContexts<H::ScriptContext>>,
) {
    debug!("Handling addition/modification of scripts");

    query.for_each(|(entity, new_scripts, tracker)| {
        if tracker.is_added() {
            new_scripts.scripts.iter().for_each(|new_script| {
                Script::<H::ScriptAsset>::insert_new_script_context::<H>(
                    &mut host,
                    new_script,
                    entity,
                    &script_assets,
                    &mut providers,
                    &mut contexts,
                )
            })
        } else {
            // changed but structure already exists in contexts
            // find out what's changed
            // we only care about added or removed scripts here
            // if the script asset gets changed we deal with that elsewhere

            let context_ids = contexts
                .context_entities
                .iter()
                .filter_map(|(sid, (e, _, _))| if *e == entity { Some(sid) } else { None })
                .cloned()
                .collect::<HashSet<u32>>();
            let script_ids = new_scripts
                .scripts
                .iter()
                .map(|s| s.id())
                .collect::<HashSet<u32>>();

            let removed_scripts = context_ids.difference(&script_ids);
            let added_scripts = script_ids.difference(&context_ids);

            for r in removed_scripts {
                contexts.remove_context(*r);
            }

            for a in added_scripts {
                let script = new_scripts.scripts.iter().find(|e| &e.id() == a).unwrap();
                Script::<H::ScriptAsset>::insert_new_script_context::<H>(
                    &mut host,
                    script,
                    entity,
                    &script_assets,
                    &mut providers,
                    &mut contexts,
                )
            }
        }
    })
}

/// Handles the removal of script components and their contexts
pub fn script_remove_synchronizer<H: ScriptHost>(
    query: RemovedComponents<ScriptCollection<H::ScriptAsset>>,
    mut contexts: ResMut<ScriptContexts<H::ScriptContext>>,
) {
    query.iter().for_each(|v| {
        // we know that this entity used to have a script component
        // ergo a script context must exist in ctxts, remove all scripts on the entity
        contexts.remove_context(v.id());
    })
}

/// Reloads hot-reloaded scripts, or loads missing contexts for scripts which were added but not loaded
pub fn script_hot_reload_handler<H: ScriptHost>(
    mut events: EventReader<AssetEvent<H::ScriptAsset>>,
    mut host: ResMut<H>,
    scripts: Query<&ScriptCollection<H::ScriptAsset>>,
    script_assets: Res<Assets<H::ScriptAsset>>,
    mut providers: ResMut<APIProviders<H>>,
    mut contexts: ResMut<ScriptContexts<H::ScriptContext>>,
) {
    for e in events.iter() {
        let (handle, created) = match e {
            AssetEvent::Modified { handle } => (handle, false),
            AssetEvent::Created { handle } => (handle, true),
            _ => continue,
        };

        // find script using this handle by handle id
        // whether this script was modified or created
        // if a script exists with this handle, we should reload it to load in a new context
        // which at this point will be either None or Some(outdated context)
        // both ways are fine
        for scripts in scripts.iter() {
            for script in &scripts.scripts {
                // the script could have well loaded in the same frame that it was added
                // in that case it will have a context attached and we do not want to reload it
                if script.handle() == handle && !(contexts.has_context(script.id()) && created) {
                    Script::<H::ScriptAsset>::reload_script::<H>(
                        &mut host,
                        script,
                        &script_assets,
                        &mut providers,
                        &mut contexts,
                    );
                }
            }
        }
    }
}

/// Lets the script host handle all script events
pub fn script_event_handler<H: ScriptHost, const MAX: u32, const MIN: u32>(world: &mut World) {
    // we need to collect the events to drop the borrow of the world
    let events = world.resource_scope(|world, mut cached_state: Mut<CachedScriptEventState<H>>| {
        let (mut cached_state, _) = cached_state.event_state.get_mut(world);
        cached_state
            .iter_prio_range(MAX, MIN)
            .collect::<Vec<H::ScriptEvent>>()
    });

    // we need a resource scope to be able to simultaneously access the contexts as well
    // as provide world access to scripts
    // afaik there is not really a better way to do this in bevy just now
    world.resource_scope(|world, mut ctxts: Mut<ScriptContexts<H::ScriptContext>>| {
        let ctx_iter =
            ctxts
                .as_mut()
                .context_entities
                .iter_mut()
                .filter_map(|(sid, (entity, o, name))| {
                    let ctx = match o {
                        Some(v) => v,
                        None => return None,
                    };

                    Some((
                        ScriptData {
                            sid: *sid,
                            entity: *entity,
                            name,
                        },
                        ctx,
                    ))
                });
        world.resource_scope(|world, host: Mut<H>| host.handle_events(world, &events, ctx_iter));
    });
}


/// system state for exclusive systems dealing with script events
pub struct CachedScriptEventState<'w, 's, H: ScriptHost> {
    pub event_state: SystemState<(
        PriorityEventReader<'w, 's, H::ScriptEvent>,
        EventWriter<'w, 's, ScriptErrorEvent>,
    )>,
}

impl<'w, 's, H: ScriptHost> FromWorld for CachedScriptEventState<'w, 's, H> {
    fn from_world(world: &mut World) -> Self {
        Self {
            event_state: SystemState::new(world),
        }
    }
}
