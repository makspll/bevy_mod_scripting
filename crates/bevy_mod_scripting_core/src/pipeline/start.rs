use crate::script::Context;

use super::*;
use bevy_asset::AssetEvent;
use bevy_ecs::{message::MessageReader, system::In};
use bevy_log::{debug, trace};
use bevy_mod_scripting_script::ScriptAttachment;

/// A handle to a script asset which can only be made from a strong handle
#[derive(Clone, Debug)]
pub struct StrongScriptHandle(Handle<ScriptAsset>);

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

/// Arguments used to filter out certain sync events
pub struct FilterAssetEventsSettings {
    /// Whether or not to trigger component sync events
    pub sync_components: bool,
    /// Whether or not to trigger hot load sync events
    pub hot_loads: bool,
}

/// A system that bridges various script events and the script processing pipeline.
pub fn filter_asset_events<P: IntoScriptPluginParams>(
    args: In<FilterAssetEventsSettings>,
    mut script_modified_events: MessageReader<AssetEvent<ScriptAsset>>,
    mut script_lifecycle_events: ScriptLifecycleQueue,
    mut active_machines: ResMut<ActiveMachines<P>>,
) {
    if args.hot_loads {
        let batch = script_modified_events.read().filter_map(|e| {
            if let AssetEvent::Modified { id } = e
                && let Some(asset) = script_lifecycle_events.assets.get(*id)
                && asset.language == P::LANGUAGE
            {
                Some(ScriptPipelineEvent::ModifiedAsset(
                    ScriptAssetModifiedMessage(*id),
                ))
            } else {
                None
            }
        });
        active_machines.queue_machines(batch);
    }

    if args.sync_components {
        // detachments are queued with the attachments, so they never overtake an attachment whose script is still loading.
        // we can't actually filter those based on their existence in the script contexts, as processing an attachment might
        // create the script, so we want to do that in order
        let batch = script_lifecycle_events
            .get_ready()
            .filter_map(|event| match event {
                (ScriptLifecycleMessage::Attached(mut a), Some((b, l))) => {
                    (l == P::LANGUAGE).then(|| {
                        trace!(
                            "dispatching script attachment event for: {a:?}, language: {}",
                            P::LANGUAGE
                        );
                        *a.script_mut() = b.0;
                        ScriptPipelineEvent::Attached(a)
                    })
                }
                (ScriptLifecycleMessage::Detached(d), _) => Some(ScriptPipelineEvent::Detached(d)),
                (ScriptLifecycleMessage::Attached(_), None) => None,
            });

        active_machines.queue_machines(batch);
    }
}

/// A discriminated union of all events that can trigger the script processing pipeline.
pub enum ScriptPipelineEvent {
    /// [`ScriptLifecycleMessage::Attached`]
    Attached(ScriptAttachment),
    /// [`ScriptLifecycleMessage::Detached`]
    Detached(ScriptAttachment),
    /// [`ScriptAssetModifiedMessage`]
    ModifiedAsset(ScriptAssetModifiedMessage),
}

impl ScriptPipelineEvent {
    fn process_attachment<P: IntoScriptPluginParams>(
        attachment_event: ScriptAttachment,
        assets: &mut Assets<ScriptAsset>,
        contexts: &mut ScriptContexts<P>,
    ) -> VecDeque<(ScriptAttachment, Box<dyn MachineState<P>>)> {
        let mut out = VecDeque::default();
        let attachment = attachment_event.clone();
        debug!("received attachment event: {attachment_event:?}");
        let id = attachment_event.script();
        let mut context = MachineContext {
            attachment: attachment.clone(),
        };
        if let Some(strong_handle) = StrongScriptHandle::from_assets(id, assets) {
            // we want the loading process to have access to asset paths
            *context.attachment.script_mut() = strong_handle.0.clone();
            let content = strong_handle.get(assets);

            // we query for the contexts to decide if this is a reload or load at runtime
            // not when queueing, in case another machine before this one affects the state, we do need the asset though

            let mut contexts = contexts.write();

            out.extend(match contexts.get_context(&attachment) {
                Some(Context::LoadedAndActive(context)) => {
                    if let Err((attachment, _)) =
                        contexts.insert(attachment.clone(), Context::Reloading(context.clone()))
                    {
                        bevy_log::warn!(
                            "Could not insert context for attachment {attachment}. Reloading interrupted."
                        );
                    };

                    vec![(
                        attachment.clone(),
                        Box::new(ReloadingInitialized {
                            attachment: attachment.clone(),
                            source: strong_handle.handle().clone(),
                            content: content.content,
                            existing_context: context,
                        })
                     as Box<dyn MachineState<P>>)]
                }
                // context is in an invalid state
                Some(_) => vec![],
                None => {
                    if let Err((attachment, _)) =
                        contexts.insert(attachment.clone(), Context::Loading)
                    {
                        bevy_log::warn!(
                            "Could not insert context for attachment {attachment}. Loading interrupted."
                        );
                    };

                    vec![(attachment.clone(),
                    Box::new(LoadingInitialized {
                        attachment: attachment.clone(),
                        source: strong_handle.handle().clone(),
                        content: content.content,
                    })as Box<dyn MachineState<P>>)]
                }
            });
        }
        out
    }

