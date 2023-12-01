#![allow(clippy::all, unused_imports, deprecated, dead_code)]
extern crate self as bevy_script_api;
use bevy::prelude::App;
use bevy_mod_scripting_core::prelude::*;
use std::sync::Mutex;
#[cfg(feature = "lua")]
use {
    crate::{lua::RegisterForeignLuaType, ReflectedValue},
    bevy_mod_scripting_lua::docs::LuaDocFragment,
    bevy_mod_scripting_lua_derive::LuaProxy,
};
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::primitives::Aabb",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Aabb {
    center: ReflectedValue,
    half_extents: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Affine2",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="MetaFunction", as_trait="std::cmp::PartialEq", composite="eq", metamethod=Eq, )]
    fn eq (&self, #[proxy] rhs : &Self,) -> bool;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct Affine2 {
    matrix2: ReflectedValue,
    translation: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Affine3A",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="MetaFunction", as_trait="std::cmp::PartialEq", composite="eq", metamethod=Eq, )]
    fn eq (&self, #[proxy] rhs : &Self,) -> bool;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct Affine3A {
    matrix3: ReflectedValue,
    translation: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::AlignContent",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct AlignContent;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::AlignItems",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct AlignItems;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::AlignSelf",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct AlignSelf;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::AlphaMode",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct AlphaMode;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::AmbientLight",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct AmbientLight {
    color: ReflectedValue,
    brightness: f32,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::sprite::Anchor",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct Anchor;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::animation::AnimationClip",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct AnimationClip {}
#[derive(LuaProxy)]
#[proxy(
    derive(),
    remote = "bevy::animation::AnimationPlayer",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct AnimationPlayer {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::asset::AssetPathId",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct AssetPathId();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::BVec2",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct BVec2 {
    x: bool,
    y: bool,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::BVec3",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct BVec3 {
    x: bool,
    y: bool,
    z: bool,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::BVec3A",
    functions[r#"
    #[lua(kind="MetaFunction", as_trait="std::cmp::PartialEq", composite="eq", metamethod=Eq, )]
    fn eq (&self, #[proxy] rhs : &Self,) -> bool;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct BVec3A();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::BVec4",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct BVec4 {
    x: bool,
    y: bool,
    z: bool,
    w: bool,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::BVec4A",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="MetaFunction", as_trait="std::cmp::PartialEq", composite="eq", metamethod=Eq, )]
    fn eq (&self, #[proxy] rhs : &Self,) -> bool;"#,
    ]
)]
pub struct BVec4A();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::BackgroundColor",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::TupleStruct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct BackgroundColor(ReflectedValue);
#[derive(LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::core_pipeline::bloom::BloomCompositeMode",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct BloomCompositeMode;
#[derive(LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::core_pipeline::bloom::BloomPrefilterSettings",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct BloomPrefilterSettings {
    threshold: f32,
    threshold_softness: f32,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::core_pipeline::bloom::BloomSettings",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct BloomSettings {
    intensity: f32,
    low_frequency_boost: f32,
    low_frequency_boost_curvature: f32,
    high_pass_frequency: f32,
    prefilter_settings: ReflectedValue,
    composite_mode: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::BorderColor",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::TupleStruct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct BorderColor(ReflectedValue);
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::text::BreakLineOn",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct BreakLineOn;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::widget::Button",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Button;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::CalculatedClip",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct CalculatedClip {
    clip: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::camera::Camera",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Camera {
    viewport: ReflectedValue,
    order: isize,
    is_active: bool,
    computed: ReflectedValue,
    target: ReflectedValue,
    hdr: bool,
    output_mode: ReflectedValue,
    msaa_writeback: bool,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::core_pipeline::core_2d::Camera2d",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Camera2d {
    clear_color: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::core_pipeline::core_3d::Camera3d",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct Camera3d {
    clear_color: ReflectedValue,
    depth_load_op: ReflectedValue,
    depth_texture_usages: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::core_pipeline::core_3d::Camera3dDepthLoadOp",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct Camera3dDepthLoadOp;
#[derive(LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::core_pipeline::core_3d::Camera3dDepthTextureUsage",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::TupleStruct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Camera3dDepthTextureUsage();
#[derive(LuaProxy)]
#[proxy(
    derive(),
    remote = "bevy::render::camera::CameraRenderGraph",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::TupleStruct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct CameraRenderGraph();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::Cascade",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Cascade {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::CascadeShadowConfig",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct CascadeShadowConfig {
    bounds: ReflectedValue,
    overlap_proportion: f32,
    minimum_distance: f32,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::Cascades",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Cascades {}
#[derive(LuaProxy)]
#[proxy(
    derive(debug),
    remote = "bevy::render::primitives::CascadesFrusta",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct CascadesFrusta {
    frusta: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::CascadesVisibleEntities",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct CascadesVisibleEntities {
    entities: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(debug),
    remote = "bevy::hierarchy::Children",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::TupleStruct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Children();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::core_pipeline::clear_color::ClearColor",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::TupleStruct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct ClearColor(ReflectedValue);
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::core_pipeline::clear_color::ClearColorConfig",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct ClearColorConfig;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::ClusterConfig",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct ClusterConfig;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::ClusterFarZMode",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct ClusterFarZMode;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::ClusterZConfig",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct ClusterZConfig {
    first_slice_depth: f32,
    far_z_mode: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::color::Color",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct Color;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::view::ColorGrading",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct ColorGrading {
    exposure: f32,
    gamma: f32,
    pre_saturation: f32,
    post_saturation: f32,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::sprite::ColorMaterial",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct ColorMaterial {
    color: ReflectedValue,
    texture: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::view::ComputedVisibility",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct ComputedVisibility {}
#[derive(LuaProxy)]
#[proxy(
    derive(debug),
    remote = "bevy::ui::ContentSize",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct ContentSize {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::core_pipeline::contrast_adaptive_sharpening::ContrastAdaptiveSharpeningSettings",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct ContrastAdaptiveSharpeningSettings {
    enabled: bool,
    sharpening_strength: f32,
    denoise: bool,
}
#[derive(LuaProxy)]
#[proxy(
    derive(debug),
    remote = "bevy::render::primitives::CubemapFrusta",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct CubemapFrusta {
    frusta: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::CubemapVisibleEntities",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct CubemapVisibleEntities {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::DAffine2",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="MetaFunction", as_trait="std::cmp::PartialEq", composite="eq", metamethod=Eq, )]
    fn eq (&self, #[proxy] rhs : &Self,) -> bool;"#,
    ]
)]
pub struct DAffine2 {
    matrix2: ReflectedValue,
    translation: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::DAffine3",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="MetaFunction", as_trait="std::cmp::PartialEq", composite="eq", metamethod=Eq, )]
    fn eq (&self, #[proxy] rhs : &Self,) -> bool;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct DAffine3 {
    matrix3: ReflectedValue,
    translation: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::DMat2",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="MetaFunction", as_trait="std::cmp::PartialEq", composite="eq", metamethod=Eq, )]
    fn eq (&self, #[proxy] rhs : &Self,) -> bool;"#,
    ]
)]
pub struct DMat2 {
    x_axis: ReflectedValue,
    y_axis: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::DMat3",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="MetaFunction", as_trait="std::cmp::PartialEq", composite="eq", metamethod=Eq, )]
    fn eq (&self, #[proxy] rhs : &Self,) -> bool;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct DMat3 {
    x_axis: ReflectedValue,
    y_axis: ReflectedValue,
    z_axis: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::DMat4",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="MetaFunction", as_trait="std::cmp::PartialEq", composite="eq", metamethod=Eq, )]
    fn eq (&self, #[proxy] rhs : &Self,) -> bool;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct DMat4 {
    x_axis: ReflectedValue,
    y_axis: ReflectedValue,
    z_axis: ReflectedValue,
    w_axis: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::DQuat",
    functions[r#"
    #[lua(kind="MetaFunction", as_trait="std::cmp::PartialEq", composite="eq", metamethod=Eq, )]
    fn eq (&self, #[proxy] rhs : &Self,) -> bool;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct DQuat {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::DVec2",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct DVec2 {
    x: f64,
    y: f64,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::DVec3",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct DVec3 {
    x: f64,
    y: f64,
    z: f64,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::DVec4",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct DVec4 {
    x: f64,
    y: f64,
    z: f64,
    w: f64,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::core_pipeline::tonemapping::DebandDither",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct DebandDither;
#[derive(LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::core_pipeline::contrast_adaptive_sharpening::DenoiseCAS",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::TupleStruct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct DenoiseCAS();
#[derive(LuaProxy)]
#[proxy(
    derive(),
    remote = "bevy::core_pipeline::prepass::DepthPrepass",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct DepthPrepass;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::Direction",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct Direction;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::DirectionalLight",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct DirectionalLight {
    color: ReflectedValue,
    illuminance: f32,
    shadows_enabled: bool,
    shadow_depth_bias: f32,
    shadow_normal_bias: f32,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::DirectionalLightShadowMap",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct DirectionalLightShadowMap {
    size: usize,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::Display",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct Display;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "core::time::Duration",
    functions[r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::DivAssign", )]
    fn div_assign (&mut self, rhs : u32, );"#,
    r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::MulAssign", )]
    fn mul_assign (&mut self, rhs : u32, );"#,
    ]
)]
pub struct Duration {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ecs::entity::Entity",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::ecs::storage::SparseSetIndex", )]
    fn sparse_set_index (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="bevy::ecs::storage::SparseSetIndex", output(proxy))]
    fn get_sparse_set_index (value : usize, ) -> Self;"#,
    ]
)]
pub struct Entity {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::animation::EntityPath",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct EntityPath {
    parts: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::pbr::EnvironmentMapLight",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct EnvironmentMapLight {
    diffuse_map: ReflectedValue,
    specular_map: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::EulerRot",
    functions[r#"
    ///Default `YXZ` as yaw (y-axis), pitch (x-axis), roll (z-axis).
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct EulerRot;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::FlexDirection",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct FlexDirection;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::FlexWrap",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct FlexWrap;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::FocusPolicy",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct FocusPolicy;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::FogFalloff",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct FogFalloff;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::FogSettings",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct FogSettings {
    color: ReflectedValue,
    directional_light_color: ReflectedValue,
    directional_light_exponent: f32,
    falloff: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::primitives::Frustum",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Frustum {
    half_spaces: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::core_pipeline::fxaa::Fxaa",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct Fxaa {
    enabled: bool,
    edge_threshold: ReflectedValue,
    edge_threshold_min: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::transform::components::GlobalTransform",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::TupleStruct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct GlobalTransform();
#[derive(LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::render::globals::GlobalsUniform",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct GlobalsUniform {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::gltf::GltfExtras",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct GltfExtras {
    value: String,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::GridAutoFlow",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct GridAutoFlow;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::GridPlacement",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct GridPlacement {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::GridTrack",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct GridTrack {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::GridTrackRepetition",
    functions[r#"
    #[lua(kind="Function", as_trait="core::convert::From", output(proxy))]
    fn from (count : i32, ) -> Self;"#,
    r#"
    #[lua(kind="Function", as_trait="core::convert::From", output(proxy))]
    fn from (count : u16, ) -> Self;"#,
    r#"
    #[lua(kind="Function", as_trait="core::convert::From", output(proxy))]
    fn from (count : usize, ) -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct GridTrackRepetition;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::asset::HandleId",
    functions[r#"
    #[lua(kind="Function", as_trait="core::convert::From", output(proxy))]
    fn from (value : &String, ) -> Self;"#,
    r#"
    #[lua(kind="Function", as_trait="core::convert::From", output(proxy))]
    fn from (value : String, ) -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct HandleId;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::IVec2",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct IVec2 {
    x: i32,
    y: i32,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::IVec3",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct IVec3 {
    x: i32,
    y: i32,
    z: i32,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::IVec4",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct IVec4 {
    x: i32,
    y: i32,
    z: i32,
    w: i32,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::texture::Image",
    functions[r#"
    ///default is a 1x1x1 all '1.0' texture
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct Image {
    data: ReflectedValue,
    texture_descriptor: ReflectedValue,
    sampler_descriptor: ReflectedValue,
    texture_view_descriptor: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "std::time::Instant", functions[])]
pub struct Instant();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::Interaction",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct Interaction;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::JustifyContent",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct JustifyContent;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::JustifyItems",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct JustifyItems;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::JustifySelf",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct JustifySelf;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::animation::Keyframes",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct Keyframes;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::widget::Label",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Label;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::asset::LabelId",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct LabelId();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::camera::ManualTextureViewHandle",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::TupleStruct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct ManualTextureViewHandle(u32);
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Mat2",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="MetaFunction", as_trait="std::cmp::PartialEq", composite="eq", metamethod=Eq, )]
    fn eq (&self, #[proxy] rhs : &Self,) -> bool;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct Mat2();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Mat3",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="MetaFunction", as_trait="std::cmp::PartialEq", composite="eq", metamethod=Eq, )]
    fn eq (&self, #[proxy] rhs : &Self,) -> bool;"#,
    ]
)]
pub struct Mat3 {
    x_axis: ReflectedValue,
    y_axis: ReflectedValue,
    z_axis: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Mat3A",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="MetaFunction", as_trait="std::cmp::PartialEq", composite="eq", metamethod=Eq, )]
    fn eq (&self, #[proxy] rhs : &Self,) -> bool;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct Mat3A {
    x_axis: ReflectedValue,
    y_axis: ReflectedValue,
    z_axis: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Mat4",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="MetaFunction", as_trait="std::cmp::PartialEq", composite="eq", metamethod=Eq, )]
    fn eq (&self, #[proxy] rhs : &Self,) -> bool;"#,
    ]
)]
pub struct Mat4 {
    x_axis: ReflectedValue,
    y_axis: ReflectedValue,
    z_axis: ReflectedValue,
    w_axis: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::ui::MaxTrackSizingFunction", functions[])]
pub struct MaxTrackSizingFunction;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::sprite::Mesh2dHandle",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::TupleStruct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Mesh2dHandle(ReflectedValue);
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::mesh::morph::MeshMorphWeights",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct MeshMorphWeights {}
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "bevy::ui::MinTrackSizingFunction", functions[])]
pub struct MinTrackSizingFunction;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::prelude::MorphWeights",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct MorphWeights {}
#[derive(LuaProxy)]
#[proxy(
    derive(),
    remote = "bevy::core_pipeline::prepass::MotionVectorPrepass",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct MotionVectorPrepass;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::view::Msaa",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct Msaa;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::core::Name",
    functions[r#"
    #[lua(kind="Function", as_trait="core::convert::From", output(proxy))]
    fn from (name : String, ) -> Self;"#,
    r#"
    #[lua(kind="MetaFunction", as_trait="std::cmp::PartialEq", composite="eq", metamethod=Eq, )]
    fn eq (&self, #[proxy] other : &Self,) -> bool;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct Name {}
#[derive(LuaProxy)]
#[proxy(
    derive(),
    remote = "bevy::render::view::NoFrustumCulling",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct NoFrustumCulling;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::Node",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct Node {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "core::num::NonZeroI128",
    functions[r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, rhs : i128, );"#,
    r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, #[proxy] rhs : Self,);"#,
    ]
)]
pub struct NonZeroI128();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "core::num::NonZeroI16",
    functions[r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, #[proxy] rhs : Self,);"#,
    r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, rhs : i16, );"#,
    ]
)]
pub struct NonZeroI16();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "core::num::NonZeroI32",
    functions[r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, #[proxy] rhs : Self,);"#,
    r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, rhs : i32, );"#,
    ]
)]
pub struct NonZeroI32();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "core::num::NonZeroI64",
    functions[r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, rhs : i64, );"#,
    r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, #[proxy] rhs : Self,);"#,
    ]
)]
pub struct NonZeroI64();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "core::num::NonZeroI8",
    functions[r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, #[proxy] rhs : Self,);"#,
    r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, rhs : i8, );"#,
    ]
)]
pub struct NonZeroI8();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "core::num::NonZeroIsize",
    functions[r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, #[proxy] rhs : Self,);"#,
    r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, rhs : isize, );"#,
    ]
)]
pub struct NonZeroIsize();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "core::num::NonZeroU128",
    functions[r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, #[proxy] rhs : Self,);"#,
    r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, rhs : u128, );"#,
    ]
)]
pub struct NonZeroU128();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "core::num::NonZeroU16",
    functions[r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, #[proxy] rhs : Self,);"#,
    r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, rhs : u16, );"#,
    ]
)]
pub struct NonZeroU16();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "core::num::NonZeroU32",
    functions[r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, rhs : u32, );"#,
    r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, #[proxy] rhs : Self,);"#,
    ]
)]
pub struct NonZeroU32();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "core::num::NonZeroU64",
    functions[r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, rhs : u64, );"#,
    r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, #[proxy] rhs : Self,);"#,
    ]
)]
pub struct NonZeroU64();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "core::num::NonZeroU8",
    functions[r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, #[proxy] rhs : Self,);"#,
    r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, rhs : u8, );"#,
    ]
)]
pub struct NonZeroU8();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "core::num::NonZeroUsize",
    functions[r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, rhs : usize, );"#,
    r#"
    #[lua(kind="MutatingMethod", as_trait="core::ops::BitOrAssign", )]
    fn bitor_assign (&mut self, #[proxy] rhs : Self,);"#,
    ]
)]
pub struct NonZeroUsize();
#[derive(LuaProxy)]
#[proxy(
    derive(),
    remote = "bevy::core_pipeline::prepass::NormalPrepass",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct NormalPrepass;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::camera::NormalizedRenderTarget",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct NormalizedRenderTarget;
