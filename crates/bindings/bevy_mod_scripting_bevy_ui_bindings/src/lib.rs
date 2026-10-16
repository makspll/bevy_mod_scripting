//! Script bindings for bevy user interface nodes, layout and styling.

use bevy_app::{App, Plugin};

mod border;
mod computed_node;
mod image_node;
mod overflow;
mod ui_rect;
mod val;

/// Registers the ui script bindings.
#[derive(Default)]
pub struct BevyUiBindingsPlugin;

impl Plugin for BevyUiBindingsPlugin {
    fn build(&self, app: &mut App) {
        // types without functions are only available to scripts once registered
        app.register_type::<bevy_ui::Node>()
            .register_type::<bevy_ui::BackgroundColor>()
            .register_type::<bevy_ui::BorderColor>()
            .register_type::<bevy_ui::Outline>()
            .register_type::<bevy_ui::ZIndex>()
            .register_type::<bevy_ui::GlobalZIndex>();
        let world = app.world_mut();
        val::register(world);
        ui_rect::register(world);
        border::register(world);
        overflow::register(world);
        image_node::register(world);
        computed_node::register(world);
    }
}
