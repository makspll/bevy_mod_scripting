//! Script bindings for bevy text.

use bevy_app::{App, Plugin};

mod text;
mod text2d;
mod text_font;
mod text_layout;
mod text_span;

/// Registers the text script bindings.
#[derive(Default)]
pub struct BevyTextBindingsPlugin;

impl Plugin for BevyTextBindingsPlugin {
    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        text::register(world);
        text2d::register(world);
        text_span::register(world);
        text_font::register(world);
        text_layout::register(world);
    }
}
