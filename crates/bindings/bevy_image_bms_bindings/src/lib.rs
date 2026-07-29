
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
extern crate std;


use bevy_mod_scripting_bindings::{
    ReflectReference,
    function::{
        from::{R, M, V},
        namespace::NamespaceBuilder,
    },
};
use bevy_ecs::prelude::*;
use bevy_app::{App, Plugin};
use bevy_mod_scripting_derive::script_bindings;
pub struct BevyImageScriptingPlugin;
pub(crate) fn register_texture_atlas_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_image::TextureAtlas,
    >::new(world)
        .register_documented(
            "assert_fields_are_eq",
            |_self: R<::bevy_image::TextureAtlas>| {
                let output: () = {
                    {
                        let output: () = <::bevy_image::TextureAtlas as ::std::cmp::Eq>::assert_fields_are_eq(
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
            |_self: R<::bevy_image::TextureAtlas>| {
                let output: V<::bevy_image::TextureAtlas> = {
                    {
                        let output: V<::bevy_image::TextureAtlas> = <::bevy_image::TextureAtlas as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_image::TextureAtlas>, other: R<::bevy_image::TextureAtlas>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_image::TextureAtlas as ::std::cmp::PartialEq<
                            ::bevy_image::TextureAtlas,
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
            "with_index",
            |_self: V<::bevy_image::TextureAtlas>, index: usize| {
                let output: V<::bevy_image::TextureAtlas> = {
                    {
                        let output: V<::bevy_image::TextureAtlas> = ::bevy_image::TextureAtlas::with_index(
                                _self.into_inner(),
                                index,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns this [`TextureAtlas`] with the specified index.",
            &["_self", "index"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_image::TextureAtlas,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_texture_atlas_layout_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_image::TextureAtlasLayout,
    >::new(world)
        .register_documented(
            "add_texture",
            |
                mut _self: M<::bevy_image::TextureAtlasLayout>,
                rect: V<::bevy_math::URect>|
            {
                let output: usize = {
                    {
                        let output: usize = ::bevy_image::TextureAtlasLayout::add_texture(
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
            "assert_fields_are_eq",
            |_self: R<::bevy_image::TextureAtlasLayout>| {
                let output: () = {
                    {
                        let output: () = <::bevy_image::TextureAtlasLayout as ::std::cmp::Eq>::assert_fields_are_eq(
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
            |_self: R<::bevy_image::TextureAtlasLayout>| {
                let output: V<::bevy_image::TextureAtlasLayout> = {
                    {
                        let output: V<::bevy_image::TextureAtlasLayout> = <::bevy_image::TextureAtlasLayout as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_image::TextureAtlasLayout>,
                other: R<::bevy_image::TextureAtlasLayout>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_image::TextureAtlasLayout as ::std::cmp::PartialEq<
                            ::bevy_image::TextureAtlasLayout,
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
            |_self: R<::bevy_image::TextureAtlasLayout>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_image::TextureAtlasLayout::is_empty(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if the atlas contains no textures.",
            &["_self"],
        )
        .register_documented(
            "len",
            |_self: R<::bevy_image::TextureAtlasLayout>| {
                let output: usize = {
                    {
                        let output: usize = ::bevy_image::TextureAtlasLayout::len(&_self)
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
            |dimensions: V<::bevy_math::UVec2>| {
                let output: V<::bevy_image::TextureAtlasLayout> = {
                    {
                        let output: V<::bevy_image::TextureAtlasLayout> = ::bevy_image::TextureAtlasLayout::new_empty(
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
            ::bevy_image::TextureAtlasLayout,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_image_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_image::Image,
    >::new(world)
        .register_documented(
            "aspect_ratio",
            |_self: R<::bevy_image::Image>| {
                let output: V<::bevy_math::AspectRatio> = {
                    {
                        let output: V<::bevy_math::AspectRatio> = ::bevy_image::Image::aspect_ratio(
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
            |_self: R<::bevy_image::Image>| {
                let output: V<::bevy_image::Image> = {
                    {
                        let output: V<::bevy_image::Image> = <::bevy_image::Image as ::std::clone::Clone>::clone(
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
                let output: V<::bevy_image::Image> = {
                    {
                        let output: V<::bevy_image::Image> = ::bevy_image::Image::default_uninit()
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
            "eq",
            |_self: R<::bevy_image::Image>, other: R<::bevy_image::Image>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_image::Image as ::std::cmp::PartialEq<
                            ::bevy_image::Image,
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
            "height",
            |_self: R<::bevy_image::Image>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_image::Image::height(&_self).into();
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
            |_self: R<::bevy_image::Image>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_image::Image::is_compressed(&_self)
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
            "size",
            |_self: R<::bevy_image::Image>| {
                let output: V<::bevy_math::UVec2> = {
                    {
                        let output: V<::bevy_math::UVec2> = ::bevy_image::Image::size(
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
            |_self: R<::bevy_image::Image>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_image::Image::size_f32(
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
                let output: V<::bevy_image::Image> = {
                    {
                        let output: V<::bevy_image::Image> = ::bevy_image::Image::transparent()
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
            |_self: R<::bevy_image::Image>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_image::Image::width(&_self).into();
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
            ::bevy_image::Image,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_image_sampler_descriptor_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_image::ImageSamplerDescriptor,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_image::ImageSamplerDescriptor>| {
                let output: V<::bevy_image::ImageSamplerDescriptor> = {
                    {
                        let output: V<::bevy_image::ImageSamplerDescriptor> = <::bevy_image::ImageSamplerDescriptor as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_image::ImageSamplerDescriptor>,
                other: R<::bevy_image::ImageSamplerDescriptor>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_image::ImageSamplerDescriptor as ::std::cmp::PartialEq<
                            ::bevy_image::ImageSamplerDescriptor,
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
            "linear",
            || {
                let output: V<::bevy_image::ImageSamplerDescriptor> = {
                    {
                        let output: V<::bevy_image::ImageSamplerDescriptor> = ::bevy_image::ImageSamplerDescriptor::linear()
                            .into();
                        output
                    }
                };
                output
            },
            " Returns a sampler descriptor with [`Linear`](ImageFilterMode::Linear) min and mag filters",
            &[],
        )
        .register_documented(
            "nearest",
            || {
                let output: V<::bevy_image::ImageSamplerDescriptor> = {
                    {
                        let output: V<::bevy_image::ImageSamplerDescriptor> = ::bevy_image::ImageSamplerDescriptor::nearest()
                            .into();
                        output
                    }
                };
                output
            },
            " Returns a sampler descriptor with [`Nearest`](ImageFilterMode::Nearest) min and mag filters",
            &[],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_image::ImageSamplerDescriptor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_image_sampler_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_image::ImageSampler,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_image::ImageSampler>| {
                let output: V<::bevy_image::ImageSampler> = {
                    {
                        let output: V<::bevy_image::ImageSampler> = <::bevy_image::ImageSampler as ::std::clone::Clone>::clone(
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
            |_self: R<::bevy_image::ImageSampler>, other: R<::bevy_image::ImageSampler>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_image::ImageSampler as ::std::cmp::PartialEq<
                            ::bevy_image::ImageSampler,
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
            "linear",
            || {
                let output: V<::bevy_image::ImageSampler> = {
                    {
                        let output: V<::bevy_image::ImageSampler> = ::bevy_image::ImageSampler::linear()
                            .into();
                        output
                    }
                };
                output
            },
            " Returns an image sampler with [`ImageFilterMode::Linear`] min and mag filters",
            &[],
        )
        .register_documented(
            "nearest",
            || {
                let output: V<::bevy_image::ImageSampler> = {
                    {
                        let output: V<::bevy_image::ImageSampler> = ::bevy_image::ImageSampler::nearest()
                            .into();
                        output
                    }
                };
                output
            },
            " Returns an image sampler with [`ImageFilterMode::Nearest`] min and mag filters",
            &[],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_image::ImageSampler,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_image_address_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_image::ImageAddressMode,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_image::ImageAddressMode>| {
                let output: V<::bevy_image::ImageAddressMode> = {
                    {
                        let output: V<::bevy_image::ImageAddressMode> = <::bevy_image::ImageAddressMode as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_image::ImageAddressMode>,
                other: R<::bevy_image::ImageAddressMode>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_image::ImageAddressMode as ::std::cmp::PartialEq<
                            ::bevy_image::ImageAddressMode,
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
            ::bevy_image::ImageAddressMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_image_filter_mode_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_image::ImageFilterMode,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_image::ImageFilterMode>| {
                let output: V<::bevy_image::ImageFilterMode> = {
                    {
                        let output: V<::bevy_image::ImageFilterMode> = <::bevy_image::ImageFilterMode as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_image::ImageFilterMode>,
                other: R<::bevy_image::ImageFilterMode>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_image::ImageFilterMode as ::std::cmp::PartialEq<
                            ::bevy_image::ImageFilterMode,
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
            ::bevy_image::ImageFilterMode,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_image_compare_function_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_image::ImageCompareFunction,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_image::ImageCompareFunction>| {
                let output: V<::bevy_image::ImageCompareFunction> = {
                    {
                        let output: V<::bevy_image::ImageCompareFunction> = <::bevy_image::ImageCompareFunction as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_image::ImageCompareFunction>,
                other: R<::bevy_image::ImageCompareFunction>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_image::ImageCompareFunction as ::std::cmp::PartialEq<
                            ::bevy_image::ImageCompareFunction,
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
            ::bevy_image::ImageCompareFunction,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_image_sampler_border_color_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_image::ImageSamplerBorderColor,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_image::ImageSamplerBorderColor>| {
                let output: V<::bevy_image::ImageSamplerBorderColor> = {
                    {
                        let output: V<::bevy_image::ImageSamplerBorderColor> = <::bevy_image::ImageSamplerBorderColor as ::std::clone::Clone>::clone(
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
                _self: R<::bevy_image::ImageSamplerBorderColor>,
                other: R<::bevy_image::ImageSamplerBorderColor>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_image::ImageSamplerBorderColor as ::std::cmp::PartialEq<
                            ::bevy_image::ImageSamplerBorderColor,
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
            ::bevy_image::ImageSamplerBorderColor,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyImageScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_texture_atlas_functions(&mut world);
        register_texture_atlas_layout_functions(&mut world);
        register_image_functions(&mut world);
        register_image_sampler_descriptor_functions(&mut world);
        register_image_sampler_functions(&mut world);
        register_image_address_mode_functions(&mut world);
        register_image_filter_mode_functions(&mut world);
        register_image_compare_function_functions(&mut world);
        register_image_sampler_border_color_functions(&mut world);
    }
}
