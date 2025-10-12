//! Module managing script registered callbacks
use std::{marker::PhantomData, sync::Arc};

use bevy_app::Plugin;
use bevy_ecs::{resource::Resource, world::WorldId};
use bevy_mod_scripting_bindings::{InteropError, ScriptValue};
use bevy_mod_scripting_script::ScriptAttachment;
use bevy_platform::collections::HashMap;
use parking_lot::RwLock;

use crate::IntoScriptPluginParams;

/// A callback function encapsulating the callback context
pub type CallbackFn<P> = dyn Fn(
        Vec<ScriptValue>,
        &mut <P as IntoScriptPluginParams>::C,
        WorldId,
    ) -> Result<ScriptValue, InteropError>
    + Send
    + Sync
    + 'static;

/// A resource containing the callbacks for this script plugin.
///
/// Scripts can opt to register callbacks which are then used to invoke event callbacks.
#[derive(Resource)]
pub struct ScriptCallbacks<P: IntoScriptPluginParams> {
    /// The callbacks registered per script
    pub callbacks: Arc<RwLock<HashMap<(ScriptAttachment, String), Arc<CallbackFn<P>>>>>,
}

impl<P: IntoScriptPluginParams> Clone for ScriptCallbacks<P> {
    fn clone(&self) -> Self {
        Self {
            callbacks: self.callbacks.clone(),
        }
    }
}

impl<P: IntoScriptPluginParams> Default for ScriptCallbacks<P> {
    fn default() -> Self {
        Self {
            callbacks: Default::default(),
        }
    }
}

/// Plugin adding necessary resources to handle callback storage for script plugins.
pub struct ScriptCallbacksPlugin<P: IntoScriptPluginParams> {
    _ph: PhantomData<fn(P)>,
}

impl<P: IntoScriptPluginParams> Default for ScriptCallbacksPlugin<P> {
    fn default() -> Self {
        Self {
            _ph: Default::default(),
        }
    }
}

impl<P: IntoScriptPluginParams> Plugin for ScriptCallbacksPlugin<P> {
    fn build(&self, app: &mut bevy_app::App) {
        app.init_resource::<ScriptCallbacks<P>>();
    }
}
