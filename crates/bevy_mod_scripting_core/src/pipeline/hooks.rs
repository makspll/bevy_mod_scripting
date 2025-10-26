use bevy_mod_scripting_bindings::ScriptValue;

use crate::{
    callbacks::ScriptCallbacks,
    commands::RunScriptCallback,
    event::{IntoCallbackLabel, OnScriptLoaded, OnScriptReloaded, OnScriptUnloaded},
};

use super::*;

const UNLOADED_SCRIPT_STATE_KEY: &str = "state";

pub(crate) struct OnLoadedListener;

impl<P: IntoScriptPluginParams> TransitionListener<ContextAssigned<P>> for OnLoadedListener {
    fn on_enter(
        &self,
        state: &mut ContextAssigned<P>,
        world: &mut World,
        ctxt: &mut Context,
    ) -> Result<(), ScriptError> {
        let world_id = world.id();
        let emit_responses = P::readonly_configuration(world_id).emit_responses;
        let callbacks = world.get_resource_or_init::<ScriptCallbacks<P>>().clone();
        let guard = WorldGuard::new_exclusive(world);

        RunScriptCallback::<P>::new(
            ctxt.attachment.clone(),
            OnScriptLoaded::into_callback_label(),
            vec![],
            emit_responses,
        )
        .run_with_context(guard.clone(), state.context.clone(), callbacks)
        .map(|_| ())
    }
}

pub(crate) struct OnUnloadedForUnloadListener;
impl<P: IntoScriptPluginParams> TransitionListener<UnloadingInitialized<P>>
    for OnUnloadedForUnloadListener
{
    fn on_enter(
        &self,
        state: &mut UnloadingInitialized<P>,
        world: &mut World,
        ctxt: &mut Context,
    ) -> Result<(), ScriptError> {
        let world_id = world.id();
        let emit_responses = P::readonly_configuration(world_id).emit_responses;
        let callbacks = world.get_resource_or_init::<ScriptCallbacks<P>>().clone();
        let guard = WorldGuard::new_exclusive(world);

        let v = RunScriptCallback::<P>::new(
            ctxt.attachment.clone(),
            OnScriptUnloaded::into_callback_label(),
            vec![],
            emit_responses,
        )
        .run_with_context(guard.clone(), state.existing_context.clone(), callbacks)?;
        ctxt.insert(UNLOADED_SCRIPT_STATE_KEY, v);
        Ok(())
    }
}

pub(crate) struct OnUnloadedForReloadListener;
impl<P: IntoScriptPluginParams> TransitionListener<ReloadingInitialized<P>>
    for OnUnloadedForReloadListener
{
    fn on_enter(
        &self,
        state: &mut ReloadingInitialized<P>,
        world: &mut World,
        ctxt: &mut Context,
    ) -> Result<(), ScriptError> {
        let world_id = world.id();
        let emit_responses = P::readonly_configuration(world_id).emit_responses;
        let callbacks = world.get_resource_or_init::<ScriptCallbacks<P>>().clone();
        let guard = WorldGuard::new_exclusive(world);

        let v = RunScriptCallback::<P>::new(
            ctxt.attachment.clone(),
            OnScriptUnloaded::into_callback_label(),
            vec![],
            emit_responses,
        )
        .run_with_context(guard.clone(), state.existing_context.clone(), callbacks)?;
        ctxt.insert(UNLOADED_SCRIPT_STATE_KEY, v);
        Ok(())
    }
}

pub(crate) struct OnReloadedListener;
impl<P: IntoScriptPluginParams> TransitionListener<ContextAssigned<P>> for OnReloadedListener {
    fn on_enter(
        &self,
        state: &mut ContextAssigned<P>,
        world: &mut World,
        ctxt: &mut Context,
    ) -> Result<(), ScriptError> {
        let world_id = world.id();
        let emit_responses = P::readonly_configuration(world_id).emit_responses;
        let callbacks = world.get_resource_or_init::<ScriptCallbacks<P>>().clone();
        let guard = WorldGuard::new_exclusive(world);

        if state.is_new_context {
            return Ok(());
        }

        let unload_state = ctxt.get_first_typed::<ScriptValue>(UNLOADED_SCRIPT_STATE_KEY);
        let unload_state = unload_state.unwrap_or(ScriptValue::Unit);

        RunScriptCallback::<P>::new(
            ctxt.attachment.clone(),
            OnScriptReloaded::into_callback_label(),
            vec![unload_state],
            emit_responses,
        )
        .run_with_context(guard.clone(), state.context.clone(), callbacks)
        .map(|_| ())
    }
}
