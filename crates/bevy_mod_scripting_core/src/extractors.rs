//! Systems which are used to extract the various resources and components used by BMS.
//!
//! These are designed to be used to pipe inputs into other systems which require them, while handling any configuration erorrs nicely.

use bevy::prelude::World;

use crate::{
    context::{ContextLoadingSettings, ScriptContexts},
    error::MissingResourceError,
    handler::CallbackSettings,
    runtime::RuntimeContainer,
    script::{Scripts, StaticScripts},
    IntoScriptPluginParams,
};

/// Context for systems which handle events for scripts
pub(crate) struct HandlerContext<P: IntoScriptPluginParams> {
    pub callback_settings: CallbackSettings<P>,
    pub context_loading_settings: ContextLoadingSettings<P>,
    pub scripts: Scripts,
    pub runtime_container: RuntimeContainer<P>,
    pub script_contexts: ScriptContexts<P>,
    pub static_scripts: StaticScripts,
}
#[profiling::function]
pub(crate) fn extract_handler_context<P: IntoScriptPluginParams>(
    world: &mut World,
) -> Result<HandlerContext<P>, MissingResourceError> {
    // we don't worry about re-inserting these resources if we fail to extract them, as the plugin is misconfigured anyway,
    // so the only solution is to stop the program and fix the configuration
    // the config is either all in or nothing

    let callback_settings = world
        .remove_resource::<CallbackSettings<P>>()
        .ok_or_else(MissingResourceError::new::<CallbackSettings<P>>)?;
    let context_loading_settings = world
        .remove_resource::<ContextLoadingSettings<P>>()
        .ok_or_else(MissingResourceError::new::<ContextLoadingSettings<P>>)?;
    let scripts = world
        .remove_resource::<Scripts>()
        .ok_or_else(MissingResourceError::new::<Scripts>)?;
    let runtime_container = world
        .remove_non_send_resource::<RuntimeContainer<P>>()
        .ok_or_else(MissingResourceError::new::<RuntimeContainer<P>>)?;
    let script_contexts = world
        .remove_non_send_resource::<ScriptContexts<P>>()
        .ok_or_else(MissingResourceError::new::<ScriptContexts<P>>)?;
    let static_scripts = world
        .remove_resource::<StaticScripts>()
        .ok_or_else(MissingResourceError::new::<StaticScripts>)?;

    Ok(HandlerContext {
        callback_settings,
        context_loading_settings,
        scripts,
        runtime_container,
        script_contexts,
        static_scripts,
    })
}
#[profiling::function]
pub(crate) fn yield_handler_context<P: IntoScriptPluginParams>(
    world: &mut World,
    context: HandlerContext<P>,
) {
    world.insert_resource(context.callback_settings);
    world.insert_resource(context.context_loading_settings);
    world.insert_resource(context.scripts);
    world.insert_non_send_resource(context.runtime_container);
    world.insert_non_send_resource(context.script_contexts);
    world.insert_resource(context.static_scripts);
}
