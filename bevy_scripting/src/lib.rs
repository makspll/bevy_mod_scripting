//! Scripting plugin for the Bevy game engine.
//! 
//! Enables multi-language scripting with custom script facing API's.
//! 
//! Supported scripting languages:
//!     - lua
//!  
//! More languages coming!

use bevy::prelude::*;

pub mod hosts;

pub use hosts::*;

#[derive(Default)]
/// Bevy plugin enabling run-time scripting
pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // custom assets
        app.add_asset::<LuaFile>();
        app.init_asset_loader::<LuaLoader>();
    }
}
