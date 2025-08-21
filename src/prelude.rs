pub use bevy_mod_scripting_core::{
    ConfigureScriptAssetSettings, ConfigureScriptPlugin, IntoScriptPluginParams,
    asset::{Language, ScriptAsset},
    bindings::{
        CoreScriptGlobalsPlugin,
        function::namespace::{GlobalNamespace, NamespaceBuilder},
        script_value::ScriptValue,
    },
    callback_labels,
    commands::{AddStaticScript, DeleteScript},
    event::ScriptCallbackEvent,
    handler::event_handler,
    script::{ScriptComponent, ScriptId},
};
#[cfg(feature = "lua")]
pub use bevy_mod_scripting_lua::LuaScriptingPlugin;
#[cfg(feature = "rhai")]
pub use bevy_mod_scripting_rhai::RhaiScriptingPlugin;

pub use crate::{BMSPlugin, ScriptFunctionsPlugin};
