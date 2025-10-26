#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use bevy_mod_scripting_bindings::{
    ReflectReference,
    function::{
        from::{Mut, Ref, Val},
        namespace::NamespaceBuilder,
    },
};
use bevy_mod_scripting_derive::script_bindings;
pub struct BevyImageScriptingPlugin;
pub(crate) fn register_texture_atlas_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_image::prelude::TextureAtlas,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_image::prelude::TextureAtlas>| {
                let output: () = {
                    {
                        let output: () = <::bevy_image::prelude::TextureAtlas as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_image::prelude::TextureAtlas>| {
                let output: Val<::bevy_image::prelude::TextureAtlas> = {
                    {
                        let output: Val<::bevy_image::prelude::TextureAtlas> = <::bevy_image::prelude::TextureAtlas as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_image::prelude::TextureAtlas>,
                other: Ref<::bevy_image::prelude::TextureAtlas>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_image::prelude::TextureAtlas as ::std::cmp::PartialEq<
                            ::bevy_image::prelude::TextureAtlas,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_image::prelude::TextureAtlas,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_texture_atlas_layout_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_image::prelude::TextureAtlasLayout,
    >::new(world)
        .register_documented(
            "add_texture",
            |
                mut _self: Mut<::bevy_image::prelude::TextureAtlasLayout>,
                rect: Val<::bevy_math::URect>|
            {
                let output: usize = {
                    {
                        let output: usize = ::bevy_image::prelude::TextureAtlasLayout::add_texture(
                                &mut _self,
                                rect.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Add a *section* to the list in the layout and returns its index\n which can be used with [`TextureAtlas`]\n # Arguments\n * `rect` - The section of the texture to be added\n [`TextureAtlas`]: crate::TextureAtlas",
            &["_self", "rect"],
        )
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_image::prelude::TextureAtlasLayout>| {
                let output: () = {
                    {
                        let output: () = <::bevy_image::prelude::TextureAtlasLayout as ::std::cmp::Eq>::assert_receiver_is_total_eq(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_image::prelude::TextureAtlasLayout>| {
                let output: Val<::bevy_image::prelude::TextureAtlasLayout> = {
                    {
                        let output: Val<::bevy_image::prelude::TextureAtlasLayout> = <::bevy_image::prelude::TextureAtlasLayout as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_image::prelude::TextureAtlasLayout>,
                other: Ref<::bevy_image::prelude::TextureAtlasLayout>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_image::prelude::TextureAtlasLayout as ::std::cmp::PartialEq<
                            ::bevy_image::prelude::TextureAtlasLayout,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "is_empty",
            |_self: Ref<::bevy_image::prelude::TextureAtlasLayout>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_image::prelude::TextureAtlasLayout::is_empty(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "len",
            |_self: Ref<::bevy_image::prelude::TextureAtlasLayout>| {
                let output: usize = {
                    {
                        let output: usize = ::bevy_image::prelude::TextureAtlasLayout::len(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The number of textures in the [`TextureAtlasLayout`]",
            &["_self"],
        )
        .register_documented(
            "new_empty",
            |dimensions: Val<::bevy_math::UVec2>| {
                let output: Val<::bevy_image::prelude::TextureAtlasLayout> = {
                    {
                        let output: Val<::bevy_image::prelude::TextureAtlasLayout> = ::bevy_image::prelude::TextureAtlasLayout::new_empty(
                                dimensions.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new empty layout with custom `dimensions`",
            &["dimensions"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_image::prelude::TextureAtlasLayout,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_image_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_image::prelude::Image,
    >::new(world)
        .register_documented(
            "aspect_ratio",
            |_self: Ref<::bevy_image::prelude::Image>| {
                let output: Val<::bevy_math::AspectRatio> = {
                    {
                        let output: Val<::bevy_math::AspectRatio> = ::bevy_image::prelude::Image::aspect_ratio(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the aspect ratio (width / height) of a 2D image.",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_image::prelude::Image>| {
                let output: Val<::bevy_image::prelude::Image> = {
                    {
                        let output: Val<::bevy_image::prelude::Image> = <::bevy_image::prelude::Image as ::std::clone::Clone>::clone(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        )
        .register_documented(
            "default_uninit",
            || {
                let output: Val<::bevy_image::prelude::Image> = {
                    {
                        let output: Val<::bevy_image::prelude::Image> = ::bevy_image::prelude::Image::default_uninit()
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new uninitialized 1x1x1 image",
            &[],
        )
        .register_documented(
            "height",
            |_self: Ref<::bevy_image::prelude::Image>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_image::prelude::Image::height(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the height of a 2D image.",
            &["_self"],
        )
        .register_documented(
            "is_compressed",
            |_self: Ref<::bevy_image::prelude::Image>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_image::prelude::Image::is_compressed(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Whether the texture format is compressed or uncompressed",
            &["_self"],
        )
        .register_documented(
            "pixel_data_offset",
            |_self: Ref<::bevy_image::prelude::Image>, coords: Val<::bevy_math::UVec3>| {
                let output: ::std::option::Option<usize> = {
                    {
                        let output: ::std::option::Option<usize> = ::bevy_image::prelude::Image::pixel_data_offset(
                                &_self,
                                coords.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the byte offset where the data of a specific pixel is stored\n Returns None if the provided coordinates are out of bounds.\n For 2D textures, Z is the layer number. For 1D textures, Y and Z are ignored.",
            &["_self", "coords"],
        )
        .register_documented(
            "reinterpret_stacked_2d_as_array",
            |mut _self: Mut<::bevy_image::prelude::Image>, layers: u32| {
                let output: () = {
                    {
                        let output: () = ::bevy_image::prelude::Image::reinterpret_stacked_2d_as_array(
                                &mut _self,
                                layers,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Takes a 2D image containing vertically stacked images of the same size, and reinterprets\n it as a 2D array texture, where each of the stacked images becomes one layer of the\n array. This is primarily for use with the `texture2DArray` shader uniform type.\n # Panics\n Panics if the texture is not 2D, has more than one layers or is not evenly dividable into\n the `layers`.",
            &["_self", "layers"],
        )
        .register_documented(
            "size",
            |_self: Ref<::bevy_image::prelude::Image>| {
                let output: Val<::bevy_math::UVec2> = {
                    {
                        let output: Val<::bevy_math::UVec2> = ::bevy_image::prelude::Image::size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the size of a 2D image.",
            &["_self"],
        )
        .register_documented(
            "size_f32",
            |_self: Ref<::bevy_image::prelude::Image>| {
                let output: Val<::bevy_math::Vec2> = {
                    {
                        let output: Val<::bevy_math::Vec2> = ::bevy_image::prelude::Image::size_f32(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the size of a 2D image as f32.",
            &["_self"],
        )
        .register_documented(
            "transparent",
            || {
                let output: Val<::bevy_image::prelude::Image> = {
                    {
                        let output: Val<::bevy_image::prelude::Image> = ::bevy_image::prelude::Image::transparent()
                            .into();
                        output
                    }
                };
                output
            },
            " A transparent white 1x1x1 image.\n Contrast to [`Image::default`], which is opaque.",
            &[],
        )
        .register_documented(
            "width",
            |_self: Ref<::bevy_image::prelude::Image>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_image::prelude::Image::width(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the width of a 2D image.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_image::prelude::Image,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyImageScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_texture_atlas_functions(&mut world);
        register_texture_atlas_layout_functions(&mut world);
        register_image_functions(&mut world);
    }
}
