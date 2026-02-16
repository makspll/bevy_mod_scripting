use crate::script::Context;

use super::*;
use bevy_asset::AssetEvent;
use bevy_ecs::message::{MessageReader, MessageWriter};
use bevy_log::{debug, trace};

/// A handle to a script asset which can only be made from a strong handle
#[derive(Clone, Debug)]
pub struct StrongScriptHandle(Handle<ScriptAsset>);

// impl GetScriptHandle for ScriptAssetModifiedEvent {
//     fn get_script_handle(&self) -> Handle<ScriptAsset> {
//         self.0.clone()
//     }
// }

impl GetScriptHandle for ScriptAttachedEvent {
    fn get_script_handle(&self) -> Handle<ScriptAsset> {
        self.0.script()
    }
}

impl GetScriptHandle for ScriptDetachedEvent {
    fn get_script_handle(&self) -> Handle<ScriptAsset> {
        self.0.script()
    }
}

impl StrongScriptHandle {
    /// Creates a new strong script handle, only if the given handle is strong itself.
    pub fn new(handle: Handle<ScriptAsset>) -> Option<Self> {
        if handle.is_strong() {
            Some(Self(handle))
        } else {
            None
        }
    }

    /// create a strong script handle using the assets resource and the possibly weak handle.
    pub fn from_assets(
        handle: Handle<ScriptAsset>,
        assets: &mut Assets<ScriptAsset>,
    ) -> Option<Self> {
        match handle {
            s @ Handle::Strong(_) => Some(Self(s)),
            _ => assets.get_strong_handle(handle.id()).map(Self),
        }
    }

    /// Returns a reference to the strong handle inside
    pub fn handle(&self) -> &Handle<ScriptAsset> {
        &self.0
    }

    /// Retrieves the asset and guarantees a successful load given this is a strong handle
    pub fn get(&self, assets: &Assets<ScriptAsset>) -> ScriptAsset {
        assets.get(&self.0).cloned().unwrap_or_default()
    }
}

/// Generate [`ScriptAssetModifiedEvent`]'s from asset modification events, filtering to only forward those matching the plugin's language
pub fn filter_script_modifications<P: IntoScriptPluginParams>(
    mut events: MessageReader<AssetEvent<ScriptAsset>>,
    mut filtered: MessageWriter<ForPlugin<ScriptAssetModifiedEvent, P>>,
    assets: Res<Assets<ScriptAsset>>,
) {
    let mut batch = events.read().filter_map(|e| {
        if let AssetEvent::Modified { id } = e
            && let Some(asset) = assets.get(*id)
            && asset.language == P::LANGUAGE
        {
            Some(ForPlugin::new(ScriptAssetModifiedEvent(*id)))
        } else {
            None
        }
    });

    if let Some(next) = batch.next() {
        filtered.write_batch(std::iter::once(next).chain(batch));
    }
}

/// Filters incoming [`ScriptAttachedEvent`]'s leaving only those which match the plugin's language
pub fn filter_script_attachments<P: IntoScriptPluginParams>(
    mut events: LoadedWithHandles<ScriptAttachedEvent>,
    mut filtered: MessageWriter<ForPlugin<ScriptAttachedEvent, P>>,
) {
    let mut batch = events
        .get_loaded()
        .filter(|(_, _, l)| *l == P::LANGUAGE)
        .map(|(mut a, b, _)| {
            trace!(
                "dispatching script attachment event for: {a:?}, language: {}",
                P::LANGUAGE
            );
            *a.0.script_mut() = b.0;
            ForPlugin::new(a)
        });

    if let Some(next) = batch.next() {
        filtered.write_batch(std::iter::once(next).chain(batch));
    }
}

/// Filters incoming [`ScriptDetachedEvent`]'s leaving only those which are currently attached and match the plugin's language
pub fn filter_script_detachments<P: IntoScriptPluginParams>(
    mut events: MessageReader<ScriptDetachedEvent>,
    mut filtered: MessageWriter<ForPlugin<ScriptDetachedEvent, P>>,
    contexts: Res<ScriptContexts<P>>,
) {
    let contexts_guard = contexts.read();
    let mut batch = events
        .read()
        .filter(|e| contexts_guard.contains(&e.0))
        .cloned()
        .map(ForPlugin::new);

    if let Some(next) = batch.next() {
        trace!("dispatching script dettachments for plugin");
        filtered.write_batch(std::iter::once(next).chain(batch));
    }
}

