use super::*;
use bevy_asset::AssetEvent;
use bevy_log::debug;

/// Emitted when a script is attached.
#[derive(Event, Clone, Debug)]
pub struct ScriptAttachedEvent(pub ScriptAttachment);

/// Emitted when a script is detached.
#[derive(Event, Clone, Debug)]
pub struct ScriptDetachedEvent(pub ScriptAttachment);

/// Emitted when a script asset is modified and all its attachments require re-loading
#[derive(Event, Clone, Debug)]
pub struct ScriptAssetModifiedEvent(pub ScriptId);

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

    /// Upgrades an asset Id pointing to a script to a strong handle if the asset hasn't been dropped
    pub fn upgrade(id: ScriptId, assets: &mut Assets<ScriptAsset>) -> Option<Self> {
        assets.get_strong_handle(id).map(Self)
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
    mut events: EventReader<AssetEvent<ScriptAsset>>,
    mut filtered: EventWriter<ForPlugin<ScriptAssetModifiedEvent, P>>,
    assets: Res<Assets<ScriptAsset>>,
    mut requests: ResMut<RequestProcessingPipelineRun<P>>,
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
        requests.request_run();
        filtered.write_batch(std::iter::once(next).chain(batch));
    }
}

/// Filters incoming [`ScriptAttachedEvent`]'s leaving only those which match the plugin's language
pub fn filter_script_attachments<P: IntoScriptPluginParams>(
    mut events: EventReader<ScriptAttachedEvent>,
    mut filtered: EventWriter<ForPlugin<ScriptAttachedEvent, P>>,
    assets: Res<Assets<ScriptAsset>>,
    mut requests: ResMut<RequestProcessingPipelineRun<P>>,
) {
    let mut batch = events
        .read()
        .filter(|e| {
            assets
                .get(&e.0.script())
                .is_some_and(|asset| asset.language == P::LANGUAGE)
        })
        .cloned()
        .map(ForPlugin::new);

    if let Some(next) = batch.next() {
        requests.request_run();
        filtered.write_batch(std::iter::once(next).chain(batch));
    }
}

/// Filters incoming [`ScriptDetachedEvent`]'s leaving only those which match the plugin's language
pub fn filter_script_detachments<P: IntoScriptPluginParams>(
    mut events: EventReader<ScriptDetachedEvent>,
    mut filtered: EventWriter<ForPlugin<ScriptDetachedEvent, P>>,
    assets: Res<Assets<ScriptAsset>>,
    mut requests: ResMut<RequestProcessingPipelineRun<P>>,
) {
    let mut batch = events
        .read()
        .filter(|e| {
            assets
                .get(&e.0.script())
                .is_some_and(|asset| asset.language == P::LANGUAGE)
        })
        .cloned()
        .map(ForPlugin::new);

    if let Some(next) = batch.next() {
        requests.request_run();
        filtered.write_batch(std::iter::once(next).chain(batch));
    }
}

/// Process [`ScriptAttachedEvent`]'s and generate loading machines with the [`LoadingInitializedState`] and [`ReloadingInitializedState`] states
pub fn process_attachments<P: IntoScriptPluginParams>(
    mut events: EventReader<ForPlugin<ScriptAttachedEvent, P>>,
    mut load_machines: StateMachine<Machine<Loading, LoadingInitialized>, P>,
    mut reload_machines: StateMachine<Machine<Loading, ReloadingInitialized<P>>, P>,
    mut assets: ResMut<Assets<ScriptAsset>>,
    contexts: Res<ScriptContext<P>>,
) {
    let contexts = contexts.read();
    let (reload_machines_batch, load_machines_batch): (Vec<_>, Vec<_>) = events
        .read()
        .filter_map(|wrapper| {
            let attachment_event = wrapper.event();
            let id = attachment_event.0.script().id();

            if let Some(strong_handle) = StrongScriptHandle::upgrade(id, &mut assets) {
                if let Some(existing_context) = contexts.get_context(&attachment_event.0) {
                    Some(Either::Left(Machine::start_reload(
                        attachment_event.0.clone(),
                        strong_handle,
                        existing_context,
                    )))
                } else {
                    Some(Either::Right(Machine::start_load(
                        attachment_event.0.clone(),
                        strong_handle,
                    )))
                }
            } else {
                None
            }
        })
        .partition_map(|p| p);
    debug!(
        "{}: script loads triggered: {}, script reload triggers: {}",
        P::LANGUAGE,
        load_machines_batch.len(),
        reload_machines_batch.len()
    );
    load_machines.write_batch(load_machines_batch);
    reload_machines.write_batch(reload_machines_batch);
}

/// Processes [`ScriptAttachedEvent`]'s and initialized unloading state machines with [`UnloadingInitializedState`] states
pub fn process_detachments<P: IntoScriptPluginParams>(
    mut events: EventReader<ForPlugin<ScriptDetachedEvent, P>>,
    mut unload_machines: StateMachine<Machine<Unloading, UnloadingInitialized<P>>, P>,
    contexts: Res<ScriptContext<P>>,
) {
    let machines_to_send = events.read().filter_map(|wrapper| {
        let attachment_event = wrapper.event();
        let contexts_guard = contexts.read();
        contexts_guard
            .get_context(&attachment_event.0)
            .map(|existing_context| {
                Machine::start_unload(attachment_event.0.clone(), existing_context)
            })
    });

    unload_machines.write_batch(machines_to_send);
}

/// Processes [`ScriptAssetModifiedEvent`]'s and initializes loading state machines with [`ReloadingInitializedState`] states
pub fn process_asset_modifications<P: IntoScriptPluginParams>(
    mut events: EventReader<ForPlugin<ScriptAssetModifiedEvent, P>>,
    mut load_machines: StateMachine<Machine<Loading, ReloadingInitialized<P>>, P>,
    contexts: Res<ScriptContext<P>>,
    mut assets: ResMut<Assets<ScriptAsset>>,
) {
    let affected_ids = events.read().map(|e| e.event().0).collect::<HashSet<_>>();

    let contexts = contexts.read();

    let affected_attachments = contexts
        .all_residents()
        .filter(|(a, _)| affected_ids.contains(&a.script().id()));

    let to_send = affected_attachments.filter_map(|(attachment, ctxt)| {
        let id = attachment.script().id();
        StrongScriptHandle::upgrade(id, &mut assets)
            .map(|strong_handle| Machine::start_reload(attachment.clone(), strong_handle, ctxt))
    });

    load_machines.write_batch(to_send);
}
