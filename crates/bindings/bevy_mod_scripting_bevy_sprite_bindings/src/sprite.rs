//! [`Sprite`] constructors.

use bevy_asset::Handle;
use bevy_color::Color;
use bevy_image::{Image, TextureAtlas};
use bevy_math::Vec2;
use bevy_sprite::Sprite;
// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::function::from::{R, V};
use bevy_mod_scripting_derive::script_bindings;

#[script_bindings(
    remote,
    name = "sprite_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Sprite {
    /// A sprite that shows the whole image.
    fn from_image(image: R<Handle<Image>>) -> V<Sprite> {
        V(Sprite::from_image((*image).clone()))
    }

    /// A sprite that shows a section of an image from a texture atlas.
    fn from_atlas_image(image: R<Handle<Image>>, atlas: R<TextureAtlas>) -> V<Sprite> {
        V(Sprite::from_atlas_image((*image).clone(), (*atlas).clone()))
    }

    /// A solid square of the given color and size.
    fn from_color(color: R<Color>, size: R<Vec2>) -> V<Sprite> {
        V(Sprite::from_color(*color, *size))
    }

    /// A sprite with no image that is drawn at the given size.
    fn sized(size: R<Vec2>) -> V<Sprite> {
        V(Sprite::sized(*size))
    }
}

/// Registers the sprite functions.
pub(crate) fn register(world: &mut World) {
    register_sprite_functions(world);
}
