pub use crate::{BMSPlugin, ScriptFunctionsPlugin};
pub use bevy_mod_scripting_core::{
    asset::{Language, ScriptAsset},
    bindings::{
        function::namespace::{GlobalNamespace, NamespaceBuilder},
        script_value::ScriptValue,
        CoreScriptGlobalsPlugin,
    },
    callback_labels,
    commands::{AddStaticScript, DeleteScript},
    event::ScriptCallbackEvent,
    handler::event_handler,
    script::{ScriptComponent, ScriptId},
    ConfigureScriptAssetSettings, ConfigureScriptPlugin, IntoScriptPluginParams,
};

#[cfg(feature = "lua")]
pub use bevy_mod_scripting_lua::LuaScriptingPlugin;
#[cfg(feature = "rhai")]
pub use bevy_mod_scripting_rhai::RhaiScriptingPlugin;
