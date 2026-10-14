//! Window resolutions: the size of a window in physical and logical pixels.

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::{M, R, V};
use bevy_mod_scripting_derive::script_bindings;
use bevy_window::WindowResolution;

#[script_bindings(
    remote,
    name = "window_resolution_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl WindowResolution {
    /// Creates a resolution from a size in physical pixels.
    ///
    /// ```multi-lang-example,lua
    /// local resolution = WindowResolution.new(800, 600)
    /// assert(resolution:physical_width() == 800)
    /// ```
    fn new(physical_width: u32, physical_height: u32) -> V<WindowResolution> {
        V(WindowResolution::new(physical_width, physical_height))
    }

    /// The width in logical pixels.
    fn width(_self: R<WindowResolution>) -> f32 {
        _self.width()
    }

    /// The height in logical pixels.
    fn height(_self: R<WindowResolution>) -> f32 {
        _self.height()
    }

    /// The ratio of physical to logical pixels.
    fn scale_factor(_self: R<WindowResolution>) -> f32 {
        _self.scale_factor()
    }

    /// Sets the size in logical pixels.
    fn set(mut _self: M<WindowResolution>, width: f32, height: f32) {
        _self.set(width, height);
    }
}

/// Registers the functions of window resolutions.
pub(crate) fn register(world: &mut World) {
    register_window_resolution_functions(world);
}