/// Process [`ScriptAttachedEvent`]'s and generate loading machines with the [`LoadingInitialized`] and [`ReloadingInitialized`] states
pub fn process_attachments<P: IntoScriptPluginParams>(
    mut events: MessageReader<ForPlugin<ScriptAttachedEvent, P>>,
    mut machines: ResMut<ActiveMachines<P>>,
    mut assets: ResMut<Assets<ScriptAsset>>,
) {
    // let contexts = contexts.read();
    events.read().for_each(|wrapper| {
        let attachment_event = wrapper.event();
        let attachment = attachment_event.0.clone();
        debug!("received attachment event: {attachment_event:?}");
        let id = attachment_event.0.script();
        let mut context = MachineContext {
            attachment: attachment.clone(),
        };
        if let Some(strong_handle) = StrongScriptHandle::from_assets(id, &mut assets) {
            // we want the loading process to have access to asset paths
            *context.attachment.script_mut() = strong_handle.0.clone();
            let content = strong_handle.get(&assets);

            // we query for the contexts to decide if this is a reload or load at runtime
            // not when queueing, in case another machine before this one affects the state, we do need the asset though
            machines.queue_machine(context, move |w| {
                let contexts = w.get_resource_or_init::<ScriptContexts<P>>();
                let mut contexts = contexts.write();

                match contexts.get_context(&attachment) {
                    Some(Context::LoadedAndActive(context)) => {
                        if let Err((attachment, _)) =
                            contexts.insert(attachment.clone(), Context::Reloading(context.clone()))
                        {
                            bevy_log::warn!("Could not insert context for attachment {attachment}. Reloading interrupted.");
                        };

                        vec![Box::new(ReloadingInitialized {
                            attachment: attachment.clone(),
                            source: strong_handle.handle().clone(),
                            content: content.content,
                            existing_context: context,
                        })]
                    }
                    // context is in an invalid state
                    Some(_) => vec![],
                    None => {
                        if let Err((attachment, _)) =
                            contexts.insert(attachment.clone(), Context::Loading)
                        {
                            bevy_log::warn!("Could not insert context for attachment {attachment}. Loading interrupted.");
                        };

                        vec![Box::new(LoadingInitialized {
                            attachment: attachment.clone(),
                            source: strong_handle.handle().clone(),
                            content: content.content,
                        })]
                    }
                }
            });
        }
    });
}

/// Processes [`ScriptAttachedEvent`]'s and initialized unloading state machines with [`UnloadingInitialized`] states
pub fn process_detachments<P: IntoScriptPluginParams>(
    mut events: MessageReader<ForPlugin<ScriptDetachedEvent, P>>,
    mut machines: ResMut<ActiveMachines<P>>,
    contexts: Res<ScriptContexts<P>>,
) {
    events.read().for_each(|wrapper| {
        let attachment_event = wrapper.event();
        let attachment = &attachment_event.0;
        let contexts_guard = contexts.read();
        contexts_guard
            .get_context(&attachment_event.0)
            .into_iter()
            .for_each(|existing_context| {
                // for the borrow checker
                let attachment = attachment.clone();
                machines.queue_machine(
                    MachineContext {
                        attachment: attachment.clone(),
                    },
                    move |world| {
                        let contexts = world.get_resource_or_init::<ScriptContexts<P>>();
                        let mut contexts = contexts.write();

                        let existing_context = if let Some(context) = existing_context.as_loaded() {
                            context
                        } else {
                            // cannot unload a context which isn't loaded
                            return vec![];
                        };

                        if let Err((attachment, _)) =
                            contexts.insert(attachment.clone(), Context::Unloading(existing_context.clone()))
                        {
                            bevy_log::warn!("Could not insert context for attachment {attachment}. Unloading interrupted.");
                        };

                        vec![Box::new(UnloadingInitialized {
                                    attachment,
                                    existing_context: existing_context.clone(),
                                }) as Box<dyn MachineState<P>>
                            ]
                    },
                );
            })
    });
}

/// Processes [`ScriptAssetModifiedEvent`]'s and initializes loading state machines with [`ReloadingInitialized`] states
pub fn process_asset_modifications<P: IntoScriptPluginParams>(
    mut events: MessageReader<ForPlugin<ScriptAssetModifiedEvent, P>>,
    mut machines: ResMut<ActiveMachines<P>>,
    mut assets: ResMut<Assets<ScriptAsset>>,
    contexts: Res<ScriptContexts<P>>,
) {
    let affected_ids = events.read().map(|e| e.event().0).collect::<HashSet<_>>();

    let contexts = contexts.read();

    let affected_attachments = contexts
        .all_residents()
        .filter(|(a, _)| affected_ids.contains(&a.script().id()));

    affected_attachments
        .into_iter()
        .for_each(|(attachment, existing_context)| {
            let id = attachment.script();
            if let Some(strong_handle) = StrongScriptHandle::from_assets(id, &mut assets) {
                let content = strong_handle.get(&assets);

                machines.queue_machine(
                    MachineContext {
                        attachment: attachment.clone(),
                    },
                    move |_| {
                        existing_context
                            .as_loaded()
                            .map(|existing_context| {
                                Box::new(ReloadingInitialized {
                                    attachment,
                                    source: strong_handle.handle().clone(),
                                    content: content.content,
                                    existing_context: existing_context.clone(),
                                }) as Box<dyn MachineState<P>>
                            })
                            .into_iter()
                            .collect::<Vec<_>>()
                    },
                );
            }
        });
}
