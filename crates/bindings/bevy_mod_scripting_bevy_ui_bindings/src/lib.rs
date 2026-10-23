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
    fn finish(&self, app: &mut App) {
        let registry = app
            .world()
            .resource::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        bevy_mod_scripting_bindings::set_crate_script_namespaces(
            &mut registry.write(),
            &["bevy_ui", "bevy_ui_render", "bevy_ui_widgets"],
            "ui",
        );
    }

    fn build(&self, app: &mut App) {
        // types without functions are only available to scripts once registered
        app.register_type::<bevy_ui::Node>()
            .register_type::<bevy_ui::BackgroundColor>()
            .register_type::<bevy_ui::BorderColor>()
            .register_type::<bevy_ui::Outline>()
            .register_type::<bevy_ui::ZIndex>()
            .register_type::<bevy_ui::GlobalZIndex>();
        {
            use bevy_mod_scripting_bindings::ScriptSystemSetAppExt;
            use bevy_ui::UiSystems;
            app.register_script_system_set("ui.UiSystems", "Focus", UiSystems::Focus)
                .register_script_system_set("ui.UiSystems", "Prepare", UiSystems::Prepare)
                .register_script_system_set("ui.UiSystems", "Propagate", UiSystems::Propagate)
                .register_script_system_set("ui.UiSystems", "Content", UiSystems::Content)
                .register_script_system_set("ui.UiSystems", "Layout", UiSystems::Layout)
                .register_script_system_set("ui.UiSystems", "PostLayout", UiSystems::PostLayout)
                .register_script_system_set("ui.UiSystems", "Stack", UiSystems::Stack)
                .register_script_system_set(
                    "ui",
                    "AccessibilityUiSystems",
                    bevy_ui::AccessibilityUiSystems,
                );
        }
        let world = app.world_mut();
        val::register(world);
        ui_rect::register(world);
        border::register(world);
        overflow::register(world);
        image_node::register(world);
        computed_node::register(world);
    }
}
