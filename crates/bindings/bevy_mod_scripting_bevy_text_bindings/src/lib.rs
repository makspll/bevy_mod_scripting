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
    fn finish(&self, app: &mut App) {
        let registry = app
            .world()
            .resource::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        bevy_mod_scripting_bindings::set_crate_script_namespaces(
            &mut registry.write(),
            &["bevy_text"],
            "text",
        );
    }

    fn build(&self, app: &mut App) {
        use bevy_mod_scripting_bindings::ScriptSystemSetAppExt;
        app.register_script_system_set(
            "text",
            "Text2dUpdateSystems",
            bevy_text::Text2dUpdateSystems,
        )
        .register_script_system_set(
            "text",
            "EditableTextSystems",
            bevy_text::EditableTextSystems,
        );
        let world = app.world_mut();
        text::register(world);
        text2d::register(world);
        text_span::register(world);
        text_font::register(world);
        text_layout::register(world);
    }
}
