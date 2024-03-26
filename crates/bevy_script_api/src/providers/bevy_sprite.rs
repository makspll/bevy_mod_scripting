#![allow(clippy::all, unused_imports, deprecated, dead_code)]
// @generated by cargo bevy-api-gen generate, modify the templates not this file

use super::bevy_ecs::*;

use super::bevy_reflect::*;

use super::bevy_asset::*;

use super::bevy_core::*;

use super::bevy_hierarchy::*;

use super::bevy_input::*;

use super::bevy_window::*;

use super::bevy_render::*;

use super::bevy_time::*;

use super::bevy_transform::*;

use super::bevy_core_pipeline::*;

extern crate self as bevy_script_api;

/// A [2d material](Material2d) that renders [2d meshes](crate::Mesh2dHandle) with a texture tinted by a uniform color

#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
derive(clone,debug),
remote="bevy::sprite::prelude::ColorMaterial",
functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::sprite::prelude::ColorMaterial;

"#]
)]

pub struct LuaColorMaterial {}

/// Component for rendering with meshes in the 2d pipeline, usually with a [2d material](crate::Material2d) such as [`ColorMaterial`](crate::ColorMaterial).

/// It wraps a [`Handle<Mesh>`] to differentiate from the 3d pipelines which use the handles directly as components

#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
derive(clone,debug),
remote="bevy::sprite::Mesh2dHandle",
functions[r#"

    #[lua(
        as_trait = "std::cmp::Eq",
        kind = "Function",
        composite = "assert_receiver_is_total_eq",
    )]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
			r#"

    #[lua(as_trait = "std::cmp::PartialEq", kind = "Method")]
    fn eq(&self, #[proxy] other: &mesh2d::mesh::Mesh2dHandle) -> bool;

"#,
			r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::sprite::Mesh2dHandle;

"#]
)]

pub struct LuaMesh2dHandle();

/// Specifies the rendering properties of a sprite.

/// This is commonly used as a component within [`SpriteBundle`](crate::bundle::SpriteBundle).

#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
derive(clone,debug),
remote="bevy::sprite::prelude::Sprite",
functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::sprite::prelude::Sprite;

"#]
)]

pub struct LuaSprite {}

/// Controls how the image is altered when scaled.

#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
derive(clone,debug),
remote="bevy::sprite::prelude::ImageScaleMode",
functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::sprite::prelude::ImageScaleMode;

"#]
)]

pub struct LuaImageScaleMode {}

/// How a sprite is positioned relative to its [`Transform`](bevy_transform::components::Transform).

/// It defaults to `Anchor::Center`.

#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
derive(clone,debug),
remote="bevy::sprite::Anchor",
functions[r#"

    #[lua(kind = "Method", output(proxy))]
    fn as_vec(&self) -> bevy::math::Vec2;

"#,
			r#"

    #[lua(as_trait = "std::cmp::PartialEq", kind = "Method")]
    fn eq(&self, #[proxy] other: &sprite::Anchor) -> bool;

"#,
			r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::sprite::Anchor;

"#]
)]

pub struct LuaAnchor {}

/// Stores a map used to lookup the position of a texture in a [`TextureAtlas`].

/// This can be used to either use and look up a specific section of a texture, or animate frame-by-frame as a sprite sheet.

/// Optionally it can store a mapping from sub texture handles to the related area index (see

/// [`TextureAtlasBuilder`]).

/// [Example usage animating sprite.](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs)

/// [Example usage loading sprite sheet.](https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs)

/// [`TextureAtlasBuilder`]: crate::TextureAtlasBuilder

#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
derive(clone,debug),
remote="bevy::sprite::prelude::TextureAtlasLayout",
functions[r#"
/// Create a new empty layout with custom `dimensions`

    #[lua(kind = "Function", output(proxy))]
    fn new_empty(
        #[proxy]
        dimensions: bevy::math::Vec2,
    ) -> bevy::sprite::prelude::TextureAtlasLayout;

"#,
			r#"
/// The number of textures in the [`TextureAtlasLayout`]

    #[lua(kind = "Method")]
    fn len(&self) -> usize;

"#,
			r#"

    #[lua(kind = "Method")]
    fn is_empty(&self) -> bool;

"#,
			r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::sprite::prelude::TextureAtlasLayout;

"#]
)]

pub struct LuaTextureAtlasLayout {}

/// Component used to draw a specific section of a texture.

/// It stores a handle to [`TextureAtlasLayout`] and the index of the current section of the atlas.

/// The texture atlas contains various *sections* of a given texture, allowing users to have a single

/// image file for either sprite animation or global mapping.

/// You can change the texture [`index`](Self::index) of the atlas to animate the sprite or display only a *section* of the texture

/// for efficient rendering of related game objects.

/// Check the following examples for usage:

/// - [`animated sprite sheet example`](https://github.com/bevyengine/bevy/blob/latest/examples/2d/sprite_sheet.rs)

/// - [`texture atlas example`](https://github.com/bevyengine/bevy/blob/latest/examples/2d/texture_atlas.rs)

#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
derive(clone,debug),
remote="bevy::sprite::prelude::TextureAtlas",
functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::sprite::prelude::TextureAtlas;

"#]
)]

pub struct LuaTextureAtlas {}

/// Struct defining a [`Sprite`](crate::Sprite) border with padding values

