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
pub struct BevyTransformScriptingPlugin;
pub(crate) fn register_global_transform_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_transform::components::GlobalTransform,
    >::new(world)
        .register_documented(
            "affine",
            |_self: Ref<::bevy_transform::components::GlobalTransform>| {
                let output: Val<::bevy_math::Affine3A> = {
                    {
                        let output: Val<::bevy_math::Affine3A> = ::bevy_transform::components::GlobalTransform::affine(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the 3d affine transformation matrix as an [`Affine3A`].",
            &["_self"],
        )
        .register_documented(
            "back",
            |_self: Ref<::bevy_transform::components::GlobalTransform>| {
                let output: Val<::bevy_math::Dir3> = {
                    {
                        let output: Val<::bevy_math::Dir3> = ::bevy_transform::components::GlobalTransform::back(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "Return the local back vector (Z).",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_transform::components::GlobalTransform>| {
                let output: Val<::bevy_transform::components::GlobalTransform> = {
                    {
                        let output: Val<::bevy_transform::components::GlobalTransform> = <::bevy_transform::components::GlobalTransform as ::core::clone::Clone>::clone(
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
            "compute_matrix",
            |_self: Ref<::bevy_transform::components::GlobalTransform>| {
                let output: Val<::bevy_math::Mat4> = {
                    {
                        let output: Val<::bevy_math::Mat4> = ::bevy_transform::components::GlobalTransform::compute_matrix(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the 3d affine transformation matrix as a [`Mat4`].",
            &["_self"],
        )
        .register_documented(
            "compute_transform",
            |_self: Ref<::bevy_transform::components::GlobalTransform>| {
                let output: Val<::bevy_transform::components::Transform> = {
                    {
                        let output: Val<::bevy_transform::components::Transform> = ::bevy_transform::components::GlobalTransform::compute_transform(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the transformation as a [`Transform`].\n The transform is expected to be non-degenerate and without shearing, or the output\n will be invalid.",
            &["_self"],
        )
        .register_documented(
            "down",
            |_self: Ref<::bevy_transform::components::GlobalTransform>| {
                let output: Val<::bevy_math::Dir3> = {
                    {
                        let output: Val<::bevy_math::Dir3> = ::bevy_transform::components::GlobalTransform::down(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "Return the local down vector (-Y).",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_transform::components::GlobalTransform>,
                other: Ref<::bevy_transform::components::GlobalTransform>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_transform::components::GlobalTransform as ::core::cmp::PartialEq<
                            ::bevy_transform::components::GlobalTransform,
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
            "forward",
            |_self: Ref<::bevy_transform::components::GlobalTransform>| {
                let output: Val<::bevy_math::Dir3> = {
                    {
                        let output: Val<::bevy_math::Dir3> = ::bevy_transform::components::GlobalTransform::forward(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "Return the local forward vector (-Z).",
            &["_self"],
        )
        .register_documented(
            "from_isometry",
            |iso: Val<::bevy_math::Isometry3d>| {
                let output: Val<::bevy_transform::components::GlobalTransform> = {
                    {
                        let output: Val<::bevy_transform::components::GlobalTransform> = ::bevy_transform::components::GlobalTransform::from_isometry(
                                iso.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["iso"],
        )
        .register_documented(
            "from_rotation",
            |rotation: Val<::bevy_math::Quat>| {
                let output: Val<::bevy_transform::components::GlobalTransform> = {
                    {
                        let output: Val<::bevy_transform::components::GlobalTransform> = ::bevy_transform::components::GlobalTransform::from_rotation(
                                rotation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["rotation"],
        )
        .register_documented(
            "from_scale",
            |scale: Val<::bevy_math::Vec3>| {
                let output: Val<::bevy_transform::components::GlobalTransform> = {
                    {
                        let output: Val<::bevy_transform::components::GlobalTransform> = ::bevy_transform::components::GlobalTransform::from_scale(
                                scale.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["scale"],
        )
        .register_documented(
            "from_translation",
            |translation: Val<::bevy_math::Vec3>| {
                let output: Val<::bevy_transform::components::GlobalTransform> = {
                    {
                        let output: Val<::bevy_transform::components::GlobalTransform> = ::bevy_transform::components::GlobalTransform::from_translation(
                                translation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["translation"],
        )
        .register_documented(
            "from_xyz",
            |x: f32, y: f32, z: f32| {
                let output: Val<::bevy_transform::components::GlobalTransform> = {
                    {
                        let output: Val<::bevy_transform::components::GlobalTransform> = ::bevy_transform::components::GlobalTransform::from_xyz(
                                x,
                                y,
                                z,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["x", "y", "z"],
        )
        .register_documented(
            "left",
            |_self: Ref<::bevy_transform::components::GlobalTransform>| {
                let output: Val<::bevy_math::Dir3> = {
                    {
                        let output: Val<::bevy_math::Dir3> = ::bevy_transform::components::GlobalTransform::left(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "Return the local left vector (-X).",
            &["_self"],
        )
        .register_documented(
            "mul",
            |
                _self: Val<::bevy_transform::components::GlobalTransform>,
                value: Val<::bevy_math::Vec3>|
            {
                let output: Val<::bevy_math::Vec3> = {
                    {
                        let output: Val<::bevy_math::Vec3> = <::bevy_transform::components::GlobalTransform as ::core::ops::Mul<
                            ::bevy_math::Vec3,
                        >>::mul(_self.into_inner(), value.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "value"],
        )
        .register_documented(
            "mul",
            |
                _self: Val<::bevy_transform::components::GlobalTransform>,
                global_transform: Val<::bevy_transform::components::GlobalTransform>|
            {
                let output: Val<::bevy_transform::components::GlobalTransform> = {
                    {
                        let output: Val<::bevy_transform::components::GlobalTransform> = <::bevy_transform::components::GlobalTransform as ::core::ops::Mul<
                            ::bevy_transform::components::GlobalTransform,
                        >>::mul(_self.into_inner(), global_transform.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "global_transform"],
        )
        .register_documented(
            "mul",
            |
                _self: Val<::bevy_transform::components::GlobalTransform>,
                transform: Val<::bevy_transform::components::Transform>|
            {
                let output: Val<::bevy_transform::components::GlobalTransform> = {
                    {
                        let output: Val<::bevy_transform::components::GlobalTransform> = <::bevy_transform::components::GlobalTransform as ::core::ops::Mul<
                            ::bevy_transform::components::Transform,
                        >>::mul(_self.into_inner(), transform.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "transform"],
        )
        .register_documented(
            "mul_transform",
            |
                _self: Ref<::bevy_transform::components::GlobalTransform>,
                transform: Val<::bevy_transform::components::Transform>|
            {
                let output: Val<::bevy_transform::components::GlobalTransform> = {
                    {
                        let output: Val<::bevy_transform::components::GlobalTransform> = ::bevy_transform::components::GlobalTransform::mul_transform(
                                &_self,
                                transform.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Multiplies `self` with `transform` component by component, returning the\n resulting [`GlobalTransform`]",
            &["_self", "transform"],
        )
        .register_documented(
            "radius_vec3a",
            |
                _self: Ref<::bevy_transform::components::GlobalTransform>,
                extents: Val<::bevy_math::Vec3A>|
            {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_transform::components::GlobalTransform::radius_vec3a(
                                &_self,
                                extents.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get an upper bound of the radius from the given `extents`.",
            &["_self", "extents"],
        )
        .register_documented(
            "reparented_to",
            |
                _self: Ref<::bevy_transform::components::GlobalTransform>,
                parent: Ref<::bevy_transform::components::GlobalTransform>|
            {
                let output: Val<::bevy_transform::components::Transform> = {
                    {
                        let output: Val<::bevy_transform::components::Transform> = ::bevy_transform::components::GlobalTransform::reparented_to(
                                &_self,
                                &parent,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the [`Transform`] `self` would have if it was a child of an entity\n with the `parent` [`GlobalTransform`].\n This is useful if you want to \"reparent\" an [`Entity`](bevy_ecs::entity::Entity).\n Say you have an entity `e1` that you want to turn into a child of `e2`,\n but you want `e1` to keep the same global transform, even after re-parenting. You would use:\n ```\n # use bevy_transform::prelude::{GlobalTransform, Transform};\n # use bevy_ecs::prelude::{Entity, Query, Component, Commands};\n #[derive(Component)]\n struct ToReparent {\n     new_parent: Entity,\n }\n fn reparent_system(\n     mut commands: Commands,\n     mut targets: Query<(&mut Transform, Entity, &GlobalTransform, &ToReparent)>,\n     transforms: Query<&GlobalTransform>,\n ) {\n     for (mut transform, entity, initial, to_reparent) in targets.iter_mut() {\n         if let Ok(parent_transform) = transforms.get(to_reparent.new_parent) {\n             *transform = initial.reparented_to(parent_transform);\n             commands.entity(entity)\n                 .remove::<ToReparent>()\n                 .set_parent(to_reparent.new_parent);\n         }\n     }\n }\n ```\n The transform is expected to be non-degenerate and without shearing, or the output\n will be invalid.",
            &["_self", "parent"],
        )
        .register_documented(
            "right",
            |_self: Ref<::bevy_transform::components::GlobalTransform>| {
                let output: Val<::bevy_math::Dir3> = {
                    {
                        let output: Val<::bevy_math::Dir3> = ::bevy_transform::components::GlobalTransform::right(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "Return the local right vector (X).",
            &["_self"],
        )
        .register_documented(
            "rotation",
            |_self: Ref<::bevy_transform::components::GlobalTransform>| {
                let output: Val<::bevy_math::Quat> = {
                    {
                        let output: Val<::bevy_math::Quat> = ::bevy_transform::components::GlobalTransform::rotation(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the rotation as a [`Quat`].\n The transform is expected to be non-degenerate and without shearing, or the output will be invalid.\n # Warning\n This is calculated using `to_scale_rotation_translation`, meaning that you\n should probably use it directly if you also need translation or scale.",
            &["_self"],
        )
        .register_documented(
            "scale",
            |_self: Ref<::bevy_transform::components::GlobalTransform>| {
                let output: Val<::bevy_math::Vec3> = {
                    {
                        let output: Val<::bevy_math::Vec3> = ::bevy_transform::components::GlobalTransform::scale(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the scale as a [`Vec3`].\n The transform is expected to be non-degenerate and without shearing, or the output will be invalid.\n Some of the computations overlap with `to_scale_rotation_translation`, which means you should use\n it instead if you also need rotation.",
            &["_self"],
        )
        .register_documented(
            "to_isometry",
            |_self: Ref<::bevy_transform::components::GlobalTransform>| {
                let output: Val<::bevy_math::Isometry3d> = {
                    {
                        let output: Val<::bevy_math::Isometry3d> = ::bevy_transform::components::GlobalTransform::to_isometry(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the isometric part of the transformation as an [isometry]. Any scaling done by the\n transformation will be ignored.\n The transform is expected to be non-degenerate and without shearing, or the output\n will be invalid.\n [isometry]: Isometry3d",
            &["_self"],
        )
        .register_documented(
            "transform_point",
            |
                _self: Ref<::bevy_transform::components::GlobalTransform>,
                point: Val<::bevy_math::Vec3>|
            {
                let output: Val<::bevy_math::Vec3> = {
                    {
                        let output: Val<::bevy_math::Vec3> = ::bevy_transform::components::GlobalTransform::transform_point(
                                &_self,
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Transforms the given point from local space to global space, applying shear, scale, rotation and translation.\n It can be used like this:\n ```\n # use bevy_transform::prelude::{GlobalTransform};\n # use bevy_math::prelude::Vec3;\n let global_transform = GlobalTransform::from_xyz(1., 2., 3.);\n let local_point = Vec3::new(1., 2., 3.);\n let global_point = global_transform.transform_point(local_point);\n assert_eq!(global_point, Vec3::new(2., 4., 6.));\n ```\n ```\n # use bevy_transform::prelude::{GlobalTransform};\n # use bevy_math::Vec3;\n let global_point = Vec3::new(2., 4., 6.);\n let global_transform = GlobalTransform::from_xyz(1., 2., 3.);\n let local_point = global_transform.affine().inverse().transform_point3(global_point);\n assert_eq!(local_point, Vec3::new(1., 2., 3.))\n ```\n To apply shear, scale, and rotation *without* applying translation, different functions are available:\n ```\n # use bevy_transform::prelude::{GlobalTransform};\n # use bevy_math::prelude::Vec3;\n let global_transform = GlobalTransform::from_xyz(1., 2., 3.);\n let local_direction = Vec3::new(1., 2., 3.);\n let global_direction = global_transform.affine().transform_vector3(local_direction);\n assert_eq!(global_direction, Vec3::new(1., 2., 3.));\n let roundtripped_local_direction = global_transform.affine().inverse().transform_vector3(global_direction);\n assert_eq!(roundtripped_local_direction, local_direction);\n ```",
            &["_self", "point"],
        )
        .register_documented(
            "translation",
            |_self: Ref<::bevy_transform::components::GlobalTransform>| {
                let output: Val<::bevy_math::Vec3> = {
                    {
                        let output: Val<::bevy_math::Vec3> = ::bevy_transform::components::GlobalTransform::translation(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the translation as a [`Vec3`].",
            &["_self"],
        )
        .register_documented(
            "translation_vec3a",
            |_self: Ref<::bevy_transform::components::GlobalTransform>| {
                let output: Val<::bevy_math::Vec3A> = {
                    {
                        let output: Val<::bevy_math::Vec3A> = ::bevy_transform::components::GlobalTransform::translation_vec3a(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the translation as a [`Vec3A`].",
            &["_self"],
        )
        .register_documented(
            "up",
            |_self: Ref<::bevy_transform::components::GlobalTransform>| {
                let output: Val<::bevy_math::Dir3> = {
                    {
                        let output: Val<::bevy_math::Dir3> = ::bevy_transform::components::GlobalTransform::up(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "Return the local up vector (Y).",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_transform::components::GlobalTransform,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_transform_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_transform::components::Transform,
    >::new(world)
        .register_documented(
            "back",
            |_self: Ref<::bevy_transform::components::Transform>| {
                let output: Val<::bevy_math::Dir3> = {
                    {
                        let output: Val<::bevy_math::Dir3> = ::bevy_transform::components::Transform::back(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Equivalent to [`local_z()`][Transform::local_z]",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_transform::components::Transform>| {
                let output: Val<::bevy_transform::components::Transform> = {
                    {
                        let output: Val<::bevy_transform::components::Transform> = <::bevy_transform::components::Transform as ::core::clone::Clone>::clone(
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
            "compute_affine",
            |_self: Ref<::bevy_transform::components::Transform>| {
                let output: Val<::bevy_math::Affine3A> = {
                    {
                        let output: Val<::bevy_math::Affine3A> = ::bevy_transform::components::Transform::compute_affine(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the 3d affine transformation matrix from this transforms translation,\n rotation, and scale.",
            &["_self"],
        )
        .register_documented(
            "compute_matrix",
            |_self: Ref<::bevy_transform::components::Transform>| {
                let output: Val<::bevy_math::Mat4> = {
                    {
                        let output: Val<::bevy_math::Mat4> = ::bevy_transform::components::Transform::compute_matrix(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the 3d affine transformation matrix from this transforms translation,\n rotation, and scale.",
            &["_self"],
        )
        .register_documented(
            "down",
            |_self: Ref<::bevy_transform::components::Transform>| {
                let output: Val<::bevy_math::Dir3> = {
                    {
                        let output: Val<::bevy_math::Dir3> = ::bevy_transform::components::Transform::down(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Equivalent to [`-local_y()`][Transform::local_y]",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_transform::components::Transform>,
                other: Ref<::bevy_transform::components::Transform>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_transform::components::Transform as ::core::cmp::PartialEq<
                            ::bevy_transform::components::Transform,
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
            "forward",
            |_self: Ref<::bevy_transform::components::Transform>| {
                let output: Val<::bevy_math::Dir3> = {
                    {
                        let output: Val<::bevy_math::Dir3> = ::bevy_transform::components::Transform::forward(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Equivalent to [`-local_z()`][Transform::local_z]",
            &["_self"],
        )
        .register_documented(
            "from_isometry",
            |iso: Val<::bevy_math::Isometry3d>| {
                let output: Val<::bevy_transform::components::Transform> = {
                    {
                        let output: Val<::bevy_transform::components::Transform> = ::bevy_transform::components::Transform::from_isometry(
                                iso.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Transform`] that is equivalent to the given [isometry].\n [isometry]: Isometry3d",
            &["iso"],
        )
        .register_documented(
            "from_matrix",
            |world_from_local: Val<::bevy_math::Mat4>| {
                let output: Val<::bevy_transform::components::Transform> = {
                    {
                        let output: Val<::bevy_transform::components::Transform> = ::bevy_transform::components::Transform::from_matrix(
                                world_from_local.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Extracts the translation, rotation, and scale from `matrix`. It must be a 3d affine\n transformation matrix.",
            &["world_from_local"],
        )
        .register_documented(
            "from_rotation",
            |rotation: Val<::bevy_math::Quat>| {
                let output: Val<::bevy_transform::components::Transform> = {
                    {
                        let output: Val<::bevy_transform::components::Transform> = ::bevy_transform::components::Transform::from_rotation(
                                rotation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Transform`], with `rotation`. Translation will be 0 and scale 1 on\n all axes.",
            &["rotation"],
        )
        .register_documented(
            "from_scale",
            |scale: Val<::bevy_math::Vec3>| {
                let output: Val<::bevy_transform::components::Transform> = {
                    {
                        let output: Val<::bevy_transform::components::Transform> = ::bevy_transform::components::Transform::from_scale(
                                scale.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Transform`], with `scale`. Translation will be 0 and rotation 0 on\n all axes.",
            &["scale"],
        )
        .register_documented(
            "from_translation",
            |translation: Val<::bevy_math::Vec3>| {
                let output: Val<::bevy_transform::components::Transform> = {
                    {
                        let output: Val<::bevy_transform::components::Transform> = ::bevy_transform::components::Transform::from_translation(
                                translation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Transform`], with `translation`. Rotation will be 0 and scale 1 on\n all axes.",
            &["translation"],
        )
        .register_documented(
            "from_xyz",
            |x: f32, y: f32, z: f32| {
                let output: Val<::bevy_transform::components::Transform> = {
                    {
                        let output: Val<::bevy_transform::components::Transform> = ::bevy_transform::components::Transform::from_xyz(
                                x,
                                y,
                                z,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new [`Transform`] at the position `(x, y, z)`. In 2d, the `z` component\n is used for z-ordering elements: higher `z`-value will be in front of lower\n `z`-value.",
            &["x", "y", "z"],
        )
        .register_documented(
            "is_finite",
            |_self: Ref<::bevy_transform::components::Transform>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_transform::components::Transform::is_finite(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if, and only if, translation, rotation and scale all are\n finite. If any of them contains a `NaN`, positive or negative infinity,\n this will return `false`.",
            &["_self"],
        )
        .register_documented(
            "left",
            |_self: Ref<::bevy_transform::components::Transform>| {
                let output: Val<::bevy_math::Dir3> = {
                    {
                        let output: Val<::bevy_math::Dir3> = ::bevy_transform::components::Transform::left(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Equivalent to [`-local_x()`][Transform::local_x()]",
            &["_self"],
        )
        .register_documented(
            "local_x",
            |_self: Ref<::bevy_transform::components::Transform>| {
                let output: Val<::bevy_math::Dir3> = {
                    {
                        let output: Val<::bevy_math::Dir3> = ::bevy_transform::components::Transform::local_x(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the unit vector in the local `X` direction.",
            &["_self"],
        )
        .register_documented(
            "local_y",
            |_self: Ref<::bevy_transform::components::Transform>| {
                let output: Val<::bevy_math::Dir3> = {
                    {
                        let output: Val<::bevy_math::Dir3> = ::bevy_transform::components::Transform::local_y(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the unit vector in the local `Y` direction.",
            &["_self"],
        )
        .register_documented(
            "local_z",
            |_self: Ref<::bevy_transform::components::Transform>| {
                let output: Val<::bevy_math::Dir3> = {
                    {
                        let output: Val<::bevy_math::Dir3> = ::bevy_transform::components::Transform::local_z(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the unit vector in the local `Z` direction.",
            &["_self"],
        )
        .register_documented(
            "mul",
            |
                _self: Val<::bevy_transform::components::Transform>,
                value: Val<::bevy_math::Vec3>|
            {
                let output: Val<::bevy_math::Vec3> = {
                    {
                        let output: Val<::bevy_math::Vec3> = <::bevy_transform::components::Transform as ::core::ops::Mul<
                            ::bevy_math::Vec3,
                        >>::mul(_self.into_inner(), value.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "value"],
        )
        .register_documented(
            "mul",
            |
                _self: Val<::bevy_transform::components::Transform>,
                global_transform: Val<::bevy_transform::components::GlobalTransform>|
            {
                let output: Val<::bevy_transform::components::GlobalTransform> = {
                    {
                        let output: Val<::bevy_transform::components::GlobalTransform> = <::bevy_transform::components::Transform as ::core::ops::Mul<
                            ::bevy_transform::components::GlobalTransform,
                        >>::mul(_self.into_inner(), global_transform.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "global_transform"],
        )
        .register_documented(
            "mul",
            |
                _self: Val<::bevy_transform::components::Transform>,
                transform: Val<::bevy_transform::components::Transform>|
            {
                let output: Val<::bevy_transform::components::Transform> = {
                    {
                        let output: Val<::bevy_transform::components::Transform> = <::bevy_transform::components::Transform as ::core::ops::Mul<
                            ::bevy_transform::components::Transform,
                        >>::mul(_self.into_inner(), transform.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "transform"],
        )
        .register_documented(
            "mul_transform",
            |
                _self: Ref<::bevy_transform::components::Transform>,
                transform: Val<::bevy_transform::components::Transform>|
            {
                let output: Val<::bevy_transform::components::Transform> = {
                    {
                        let output: Val<::bevy_transform::components::Transform> = ::bevy_transform::components::Transform::mul_transform(
                                &_self,
                                transform.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Multiplies `self` with `transform` component by component, returning the\n resulting [`Transform`]",
            &["_self", "transform"],
        )
        .register_documented(
            "right",
            |_self: Ref<::bevy_transform::components::Transform>| {
                let output: Val<::bevy_math::Dir3> = {
                    {
                        let output: Val<::bevy_math::Dir3> = ::bevy_transform::components::Transform::right(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Equivalent to [`local_x()`][Transform::local_x()]",
            &["_self"],
        )
        .register_documented(
            "rotate",
            |
                mut _self: Mut<::bevy_transform::components::Transform>,
                rotation: Val<::bevy_math::Quat>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_transform::components::Transform::rotate(
                                &mut _self,
                                rotation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rotates this [`Transform`] by the given rotation.\n If this [`Transform`] has a parent, the `rotation` is relative to the rotation of the parent.\n # Examples\n - [`3d_rotation`]\n [`3d_rotation`]: https://github.com/bevyengine/bevy/blob/latest/examples/transforms/3d_rotation.rs",
            &["_self", "rotation"],
        )
        .register_documented(
            "rotate_around",
            |
                mut _self: Mut<::bevy_transform::components::Transform>,
                point: Val<::bevy_math::Vec3>,
                rotation: Val<::bevy_math::Quat>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_transform::components::Transform::rotate_around(
                                &mut _self,
                                point.into_inner(),
                                rotation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rotates this [`Transform`] around a `point` in space.\n If this [`Transform`] has a parent, the `point` is relative to the [`Transform`] of the parent.",
            &["_self", "point", "rotation"],
        )
        .register_documented(
            "rotate_axis",
            |
                mut _self: Mut<::bevy_transform::components::Transform>,
                axis: Val<::bevy_math::Dir3>,
                angle: f32|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_transform::components::Transform::rotate_axis(
                                &mut _self,
                                axis.into_inner(),
                                angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rotates this [`Transform`] around the given `axis` by `angle` (in radians).\n If this [`Transform`] has a parent, the `axis` is relative to the rotation of the parent.\n # Warning\n If you pass in an `axis` based on the current rotation (e.g. obtained via [`Transform::local_x`]),\n floating point errors can accumulate exponentially when applying rotations repeatedly this way. This will\n result in a denormalized rotation. In this case, it is recommended to normalize the [`Transform::rotation`] after\n each call to this method.",
            &["_self", "axis", "angle"],
        )
        .register_documented(
            "rotate_local",
            |
                mut _self: Mut<::bevy_transform::components::Transform>,
                rotation: Val<::bevy_math::Quat>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_transform::components::Transform::rotate_local(
                                &mut _self,
                                rotation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rotates this [`Transform`] by the given `rotation`.\n The `rotation` is relative to this [`Transform`]'s current rotation.",
            &["_self", "rotation"],
        )
        .register_documented(
            "rotate_local_axis",
            |
                mut _self: Mut<::bevy_transform::components::Transform>,
                axis: Val<::bevy_math::Dir3>,
                angle: f32|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_transform::components::Transform::rotate_local_axis(
                                &mut _self,
                                axis.into_inner(),
                                angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rotates this [`Transform`] around its local `axis` by `angle` (in radians).\n # Warning\n If you pass in an `axis` based on the current rotation (e.g. obtained via [`Transform::local_x`]),\n floating point errors can accumulate exponentially when applying rotations repeatedly this way. This will\n result in a denormalized rotation. In this case, it is recommended to normalize the [`Transform::rotation`] after\n each call to this method.",
            &["_self", "axis", "angle"],
        )
        .register_documented(
            "rotate_local_x",
            |mut _self: Mut<::bevy_transform::components::Transform>, angle: f32| {
                let output: () = {
                    {
                        let output: () = ::bevy_transform::components::Transform::rotate_local_x(
                                &mut _self,
                                angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rotates this [`Transform`] around its local `X` axis by `angle` (in radians).",
            &["_self", "angle"],
        )
        .register_documented(
            "rotate_local_y",
            |mut _self: Mut<::bevy_transform::components::Transform>, angle: f32| {
                let output: () = {
                    {
                        let output: () = ::bevy_transform::components::Transform::rotate_local_y(
                                &mut _self,
                                angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rotates this [`Transform`] around its local `Y` axis by `angle` (in radians).",
            &["_self", "angle"],
        )
        .register_documented(
            "rotate_local_z",
            |mut _self: Mut<::bevy_transform::components::Transform>, angle: f32| {
                let output: () = {
                    {
                        let output: () = ::bevy_transform::components::Transform::rotate_local_z(
                                &mut _self,
                                angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rotates this [`Transform`] around its local `Z` axis by `angle` (in radians).",
            &["_self", "angle"],
        )
        .register_documented(
            "rotate_x",
            |mut _self: Mut<::bevy_transform::components::Transform>, angle: f32| {
                let output: () = {
                    {
                        let output: () = ::bevy_transform::components::Transform::rotate_x(
                                &mut _self,
                                angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rotates this [`Transform`] around the `X` axis by `angle` (in radians).\n If this [`Transform`] has a parent, the axis is relative to the rotation of the parent.",
            &["_self", "angle"],
        )
        .register_documented(
            "rotate_y",
            |mut _self: Mut<::bevy_transform::components::Transform>, angle: f32| {
                let output: () = {
                    {
                        let output: () = ::bevy_transform::components::Transform::rotate_y(
                                &mut _self,
                                angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rotates this [`Transform`] around the `Y` axis by `angle` (in radians).\n If this [`Transform`] has a parent, the axis is relative to the rotation of the parent.",
            &["_self", "angle"],
        )
        .register_documented(
            "rotate_z",
            |mut _self: Mut<::bevy_transform::components::Transform>, angle: f32| {
                let output: () = {
                    {
                        let output: () = ::bevy_transform::components::Transform::rotate_z(
                                &mut _self,
                                angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rotates this [`Transform`] around the `Z` axis by `angle` (in radians).\n If this [`Transform`] has a parent, the axis is relative to the rotation of the parent.",
            &["_self", "angle"],
        )
        .register_documented(
            "to_isometry",
            |_self: Ref<::bevy_transform::components::Transform>| {
                let output: Val<::bevy_math::Isometry3d> = {
                    {
                        let output: Val<::bevy_math::Isometry3d> = ::bevy_transform::components::Transform::to_isometry(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the [isometry] defined by this transform's rotation and translation, ignoring scale.\n [isometry]: Isometry3d",
            &["_self"],
        )
        .register_documented(
            "transform_point",
            |
                _self: Ref<::bevy_transform::components::Transform>,
                point: Val<::bevy_math::Vec3>|
            {
                let output: Val<::bevy_math::Vec3> = {
                    {
                        let output: Val<::bevy_math::Vec3> = ::bevy_transform::components::Transform::transform_point(
                                &_self,
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Transforms the given `point`, applying scale, rotation and translation.\n If this [`Transform`] has an ancestor entity with a [`Transform`] component,\n [`Transform::transform_point`] will transform a point in local space into its\n parent transform's space.\n If this [`Transform`] does not have a parent, [`Transform::transform_point`] will\n transform a point in local space into worldspace coordinates.\n If you always want to transform a point in local space to worldspace, or if you need\n the inverse transformations, see [`GlobalTransform::transform_point()`].",
            &["_self", "point"],
        )
        .register_documented(
            "translate_around",
            |
                mut _self: Mut<::bevy_transform::components::Transform>,
                point: Val<::bevy_math::Vec3>,
                rotation: Val<::bevy_math::Quat>|
            {
                let output: () = {
                    {
                        let output: () = ::bevy_transform::components::Transform::translate_around(
                                &mut _self,
                                point.into_inner(),
                                rotation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Translates this [`Transform`] around a `point` in space.\n If this [`Transform`] has a parent, the `point` is relative to the [`Transform`] of the parent.",
            &["_self", "point", "rotation"],
        )
        .register_documented(
            "up",
            |_self: Ref<::bevy_transform::components::Transform>| {
                let output: Val<::bevy_math::Dir3> = {
                    {
                        let output: Val<::bevy_math::Dir3> = ::bevy_transform::components::Transform::up(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Equivalent to [`local_y()`][Transform::local_y]",
            &["_self"],
        )
        .register_documented(
            "with_rotation",
            |
                _self: Val<::bevy_transform::components::Transform>,
                rotation: Val<::bevy_math::Quat>|
            {
                let output: Val<::bevy_transform::components::Transform> = {
                    {
                        let output: Val<::bevy_transform::components::Transform> = ::bevy_transform::components::Transform::with_rotation(
                                _self.into_inner(),
                                rotation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns this [`Transform`] with a new rotation.",
            &["_self", "rotation"],
        )
        .register_documented(
            "with_scale",
            |
                _self: Val<::bevy_transform::components::Transform>,
                scale: Val<::bevy_math::Vec3>|
            {
                let output: Val<::bevy_transform::components::Transform> = {
                    {
                        let output: Val<::bevy_transform::components::Transform> = ::bevy_transform::components::Transform::with_scale(
                                _self.into_inner(),
                                scale.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns this [`Transform`] with a new scale.",
            &["_self", "scale"],
        )
        .register_documented(
            "with_translation",
            |
                _self: Val<::bevy_transform::components::Transform>,
                translation: Val<::bevy_math::Vec3>|
            {
                let output: Val<::bevy_transform::components::Transform> = {
                    {
                        let output: Val<::bevy_transform::components::Transform> = ::bevy_transform::components::Transform::with_translation(
                                _self.into_inner(),
                                translation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns this [`Transform`] with a new translation.",
            &["_self", "translation"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_transform::components::Transform,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_transform_tree_changed_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_transform::components::TransformTreeChanged,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_transform::components::TransformTreeChanged>| {
                let output: Val<::bevy_transform::components::TransformTreeChanged> = {
                    {
                        let output: Val<
                            ::bevy_transform::components::TransformTreeChanged,
                        > = <::bevy_transform::components::TransformTreeChanged as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_transform::components::TransformTreeChanged>,
                other: Ref<::bevy_transform::components::TransformTreeChanged>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_transform::components::TransformTreeChanged as ::core::cmp::PartialEq<
                            ::bevy_transform::components::TransformTreeChanged,
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
            ::bevy_transform::components::TransformTreeChanged,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyTransformScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_global_transform_functions(&mut world);
        register_transform_functions(&mut world);
        register_transform_tree_changed_functions(&mut world);
    }
}
