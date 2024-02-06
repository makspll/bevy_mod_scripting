use std::collections::HashSet;

use bevy::{ecs::system::SystemState, prelude::*};
use bevy_event_priority::PriorityEventReader;

use crate::{
    event::ScriptLoaded,
    prelude::{APIProviders, Script, ScriptCollection, ScriptContexts, ScriptData, ScriptHost},
    ScriptErrorEvent,
};

/// Labels for scripting related systems
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, SystemSet)]
pub enum ScriptSystemSet {
    /// event handling systems are always marked with this label
    EventHandling,
}

/// Handles creating contexts for new/modified scripts
/// Scripts are likely not loaded instantly at this point, so most of the time
/// this system simply inserts an empty context
pub fn script_add_synchronizer<H: ScriptHost + 'static>(world: &mut World) {
    debug!("Handling addition/modification of scripts");

    let mut state: CachedScriptLoadState<H> = world.remove_resource().unwrap();

    // Entity,
    // &'static ScriptCollection<H::ScriptAsset>,
    // Ref<'static, ScriptCollection<H::ScriptAsset>>,

    let script_assets: Assets<H::ScriptAsset> = world.remove_resource().unwrap();
    let mut contexts: ScriptContexts<H::ScriptContext> = world.remove_resource().unwrap();
    let mut host: H = world.remove_resource().unwrap();
    let mut providers: APIProviders<H> = world.remove_resource().unwrap();

    let query: Vec<_> = {
        let mut q = vec![];
        let changed = state.scripts_changed_query.get(world);
        for (entity, new_scripts, tracker) in changed.iter() {
            q.push((entity, new_scripts.scripts.to_vec(), tracker.is_added()))
        }
        q
    };
    world.insert_resource(state);

    for (entity, new_scripts, tracker) in query.iter() {
        if *tracker {
            for new_script in new_scripts {
                Script::<H::ScriptAsset>::insert_new_script_context::<H>(
                    world,
                    &mut host,
                    new_script,
                    *entity,
                    &script_assets,
                    &mut providers,
                    &mut contexts,
                )
            }
        } else {
            // changed but structure already exists in contexts
            // find out what's changed
            // we only care about added or removed scripts here
            // if the script asset gets changed we deal with that elsewhere

            let context_ids = contexts
                .context_entities
                .iter()
                .filter_map(|(sid, (e, _, _))| if e == entity { Some(sid) } else { None })
                .cloned()
                .collect::<HashSet<u32>>();
            let script_ids = new_scripts.iter().map(|s| s.id()).collect::<HashSet<u32>>();

            let removed_scripts = context_ids.difference(&script_ids);
            let added_scripts = script_ids.difference(&context_ids);

            for r in removed_scripts {
                contexts.remove_context(*r);
            }

            for a in added_scripts {
                let script = new_scripts.iter().find(|e| &e.id() == a).unwrap();
                Script::<H::ScriptAsset>::insert_new_script_context::<H>(
                    world,
                    &mut host,
                    script,
                    *entity,
                    &script_assets,
                    &mut providers,
                    &mut contexts,
                )
            }
        }
    }

    // return ownership
    world.insert_resource(script_assets);
    world.insert_resource(contexts);
    world.insert_resource(host);
    world.insert_resource(providers);
}

/// Handles the removal of script components and their contexts
pub fn script_remove_synchronizer<H: ScriptHost>(
    mut query: RemovedComponents<ScriptCollection<H::ScriptAsset>>,
    mut contexts: ResMut<ScriptContexts<H::ScriptContext>>,
) {
    for v in query.iter() {
        // we know that this entity used to have a script component
        // ergo a script context must exist in ctxts, remove all scripts on the entity
        let script_ids = contexts
            .context_entities
            .iter()
            .filter_map(|(script_id, (entity, ..))| {
                (entity.index() == v.index()).then_some(*script_id)
            })
            .collect::<Vec<_>>();
        for script_id in script_ids {
            contexts.remove_context(script_id);
        }
    }
}