    fn process_detachment<P: IntoScriptPluginParams>(
        attachment_event: ScriptAttachment,
        contexts: &ScriptContexts<P>,
    ) -> VecDeque<(ScriptAttachment, Box<dyn MachineState<P>>)> {
        debug!("received detachment event: {attachment_event:?}");
        let attachment = &attachment_event;
        let mut contexts = contexts.write();
        contexts
            .get_context(attachment)
            .into_iter()
            .filter_map(|existing_context| {
                // for the borrow checker
                let attachment = attachment.clone();

                let existing_context = existing_context.as_loaded()?;

                if let Err((attachment, _)) =
                    contexts.insert(attachment.clone(), Context::Unloading(existing_context.clone()))
                {
                    bevy_log::warn!("Could not insert context for attachment {attachment}. Unloading interrupted.");
                };

                Some((attachment.clone(), Box::new(UnloadingInitialized {
                            attachment,
                            existing_context: existing_context.clone(),
                        }) as Box<dyn MachineState<P>>))
            }).collect()
    }

    fn process_modified_asset<P: IntoScriptPluginParams>(
        event: ScriptAssetModifiedMessage,
        assets: &mut Assets<ScriptAsset>,
        contexts: &ScriptContexts<P>,
    ) -> VecDeque<(ScriptAttachment, Box<dyn MachineState<P>>)> {
        let contexts = contexts.read();
        debug!("received modified event: {event:?}");

        let affected_attachments = contexts
            .all_residents()
            .filter(|(a, _)| event.0 == a.script().id());

        let mut out = VecDeque::default();
        for (attachment, existing_context) in affected_attachments {
            let id = attachment.script();
            if let Some(strong_handle) = StrongScriptHandle::from_assets(id, assets) {
                let content = strong_handle.get(assets);
                if let Some(existing_context) = existing_context.as_loaded() {
                    out.push_back((
                        attachment.clone(),
                        Box::new(ReloadingInitialized {
                            attachment: attachment.clone(),
                            source: strong_handle.handle().clone(),
                            content: content.content,
                            existing_context: existing_context.clone(),
                        }) as Box<dyn MachineState<P>>,
                    ));
                }
            }
        }
        out
    }

    /// Initializes a machine that can be run by the script processing pipeline.
    /// This is the moment we "commit" to the checks required by each machine.
    /// For example detachments check if the attachment exists and don't do anything otherwise.
    pub fn process<P: IntoScriptPluginParams>(
        self,
        assets: &mut Assets<ScriptAsset>,
        contexts: &mut ScriptContexts<P>,
    ) -> VecDeque<(ScriptAttachment, Box<dyn MachineState<P>>)> {
        match self {
            ScriptPipelineEvent::Attached(script_attached_event) => {
                Self::process_attachment(script_attached_event, assets, contexts)
            }
            ScriptPipelineEvent::Detached(script_detached_event) => {
                Self::process_detachment(script_detached_event, contexts)
            }
            ScriptPipelineEvent::ModifiedAsset(script_asset_modified_event) => {
                Self::process_modified_asset(script_asset_modified_event, assets, contexts)
            }
        }
    }
}
