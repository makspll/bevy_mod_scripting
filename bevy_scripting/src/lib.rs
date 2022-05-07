
use bevy::{prelude::*};
pub mod hosts;
pub mod assets;
pub use {hosts::*, assets::*};


#[derive(Default)]
pub struct LuaPlugin;

impl Plugin for LuaPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        // custom assets
        app.add_asset::<LuaFile>();
        app.init_asset_loader::<LuaLoader>();

    }
}