/// Reloads hot-reloaded scripts, or loads missing contexts for scripts which were added but not loaded
pub fn script_hot_reload_handler<H: ScriptHost>(world: &mut World) {
    let mut state: CachedScriptLoadState<H> = world.remove_resource().unwrap();

    let events = {
        state
            .event_state
            .get_mut(world)
            .1
            .iter()
            .filter_map(|e| match e {
                AssetEvent::Modified { handle } => Some((handle.clone(), false)),
                AssetEvent::Created { handle } => Some((handle.clone(), true)),
                _ => None,
            })
            .collect::<Vec<_>>()
    };
    // collect all asset events up front
    // let events = events.iter().collect::<Vec<H::ScriptAsset>>();
    // collect all scripts from query upfront
    let scripts = state
        .scripts_query
        .get(world)
        .iter()
        .cloned()
        .collect::<Vec<_>>();

    world.insert_resource(state);

    let script_assets: Assets<H::ScriptAsset> = world.remove_resource().unwrap();
    let mut contexts: ScriptContexts<H::ScriptContext> = world.remove_resource().unwrap();
    let mut host: H = world.remove_resource().unwrap();
    let mut providers: APIProviders<H> = world.remove_resource().unwrap();

    for (handle, created) in events {
        // find script using this handle by handle id
        // whether this script was modified or created
        // if a script exists with this handle, we should reload it to load in a new context
        // which at this point will be either None or Some(outdated context)
        // both ways are fine
        for scripts in scripts.iter() {
            for script in scripts.scripts.iter() {
                // the script could have well loaded in the same frame that it was added
                // in that case it will have a context attached and we do not want to reload it
                if script.handle() == &handle && !(contexts.has_context(script.id()) && created) {
                    Script::<H::ScriptAsset>::reload_script::<H>(
                        world,
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
    world.insert_resource(script_assets);
    world.insert_resource(contexts);
    world.insert_resource(host);
    world.insert_resource(providers);
}

/// Lets the script host handle all script events
pub fn script_event_handler<H: ScriptHost, const MAX: u32, const MIN: u32>(world: &mut World) {
    // we need to collect the events to drop the borrow of the world
    let mut state: CachedScriptState<H> = world.remove_resource().unwrap();

    let events = state
        .event_state
        .get_mut(world)
        .0
        .iter_prio_range(MAX, MIN)
        .collect::<Vec<H::ScriptEvent>>();

    world.insert_resource(state);

    // should help a lot with performance on frames where no events are fired
    if events.is_empty() {
        return;
    }

    let mut ctxts: ScriptContexts<H::ScriptContext> = world.remove_resource().unwrap();

    let mut host: H = world.remove_resource().unwrap();
    let mut providers: APIProviders<H> = world.remove_resource().unwrap();

    // we need a resource scope to be able to simultaneously access the contexts as well
    // as provide world access to scripts
    // afaik there is not really a better way to do this in bevy just now
    let ctx_iter = ctxts
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
    host.handle_events(world, &events, ctx_iter, &mut providers);

    world.insert_resource(ctxts);
    world.insert_resource(host);
    world.insert_resource(providers);
}

#[derive(Resource)]
/// system state for exclusive systems dealing with script events
pub struct CachedScriptState<H: ScriptHost> {
    pub event_state: SystemState<(
        PriorityEventReader<'static, 'static, H::ScriptEvent>,
        EventWriter<'static, ScriptErrorEvent>,
        EventReader<'static, 'static, ScriptLoaded>,
    )>,
}

impl<H: ScriptHost> FromWorld for CachedScriptState<H> {
    fn from_world(world: &mut World) -> Self {
        Self {
            event_state: SystemState::new(world),
        }
    }
}

#[derive(Resource)]
/// system state for exclusive systems dealing with script load events
pub struct CachedScriptLoadState<H: ScriptHost> {
    pub event_state: SystemState<(
        EventWriter<'static, ScriptLoaded>,
        EventReader<'static, 'static, AssetEvent<H::ScriptAsset>>,
    )>,
    pub scripts_query:
        SystemState<Query<'static, 'static, &'static ScriptCollection<H::ScriptAsset>>>,
    pub scripts_changed_query: SystemState<
        Query<
            'static,
            'static,
            (
                Entity,
                &'static ScriptCollection<H::ScriptAsset>,
                Ref<'static, ScriptCollection<H::ScriptAsset>>,
            ),
            Changed<ScriptCollection<H::ScriptAsset>>,
        >,
    >,
}

impl<H: ScriptHost> FromWorld for crate::systems::CachedScriptLoadState<H> {
    fn from_world(world: &mut World) -> Self {
        Self {
            event_state: SystemState::new(world),
            scripts_query: SystemState::new(world),
            scripts_changed_query: SystemState::new(world),
        }
    }
}
