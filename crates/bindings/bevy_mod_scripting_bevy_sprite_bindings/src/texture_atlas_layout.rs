//! [`TextureAtlasLayout`] grids.

use bevy_image::TextureAtlasLayout;
use bevy_math::UVec2;
// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::{R, V};
use bevy_mod_scripting_derive::script_bindings;

#[script_bindings(
    remote,
    name = "texture_atlas_layout_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "sprite",
    core
)]
impl TextureAtlasLayout {
    /// A layout of equal tiles in a grid, numbered left to right and then top to bottom. Padding and offset are in pixels, and default to none.
    fn from_grid(
        tile_size: R<UVec2>,
        columns: u32,
        rows: u32,
        padding: Option<R<UVec2>>,
        offset: Option<R<UVec2>>,
    ) -> V<TextureAtlasLayout> {
        V(TextureAtlasLayout::from_grid(
            *tile_size,
            columns,
            rows,
            padding.map(|padding| *padding),
            offset.map(|offset| *offset),
        ))
    }

    /// The number of tiles in the layout.
    fn len(_self: R<TextureAtlasLayout>) -> usize {
        _self.len()
    }
}

/// Registers the texture atlas layout functions.
pub(crate) fn register(world: &mut World) {
    register_texture_atlas_layout_functions(world);
}