#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
derive(clone,debug),
remote="bevy::sprite::prelude::BorderRect",
functions[r#"
/// Creates a new border as a square, with identical pixel padding values on every direction

    #[lua(kind = "Function", output(proxy))]
    fn square(value: f32) -> bevy::sprite::prelude::BorderRect;

"#,
			r#"
/// Creates a new border as a rectangle, with:
/// - `horizontal` for left and right pixel padding
/// - `vertical` for top and bottom pixel padding

    #[lua(kind = "Function", output(proxy))]
    fn rectangle(horizontal: f32, vertical: f32) -> bevy::sprite::prelude::BorderRect;

"#,
			r#"

    #[lua(as_trait = "std::cmp::PartialEq", kind = "Method")]
    fn eq(&self, #[proxy] other: &texture_slice::border_rect::BorderRect) -> bool;

"#,
			r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::sprite::prelude::BorderRect;

"#]
)]

pub struct LuaBorderRect {}

/// Slices a texture using the **9-slicing** technique. This allows to reuse an image at various sizes

/// without needing to prepare multiple assets. The associated texture will be split into nine portions,

/// so that on resize the different portions scale or tile in different ways to keep the texture in proportion.

/// For example, when resizing a 9-sliced texture the corners will remain unscaled while the other

/// sections will be scaled or tiled.

/// See [9-sliced](https://en.wikipedia.org/wiki/9-slice_scaling) textures.

#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
derive(clone,debug),
remote="bevy::sprite::prelude::TextureSlicer",
functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::sprite::prelude::TextureSlicer;

"#]
)]

pub struct LuaTextureSlicer {}

/// Defines how a texture slice scales when resized

#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
derive(clone,debug),
remote="bevy::sprite::prelude::SliceScaleMode",
functions[r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::sprite::prelude::SliceScaleMode;

"#]
)]

pub struct LuaSliceScaleMode {}

crate::impl_tealr_generic!(pub(crate) struct T);

#[derive(Default)]
pub(crate) struct Globals;

impl bevy_mod_scripting_lua::tealr::mlu::ExportInstances for Globals {
    fn add_instances<'lua, T: bevy_mod_scripting_lua::tealr::mlu::InstanceCollector<'lua>>(
        self,
        instances: &mut T,
    ) -> bevy_mod_scripting_lua::tealr::mlu::mlua::Result<()> {
        instances.add_instance(
            "LuaTextureAtlasLayout",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaTextureAtlasLayout>::new,
        )?;

        instances.add_instance(
            "LuaBorderRect",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaBorderRect>::new,
        )?;

        Ok(())
    }
}

pub struct BevySpriteAPIProvider;

impl bevy_mod_scripting_core::hosts::APIProvider for BevySpriteAPIProvider {
    type APITarget = std::sync::Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type ScriptContext = std::sync::Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type DocTarget = bevy_mod_scripting_lua::docs::LuaDocFragment;

    fn attach_api(
        &mut self,
        ctx: &mut Self::APITarget,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        let ctx = ctx
            .get_mut()
            .expect("Unable to acquire lock on Lua context");
        bevy_mod_scripting_lua::tealr::mlu::set_global_env(Globals, ctx)
            .map_err(|e| bevy_mod_scripting_core::error::ScriptError::Other(e.to_string()))
    }

    fn get_doc_fragment(&self) -> Option<Self::DocTarget> {
        Some(bevy_mod_scripting_lua::docs::LuaDocFragment::new(
            "BevySpriteAPI",
            |tw| {
                tw
                .document_global_instance::<Globals>().expect("Something went wrong documenting globals")
            
                .process_type::<LuaColorMaterial>()
                
            
                .process_type::<LuaMesh2dHandle>()
                
            
                .process_type::<LuaSprite>()
                
            
                .process_type::<LuaImageScaleMode>()
                
            
                .process_type::<LuaAnchor>()
                
            
                .process_type::<LuaTextureAtlasLayout>()
                
                .process_type::<bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaTextureAtlasLayout>>()
                
            
                .process_type::<LuaTextureAtlas>()
                
            
                .process_type::<LuaBorderRect>()
                
                .process_type::<bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaBorderRect>>()
                
            
                .process_type::<LuaTextureSlicer>()
                
            
                .process_type::<LuaSliceScaleMode>()
            },
        ))
    }

    fn setup_script(
        &mut self,
        script_data: &bevy_mod_scripting_core::hosts::ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        Ok(())
    }

    fn setup_script_runtime(
        &mut self,
        world_ptr: bevy_mod_scripting_core::world::WorldPointer,
        _script_data: &bevy_mod_scripting_core::hosts::ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        Ok(())
    }

    fn register_with_app(&self, app: &mut bevy::app::App) {
        app.register_foreign_lua_type::<bevy::sprite::prelude::ColorMaterial>();

        app.register_foreign_lua_type::<bevy::sprite::Mesh2dHandle>();

        app.register_foreign_lua_type::<bevy::sprite::prelude::Sprite>();

        app.register_foreign_lua_type::<bevy::sprite::prelude::ImageScaleMode>();

        app.register_foreign_lua_type::<bevy::sprite::Anchor>();

        app.register_foreign_lua_type::<bevy::sprite::prelude::TextureAtlasLayout>();

        app.register_foreign_lua_type::<bevy::sprite::prelude::TextureAtlas>();

        app.register_foreign_lua_type::<bevy::sprite::prelude::BorderRect>();

        app.register_foreign_lua_type::<bevy::sprite::prelude::TextureSlicer>();

        app.register_foreign_lua_type::<bevy::sprite::prelude::SliceScaleMode>();
    }
}
