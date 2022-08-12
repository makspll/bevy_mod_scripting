use std::collections::HashSet;

use bevy::{
    ecs::system::SystemState,
    prelude::{
        debug, AssetEvent, Assets, ChangeTrackers, Changed, Entity, EventReader, EventWriter,
        FromWorld, Mut, Query, RemovedComponents, Res, ResMut, SystemLabel, World,
    },
};
use bevy_event_priority::PriorityEventReader;

use crate::{
    event::ScriptLoaded,
    prelude::{APIProviders, Script, ScriptCollection, ScriptContexts, ScriptData, ScriptHost},
    world::WorldPointer,
    ScriptErrorEvent,
};

/// Labels for scripting related systems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemLabel)]
pub enum ScriptSystemLabel {
    /// event handling systems are always marked with this label
    EventHandling,
}

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
    mut event_writer: EventWriter<ScriptLoaded>,
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
                    &mut event_writer,
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
                    &mut event_writer,
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

/// Handles the setup of all scripts which were just loaded or reloaded
pub fn script_setup_handler<H: ScriptHost>(world: &mut World) {
    let mut state: CachedScriptState<H> = world.remove_resource().unwrap();
    let mut host: H = world.remove_resource().unwrap();
    let mut ctxts: ScriptContexts<H::ScriptContext> = world.remove_resource().unwrap();
    let mut providers: APIProviders<H> = world.remove_resource().unwrap();

    let events = state
        .event_state
        .get_mut(world)
        .2
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    for ScriptLoaded { sid } in events {
        let (entity, ctx, name) = ctxts
            .context_entities
            .get_mut(&sid)
            .expect("Script context was removed before it was fully loaded");

        host.setup_script(
            unsafe { WorldPointer::new(world) },
            &ScriptData {
                sid,
                entity: *entity,
                name,
            },
            ctx.as_mut()
                .expect("Loaded event was sent but context is missing"),
            &mut providers,
        )
        .expect("Failed to setup script");
    }

    world.insert_resource(state);
    world.insert_resource(host);
    world.insert_resource(ctxts);
    world.insert_resource(providers);
}

/// Reloads hot-reloaded scripts, or loads missing contexts for scripts which were added but not loaded
pub fn script_hot_reload_handler<H: ScriptHost>(
    mut events: EventReader<AssetEvent<H::ScriptAsset>>,
    mut host: ResMut<H>,
    scripts: Query<&ScriptCollection<H::ScriptAsset>>,
    script_assets: Res<Assets<H::ScriptAsset>>,
    mut providers: ResMut<APIProviders<H>>,
    mut contexts: ResMut<ScriptContexts<H::ScriptContext>>,
    mut event_writer: EventWriter<ScriptLoaded>,
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
                        &mut event_writer,
                    );
                }
            }
        }
    }
}

/// Lets the script host handle all script events
pub fn script_event_handler<H: ScriptHost, const MAX: u32, const MIN: u32>(world: &mut World) {
    // we need to collect the events to drop the borrow of the world
    let events = world.resource_scope(|world, mut cached_state: Mut<CachedScriptState<H>>| {
        let (mut cached_state, _, _) = cached_state.event_state.get_mut(world);
        cached_state
            .iter_prio_range(MAX, MIN)
            .collect::<Vec<H::ScriptEvent>>()
    });

    // should help a lot with performance on frames where no events are fired
    if events.is_empty() {
        return;
    }

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
        // safety: we have unique access to world, future accesses are protected
        // by the lock in the pointer
        world.resource_scope(|world, host: Mut<H>| {
            host.handle_events(unsafe { WorldPointer::new(world) }, &events, ctx_iter)
        });
    });
}

/// system state for exclusive systems dealing with script events
pub struct CachedScriptState<'w, 's, H: ScriptHost> {
    pub event_state: SystemState<(
        PriorityEventReader<'w, 's, H::ScriptEvent>,
        EventWriter<'w, 's, ScriptErrorEvent>,
        EventReader<'w, 's, ScriptLoaded>,
    )>,
}

impl<'w, 's, H: ScriptHost> FromWorld for CachedScriptState<'w, 's, H> {
    fn from_world(world: &mut World) -> Self {
        Self {
            event_state: SystemState::new(world),
        }
    }
}
