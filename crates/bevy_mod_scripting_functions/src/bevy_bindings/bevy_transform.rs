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
            );
        NamespaceBuilder::<::bevy::transform::components::Transform>::new(world)
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
            );
    }
}