#[derive(LuaProxy)]
#[proxy(
    derive(),
    remote = "bevy::pbr::NotShadowCaster",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct NotShadowCaster;
#[derive(LuaProxy)]
#[proxy(
    derive(),
    remote = "bevy::pbr::NotShadowReceiver",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct NotShadowReceiver;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::camera::OrthographicProjection",
    functions[r#"
    #[lua(kind="MutatingMethod", as_trait="bevy::render::camera::CameraProjection", )]
    fn update (&mut self, width : f32, height : f32, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::render::camera::CameraProjection", )]
    fn far (&self, ) -> f32;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct OrthographicProjection {
    near: f32,
    far: f32,
    viewport_origin: ReflectedValue,
    scaling_mode: ReflectedValue,
    scale: f32,
    area: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "std::ffi::OsString",
    functions[r#"
    #[lua(kind="Method", as_trait="core::clone::Clone", output(proxy))]
    fn clone (&self, ) -> Self;"#,
    r#"
    #[lua(kind="MutatingMethod", as_trait="core::clone::Clone", )]
    fn clone_from (&mut self, #[proxy] source : &Self,);"#,
    ]
)]
pub struct OsString {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::Overflow",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct Overflow {
    x: ReflectedValue,
    y: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::OverflowAxis",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct OverflowAxis;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::ParallaxMappingMethod",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct ParallaxMappingMethod;
#[derive(LuaProxy)]
#[proxy(
    derive(debug),
    remote = "bevy::hierarchy::Parent",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::TupleStruct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Parent();
#[derive(LuaProxy)]
#[proxy(derive(debug), remote = "std::path::Path", functions[])]
pub struct Path {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "std::path::PathBuf",
    functions[r#"
    #[lua(kind="Method", as_trait="core::clone::Clone", output(proxy))]
    fn clone (&self, ) -> Self;"#,
    r#"
    #[lua(kind="MutatingMethod", as_trait="core::clone::Clone", )]
    fn clone_from (&mut self, #[proxy] source : &Self,);"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct PathBuf {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::camera::PerspectiveProjection",
    functions[r#"
    #[lua(kind="MutatingMethod", as_trait="bevy::render::camera::CameraProjection", )]
    fn update (&mut self, width : f32, height : f32, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::render::camera::CameraProjection", )]
    fn far (&self, ) -> f32;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct PerspectiveProjection {
    fov: f32,
    aspect_ratio: f32,
    near: f32,
    far: f32,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::PointLight",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct PointLight {
    color: ReflectedValue,
    intensity: f32,
    range: f32,
    radius: f32,
    shadows_enabled: bool,
    shadow_depth_bias: f32,
    shadow_normal_bias: f32,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::PointLightShadowMap",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct PointLightShadowMap {
    size: usize,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::PositionType",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct PositionType;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::camera::Projection",
    functions[r#"
    #[lua(kind="MutatingMethod", as_trait="bevy::render::camera::CameraProjection", )]
    fn update (&mut self, width : f32, height : f32, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::render::camera::CameraProjection", )]
    fn far (&self, ) -> f32;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct Projection;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Quat",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="MetaFunction", as_trait="std::cmp::PartialEq", composite="eq", metamethod=Eq, )]
    fn eq (&self, #[proxy] rhs : &Self,) -> bool;"#,
    ]
)]
pub struct Quat();
#[derive(LuaProxy)]
#[proxy(derive(clone, debug), remote = "core::ops::RangeFull", functions[])]
pub struct RangeFull;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Rect",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Rect {
    min: ReflectedValue,
    max: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::RelativeCursorPosition",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct RelativeCursorPosition {
    normalized: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::view::RenderLayers",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::TupleStruct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct RenderLayers();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::camera::RenderTarget",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct RenderTarget;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::RepeatedGridTrack",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct RepeatedGridTrack {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::camera::ScalingMode",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct ScalingMode;
#[derive(LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct ScreenSpaceAmbientOcclusionQualityLevel;
#[derive(LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::pbr::ScreenSpaceAmbientOcclusionSettings",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct ScreenSpaceAmbientOcclusionSettings {
    quality_level: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::core_pipeline::fxaa::Sensitivity",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct Sensitivity;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::mesh::skinning::SkinnedMesh",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct SkinnedMesh {
    inverse_bindposes: ReflectedValue,
    joints: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::asset::SourcePathId",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct SourcePathId();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::SpotLight",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct SpotLight {
    color: ReflectedValue,
    intensity: f32,
    range: f32,
    radius: f32,
    shadows_enabled: bool,
    shadow_depth_bias: f32,
    shadow_normal_bias: f32,
    outer_angle: f32,
    inner_angle: f32,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::sprite::Sprite",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Sprite {
    color: ReflectedValue,
    flip_x: bool,
    flip_y: bool,
    custom_size: ReflectedValue,
    rect: ReflectedValue,
    anchor: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::StandardMaterial",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::pbr::Material", )]
    fn depth_bias (&self, ) -> f32;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct StandardMaterial {
    base_color: ReflectedValue,
    base_color_texture: ReflectedValue,
    emissive: ReflectedValue,
    emissive_texture: ReflectedValue,
    perceptual_roughness: f32,
    metallic: f32,
    metallic_roughness_texture: ReflectedValue,
    reflectance: f32,
    normal_map_texture: ReflectedValue,
    flip_normal_map_y: bool,
    occlusion_texture: ReflectedValue,
    double_sided: bool,
    cull_mode: ReflectedValue,
    unlit: bool,
    fog_enabled: bool,
    alpha_mode: ReflectedValue,
    depth_bias: f32,
    depth_map: ReflectedValue,
    parallax_depth_scale: f32,
    parallax_mapping_method: ReflectedValue,
    max_parallax_layer_count: f32,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::time::Stopwatch",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct Stopwatch {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::Style",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct Style {
    display: ReflectedValue,
    position_type: ReflectedValue,
    overflow: ReflectedValue,
    direction: ReflectedValue,
    left: ReflectedValue,
    right: ReflectedValue,
    top: ReflectedValue,
    bottom: ReflectedValue,
    width: ReflectedValue,
    height: ReflectedValue,
    min_width: ReflectedValue,
    min_height: ReflectedValue,
    max_width: ReflectedValue,
    max_height: ReflectedValue,
    aspect_ratio: ReflectedValue,
    align_items: ReflectedValue,
    justify_items: ReflectedValue,
    align_self: ReflectedValue,
    justify_self: ReflectedValue,
    align_content: ReflectedValue,
    justify_content: ReflectedValue,
    margin: ReflectedValue,
    padding: ReflectedValue,
    border: ReflectedValue,
    flex_direction: ReflectedValue,
    flex_wrap: ReflectedValue,
    flex_grow: f32,
    flex_shrink: f32,
    flex_basis: ReflectedValue,
    row_gap: ReflectedValue,
    column_gap: ReflectedValue,
    grid_auto_flow: ReflectedValue,
    grid_template_rows: ReflectedValue,
    grid_template_columns: ReflectedValue,
    grid_auto_rows: ReflectedValue,
    grid_auto_columns: ReflectedValue,
    grid_row: ReflectedValue,
    grid_column: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::text::Text",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct Text {
    sections: ReflectedValue,
    alignment: ReflectedValue,
    linebreak_behavior: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::text::Text2dBounds",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Text2dBounds {
    size: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::text::TextAlignment",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct TextAlignment;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::widget::TextFlags",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct TextFlags {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::text::TextSection",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct TextSection {
    value: String,
    style: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::text::TextStyle",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct TextStyle {
    font: ReflectedValue,
    font_size: f32,
    color: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::sprite::TextureAtlas",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct TextureAtlas {
    texture: ReflectedValue,
    size: ReflectedValue,
    textures: ReflectedValue,
    texture_handles: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::sprite::TextureAtlasSprite",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct TextureAtlasSprite {
    color: ReflectedValue,
    index: usize,
    flip_x: bool,
    flip_y: bool,
    custom_size: ReflectedValue,
    anchor: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::time::Time",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Time {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::time::Timer",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct Timer {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::time::TimerMode",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct TimerMode;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::core_pipeline::tonemapping::Tonemapping",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct Tonemapping;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::transform::components::Transform",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Transform {
    translation: ReflectedValue,
    rotation: ReflectedValue,
    scale: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::UVec2",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct UVec2 {
    x: u32,
    y: u32,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::UVec3",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct UVec3 {
    x: u32,
    y: u32,
    z: u32,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::UVec4",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct UVec4 {
    x: u32,
    y: u32,
    z: u32,
    w: u32,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone),
    remote = "bevy::ui::camera_config::UiCameraConfig",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct UiCameraConfig {
    show_ui: bool,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::UiImage",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct UiImage {
    texture: ReflectedValue,
    flip_x: bool,
    flip_y: bool,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::widget::UiImageSize",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct UiImageSize {}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::UiRect",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct UiRect {
    left: ReflectedValue,
    right: ReflectedValue,
    top: ReflectedValue,
    bottom: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::UiTextureAtlasImage",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct UiTextureAtlasImage {
    index: usize,
    flip_x: bool,
    flip_y: bool,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::Val",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct Val;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::animation::VariableCurve",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct VariableCurve {
    keyframe_timestamps: ReflectedValue,
    keyframes: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Vec2",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct Vec2 {
    x: f32,
    y: f32,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Vec3",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    ]
)]
pub struct Vec3 {
    x: f32,
    y: f32,
    z: f32,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Vec3A",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="MetaFunction", as_trait="std::cmp::PartialEq", composite="eq", metamethod=Eq, )]
    fn eq (&self, #[proxy] rhs : &Self,) -> bool;"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    ]
)]
pub struct Vec3A();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::math::Vec4",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="MetaFunction", as_trait="std::cmp::PartialEq", composite="eq", metamethod=Eq, )]
    fn eq (&self, #[proxy] rhs : &Self,) -> bool;"#,
    ]
)]
pub struct Vec4();
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::camera::Viewport",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Viewport {
    physical_position: ReflectedValue,
    physical_size: ReflectedValue,
    depth: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::view::Visibility",
    functions[r#"
    #[lua(kind="Method", as_trait="core::cmp::Eq", )]
    fn assert_receiver_is_total_eq (&self, );"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct Visibility;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::render::view::VisibleEntities",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct VisibleEntities {
    entities: ReflectedValue,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::wireframe::Wireframe",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct Wireframe;
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::pbr::wireframe::WireframeConfig",
    functions[r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Struct", )]
    fn field_len (&self, ) -> usize;"#,
    ]
)]
pub struct WireframeConfig {
    global: bool,
}
#[derive(LuaProxy)]
#[proxy(
    derive(clone, debug),
    remote = "bevy::ui::ZIndex",
    functions[r#"
    #[lua(kind="Function", as_trait="core::default::Default", output(proxy))]
    fn default () -> Self;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn field_len (&self, ) -> usize;"#,
    r#"
    #[lua(kind="Method", as_trait="bevy::reflect::Enum", )]
    fn variant_index (&self, ) -> usize;"#,
    ]
)]
pub struct ZIndex;
#[cfg(feature = "lua")]
crate::impl_tealr_generic!(pub (crate) struct T);
#[cfg(feature = "lua")]
#[derive(Default)]
pub(crate) struct BevyAPIGlobals;
#[cfg(feature = "lua")]
impl bevy_mod_scripting_lua::tealr::mlu::ExportInstances for BevyAPIGlobals {
    fn add_instances<'lua, T: bevy_mod_scripting_lua::tealr::mlu::InstanceCollector<'lua>>(
        self,
        instances: &mut T,
    ) -> bevy_mod_scripting_lua::tealr::mlu::mlua::Result<()> {
        instances.add_instance(
            "Affine2",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaAffine2>::new,
        )?;
        instances.add_instance(
            "Affine3A",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaAffine3A>::new,
        )?;
        instances.add_instance(
            "AlignContent",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaAlignContent>::new,
        )?;
        instances.add_instance(
            "AlignItems",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaAlignItems>::new,
        )?;
        instances.add_instance(
            "AlignSelf",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaAlignSelf>::new,
        )?;
        instances.add_instance(
            "AmbientLight",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaAmbientLight>::new,
        )?;
        instances.add_instance(
            "BVec2",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaBVec2>::new,
        )?;
        instances.add_instance(
            "BVec3",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaBVec3>::new,
        )?;
        instances.add_instance(
            "BVec3A",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaBVec3A>::new,
        )?;
        instances.add_instance(
            "BVec4",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaBVec4>::new,
        )?;
        instances.add_instance(
            "BVec4A",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaBVec4A>::new,
        )?;
        instances.add_instance(
            "BackgroundColor",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaBackgroundColor>::new,
        )?;
        instances.add_instance(
            "BloomSettings",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaBloomSettings>::new,
        )?;
        instances.add_instance(
            "BorderColor",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaBorderColor>::new,
        )?;
        instances.add_instance(
            "Camera",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaCamera>::new,
        )?;
        instances.add_instance(
            "Camera3d",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaCamera3d>::new,
        )?;
        instances.add_instance(
            "Camera3dDepthLoadOp",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaCamera3dDepthLoadOp>::new,
        )?;
        instances.add_instance(
            "CascadeShadowConfig",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaCascadeShadowConfig>::new,
        )?;
        instances.add_instance(
            "ClearColor",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaClearColor>::new,
        )?;
        instances.add_instance(
            "ClusterConfig",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaClusterConfig>::new,
        )?;
        instances.add_instance(
            "ClusterZConfig",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaClusterZConfig>::new,
        )?;
        instances.add_instance(
            "Color",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaColor>::new,
        )?;
        instances.add_instance(
            "ColorGrading",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaColorGrading>::new,
        )?;
        instances.add_instance(
            "ColorMaterial",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaColorMaterial>::new,
        )?;
        instances.add_instance(
            "ComputedVisibility",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaComputedVisibility>::new,
        )?;
        instances.add_instance(
            "ContentSize",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaContentSize>::new,
        )?;
        instances.add_instance(
            "ContrastAdaptiveSharpeningSettings",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<
                LuaContrastAdaptiveSharpeningSettings,
            >::new,
        )?;
        instances.add_instance(
            "DAffine2",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDAffine2>::new,
        )?;
        instances.add_instance(
            "DAffine3",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDAffine3>::new,
        )?;
        instances.add_instance(
            "DMat2",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDMat2>::new,
        )?;
        instances.add_instance(
            "DMat3",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDMat3>::new,
        )?;
        instances.add_instance(
            "DMat4",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDMat4>::new,
        )?;
        instances.add_instance(
            "DQuat",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDQuat>::new,
        )?;
        instances.add_instance(
            "DVec2",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDVec2>::new,
        )?;
        instances.add_instance(
            "DVec3",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDVec3>::new,
        )?;
        instances.add_instance(
            "DVec4",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDVec4>::new,
        )?;
        instances.add_instance(
            "Direction",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDirection>::new,
        )?;
        instances.add_instance(
            "DirectionalLight",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDirectionalLight>::new,
        )?;
        instances.add_instance(
            "DirectionalLightShadowMap",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDirectionalLightShadowMap>::new,
        )?;
        instances.add_instance(
            "Display",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaDisplay>::new,
        )?;
        instances.add_instance(
            "Entity",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaEntity>::new,
        )?;
        instances.add_instance(
            "EulerRot",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaEulerRot>::new,
        )?;
        instances.add_instance(
            "FlexDirection",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaFlexDirection>::new,
        )?;
        instances.add_instance(
            "FlexWrap",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaFlexWrap>::new,
        )?;
        instances.add_instance(
            "FocusPolicy",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaFocusPolicy>::new,
        )?;
        instances.add_instance(
            "FogSettings",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaFogSettings>::new,
        )?;
        instances.add_instance(
            "Fxaa",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaFxaa>::new,
        )?;
        instances.add_instance(
            "GlobalTransform",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaGlobalTransform>::new,
        )?;
        instances.add_instance(
            "GridAutoFlow",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaGridAutoFlow>::new,
        )?;
        instances.add_instance(
            "GridPlacement",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaGridPlacement>::new,
        )?;
        instances.add_instance(
            "GridTrack",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaGridTrack>::new,
        )?;
        instances.add_instance(
            "GridTrackRepetition",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaGridTrackRepetition>::new,
        )?;
        instances.add_instance(
            "HandleId",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaHandleId>::new,
        )?;
        instances.add_instance(
            "IVec2",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaIVec2>::new,
        )?;
        instances.add_instance(
            "IVec3",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaIVec3>::new,
        )?;
        instances.add_instance(
            "IVec4",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaIVec4>::new,
        )?;
        instances.add_instance(
            "Image",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaImage>::new,
        )?;
        instances.add_instance(
            "Interaction",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaInteraction>::new,
        )?;
        instances.add_instance(
            "JustifyContent",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaJustifyContent>::new,
        )?;
        instances.add_instance(
            "JustifyItems",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaJustifyItems>::new,
        )?;
        instances.add_instance(
            "JustifySelf",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaJustifySelf>::new,
        )?;
        instances.add_instance(
            "Mat2",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaMat2>::new,
        )?;
        instances.add_instance(
            "Mat3",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaMat3>::new,
        )?;
        instances.add_instance(
            "Mat3A",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaMat3A>::new,
        )?;
        instances.add_instance(
            "Mat4",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaMat4>::new,
        )?;
        instances.add_instance(
            "Name",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaName>::new,
        )?;
        instances.add_instance(
            "Node",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaNode>::new,
        )?;
        instances.add_instance(
            "OrthographicProjection",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaOrthographicProjection>::new,
        )?;
        instances.add_instance(
            "Overflow",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaOverflow>::new,
        )?;
        instances.add_instance(
            "OverflowAxis",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaOverflowAxis>::new,
        )?;
        instances.add_instance(
            "PathBuf",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaPathBuf>::new,
        )?;
        instances.add_instance(
            "PerspectiveProjection",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaPerspectiveProjection>::new,
        )?;
        instances.add_instance(
            "PointLight",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaPointLight>::new,
        )?;
        instances.add_instance(
            "PointLightShadowMap",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaPointLightShadowMap>::new,
        )?;
        instances.add_instance(
            "PositionType",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaPositionType>::new,
        )?;
        instances.add_instance(
            "Projection",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaProjection>::new,
        )?;
        instances.add_instance(
            "Quat",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaQuat>::new,
        )?;
        instances.add_instance(
            "RenderLayers",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaRenderLayers>::new,
        )?;
        instances.add_instance(
            "RenderTarget",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaRenderTarget>::new,
        )?;
        instances.add_instance(
            "SpotLight",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaSpotLight>::new,
        )?;
        instances.add_instance(
            "StandardMaterial",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaStandardMaterial>::new,
        )?;
        instances.add_instance(
            "Style",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaStyle>::new,
        )?;
        instances.add_instance(
            "Text",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaText>::new,
        )?;
        instances.add_instance(
            "Text2dBounds",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaText2dBounds>::new,
        )?;
        instances.add_instance(
            "TextFlags",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaTextFlags>::new,
        )?;
        instances.add_instance(
            "TextStyle",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaTextStyle>::new,
        )?;
        instances.add_instance(
            "TextureAtlasSprite",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaTextureAtlasSprite>::new,
        )?;
        instances.add_instance(
            "Time",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaTime>::new,
        )?;
        instances.add_instance(
            "Transform",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaTransform>::new,
        )?;
        instances.add_instance(
            "UVec2",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaUVec2>::new,
        )?;
        instances.add_instance(
            "UVec3",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaUVec3>::new,
        )?;
        instances.add_instance(
            "UVec4",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaUVec4>::new,
        )?;
        instances.add_instance(
            "UiCameraConfig",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaUiCameraConfig>::new,
        )?;
        instances.add_instance(
            "UiRect",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaUiRect>::new,
        )?;
        instances.add_instance(
            "Val",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaVal>::new,
        )?;
        instances.add_instance(
            "Vec2",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaVec2>::new,
        )?;
        instances.add_instance(
            "Vec3",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaVec3>::new,
        )?;
        instances.add_instance(
            "Vec3A",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaVec3A>::new,
        )?;
        instances.add_instance(
            "Vec4",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaVec4>::new,
        )?;
        instances.add_instance(
            "Viewport",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaViewport>::new,
        )?;
        instances.add_instance(
            "ZIndex",
            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy::<LuaZIndex>::new,
        )?;
        instances.add_instance(
            "world",
            crate::lua::util::DummyTypeName::<crate::lua::bevy::LuaWorld>::new,
        )?;
        instances.add_instance(
            "script",
            crate::lua::util::DummyTypeName::<crate::lua::bevy::LuaScriptData>::new,
        )?;
        instances.add_instance(
            "entity",
            crate::lua::util::DummyTypeName::<crate::lua::bevy::LuaEntity>::new,
        )?;
        Ok(())
    }
}
#[cfg(feature = "lua")]
pub struct LuaBevyAPIProvider;
#[cfg(feature = "lua")]
impl APIProvider for LuaBevyAPIProvider {
    type APITarget = Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type ScriptContext = Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type DocTarget = LuaDocFragment;
    fn attach_api(&mut self, ctx: &mut Self::APITarget) -> Result<(), ScriptError> {
        let ctx = ctx
            .get_mut()
            .expect("Unable to acquire lock on Lua context");
        bevy_mod_scripting_lua::tealr::mlu::set_global_env(BevyAPIGlobals, ctx)
            .map_err(|e| ScriptError::Other(e.to_string()))
    }
    fn get_doc_fragment(&self) -> Option<Self::DocTarget> {
        Some(LuaDocFragment::new("BevyAPI", |tw| {
            tw.document_global_instance::<BevyAPIGlobals>()
                        .expect("Something went wrong documenting globals")
                        .process_type::<LuaAabb>()
                        .process_type::<LuaAffine2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaAffine2>,
                        >()
                        .process_type::<LuaAffine3A>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaAffine3A,
                            >,
                        >()
                        .process_type::<LuaAlignContent>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaAlignContent,
                            >,
                        >()
                        .process_type::<LuaAlignItems>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaAlignItems,
                            >,
                        >()
                        .process_type::<LuaAlignSelf>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaAlignSelf,
                            >,
                        >()
                        .process_type::<LuaAlphaMode>()
                        .process_type::<LuaAmbientLight>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaAmbientLight,
                            >,
                        >()
                        .process_type::<LuaAnchor>()
                        .process_type::<LuaAnimationClip>()
                        .process_type::<LuaAnimationPlayer>()
                        .process_type::<LuaAssetPathId>()
                        .process_type::<LuaBVec2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaBVec2>,
                        >()
                        .process_type::<LuaBVec3>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaBVec3>,
                        >()
                        .process_type::<LuaBVec3A>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaBVec3A>,
                        >()
                        .process_type::<LuaBVec4>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaBVec4>,
                        >()
                        .process_type::<LuaBVec4A>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaBVec4A>,
                        >()
                        .process_type::<LuaBackgroundColor>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaBackgroundColor,
                            >,
                        >()
                        .process_type::<LuaBloomCompositeMode>()
                        .process_type::<LuaBloomPrefilterSettings>()
                        .process_type::<LuaBloomSettings>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaBloomSettings,
                            >,
                        >()
                        .process_type::<LuaBorderColor>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaBorderColor,
                            >,
                        >()
                        .process_type::<LuaBreakLineOn>()
                        .process_type::<LuaButton>()
                        .process_type::<LuaCalculatedClip>()
                        .process_type::<LuaCamera>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaCamera>,
                        >()
                        .process_type::<LuaCamera2d>()
                        .process_type::<LuaCamera3d>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaCamera3d,
                            >,
                        >()
                        .process_type::<LuaCamera3dDepthLoadOp>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaCamera3dDepthLoadOp,
                            >,
                        >()
                        .process_type::<LuaCamera3dDepthTextureUsage>()
                        .process_type::<LuaCameraRenderGraph>()
                        .process_type::<LuaCascade>()
                        .process_type::<LuaCascadeShadowConfig>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaCascadeShadowConfig,
                            >,
                        >()
                        .process_type::<LuaCascades>()
                        .process_type::<LuaCascadesFrusta>()
                        .process_type::<LuaCascadesVisibleEntities>()
                        .process_type::<LuaChildren>()
                        .process_type::<LuaClearColor>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaClearColor,
                            >,
                        >()
                        .process_type::<LuaClearColorConfig>()
                        .process_type::<LuaClusterConfig>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaClusterConfig,
                            >,
                        >()
                        .process_type::<LuaClusterFarZMode>()
                        .process_type::<LuaClusterZConfig>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaClusterZConfig,
                            >,
                        >()
                        .process_type::<LuaColor>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaColor>,
                        >()
                        .process_type::<LuaColorGrading>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaColorGrading,
                            >,
                        >()
                        .process_type::<LuaColorMaterial>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaColorMaterial,
                            >,
                        >()
                        .process_type::<LuaComputedVisibility>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaComputedVisibility,
                            >,
                        >()
                        .process_type::<LuaContentSize>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaContentSize,
                            >,
                        >()
                        .process_type::<LuaContrastAdaptiveSharpeningSettings>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaContrastAdaptiveSharpeningSettings,
                            >,
                        >()
                        .process_type::<LuaCubemapFrusta>()
                        .process_type::<LuaCubemapVisibleEntities>()
                        .process_type::<LuaDAffine2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaDAffine2,
                            >,
                        >()
                        .process_type::<LuaDAffine3>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaDAffine3,
                            >,
                        >()
                        .process_type::<LuaDMat2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaDMat2>,
                        >()
                        .process_type::<LuaDMat3>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaDMat3>,
                        >()
                        .process_type::<LuaDMat4>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaDMat4>,
                        >()
                        .process_type::<LuaDQuat>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaDQuat>,
                        >()
                        .process_type::<LuaDVec2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaDVec2>,
                        >()
                        .process_type::<LuaDVec3>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaDVec3>,
                        >()
                        .process_type::<LuaDVec4>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaDVec4>,
                        >()
                        .process_type::<LuaDebandDither>()
                        .process_type::<LuaDenoiseCAS>()
                        .process_type::<LuaDepthPrepass>()
                        .process_type::<LuaDirection>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaDirection,
                            >,
                        >()
                        .process_type::<LuaDirectionalLight>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaDirectionalLight,
                            >,
                        >()
                        .process_type::<LuaDirectionalLightShadowMap>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaDirectionalLightShadowMap,
                            >,
                        >()
                        .process_type::<LuaDisplay>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaDisplay>,
                        >()
                        .process_type::<LuaDuration>()
                        .process_type::<LuaEntity>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaEntity>,
                        >()
                        .process_type::<LuaEntityPath>()
                        .process_type::<LuaEnvironmentMapLight>()
                        .process_type::<LuaEulerRot>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaEulerRot,
                            >,
                        >()
                        .process_type::<LuaFlexDirection>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaFlexDirection,
                            >,
                        >()
                        .process_type::<LuaFlexWrap>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaFlexWrap,
                            >,
                        >()
                        .process_type::<LuaFocusPolicy>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaFocusPolicy,
                            >,
                        >()
                        .process_type::<LuaFogFalloff>()
                        .process_type::<LuaFogSettings>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaFogSettings,
                            >,
                        >()
                        .process_type::<LuaFrustum>()
                        .process_type::<LuaFxaa>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaFxaa>,
                        >()
                        .process_type::<LuaGlobalTransform>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaGlobalTransform,
                            >,
                        >()
                        .process_type::<LuaGlobalsUniform>()
                        .process_type::<LuaGltfExtras>()
                        .process_type::<LuaGridAutoFlow>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaGridAutoFlow,
                            >,
                        >()
                        .process_type::<LuaGridPlacement>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaGridPlacement,
                            >,
                        >()
                        .process_type::<LuaGridTrack>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaGridTrack,
                            >,
                        >()
                        .process_type::<LuaGridTrackRepetition>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaGridTrackRepetition,
                            >,
                        >()
                        .process_type::<LuaHandleId>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaHandleId,
                            >,
                        >()
                        .process_type::<LuaIVec2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaIVec2>,
                        >()
                        .process_type::<LuaIVec3>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaIVec3>,
                        >()
                        .process_type::<LuaIVec4>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaIVec4>,
                        >()
                        .process_type::<LuaImage>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaImage>,
                        >()
                        .process_type::<LuaInstant>()
                        .process_type::<LuaInteraction>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaInteraction,
                            >,
                        >()
                        .process_type::<LuaJustifyContent>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaJustifyContent,
                            >,
                        >()
                        .process_type::<LuaJustifyItems>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaJustifyItems,
                            >,
                        >()
                        .process_type::<LuaJustifySelf>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaJustifySelf,
                            >,
                        >()
                        .process_type::<LuaKeyframes>()
                        .process_type::<LuaLabel>()
                        .process_type::<LuaLabelId>()
                        .process_type::<LuaManualTextureViewHandle>()
                        .process_type::<LuaMat2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaMat2>,
                        >()
                        .process_type::<LuaMat3>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaMat3>,
                        >()
                        .process_type::<LuaMat3A>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaMat3A>,
                        >()
                        .process_type::<LuaMat4>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaMat4>,
                        >()
                        .process_type::<LuaMaxTrackSizingFunction>()
                        .process_type::<LuaMesh2dHandle>()
                        .process_type::<LuaMeshMorphWeights>()
                        .process_type::<LuaMinTrackSizingFunction>()
                        .process_type::<LuaMorphWeights>()
                        .process_type::<LuaMotionVectorPrepass>()
                        .process_type::<LuaMsaa>()
                        .process_type::<LuaName>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaName>,
                        >()
                        .process_type::<LuaNoFrustumCulling>()
                        .process_type::<LuaNode>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaNode>,
                        >()
                        .process_type::<LuaNonZeroI128>()
                        .process_type::<LuaNonZeroI16>()
                        .process_type::<LuaNonZeroI32>()
                        .process_type::<LuaNonZeroI64>()
                        .process_type::<LuaNonZeroI8>()
                        .process_type::<LuaNonZeroIsize>()
                        .process_type::<LuaNonZeroU128>()
                        .process_type::<LuaNonZeroU16>()
                        .process_type::<LuaNonZeroU32>()
                        .process_type::<LuaNonZeroU64>()
                        .process_type::<LuaNonZeroU8>()
                        .process_type::<LuaNonZeroUsize>()
                        .process_type::<LuaNormalPrepass>()
                        .process_type::<LuaNormalizedRenderTarget>()
                        .process_type::<LuaNotShadowCaster>()
                        .process_type::<LuaNotShadowReceiver>()
                        .process_type::<LuaOrthographicProjection>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaOrthographicProjection,
                            >,
                        >()
                        .process_type::<LuaOsString>()
                        .process_type::<LuaOverflow>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaOverflow,
                            >,
                        >()
                        .process_type::<LuaOverflowAxis>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaOverflowAxis,
                            >,
                        >()
                        .process_type::<LuaParallaxMappingMethod>()
                        .process_type::<LuaParent>()
                        .process_type::<LuaPath>()
                        .process_type::<LuaPathBuf>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaPathBuf>,
                        >()
                        .process_type::<LuaPerspectiveProjection>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaPerspectiveProjection,
                            >,
                        >()
                        .process_type::<LuaPointLight>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaPointLight,
                            >,
                        >()
                        .process_type::<LuaPointLightShadowMap>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaPointLightShadowMap,
                            >,
                        >()
                        .process_type::<LuaPositionType>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaPositionType,
                            >,
                        >()
                        .process_type::<LuaProjection>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaProjection,
                            >,
                        >()
                        .process_type::<LuaQuat>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaQuat>,
                        >()
                        .process_type::<LuaRangeFull>()
                        .process_type::<LuaRect>()
                        .process_type::<LuaRelativeCursorPosition>()
                        .process_type::<LuaRenderLayers>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaRenderLayers,
                            >,
                        >()
                        .process_type::<LuaRenderTarget>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaRenderTarget,
                            >,
                        >()
                        .process_type::<LuaRepeatedGridTrack>()
                        .process_type::<LuaScalingMode>()
                        .process_type::<LuaScreenSpaceAmbientOcclusionQualityLevel>()
                        .process_type::<LuaScreenSpaceAmbientOcclusionSettings>()
                        .process_type::<LuaSensitivity>()
                        .process_type::<LuaSkinnedMesh>()
                        .process_type::<LuaSourcePathId>()
                        .process_type::<LuaSpotLight>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaSpotLight,
                            >,
                        >()
                        .process_type::<LuaSprite>()
                        .process_type::<LuaStandardMaterial>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaStandardMaterial,
                            >,
                        >()
                        .process_type::<LuaStopwatch>()
                        .process_type::<LuaStyle>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaStyle>,
                        >()
                        .process_type::<LuaText>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaText>,
                        >()
                        .process_type::<LuaText2dBounds>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaText2dBounds,
                            >,
                        >()
                        .process_type::<LuaTextAlignment>()
                        .process_type::<LuaTextFlags>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaTextFlags,
                            >,
                        >()
                        .process_type::<LuaTextSection>()
                        .process_type::<LuaTextStyle>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaTextStyle,
                            >,
                        >()
                        .process_type::<LuaTextureAtlas>()
                        .process_type::<LuaTextureAtlasSprite>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaTextureAtlasSprite,
                            >,
                        >()
                        .process_type::<LuaTime>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaTime>,
                        >()
                        .process_type::<LuaTimer>()
                        .process_type::<LuaTimerMode>()
                        .process_type::<LuaTonemapping>()
                        .process_type::<LuaTransform>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaTransform,
                            >,
                        >()
                        .process_type::<LuaUVec2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaUVec2>,
                        >()
                        .process_type::<LuaUVec3>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaUVec3>,
                        >()
                        .process_type::<LuaUVec4>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaUVec4>,
                        >()
                        .process_type::<LuaUiCameraConfig>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaUiCameraConfig,
                            >,
                        >()
                        .process_type::<LuaUiImage>()
                        .process_type::<LuaUiImageSize>()
                        .process_type::<LuaUiRect>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaUiRect>,
                        >()
                        .process_type::<LuaUiTextureAtlasImage>()
                        .process_type::<LuaVal>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaVal>,
                        >()
                        .process_type::<LuaVariableCurve>()
                        .process_type::<LuaVec2>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaVec2>,
                        >()
                        .process_type::<LuaVec3>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaVec3>,
                        >()
                        .process_type::<LuaVec3A>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaVec3A>,
                        >()
                        .process_type::<LuaVec4>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaVec4>,
                        >()
                        .process_type::<LuaViewport>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                LuaViewport,
                            >,
                        >()
                        .process_type::<LuaVisibility>()
                        .process_type::<LuaVisibleEntities>()
                        .process_type::<LuaWireframe>()
                        .process_type::<LuaWireframeConfig>()
                        .process_type::<LuaZIndex>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<LuaZIndex>,
                        >()
                        .process_type::<ReflectedValue>()
                        .process_type::<crate::lua::bevy::LuaWorld>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                crate::lua::bevy::LuaWorld,
                            >,
                        >()
                        .process_type::<crate::lua::bevy::LuaScriptData>()
                        .process_type::<
                            bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<
                                crate::lua::bevy::LuaScriptData,
                            >,
                        >()
                        .process_type::<crate::lua::bevy::LuaTypeRegistration>()
                        .process_type::<crate::lua::std::LuaVec<T>>()
        }))
    }
    fn setup_script(
        &mut self,
        script_data: &ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), ScriptError> {
        let ctx = ctx.get_mut().expect("Could not get context");
        let globals = ctx.globals();
        globals
            .set(
                "entity",
                crate::lua::bevy::LuaEntity::new(script_data.entity),
            )
            .map_err(ScriptError::new_other)?;
        globals
            .set::<_, crate::lua::bevy::LuaScriptData>("script", script_data.into())
            .map_err(ScriptError::new_other)?;
        Ok(())
    }
    fn setup_script_runtime(
        &mut self,
        world_ptr: bevy_mod_scripting_core::world::WorldPointer,
        _script_data: &ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), ScriptError> {
        let ctx = ctx.get_mut().expect("Could not get context");
        let globals = ctx.globals();
        globals
            .set("world", crate::lua::bevy::LuaWorld::new(world_ptr))
            .map_err(ScriptError::new_other)
    }
    fn register_with_app(&self, app: &mut App) {
        app.register_foreign_lua_type::<bevy::render::primitives::Aabb>();
        app.register_foreign_lua_type::<bevy::math::Affine2>();
        app.register_foreign_lua_type::<bevy::math::Affine3A>();
        app.register_foreign_lua_type::<bevy::ui::AlignContent>();
        app.register_foreign_lua_type::<bevy::ui::AlignItems>();
        app.register_foreign_lua_type::<bevy::ui::AlignSelf>();
        app.register_foreign_lua_type::<bevy::pbr::AlphaMode>();
        app.register_foreign_lua_type::<bevy::pbr::AmbientLight>();
        app.register_foreign_lua_type::<bevy::sprite::Anchor>();
        app.register_foreign_lua_type::<bevy::animation::AnimationClip>();
        app.register_foreign_lua_type::<bevy::animation::AnimationPlayer>();
        app.register_foreign_lua_type::<bevy::asset::AssetPathId>();
        app.register_foreign_lua_type::<bevy::math::BVec2>();
        app.register_foreign_lua_type::<bevy::math::BVec3>();
        app.register_foreign_lua_type::<bevy::math::BVec3A>();
        app.register_foreign_lua_type::<bevy::math::BVec4>();
        app.register_foreign_lua_type::<bevy::math::BVec4A>();
        app.register_foreign_lua_type::<bevy::ui::BackgroundColor>();
        app.register_foreign_lua_type::<bevy::core_pipeline::bloom::BloomCompositeMode>();
        app.register_foreign_lua_type::<bevy::core_pipeline::bloom::BloomPrefilterSettings>();
        app.register_foreign_lua_type::<bevy::core_pipeline::bloom::BloomSettings>();
        app.register_foreign_lua_type::<bevy::ui::BorderColor>();
        app.register_foreign_lua_type::<bevy::text::BreakLineOn>();
        app.register_foreign_lua_type::<bevy::ui::widget::Button>();
        app.register_foreign_lua_type::<bevy::ui::CalculatedClip>();
        app.register_foreign_lua_type::<bevy::render::camera::Camera>();
        app.register_foreign_lua_type::<bevy::core_pipeline::core_2d::Camera2d>();
        app.register_foreign_lua_type::<bevy::core_pipeline::core_3d::Camera3d>();
        app.register_foreign_lua_type::<bevy::core_pipeline::core_3d::Camera3dDepthLoadOp>();
        app.register_foreign_lua_type::<bevy::core_pipeline::core_3d::Camera3dDepthTextureUsage>();
        app.register_foreign_lua_type::<bevy::render::camera::CameraRenderGraph>();
        app.register_foreign_lua_type::<bevy::pbr::Cascade>();
        app.register_foreign_lua_type::<bevy::pbr::CascadeShadowConfig>();
        app.register_foreign_lua_type::<bevy::pbr::Cascades>();
        app.register_foreign_lua_type::<bevy::render::primitives::CascadesFrusta>();
        app.register_foreign_lua_type::<bevy::pbr::CascadesVisibleEntities>();
        app.register_foreign_lua_type::<bevy::hierarchy::Children>();
        app.register_foreign_lua_type::<bevy::core_pipeline::clear_color::ClearColor>();
        app.register_foreign_lua_type::<bevy::core_pipeline::clear_color::ClearColorConfig>();
        app.register_foreign_lua_type::<bevy::pbr::ClusterConfig>();
        app.register_foreign_lua_type::<bevy::pbr::ClusterFarZMode>();
        app.register_foreign_lua_type::<bevy::pbr::ClusterZConfig>();
        app.register_foreign_lua_type::<bevy::render::color::Color>();
        app.register_foreign_lua_type::<bevy::render::view::ColorGrading>();
        app.register_foreign_lua_type::<bevy::sprite::ColorMaterial>();
        app.register_foreign_lua_type::<bevy::render::view::ComputedVisibility>();
        app.register_foreign_lua_type::<bevy::ui::ContentSize>();
        app.register_foreign_lua_type::<
                bevy::core_pipeline::contrast_adaptive_sharpening::ContrastAdaptiveSharpeningSettings,
            >();
        app.register_foreign_lua_type::<bevy::render::primitives::CubemapFrusta>();
        app.register_foreign_lua_type::<bevy::pbr::CubemapVisibleEntities>();
        app.register_foreign_lua_type::<bevy::math::DAffine2>();
        app.register_foreign_lua_type::<bevy::math::DAffine3>();
        app.register_foreign_lua_type::<bevy::math::DMat2>();
        app.register_foreign_lua_type::<bevy::math::DMat3>();
        app.register_foreign_lua_type::<bevy::math::DMat4>();
        app.register_foreign_lua_type::<bevy::math::DQuat>();
        app.register_foreign_lua_type::<bevy::math::DVec2>();
        app.register_foreign_lua_type::<bevy::math::DVec3>();
        app.register_foreign_lua_type::<bevy::math::DVec4>();
        app.register_foreign_lua_type::<bevy::core_pipeline::tonemapping::DebandDither>();
        app.register_foreign_lua_type::<
                bevy::core_pipeline::contrast_adaptive_sharpening::DenoiseCAS,
            >();
        app.register_foreign_lua_type::<bevy::core_pipeline::prepass::DepthPrepass>();
        app.register_foreign_lua_type::<bevy::ui::Direction>();
        app.register_foreign_lua_type::<bevy::pbr::DirectionalLight>();
        app.register_foreign_lua_type::<bevy::pbr::DirectionalLightShadowMap>();
        app.register_foreign_lua_type::<bevy::ui::Display>();
        app.register_foreign_lua_type::<core::time::Duration>();
        app.register_foreign_lua_type::<bevy::ecs::entity::Entity>();
        app.register_foreign_lua_type::<bevy::animation::EntityPath>();
        app.register_foreign_lua_type::<bevy::pbr::EnvironmentMapLight>();
        app.register_foreign_lua_type::<bevy::math::EulerRot>();
        app.register_foreign_lua_type::<bevy::ui::FlexDirection>();
        app.register_foreign_lua_type::<bevy::ui::FlexWrap>();
        app.register_foreign_lua_type::<bevy::ui::FocusPolicy>();
        app.register_foreign_lua_type::<bevy::pbr::FogFalloff>();
        app.register_foreign_lua_type::<bevy::pbr::FogSettings>();
        app.register_foreign_lua_type::<bevy::render::primitives::Frustum>();
        app.register_foreign_lua_type::<bevy::core_pipeline::fxaa::Fxaa>();
        app.register_foreign_lua_type::<bevy::transform::components::GlobalTransform>();
        app.register_foreign_lua_type::<bevy::render::globals::GlobalsUniform>();
        app.register_foreign_lua_type::<bevy::gltf::GltfExtras>();
        app.register_foreign_lua_type::<bevy::ui::GridAutoFlow>();
        app.register_foreign_lua_type::<bevy::ui::GridPlacement>();
        app.register_foreign_lua_type::<bevy::ui::GridTrack>();
        app.register_foreign_lua_type::<bevy::ui::GridTrackRepetition>();
        app.register_foreign_lua_type::<bevy::asset::HandleId>();
        app.register_foreign_lua_type::<bevy::math::IVec2>();
        app.register_foreign_lua_type::<bevy::math::IVec3>();
        app.register_foreign_lua_type::<bevy::math::IVec4>();
        app.register_foreign_lua_type::<bevy::render::texture::Image>();
        app.register_foreign_lua_type::<std::time::Instant>();
        app.register_foreign_lua_type::<bevy::ui::Interaction>();
        app.register_foreign_lua_type::<bevy::ui::JustifyContent>();
        app.register_foreign_lua_type::<bevy::ui::JustifyItems>();
        app.register_foreign_lua_type::<bevy::ui::JustifySelf>();
        app.register_foreign_lua_type::<bevy::animation::Keyframes>();
        app.register_foreign_lua_type::<bevy::ui::widget::Label>();
        app.register_foreign_lua_type::<bevy::asset::LabelId>();
        app.register_foreign_lua_type::<bevy::render::camera::ManualTextureViewHandle>();
        app.register_foreign_lua_type::<bevy::math::Mat2>();
        app.register_foreign_lua_type::<bevy::math::Mat3>();
        app.register_foreign_lua_type::<bevy::math::Mat3A>();
        app.register_foreign_lua_type::<bevy::math::Mat4>();
        app.register_foreign_lua_type::<bevy::ui::MaxTrackSizingFunction>();
        app.register_foreign_lua_type::<bevy::sprite::Mesh2dHandle>();
        app.register_foreign_lua_type::<bevy::render::mesh::morph::MeshMorphWeights>();
        app.register_foreign_lua_type::<bevy::ui::MinTrackSizingFunction>();
        app.register_foreign_lua_type::<bevy::render::prelude::MorphWeights>();
        app.register_foreign_lua_type::<bevy::core_pipeline::prepass::MotionVectorPrepass>();
        app.register_foreign_lua_type::<bevy::render::view::Msaa>();
        app.register_foreign_lua_type::<bevy::core::Name>();
        app.register_foreign_lua_type::<bevy::render::view::NoFrustumCulling>();
        app.register_foreign_lua_type::<bevy::ui::Node>();
        app.register_foreign_lua_type::<core::num::NonZeroI128>();
        app.register_foreign_lua_type::<core::num::NonZeroI16>();
        app.register_foreign_lua_type::<core::num::NonZeroI32>();
        app.register_foreign_lua_type::<core::num::NonZeroI64>();
        app.register_foreign_lua_type::<core::num::NonZeroI8>();
        app.register_foreign_lua_type::<core::num::NonZeroIsize>();
        app.register_foreign_lua_type::<core::num::NonZeroU128>();
        app.register_foreign_lua_type::<core::num::NonZeroU16>();
        app.register_foreign_lua_type::<core::num::NonZeroU32>();
        app.register_foreign_lua_type::<core::num::NonZeroU64>();
        app.register_foreign_lua_type::<core::num::NonZeroU8>();
        app.register_foreign_lua_type::<core::num::NonZeroUsize>();
        app.register_foreign_lua_type::<bevy::core_pipeline::prepass::NormalPrepass>();
        app.register_foreign_lua_type::<bevy::render::camera::NormalizedRenderTarget>();
        app.register_foreign_lua_type::<bevy::pbr::NotShadowCaster>();
        app.register_foreign_lua_type::<bevy::pbr::NotShadowReceiver>();
        app.register_foreign_lua_type::<bevy::render::camera::OrthographicProjection>();
        app.register_foreign_lua_type::<std::ffi::OsString>();
        app.register_foreign_lua_type::<bevy::ui::Overflow>();
        app.register_foreign_lua_type::<bevy::ui::OverflowAxis>();
        app.register_foreign_lua_type::<bevy::pbr::ParallaxMappingMethod>();
        app.register_foreign_lua_type::<bevy::hierarchy::Parent>();
        app.register_foreign_lua_type::<std::path::Path>();
        app.register_foreign_lua_type::<std::path::PathBuf>();
        app.register_foreign_lua_type::<bevy::render::camera::PerspectiveProjection>();
        app.register_foreign_lua_type::<bevy::pbr::PointLight>();
        app.register_foreign_lua_type::<bevy::pbr::PointLightShadowMap>();
        app.register_foreign_lua_type::<bevy::ui::PositionType>();
        app.register_foreign_lua_type::<bevy::render::camera::Projection>();
        app.register_foreign_lua_type::<bevy::math::Quat>();
        app.register_foreign_lua_type::<core::ops::RangeFull>();
        app.register_foreign_lua_type::<bevy::math::Rect>();
        app.register_foreign_lua_type::<bevy::ui::RelativeCursorPosition>();
        app.register_foreign_lua_type::<bevy::render::view::RenderLayers>();
        app.register_foreign_lua_type::<bevy::render::camera::RenderTarget>();
        app.register_foreign_lua_type::<bevy::ui::RepeatedGridTrack>();
        app.register_foreign_lua_type::<bevy::render::camera::ScalingMode>();
        app.register_foreign_lua_type::<bevy::pbr::ScreenSpaceAmbientOcclusionQualityLevel>();
        app.register_foreign_lua_type::<bevy::pbr::ScreenSpaceAmbientOcclusionSettings>();
        app.register_foreign_lua_type::<bevy::core_pipeline::fxaa::Sensitivity>();
        app.register_foreign_lua_type::<bevy::render::mesh::skinning::SkinnedMesh>();
        app.register_foreign_lua_type::<bevy::asset::SourcePathId>();
        app.register_foreign_lua_type::<bevy::pbr::SpotLight>();
        app.register_foreign_lua_type::<bevy::sprite::Sprite>();
        app.register_foreign_lua_type::<bevy::pbr::StandardMaterial>();
        app.register_foreign_lua_type::<bevy::time::Stopwatch>();
        app.register_foreign_lua_type::<bevy::ui::Style>();
        app.register_foreign_lua_type::<bevy::text::Text>();
        app.register_foreign_lua_type::<bevy::text::Text2dBounds>();
        app.register_foreign_lua_type::<bevy::text::TextAlignment>();
        app.register_foreign_lua_type::<bevy::ui::widget::TextFlags>();
        app.register_foreign_lua_type::<bevy::text::TextSection>();
        app.register_foreign_lua_type::<bevy::text::TextStyle>();
        app.register_foreign_lua_type::<bevy::sprite::TextureAtlas>();
        app.register_foreign_lua_type::<bevy::sprite::TextureAtlasSprite>();
        app.register_foreign_lua_type::<bevy::time::Time>();
        app.register_foreign_lua_type::<bevy::time::Timer>();
        app.register_foreign_lua_type::<bevy::time::TimerMode>();
        app.register_foreign_lua_type::<bevy::core_pipeline::tonemapping::Tonemapping>();
        app.register_foreign_lua_type::<bevy::transform::components::Transform>();
        app.register_foreign_lua_type::<bevy::math::UVec2>();
        app.register_foreign_lua_type::<bevy::math::UVec3>();
        app.register_foreign_lua_type::<bevy::math::UVec4>();
        app.register_foreign_lua_type::<bevy::ui::camera_config::UiCameraConfig>();
        app.register_foreign_lua_type::<bevy::ui::UiImage>();
        app.register_foreign_lua_type::<bevy::ui::widget::UiImageSize>();
        app.register_foreign_lua_type::<bevy::ui::UiRect>();
        app.register_foreign_lua_type::<bevy::ui::UiTextureAtlasImage>();
        app.register_foreign_lua_type::<bevy::ui::Val>();
        app.register_foreign_lua_type::<bevy::animation::VariableCurve>();
        app.register_foreign_lua_type::<bevy::math::Vec2>();
        app.register_foreign_lua_type::<bevy::math::Vec3>();
        app.register_foreign_lua_type::<bevy::math::Vec3A>();
        app.register_foreign_lua_type::<bevy::math::Vec4>();
        app.register_foreign_lua_type::<bevy::render::camera::Viewport>();
        app.register_foreign_lua_type::<bevy::render::view::Visibility>();
        app.register_foreign_lua_type::<bevy::render::view::VisibleEntities>();
        app.register_foreign_lua_type::<bevy::pbr::wireframe::Wireframe>();
        app.register_foreign_lua_type::<bevy::pbr::wireframe::WireframeConfig>();
        app.register_foreign_lua_type::<bevy::ui::ZIndex>();
        app.register_foreign_lua_type::<bevy::prelude::Entity>();
        app.register_foreign_lua_type::<usize>();
        app.register_foreign_lua_type::<isize>();
        app.register_foreign_lua_type::<f32>();
        app.register_foreign_lua_type::<f64>();
        app.register_foreign_lua_type::<u128>();
        app.register_foreign_lua_type::<u64>();
        app.register_foreign_lua_type::<u32>();
        app.register_foreign_lua_type::<u16>();
        app.register_foreign_lua_type::<u8>();
        app.register_foreign_lua_type::<i128>();
        app.register_foreign_lua_type::<i64>();
        app.register_foreign_lua_type::<i32>();
        app.register_foreign_lua_type::<i16>();
        app.register_foreign_lua_type::<i8>();
        app.register_foreign_lua_type::<String>();
        app.register_foreign_lua_type::<bool>();
    }
}
