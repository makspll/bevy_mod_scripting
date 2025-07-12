
pub use crate::{BMSPlugin, ScriptFunctionsPlugin};
pub use bevy_mod_scripting_core::{
    ConfigureScriptPlugin,
    asset::ScriptAsset,
    bindings::{
        function::namespace::{GlobalNamespace, NamespaceBuilder},
        script_value::ScriptValue,
        AllocatorDiagnosticPlugin, CoreScriptGlobalsPlugin,
    },
    callback_labels,
    commands::{DeleteScript, AddStaticScript},
    event::ScriptCallbackEvent,
    handler::event_handler,
    script::{ScriptId, ScriptComponent},
};

#[cfg(feature = "lua")]
pub use bevy_mod_scripting_lua::LuaScriptingPlugin;
#[cfg(feature = "rhai")]
pub use bevy_mod_scripting_rhai::RhaiScriptingPlugin;
