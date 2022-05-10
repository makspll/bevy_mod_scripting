#![doc=include_str!("../../readme.md")]

use bevy::prelude::*;

pub mod hosts;

pub use hosts::*;

#[derive(Default)]
/// Bevy plugin enabling run-time scripting
pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, _app: &mut bevy::prelude::App) {}
}
