// @generated by cargo bevy-api-gen generate, modify the templates not this file
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
use super::bevy_ecs::*;
use super::bevy_reflect::*;
use super::bevy_core::*;
use super::bevy_math::*;
use super::bevy_hierarchy::*;
use bevy_mod_scripting_core::bindings::{
    ReflectReference,
    function::{
        from::{Ref, Mut, Val},
        namespace::NamespaceBuilder,
    },
};
use crate::*;
pub struct BevyTransformScriptingPlugin;
impl ::bevy::app::Plugin for BevyTransformScriptingPlugin {
    fn build(&self, app: &mut ::bevy::prelude::App) {
        let mut world = app.world_mut();
        NamespaceBuilder::<::bevy::transform::components::GlobalTransform>::new(world)
            .register(
                "affine",
                |_self: Ref<bevy::transform::components::GlobalTransform>| {
                    let output: Val<bevy::math::Affine3A> = bevy::transform::components::GlobalTransform::affine(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "back",
                |_self: Ref<bevy::transform::components::GlobalTransform>| {
                    let output: Val<bevy::math::Dir3> = bevy::transform::components::GlobalTransform::back(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "clone",
                |_self: Ref<bevy::transform::components::GlobalTransform>| {
                    let output: Val<bevy::transform::components::GlobalTransform> = <bevy::transform::components::GlobalTransform as std::clone::Clone>::clone(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "compute_matrix",
                |_self: Ref<bevy::transform::components::GlobalTransform>| {
                    let output: Val<bevy::math::Mat4> = bevy::transform::components::GlobalTransform::compute_matrix(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "compute_transform",
                |_self: Ref<bevy::transform::components::GlobalTransform>| {
                    let output: Val<bevy::transform::components::Transform> = bevy::transform::components::GlobalTransform::compute_transform(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "down",
                |_self: Ref<bevy::transform::components::GlobalTransform>| {
                    let output: Val<bevy::math::Dir3> = bevy::transform::components::GlobalTransform::down(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "eq",
                |
                    _self: Ref<bevy::transform::components::GlobalTransform>,
                    other: Ref<bevy::transform::components::GlobalTransform>|
                {
                    let output: bool = <bevy::transform::components::GlobalTransform as std::cmp::PartialEq<
                        bevy::transform::components::GlobalTransform,
                    >>::eq(&_self, &other)
                        .into();
                    output
                },
            )
            .register(
                "forward",
                |_self: Ref<bevy::transform::components::GlobalTransform>| {
                    let output: Val<bevy::math::Dir3> = bevy::transform::components::GlobalTransform::forward(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "from_isometry",
                |iso: Val<bevy::math::Isometry3d>| {
                    let output: Val<bevy::transform::components::GlobalTransform> = bevy::transform::components::GlobalTransform::from_isometry(
                            iso.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "from_rotation",
                |rotation: Val<bevy::math::Quat>| {
                    let output: Val<bevy::transform::components::GlobalTransform> = bevy::transform::components::GlobalTransform::from_rotation(
                            rotation.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "from_scale",
                |scale: Val<bevy::math::Vec3>| {
                    let output: Val<bevy::transform::components::GlobalTransform> = bevy::transform::components::GlobalTransform::from_scale(
                            scale.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "from_translation",
                |translation: Val<bevy::math::Vec3>| {
                    let output: Val<bevy::transform::components::GlobalTransform> = bevy::transform::components::GlobalTransform::from_translation(
                            translation.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "from_xyz",
                |x: f32, y: f32, z: f32| {
                    let output: Val<bevy::transform::components::GlobalTransform> = bevy::transform::components::GlobalTransform::from_xyz(
                            x,
                            y,
                            z,
                        )
                        .into();
                    output
                },
            )
            .register(
                "left",
                |_self: Ref<bevy::transform::components::GlobalTransform>| {
                    let output: Val<bevy::math::Dir3> = bevy::transform::components::GlobalTransform::left(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "mul",
                |
                    _self: Val<bevy::transform::components::GlobalTransform>,
                    value: Val<bevy::math::Vec3>|
                {
                    let output: Val<bevy::math::Vec3> = <bevy::transform::components::GlobalTransform as std::ops::Mul<
                        bevy::math::Vec3,
                    >>::mul(_self.into_inner(), value.into_inner())
                        .into();
                    output
                },
            )
            .register(
                "mul",
                |
                    _self: Val<bevy::transform::components::GlobalTransform>,
                    global_transform: Val<bevy::transform::components::GlobalTransform>|
                {
                    let output: Val<bevy::transform::components::GlobalTransform> = <bevy::transform::components::GlobalTransform as std::ops::Mul<
                        bevy::transform::components::GlobalTransform,
                    >>::mul(_self.into_inner(), global_transform.into_inner())
                        .into();
                    output
                },
            )
            .register(
                "mul",
                |
                    _self: Val<bevy::transform::components::GlobalTransform>,
                    transform: Val<bevy::transform::components::Transform>|
                {
                    let output: Val<bevy::transform::components::GlobalTransform> = <bevy::transform::components::GlobalTransform as std::ops::Mul<
                        bevy::transform::components::Transform,
                    >>::mul(_self.into_inner(), transform.into_inner())
                        .into();
                    output
                },
            )
            .register(
                "mul_transform",
                |
                    _self: Ref<bevy::transform::components::GlobalTransform>,
                    transform: Val<bevy::transform::components::Transform>|
                {
                    let output: Val<bevy::transform::components::GlobalTransform> = bevy::transform::components::GlobalTransform::mul_transform(
                            &_self,
                            transform.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "radius_vec3a",
                |
                    _self: Ref<bevy::transform::components::GlobalTransform>,
                    extents: Val<bevy::math::Vec3A>|
                {
                    let output: f32 = bevy::transform::components::GlobalTransform::radius_vec3a(
                            &_self,
                            extents.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "reparented_to",
                |
                    _self: Ref<bevy::transform::components::GlobalTransform>,
                    parent: Ref<bevy::transform::components::GlobalTransform>|
                {
                    let output: Val<bevy::transform::components::Transform> = bevy::transform::components::GlobalTransform::reparented_to(
                            &_self,
                            &parent,
                        )
                        .into();
                    output
                },
            )
            .register(
                "right",
                |_self: Ref<bevy::transform::components::GlobalTransform>| {
                    let output: Val<bevy::math::Dir3> = bevy::transform::components::GlobalTransform::right(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "rotation",
                |_self: Ref<bevy::transform::components::GlobalTransform>| {
                    let output: Val<bevy::math::Quat> = bevy::transform::components::GlobalTransform::rotation(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "scale",
                |_self: Ref<bevy::transform::components::GlobalTransform>| {
                    let output: Val<bevy::math::Vec3> = bevy::transform::components::GlobalTransform::scale(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "to_isometry",
                |_self: Ref<bevy::transform::components::GlobalTransform>| {
                    let output: Val<bevy::math::Isometry3d> = bevy::transform::components::GlobalTransform::to_isometry(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "transform_point",
                |
                    _self: Ref<bevy::transform::components::GlobalTransform>,
                    point: Val<bevy::math::Vec3>|
                {
                    let output: Val<bevy::math::Vec3> = bevy::transform::components::GlobalTransform::transform_point(
                            &_self,
                            point.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "translation",
                |_self: Ref<bevy::transform::components::GlobalTransform>| {
                    let output: Val<bevy::math::Vec3> = bevy::transform::components::GlobalTransform::translation(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "translation_vec3a",
                |_self: Ref<bevy::transform::components::GlobalTransform>| {
                    let output: Val<bevy::math::Vec3A> = bevy::transform::components::GlobalTransform::translation_vec3a(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "up",
                |_self: Ref<bevy::transform::components::GlobalTransform>| {
                    let output: Val<bevy::math::Dir3> = bevy::transform::components::GlobalTransform::up(
                            &_self,
                        )
                        .into();
                    output
                },
            );
        NamespaceBuilder::<::bevy::transform::components::Transform>::new(world)
            .register(
                "back",
                |_self: Ref<bevy::transform::components::Transform>| {
                    let output: Val<bevy::math::Dir3> = bevy::transform::components::Transform::back(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "clone",
                |_self: Ref<bevy::transform::components::Transform>| {
                    let output: Val<bevy::transform::components::Transform> = <bevy::transform::components::Transform as std::clone::Clone>::clone(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "compute_affine",
                |_self: Ref<bevy::transform::components::Transform>| {
                    let output: Val<bevy::math::Affine3A> = bevy::transform::components::Transform::compute_affine(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "compute_matrix",
                |_self: Ref<bevy::transform::components::Transform>| {
                    let output: Val<bevy::math::Mat4> = bevy::transform::components::Transform::compute_matrix(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "down",
                |_self: Ref<bevy::transform::components::Transform>| {
                    let output: Val<bevy::math::Dir3> = bevy::transform::components::Transform::down(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "eq",
                |
                    _self: Ref<bevy::transform::components::Transform>,
                    other: Ref<bevy::transform::components::Transform>|
                {
                    let output: bool = <bevy::transform::components::Transform as std::cmp::PartialEq<
                        bevy::transform::components::Transform,
                    >>::eq(&_self, &other)
                        .into();
                    output
                },
            )
            .register(
                "forward",
                |_self: Ref<bevy::transform::components::Transform>| {
                    let output: Val<bevy::math::Dir3> = bevy::transform::components::Transform::forward(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "from_isometry",
                |iso: Val<bevy::math::Isometry3d>| {
                    let output: Val<bevy::transform::components::Transform> = bevy::transform::components::Transform::from_isometry(
                            iso.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "from_matrix",
                |world_from_local: Val<bevy::math::Mat4>| {
                    let output: Val<bevy::transform::components::Transform> = bevy::transform::components::Transform::from_matrix(
                            world_from_local.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "from_rotation",
                |rotation: Val<bevy::math::Quat>| {
                    let output: Val<bevy::transform::components::Transform> = bevy::transform::components::Transform::from_rotation(
                            rotation.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "from_scale",
                |scale: Val<bevy::math::Vec3>| {
                    let output: Val<bevy::transform::components::Transform> = bevy::transform::components::Transform::from_scale(
                            scale.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "from_translation",
                |translation: Val<bevy::math::Vec3>| {
                    let output: Val<bevy::transform::components::Transform> = bevy::transform::components::Transform::from_translation(
                            translation.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "from_xyz",
                |x: f32, y: f32, z: f32| {
                    let output: Val<bevy::transform::components::Transform> = bevy::transform::components::Transform::from_xyz(
                            x,
                            y,
                            z,
                        )
                        .into();
                    output
                },
            )
            .register(
                "is_finite",
                |_self: Ref<bevy::transform::components::Transform>| {
                    let output: bool = bevy::transform::components::Transform::is_finite(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "left",
                |_self: Ref<bevy::transform::components::Transform>| {
                    let output: Val<bevy::math::Dir3> = bevy::transform::components::Transform::left(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "local_x",
                |_self: Ref<bevy::transform::components::Transform>| {
                    let output: Val<bevy::math::Dir3> = bevy::transform::components::Transform::local_x(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "local_y",
                |_self: Ref<bevy::transform::components::Transform>| {
                    let output: Val<bevy::math::Dir3> = bevy::transform::components::Transform::local_y(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "local_z",
                |_self: Ref<bevy::transform::components::Transform>| {
                    let output: Val<bevy::math::Dir3> = bevy::transform::components::Transform::local_z(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "mul",
                |
                    _self: Val<bevy::transform::components::Transform>,
                    value: Val<bevy::math::Vec3>|
                {
                    let output: Val<bevy::math::Vec3> = <bevy::transform::components::Transform as std::ops::Mul<
                        bevy::math::Vec3,
                    >>::mul(_self.into_inner(), value.into_inner())
                        .into();
                    output
                },
            )
            .register(
                "mul",
                |
                    _self: Val<bevy::transform::components::Transform>,
                    global_transform: Val<bevy::transform::components::GlobalTransform>|
                {
                    let output: Val<bevy::transform::components::GlobalTransform> = <bevy::transform::components::Transform as std::ops::Mul<
                        bevy::transform::components::GlobalTransform,
                    >>::mul(_self.into_inner(), global_transform.into_inner())
                        .into();
                    output
                },
            )
            .register(
                "mul",
                |
                    _self: Val<bevy::transform::components::Transform>,
                    transform: Val<bevy::transform::components::Transform>|
                {
                    let output: Val<bevy::transform::components::Transform> = <bevy::transform::components::Transform as std::ops::Mul<
                        bevy::transform::components::Transform,
                    >>::mul(_self.into_inner(), transform.into_inner())
                        .into();
                    output
                },
            )
            .register(
                "mul_transform",
                |
                    _self: Ref<bevy::transform::components::Transform>,
                    transform: Val<bevy::transform::components::Transform>|
                {
                    let output: Val<bevy::transform::components::Transform> = bevy::transform::components::Transform::mul_transform(
                            &_self,
                            transform.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "right",
                |_self: Ref<bevy::transform::components::Transform>| {
                    let output: Val<bevy::math::Dir3> = bevy::transform::components::Transform::right(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "rotate",
                |
                    mut _self: Mut<bevy::transform::components::Transform>,
                    rotation: Val<bevy::math::Quat>|
                {
                    let output: () = bevy::transform::components::Transform::rotate(
                            &mut _self,
                            rotation.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "rotate_around",
                |
                    mut _self: Mut<bevy::transform::components::Transform>,
                    point: Val<bevy::math::Vec3>,
                    rotation: Val<bevy::math::Quat>|
                {
                    let output: () = bevy::transform::components::Transform::rotate_around(
                            &mut _self,
                            point.into_inner(),
                            rotation.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "rotate_axis",
                |
                    mut _self: Mut<bevy::transform::components::Transform>,
                    axis: Val<bevy::math::Dir3>,
                    angle: f32|
                {
                    let output: () = bevy::transform::components::Transform::rotate_axis(
                            &mut _self,
                            axis.into_inner(),
                            angle,
                        )
                        .into();
                    output
                },
            )
            .register(
                "rotate_local",
                |
                    mut _self: Mut<bevy::transform::components::Transform>,
                    rotation: Val<bevy::math::Quat>|
                {
                    let output: () = bevy::transform::components::Transform::rotate_local(
                            &mut _self,
                            rotation.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "rotate_local_axis",
                |
                    mut _self: Mut<bevy::transform::components::Transform>,
                    axis: Val<bevy::math::Dir3>,
                    angle: f32|
                {
                    let output: () = bevy::transform::components::Transform::rotate_local_axis(
                            &mut _self,
                            axis.into_inner(),
                            angle,
                        )
                        .into();
                    output
                },
            )
            .register(
                "rotate_local_x",
                |mut _self: Mut<bevy::transform::components::Transform>, angle: f32| {
                    let output: () = bevy::transform::components::Transform::rotate_local_x(
                            &mut _self,
                            angle,
                        )
                        .into();
                    output
                },
            )
            .register(
                "rotate_local_y",
                |mut _self: Mut<bevy::transform::components::Transform>, angle: f32| {
                    let output: () = bevy::transform::components::Transform::rotate_local_y(
                            &mut _self,
                            angle,
                        )
                        .into();
                    output
                },
            )
            .register(
                "rotate_local_z",
                |mut _self: Mut<bevy::transform::components::Transform>, angle: f32| {
                    let output: () = bevy::transform::components::Transform::rotate_local_z(
                            &mut _self,
                            angle,
                        )
                        .into();
                    output
                },
            )
            .register(
                "rotate_x",
                |mut _self: Mut<bevy::transform::components::Transform>, angle: f32| {
                    let output: () = bevy::transform::components::Transform::rotate_x(
                            &mut _self,
                            angle,
                        )
                        .into();
                    output
                },
            )
            .register(
                "rotate_y",
                |mut _self: Mut<bevy::transform::components::Transform>, angle: f32| {
                    let output: () = bevy::transform::components::Transform::rotate_y(
                            &mut _self,
                            angle,
                        )
                        .into();
                    output
                },
            )
            .register(
                "rotate_z",
                |mut _self: Mut<bevy::transform::components::Transform>, angle: f32| {
                    let output: () = bevy::transform::components::Transform::rotate_z(
                            &mut _self,
                            angle,
                        )
                        .into();
                    output
                },
            )
            .register(
                "to_isometry",
                |_self: Ref<bevy::transform::components::Transform>| {
                    let output: Val<bevy::math::Isometry3d> = bevy::transform::components::Transform::to_isometry(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "transform_point",
                |
                    _self: Ref<bevy::transform::components::Transform>,
                    point: Val<bevy::math::Vec3>|
                {
                    let output: Val<bevy::math::Vec3> = bevy::transform::components::Transform::transform_point(
                            &_self,
                            point.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "translate_around",
                |
                    mut _self: Mut<bevy::transform::components::Transform>,
                    point: Val<bevy::math::Vec3>,
                    rotation: Val<bevy::math::Quat>|
                {
                    let output: () = bevy::transform::components::Transform::translate_around(
                            &mut _self,
                            point.into_inner(),
                            rotation.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "up",
                |_self: Ref<bevy::transform::components::Transform>| {
                    let output: Val<bevy::math::Dir3> = bevy::transform::components::Transform::up(
                            &_self,
                        )
                        .into();
                    output
                },
            )
            .register(
                "with_rotation",
                |
                    _self: Val<bevy::transform::components::Transform>,
                    rotation: Val<bevy::math::Quat>|
                {
                    let output: Val<bevy::transform::components::Transform> = bevy::transform::components::Transform::with_rotation(
                            _self.into_inner(),
                            rotation.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "with_scale",
                |
                    _self: Val<bevy::transform::components::Transform>,
                    scale: Val<bevy::math::Vec3>|
                {
                    let output: Val<bevy::transform::components::Transform> = bevy::transform::components::Transform::with_scale(
                            _self.into_inner(),
                            scale.into_inner(),
                        )
                        .into();
                    output
                },
            )
            .register(
                "with_translation",
                |
                    _self: Val<bevy::transform::components::Transform>,
                    translation: Val<bevy::math::Vec3>|
                {
                    let output: Val<bevy::transform::components::Transform> = bevy::transform::components::Transform::with_translation(
                            _self.into_inner(),
                            translation.into_inner(),
                        )
                        .into();
                    output
                },
            );
    }
}
