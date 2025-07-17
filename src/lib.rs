#![doc=include_str!("../readme.md")]

pub mod core {
    pub use bevy_mod_scripting_core::*;
}

pub mod prelude;

#[cfg(feature = "lua")]
pub mod lua {
    pub use bevy_mod_scripting_lua::*;
}

#[cfg(feature = "rhai")]
pub mod rhai {
    pub use bevy_mod_scripting_rhai::*;
}

// #[cfg(feature = "rune")]
// pub mod rune {
//     pub use bevy_mod_scripting_rune::*;
// }

use bevy::app::plugin_group;
use bevy_mod_scripting_core::{
    bindings::CoreScriptGlobalsPlugin, BMSScriptingInfrastructurePlugin,
};
pub use bevy_mod_scripting_derive::*;
pub use bevy_mod_scripting_functions::*;

plugin_group! {
    pub struct BMSPlugin {
        :ScriptFunctionsPlugin,
        :CoreScriptGlobalsPlugin,
        :BMSScriptingInfrastructurePlugin,
        #[custom(cfg(feature = "lua"))]
        bevy_mod_scripting_lua:::LuaScriptingPlugin,
        #[custom(cfg(feature = "rhai"))]
        bevy_mod_scripting_rhai:::RhaiScriptingPlugin,
    }
}
