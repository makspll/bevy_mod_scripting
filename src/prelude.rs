pub use bevy_mod_scripting_core::{
    ConfigureScriptPlugin, IntoScriptPluginParams, callback_labels,
    commands::{AddStaticScript, DeleteScript},
    event::ScriptCallbackEvent,
    handler::event_handler,
    script::{ScriptComponent, ScriptId},
};

pub use bevy_mod_scripting_bindings::{
    CoreScriptGlobalsPlugin,
    function::namespace::{GlobalNamespace, NamespaceBuilder},
    script_value::ScriptValue,
};

pub use bevy_mod_scripting_asset::*;

#[cfg(feature = "lua")]
pub use bevy_mod_scripting_lua::LuaScriptingPlugin;
#[cfg(feature = "rhai")]
pub use bevy_mod_scripting_rhai::RhaiScriptingPlugin;

pub use crate::{BMSPlugin, ScriptFunctionsPlugin};
