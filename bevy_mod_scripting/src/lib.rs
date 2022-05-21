#![doc=include_str!("../../readme.md")]

use bevy::prelude::*;

pub mod hosts;

pub use bevy_event_priority as events;
pub use hosts::*;

#[derive(Default)]
/// Bevy plugin enabling run-time scripting
pub struct ScriptingPlugin;

impl Plugin for ScriptingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {}
}
