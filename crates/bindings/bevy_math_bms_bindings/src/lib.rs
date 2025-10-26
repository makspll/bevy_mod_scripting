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
pub struct BevyMathScriptingPlugin;
pub(crate) fn register_aspect_ratio_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::AspectRatio,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::AspectRatio>| {
                let output: Val<::bevy_math::AspectRatio> = {
                    {
                        let output: Val<::bevy_math::AspectRatio> = <::bevy_math::AspectRatio as ::core::clone::Clone>::clone(
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
            |_self: Ref<::bevy_math::AspectRatio>, other: Ref<::bevy_math::AspectRatio>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::AspectRatio as ::core::cmp::PartialEq<
                            ::bevy_math::AspectRatio,
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
            "inverse",
            |_self: Ref<::bevy_math::AspectRatio>| {
                let output: Val<::bevy_math::AspectRatio> = {
                    {
                        let output: Val<::bevy_math::AspectRatio> = ::bevy_math::AspectRatio::inverse(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the inverse of this aspect ratio (height/width).",
            &["_self"],
        )
        .register_documented(
            "is_landscape",
            |_self: Ref<::bevy_math::AspectRatio>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::AspectRatio::is_landscape(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Returns true if the aspect ratio represents a landscape orientation.",
            &["_self"],
        )
        .register_documented(
            "is_portrait",
            |_self: Ref<::bevy_math::AspectRatio>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::AspectRatio::is_portrait(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Returns true if the aspect ratio represents a portrait orientation.",
            &["_self"],
        )
        .register_documented(
            "is_square",
            |_self: Ref<::bevy_math::AspectRatio>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::AspectRatio::is_square(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Returns true if the aspect ratio is exactly square.",
            &["_self"],
        )
        .register_documented(
            "ratio",
            |_self: Ref<::bevy_math::AspectRatio>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::AspectRatio::ratio(&_self).into();
                        output
                    }
                };
                output
            },
            " Returns the aspect ratio as a f32 value.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::AspectRatio,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_compass_octant_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::CompassOctant,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_math::CompassOctant>| {
                let output: () = {
                    {
                        let output: () = <::bevy_math::CompassOctant as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_math::CompassOctant>| {
                let output: Val<::bevy_math::CompassOctant> = {
                    {
                        let output: Val<::bevy_math::CompassOctant> = <::bevy_math::CompassOctant as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_math::CompassOctant>,
                other: Ref<::bevy_math::CompassOctant>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::CompassOctant as ::core::cmp::PartialEq<
                            ::bevy_math::CompassOctant,
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
            "neg",
            |_self: Val<::bevy_math::CompassOctant>| {
                let output: Val<::bevy_math::CompassOctant> = {
                    {
                        let output: Val<::bevy_math::CompassOctant> = <::bevy_math::CompassOctant as ::core::ops::Neg>::neg(
                                _self.into_inner(),
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
            "opposite",
            |_self: Ref<::bevy_math::CompassOctant>| {
                let output: Val<::bevy_math::CompassOctant> = {
                    {
                        let output: Val<::bevy_math::CompassOctant> = ::bevy_math::CompassOctant::opposite(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the opposite [`CompassOctant`], located 180 degrees from `self`.\n This can also be accessed via the `-` operator, using the [`Neg`] trait.",
            &["_self"],
        )
        .register_documented(
            "to_index",
            |_self: Val<::bevy_math::CompassOctant>| {
                let output: usize = {
                    {
                        let output: usize = ::bevy_math::CompassOctant::to_index(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Converts a [`CompassOctant`] to a standard index.\n Starts at 0 for [`CompassOctant::North`] and increments clockwise.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::CompassOctant,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_compass_quadrant_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::CompassQuadrant,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_math::CompassQuadrant>| {
                let output: () = {
                    {
                        let output: () = <::bevy_math::CompassQuadrant as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_math::CompassQuadrant>| {
                let output: Val<::bevy_math::CompassQuadrant> = {
                    {
                        let output: Val<::bevy_math::CompassQuadrant> = <::bevy_math::CompassQuadrant as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_math::CompassQuadrant>,
                other: Ref<::bevy_math::CompassQuadrant>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::CompassQuadrant as ::core::cmp::PartialEq<
                            ::bevy_math::CompassQuadrant,
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
            "neg",
            |_self: Val<::bevy_math::CompassQuadrant>| {
                let output: Val<::bevy_math::CompassQuadrant> = {
                    {
                        let output: Val<::bevy_math::CompassQuadrant> = <::bevy_math::CompassQuadrant as ::core::ops::Neg>::neg(
                                _self.into_inner(),
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
            "opposite",
            |_self: Ref<::bevy_math::CompassQuadrant>| {
                let output: Val<::bevy_math::CompassQuadrant> = {
                    {
                        let output: Val<::bevy_math::CompassQuadrant> = ::bevy_math::CompassQuadrant::opposite(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the opposite [`CompassQuadrant`], located 180 degrees from `self`.\n This can also be accessed via the `-` operator, using the [`Neg`] trait.",
            &["_self"],
        )
        .register_documented(
            "to_index",
            |_self: Val<::bevy_math::CompassQuadrant>| {
                let output: usize = {
                    {
                        let output: usize = ::bevy_math::CompassQuadrant::to_index(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Converts a [`CompassQuadrant`] to a standard index.\n Starts at 0 for [`CompassQuadrant::North`] and increments clockwise.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::CompassQuadrant,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_isometry_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::Isometry2d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::Isometry2d>| {
                let output: Val<::bevy_math::Isometry2d> = {
                    {
                        let output: Val<::bevy_math::Isometry2d> = <::bevy_math::Isometry2d as ::core::clone::Clone>::clone(
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
            |_self: Ref<::bevy_math::Isometry2d>, other: Ref<::bevy_math::Isometry2d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::Isometry2d as ::core::cmp::PartialEq<
                            ::bevy_math::Isometry2d,
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
            "from_rotation",
            |rotation: Val<::bevy_math::Rot2>| {
                let output: Val<::bevy_math::Isometry2d> = {
                    {
                        let output: Val<::bevy_math::Isometry2d> = ::bevy_math::Isometry2d::from_rotation(
                                rotation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a two-dimensional isometry from a rotation.",
            &["rotation"],
        )
        .register_documented(
            "from_translation",
            |translation: Val<::bevy_math::prelude::Vec2>| {
                let output: Val<::bevy_math::Isometry2d> = {
                    {
                        let output: Val<::bevy_math::Isometry2d> = ::bevy_math::Isometry2d::from_translation(
                                translation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a two-dimensional isometry from a translation.",
            &["translation"],
        )
        .register_documented(
            "from_xy",
            |x: f32, y: f32| {
                let output: Val<::bevy_math::Isometry2d> = {
                    {
                        let output: Val<::bevy_math::Isometry2d> = ::bevy_math::Isometry2d::from_xy(
                                x,
                                y,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a two-dimensional isometry from a translation with the given `x` and `y` components.",
            &["x", "y"],
        )
        .register_documented(
            "inverse",
            |_self: Ref<::bevy_math::Isometry2d>| {
                let output: Val<::bevy_math::Isometry2d> = {
                    {
                        let output: Val<::bevy_math::Isometry2d> = ::bevy_math::Isometry2d::inverse(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The inverse isometry that undoes this one.",
            &["_self"],
        )
        .register_documented(
            "inverse_mul",
            |_self: Ref<::bevy_math::Isometry2d>, rhs: Val<::bevy_math::Isometry2d>| {
                let output: Val<::bevy_math::Isometry2d> = {
                    {
                        let output: Val<::bevy_math::Isometry2d> = ::bevy_math::Isometry2d::inverse_mul(
                                &_self,
                                rhs.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute `iso1.inverse() * iso2` in a more efficient way for one-shot cases.\n If the same isometry is used multiple times, it is more efficient to instead compute\n the inverse once and use that for each transformation.",
            &["_self", "rhs"],
        )
        .register_documented(
            "inverse_transform_point",
            |
                _self: Ref<::bevy_math::Isometry2d>,
                point: Val<::bevy_math::prelude::Vec2>|
            {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::Isometry2d::inverse_transform_point(
                                &_self,
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Transform a point by rotating and translating it using the inverse of this isometry.\n This is more efficient than `iso.inverse().transform_point(point)` for one-shot cases.\n If the same isometry is used multiple times, it is more efficient to instead compute\n the inverse once and use that for each transformation.",
            &["_self", "point"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_math::Isometry2d>, rhs: Val<::bevy_math::Isometry2d>| {
                let output: Val<::bevy_math::Isometry2d> = {
                    {
                        let output: Val<::bevy_math::Isometry2d> = <::bevy_math::Isometry2d as ::core::ops::Mul<
                            ::bevy_math::Isometry2d,
                        >>::mul(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_math::Isometry2d>, rhs: Val<::bevy_math::prelude::Dir2>| {
                let output: Val<::bevy_math::prelude::Dir2> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir2> = <::bevy_math::Isometry2d as ::core::ops::Mul<
                            ::bevy_math::prelude::Dir2,
                        >>::mul(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_math::Isometry2d>, rhs: Val<::bevy_math::prelude::Vec2>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = <::bevy_math::Isometry2d as ::core::ops::Mul<
                            ::bevy_math::prelude::Vec2,
                        >>::mul(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "new",
            |
                translation: Val<::bevy_math::prelude::Vec2>,
                rotation: Val<::bevy_math::Rot2>|
            {
                let output: Val<::bevy_math::Isometry2d> = {
                    {
                        let output: Val<::bevy_math::Isometry2d> = ::bevy_math::Isometry2d::new(
                                translation.into_inner(),
                                rotation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a two-dimensional isometry from a rotation and a translation.",
            &["translation", "rotation"],
        )
        .register_documented(
            "transform_point",
            |
                _self: Ref<::bevy_math::Isometry2d>,
                point: Val<::bevy_math::prelude::Vec2>|
            {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::Isometry2d::transform_point(
                                &_self,
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Transform a point by rotating and translating it using this isometry.",
            &["_self", "point"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::Isometry2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_isometry_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::Isometry3d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::Isometry3d>| {
                let output: Val<::bevy_math::Isometry3d> = {
                    {
                        let output: Val<::bevy_math::Isometry3d> = <::bevy_math::Isometry3d as ::core::clone::Clone>::clone(
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
            |_self: Ref<::bevy_math::Isometry3d>, other: Ref<::bevy_math::Isometry3d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::Isometry3d as ::core::cmp::PartialEq<
                            ::bevy_math::Isometry3d,
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
            "from_rotation",
            |rotation: Val<::bevy_math::prelude::Quat>| {
                let output: Val<::bevy_math::Isometry3d> = {
                    {
                        let output: Val<::bevy_math::Isometry3d> = ::bevy_math::Isometry3d::from_rotation(
                                rotation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a three-dimensional isometry from a rotation.",
            &["rotation"],
        )
        .register_documented(
            "from_xyz",
            |x: f32, y: f32, z: f32| {
                let output: Val<::bevy_math::Isometry3d> = {
                    {
                        let output: Val<::bevy_math::Isometry3d> = ::bevy_math::Isometry3d::from_xyz(
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
            " Create a three-dimensional isometry from a translation with the given `x`, `y`, and `z` components.",
            &["x", "y", "z"],
        )
        .register_documented(
            "inverse",
            |_self: Ref<::bevy_math::Isometry3d>| {
                let output: Val<::bevy_math::Isometry3d> = {
                    {
                        let output: Val<::bevy_math::Isometry3d> = ::bevy_math::Isometry3d::inverse(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The inverse isometry that undoes this one.",
            &["_self"],
        )
        .register_documented(
            "inverse_mul",
            |_self: Ref<::bevy_math::Isometry3d>, rhs: Val<::bevy_math::Isometry3d>| {
                let output: Val<::bevy_math::Isometry3d> = {
                    {
                        let output: Val<::bevy_math::Isometry3d> = ::bevy_math::Isometry3d::inverse_mul(
                                &_self,
                                rhs.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute `iso1.inverse() * iso2` in a more efficient way for one-shot cases.\n If the same isometry is used multiple times, it is more efficient to instead compute\n the inverse once and use that for each transformation.",
            &["_self", "rhs"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_math::Isometry3d>, rhs: Val<::bevy_math::Isometry3d>| {
                let output: Val<::bevy_math::Isometry3d> = {
                    {
                        let output: Val<::bevy_math::Isometry3d> = <::bevy_math::Isometry3d as ::core::ops::Mul<
                            ::bevy_math::Isometry3d,
                        >>::mul(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_math::Isometry3d>, rhs: Val<::bevy_math::prelude::Dir3>| {
                let output: Val<::bevy_math::prelude::Dir3> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir3> = <::bevy_math::Isometry3d as ::core::ops::Mul<
                            ::bevy_math::prelude::Dir3,
                        >>::mul(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_math::Isometry3d>, rhs: Val<::bevy_math::prelude::Vec3>| {
                let output: Val<::bevy_math::prelude::Vec3> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec3> = <::bevy_math::Isometry3d as ::core::ops::Mul<
                            ::bevy_math::prelude::Vec3,
                        >>::mul(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_math::Isometry3d>, rhs: Val<::bevy_math::prelude::Vec3A>| {
                let output: Val<::bevy_math::prelude::Vec3A> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec3A> = <::bevy_math::Isometry3d as ::core::ops::Mul<
                            ::bevy_math::prelude::Vec3A,
                        >>::mul(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::Isometry3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ray_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<::bevy_math::Ray2d>::new(
        world,
    )
    .register_documented(
        "clone",
        |_self: Ref<::bevy_math::Ray2d>| {
            let output: Val<::bevy_math::Ray2d> = {
                {
                    let output: Val<::bevy_math::Ray2d> =
                        <::bevy_math::Ray2d as ::core::clone::Clone>::clone(&_self).into();
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
        |_self: Ref<::bevy_math::Ray2d>, other: Ref<::bevy_math::Ray2d>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_math::Ray2d as ::core::cmp::PartialEq<
                        ::bevy_math::Ray2d,
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
        "get_point",
        |_self: Ref<::bevy_math::Ray2d>, distance: f32| {
            let output: Val<::bevy_math::prelude::Vec2> = {
                {
                    let output: Val<::bevy_math::prelude::Vec2> =
                        ::bevy_math::Ray2d::get_point(&_self, distance).into();
                    output
                }
            };
            output
        },
        " Get a point at a given distance along the ray",
        &["_self", "distance"],
    )
    .register_documented(
        "intersect_plane",
        |_self: Ref<::bevy_math::Ray2d>,
         plane_origin: Val<::bevy_math::prelude::Vec2>,
         plane: Val<::bevy_math::primitives::Plane2d>| {
            let output: ::core::option::Option<f32> = {
                {
                    let output: ::core::option::Option<f32> = ::bevy_math::Ray2d::intersect_plane(
                        &_self,
                        plane_origin.into_inner(),
                        plane.into_inner(),
                    )
                    .into();
                    output
                }
            };
            output
        },
        " Get the distance to a plane if the ray intersects it",
        &["_self", "plane_origin", "plane"],
    )
    .register_documented(
        "new",
        |origin: Val<::bevy_math::prelude::Vec2>, direction: Val<::bevy_math::prelude::Dir2>| {
            let output: Val<::bevy_math::Ray2d> = {
                {
                    let output: Val<::bevy_math::Ray2d> =
                        ::bevy_math::Ray2d::new(origin.into_inner(), direction.into_inner()).into();
                    output
                }
            };
            output
        },
        " Create a new `Ray2d` from a given origin and direction",
        &["origin", "direction"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_math::Ray2d, bevy_mod_scripting_bindings::MarkAsGenerated>();
}
pub(crate) fn register_ray_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<::bevy_math::Ray3d>::new(
        world,
    )
    .register_documented(
        "clone",
        |_self: Ref<::bevy_math::Ray3d>| {
            let output: Val<::bevy_math::Ray3d> = {
                {
                    let output: Val<::bevy_math::Ray3d> =
                        <::bevy_math::Ray3d as ::core::clone::Clone>::clone(&_self).into();
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
        |_self: Ref<::bevy_math::Ray3d>, other: Ref<::bevy_math::Ray3d>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_math::Ray3d as ::core::cmp::PartialEq<
                        ::bevy_math::Ray3d,
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
        "get_point",
        |_self: Ref<::bevy_math::Ray3d>, distance: f32| {
            let output: Val<::bevy_math::prelude::Vec3> = {
                {
                    let output: Val<::bevy_math::prelude::Vec3> =
                        ::bevy_math::Ray3d::get_point(&_self, distance).into();
                    output
                }
            };
            output
        },
        " Get a point at a given distance along the ray",
        &["_self", "distance"],
    )
    .register_documented(
        "intersect_plane",
        |_self: Ref<::bevy_math::Ray3d>,
         plane_origin: Val<::bevy_math::prelude::Vec3>,
         plane: Val<::bevy_math::primitives::InfinitePlane3d>| {
            let output: ::core::option::Option<f32> = {
                {
                    let output: ::core::option::Option<f32> = ::bevy_math::Ray3d::intersect_plane(
                        &_self,
                        plane_origin.into_inner(),
                        plane.into_inner(),
                    )
                    .into();
                    output
                }
            };
            output
        },
        " Get the distance to a plane if the ray intersects it",
        &["_self", "plane_origin", "plane"],
    )
    .register_documented(
        "new",
        |origin: Val<::bevy_math::prelude::Vec3>, direction: Val<::bevy_math::prelude::Dir3>| {
            let output: Val<::bevy_math::Ray3d> = {
                {
                    let output: Val<::bevy_math::Ray3d> =
                        ::bevy_math::Ray3d::new(origin.into_inner(), direction.into_inner()).into();
                    output
                }
            };
            output
        },
        " Create a new `Ray3d` from a given origin and direction",
        &["origin", "direction"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_math::Ray3d, bevy_mod_scripting_bindings::MarkAsGenerated>();
}
pub(crate) fn register_rot_2_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::Rot2,
    >::new(world)
        .register_documented(
            "angle_to",
            |_self: Val<::bevy_math::Rot2>, other: Val<::bevy_math::Rot2>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::Rot2::angle_to(
                                _self.into_inner(),
                                other.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the angle in radians needed to make `self` and `other` coincide.",
            &["_self", "other"],
        )
        .register_documented(
            "as_degrees",
            |_self: Val<::bevy_math::Rot2>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::Rot2::as_degrees(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the rotation in degrees in the `(-180, 180]` range.",
            &["_self"],
        )
        .register_documented(
            "as_radians",
            |_self: Val<::bevy_math::Rot2>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::Rot2::as_radians(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the rotation in radians in the `(-pi, pi]` range.",
            &["_self"],
        )
        .register_documented(
            "as_turn_fraction",
            |_self: Val<::bevy_math::Rot2>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::Rot2::as_turn_fraction(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the rotation as a fraction of a full 360 degree turn.",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::Rot2>| {
                let output: Val<::bevy_math::Rot2> = {
                    {
                        let output: Val<::bevy_math::Rot2> = <::bevy_math::Rot2 as ::core::clone::Clone>::clone(
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
            "degrees",
            |degrees: f32| {
                let output: Val<::bevy_math::Rot2> = {
                    {
                        let output: Val<::bevy_math::Rot2> = ::bevy_math::Rot2::degrees(
                                degrees,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a [`Rot2`] from a counterclockwise angle in degrees.\n # Note\n The input rotation will always be clamped to the range `(-180°, 180°]` by design.\n # Example\n ```\n # use bevy_math::Rot2;\n # use approx::assert_relative_eq;\n let rot1 = Rot2::degrees(270.0);\n let rot2 = Rot2::degrees(-90.0);\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(rot1, rot2);\n let rot3 = Rot2::degrees(180.0);\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(rot1 * rot1, rot3);\n ```",
            &["degrees"],
        )
        .register_documented(
            "eq",
            |_self: Ref<::bevy_math::Rot2>, other: Ref<::bevy_math::Rot2>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::Rot2 as ::core::cmp::PartialEq<
                            ::bevy_math::Rot2,
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
            "fast_renormalize",
            |_self: Val<::bevy_math::Rot2>| {
                let output: Val<::bevy_math::Rot2> = {
                    {
                        let output: Val<::bevy_math::Rot2> = ::bevy_math::Rot2::fast_renormalize(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `self` after an approximate normalization, assuming the value is already nearly normalized.\n Useful for preventing numerical error accumulation.\n See [`Dir3::fast_renormalize`](crate::Dir3::fast_renormalize) for an example of when such error accumulation might occur.",
            &["_self"],
        )
        .register_documented(
            "from_sin_cos",
            |sin: f32, cos: f32| {
                let output: Val<::bevy_math::Rot2> = {
                    {
                        let output: Val<::bevy_math::Rot2> = ::bevy_math::Rot2::from_sin_cos(
                                sin,
                                cos,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a [`Rot2`] from the sine and cosine of an angle in radians.\n The rotation is only valid if `sin * sin + cos * cos == 1.0`.\n # Panics\n Panics if `sin * sin + cos * cos != 1.0` when the `glam_assert` feature is enabled.",
            &["sin", "cos"],
        )
        .register_documented(
            "inverse",
            |_self: Val<::bevy_math::Rot2>| {
                let output: Val<::bevy_math::Rot2> = {
                    {
                        let output: Val<::bevy_math::Rot2> = ::bevy_math::Rot2::inverse(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the inverse of the rotation. This is also the conjugate\n of the unit complex number representing the rotation.",
            &["_self"],
        )
        .register_documented(
            "is_finite",
            |_self: Val<::bevy_math::Rot2>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::Rot2::is_finite(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if the rotation is neither infinite nor NaN.",
            &["_self"],
        )
        .register_documented(
            "is_nan",
            |_self: Val<::bevy_math::Rot2>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::Rot2::is_nan(_self.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if the rotation is NaN.",
            &["_self"],
        )
        .register_documented(
            "is_near_identity",
            |_self: Val<::bevy_math::Rot2>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::Rot2::is_near_identity(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if the rotation is near [`Rot2::IDENTITY`].",
            &["_self"],
        )
        .register_documented(
            "is_normalized",
            |_self: Val<::bevy_math::Rot2>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::Rot2::is_normalized(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns whether `self` has a length of `1.0` or not.\n Uses a precision threshold of approximately `1e-4`.",
            &["_self"],
        )
        .register_documented(
            "length",
            |_self: Val<::bevy_math::Rot2>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::Rot2::length(_self.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            " Computes the length or norm of the complex number used to represent the rotation.\n The length is typically expected to be `1.0`. Unexpectedly denormalized rotations\n can be a result of incorrect construction or floating point error caused by\n successive operations.",
            &["_self"],
        )
        .register_documented(
            "length_recip",
            |_self: Val<::bevy_math::Rot2>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::Rot2::length_recip(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Computes `1.0 / self.length()`.\n For valid results, `self` must _not_ have a length of zero.",
            &["_self"],
        )
        .register_documented(
            "length_squared",
            |_self: Val<::bevy_math::Rot2>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::Rot2::length_squared(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Computes the squared length or norm of the complex number used to represent the rotation.\n This is generally faster than [`Rot2::length()`], as it avoids a square\n root operation.\n The length is typically expected to be `1.0`. Unexpectedly denormalized rotations\n can be a result of incorrect construction or floating point error caused by\n successive operations.",
            &["_self"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_math::Rot2>, rhs: Val<::bevy_math::Rot2>| {
                let output: Val<::bevy_math::Rot2> = {
                    {
                        let output: Val<::bevy_math::Rot2> = <::bevy_math::Rot2 as ::core::ops::Mul<
                            ::bevy_math::Rot2,
                        >>::mul(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_math::Rot2>, direction: Val<::bevy_math::prelude::Dir2>| {
                let output: Val<::bevy_math::prelude::Dir2> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir2> = <::bevy_math::Rot2 as ::core::ops::Mul<
                            ::bevy_math::prelude::Dir2,
                        >>::mul(_self.into_inner(), direction.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            " Rotates the [`Dir2`] using a [`Rot2`].",
            &["_self", "direction"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_math::Rot2>, rhs: Val<::bevy_math::prelude::Vec2>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = <::bevy_math::Rot2 as ::core::ops::Mul<
                            ::bevy_math::prelude::Vec2,
                        >>::mul(_self.into_inner(), rhs.into_inner())
                            .into();
                        output
                    }
                };
                output
            },
            " Rotates a [`Vec2`] by a [`Rot2`].",
            &["_self", "rhs"],
        )
        .register_documented(
            "nlerp",
            |_self: Val<::bevy_math::Rot2>, end: Val<::bevy_math::Rot2>, s: f32| {
                let output: Val<::bevy_math::Rot2> = {
                    {
                        let output: Val<::bevy_math::Rot2> = ::bevy_math::Rot2::nlerp(
                                _self.into_inner(),
                                end.into_inner(),
                                s,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Performs a linear interpolation between `self` and `rhs` based on\n the value `s`, and normalizes the rotation afterwards.\n When `s == 0.0`, the result will be equal to `self`.\n When `s == 1.0`, the result will be equal to `rhs`.\n This is slightly more efficient than [`slerp`](Self::slerp), and produces a similar result\n when the difference between the two rotations is small. At larger differences,\n the result resembles a kind of ease-in-out effect.\n If you would like the angular velocity to remain constant, consider using [`slerp`](Self::slerp) instead.\n # Details\n `nlerp` corresponds to computing an angle for a point at position `s` on a line drawn\n between the endpoints of the arc formed by `self` and `rhs` on a unit circle,\n and normalizing the result afterwards.\n Note that if the angles are opposite like 0 and π, the line will pass through the origin,\n and the resulting angle will always be either `self` or `rhs` depending on `s`.\n If `s` happens to be `0.5` in this case, a valid rotation cannot be computed, and `self`\n will be returned as a fallback.\n # Example\n ```\n # use bevy_math::Rot2;\n #\n let rot1 = Rot2::IDENTITY;\n let rot2 = Rot2::degrees(135.0);\n let result1 = rot1.nlerp(rot2, 1.0 / 3.0);\n assert_eq!(result1.as_degrees(), 28.675055);\n let result2 = rot1.nlerp(rot2, 0.5);\n assert_eq!(result2.as_degrees(), 67.5);\n ```",
            &["_self", "end", "s"],
        )
        .register_documented(
            "normalize",
            |_self: Val<::bevy_math::Rot2>| {
                let output: Val<::bevy_math::Rot2> = {
                    {
                        let output: Val<::bevy_math::Rot2> = ::bevy_math::Rot2::normalize(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `self` with a length of `1.0`.\n Note that [`Rot2`] should typically already be normalized by design.\n Manual normalization is only needed when successive operations result in\n accumulated floating point error, or if the rotation was constructed\n with invalid values.\n # Panics\n Panics if `self` has a length of zero, NaN, or infinity when debug assertions are enabled.",
            &["_self"],
        )
        .register_documented(
            "radians",
            |radians: f32| {
                let output: Val<::bevy_math::Rot2> = {
                    {
                        let output: Val<::bevy_math::Rot2> = ::bevy_math::Rot2::radians(
                                radians,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a [`Rot2`] from a counterclockwise angle in radians.\n # Note\n The input rotation will always be clamped to the range `(-π, π]` by design.\n # Example\n ```\n # use bevy_math::Rot2;\n # use approx::assert_relative_eq;\n # use std::f32::consts::{FRAC_PI_2, PI};\n let rot1 = Rot2::radians(3.0 * FRAC_PI_2);\n let rot2 = Rot2::radians(-FRAC_PI_2);\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(rot1, rot2);\n let rot3 = Rot2::radians(PI);\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(rot1 * rot1, rot3);\n ```",
            &["radians"],
        )
        .register_documented(
            "sin_cos",
            |_self: Val<::bevy_math::Rot2>| {
                let output: (f32, f32) = {
                    {
                        let output: (f32, f32) = ::bevy_math::Rot2::sin_cos(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the sine and cosine of the rotation angle in radians.",
            &["_self"],
        )
        .register_documented(
            "slerp",
            |_self: Val<::bevy_math::Rot2>, end: Val<::bevy_math::Rot2>, s: f32| {
                let output: Val<::bevy_math::Rot2> = {
                    {
                        let output: Val<::bevy_math::Rot2> = ::bevy_math::Rot2::slerp(
                                _self.into_inner(),
                                end.into_inner(),
                                s,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Performs a spherical linear interpolation between `self` and `end`\n based on the value `s`.\n This corresponds to interpolating between the two angles at a constant angular velocity.\n When `s == 0.0`, the result will be equal to `self`.\n When `s == 1.0`, the result will be equal to `rhs`.\n If you would like the rotation to have a kind of ease-in-out effect, consider\n using the slightly more efficient [`nlerp`](Self::nlerp) instead.\n # Example\n ```\n # use bevy_math::Rot2;\n #\n let rot1 = Rot2::IDENTITY;\n let rot2 = Rot2::degrees(135.0);\n let result1 = rot1.slerp(rot2, 1.0 / 3.0);\n assert_eq!(result1.as_degrees(), 45.0);\n let result2 = rot1.slerp(rot2, 0.5);\n assert_eq!(result2.as_degrees(), 67.5);\n ```",
            &["_self", "end", "s"],
        )
        .register_documented(
            "turn_fraction",
            |fraction: f32| {
                let output: Val<::bevy_math::Rot2> = {
                    {
                        let output: Val<::bevy_math::Rot2> = ::bevy_math::Rot2::turn_fraction(
                                fraction,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a [`Rot2`] from a counterclockwise fraction of a full turn of 360 degrees.\n # Note\n The input rotation will always be clamped to the range `(-50%, 50%]` by design.\n # Example\n ```\n # use bevy_math::Rot2;\n # use approx::assert_relative_eq;\n let rot1 = Rot2::turn_fraction(0.75);\n let rot2 = Rot2::turn_fraction(-0.25);\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(rot1, rot2);\n let rot3 = Rot2::turn_fraction(0.5);\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(rot1 * rot1, rot3);\n ```",
            &["fraction"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_math::Rot2, bevy_mod_scripting_bindings::MarkAsGenerated>();
}
pub(crate) fn register_dir_2_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::prelude::Dir2,
    >::new(world)
        .register_documented(
            "as_vec2",
            |_self: Ref<::bevy_math::prelude::Dir2>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::prelude::Dir2::as_vec2(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the inner [`Vec2`]",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::prelude::Dir2>| {
                let output: Val<::bevy_math::prelude::Dir2> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir2> = <::bevy_math::prelude::Dir2 as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_math::prelude::Dir2>,
                other: Ref<::bevy_math::prelude::Dir2>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::prelude::Dir2 as ::core::cmp::PartialEq<
                            ::bevy_math::prelude::Dir2,
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
            "fast_renormalize",
            |_self: Val<::bevy_math::prelude::Dir2>| {
                let output: Val<::bevy_math::prelude::Dir2> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir2> = ::bevy_math::prelude::Dir2::fast_renormalize(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `self` after an approximate normalization, assuming the value is already nearly normalized.\n Useful for preventing numerical error accumulation.\n See [`Dir3::fast_renormalize`] for an example of when such error accumulation might occur.",
            &["_self"],
        )
        .register_documented(
            "from_xy_unchecked",
            |x: f32, y: f32| {
                let output: Val<::bevy_math::prelude::Dir2> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir2> = ::bevy_math::prelude::Dir2::from_xy_unchecked(
                                x,
                                y,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a direction from its `x` and `y` components, assuming the resulting vector is normalized.\n # Warning\n The vector produced from `x` and `y` must be normalized, i.e its length must be `1.0`.",
            &["x", "y"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_math::prelude::Dir2>, rhs: f32| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = <::bevy_math::prelude::Dir2 as ::core::ops::Mul<
                            f32,
                        >>::mul(_self.into_inner(), rhs)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "neg",
            |_self: Val<::bevy_math::prelude::Dir2>| {
                let output: Val<::bevy_math::prelude::Dir2> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir2> = <::bevy_math::prelude::Dir2 as ::core::ops::Neg>::neg(
                                _self.into_inner(),
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
            "new_unchecked",
            |value: Val<::bevy_math::prelude::Vec2>| {
                let output: Val<::bevy_math::prelude::Dir2> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir2> = ::bevy_math::prelude::Dir2::new_unchecked(
                                value.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a [`Dir2`] from a [`Vec2`] that is already normalized.\n # Warning\n `value` must be normalized, i.e its length must be `1.0`.",
            &["value"],
        )
        .register_documented(
            "rotation_from",
            |
                _self: Val<::bevy_math::prelude::Dir2>,
                other: Val<::bevy_math::prelude::Dir2>|
            {
                let output: Val<::bevy_math::Rot2> = {
                    {
                        let output: Val<::bevy_math::Rot2> = ::bevy_math::prelude::Dir2::rotation_from(
                                _self.into_inner(),
                                other.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the rotation that rotates `other` to this direction.",
            &["_self", "other"],
        )
        .register_documented(
            "rotation_from_x",
            |_self: Val<::bevy_math::prelude::Dir2>| {
                let output: Val<::bevy_math::Rot2> = {
                    {
                        let output: Val<::bevy_math::Rot2> = ::bevy_math::prelude::Dir2::rotation_from_x(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the rotation that rotates the X-axis to this direction.",
            &["_self"],
        )
        .register_documented(
            "rotation_from_y",
            |_self: Val<::bevy_math::prelude::Dir2>| {
                let output: Val<::bevy_math::Rot2> = {
                    {
                        let output: Val<::bevy_math::Rot2> = ::bevy_math::prelude::Dir2::rotation_from_y(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the rotation that rotates the Y-axis to this direction.",
            &["_self"],
        )
        .register_documented(
            "rotation_to",
            |
                _self: Val<::bevy_math::prelude::Dir2>,
                other: Val<::bevy_math::prelude::Dir2>|
            {
                let output: Val<::bevy_math::Rot2> = {
                    {
                        let output: Val<::bevy_math::Rot2> = ::bevy_math::prelude::Dir2::rotation_to(
                                _self.into_inner(),
                                other.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the rotation that rotates this direction to `other`.",
            &["_self", "other"],
        )
        .register_documented(
            "rotation_to_x",
            |_self: Val<::bevy_math::prelude::Dir2>| {
                let output: Val<::bevy_math::Rot2> = {
                    {
                        let output: Val<::bevy_math::Rot2> = ::bevy_math::prelude::Dir2::rotation_to_x(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the rotation that rotates this direction to the X-axis.",
            &["_self"],
        )
        .register_documented(
            "rotation_to_y",
            |_self: Val<::bevy_math::prelude::Dir2>| {
                let output: Val<::bevy_math::Rot2> = {
                    {
                        let output: Val<::bevy_math::Rot2> = ::bevy_math::prelude::Dir2::rotation_to_y(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the rotation that rotates this direction to the Y-axis.",
            &["_self"],
        )
        .register_documented(
            "slerp",
            |
                _self: Val<::bevy_math::prelude::Dir2>,
                rhs: Val<::bevy_math::prelude::Dir2>,
                s: f32|
            {
                let output: Val<::bevy_math::prelude::Dir2> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir2> = ::bevy_math::prelude::Dir2::slerp(
                                _self.into_inner(),
                                rhs.into_inner(),
                                s,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Performs a spherical linear interpolation between `self` and `rhs`\n based on the value `s`.\n This corresponds to interpolating between the two directions at a constant angular velocity.\n When `s == 0.0`, the result will be equal to `self`.\n When `s == 1.0`, the result will be equal to `rhs`.\n # Example\n ```\n # use bevy_math::Dir2;\n # use approx::{assert_relative_eq, RelativeEq};\n #\n let dir1 = Dir2::X;\n let dir2 = Dir2::Y;\n let result1 = dir1.slerp(dir2, 1.0 / 3.0);\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(result1, Dir2::from_xy(0.75_f32.sqrt(), 0.5).unwrap());\n let result2 = dir1.slerp(dir2, 0.5);\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(result2, Dir2::from_xy(0.5_f32.sqrt(), 0.5_f32.sqrt()).unwrap());\n ```",
            &["_self", "rhs", "s"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::prelude::Dir2,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_dir_3_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::prelude::Dir3,
    >::new(world)
        .register_documented(
            "as_vec3",
            |_self: Ref<::bevy_math::prelude::Dir3>| {
                let output: Val<::bevy_math::prelude::Vec3> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec3> = ::bevy_math::prelude::Dir3::as_vec3(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the inner [`Vec3`]",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::prelude::Dir3>| {
                let output: Val<::bevy_math::prelude::Dir3> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir3> = <::bevy_math::prelude::Dir3 as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_math::prelude::Dir3>,
                other: Ref<::bevy_math::prelude::Dir3>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::prelude::Dir3 as ::core::cmp::PartialEq<
                            ::bevy_math::prelude::Dir3,
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
            "fast_renormalize",
            |_self: Val<::bevy_math::prelude::Dir3>| {
                let output: Val<::bevy_math::prelude::Dir3> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir3> = ::bevy_math::prelude::Dir3::fast_renormalize(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `self` after an approximate normalization, assuming the value is already nearly normalized.\n Useful for preventing numerical error accumulation.\n # Example\n The following seemingly benign code would start accumulating errors over time,\n leading to `dir` eventually not being normalized anymore.\n ```\n # use bevy_math::prelude::*;\n # let N: usize = 200;\n let mut dir = Dir3::X;\n let quaternion = Quat::from_euler(EulerRot::XYZ, 1.0, 2.0, 3.0);\n for i in 0..N {\n     dir = quaternion * dir;\n }\n ```\n Instead, do the following.\n ```\n # use bevy_math::prelude::*;\n # let N: usize = 200;\n let mut dir = Dir3::X;\n let quaternion = Quat::from_euler(EulerRot::XYZ, 1.0, 2.0, 3.0);\n for i in 0..N {\n     dir = quaternion * dir;\n     dir = dir.fast_renormalize();\n }\n ```",
            &["_self"],
        )
        .register_documented(
            "from_xyz_unchecked",
            |x: f32, y: f32, z: f32| {
                let output: Val<::bevy_math::prelude::Dir3> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir3> = ::bevy_math::prelude::Dir3::from_xyz_unchecked(
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
            " Create a direction from its `x`, `y`, and `z` components, assuming the resulting vector is normalized.\n # Warning\n The vector produced from `x`, `y`, and `z` must be normalized, i.e its length must be `1.0`.",
            &["x", "y", "z"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_math::prelude::Dir3>, rhs: f32| {
                let output: Val<::bevy_math::prelude::Vec3> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec3> = <::bevy_math::prelude::Dir3 as ::core::ops::Mul<
                            f32,
                        >>::mul(_self.into_inner(), rhs)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "neg",
            |_self: Val<::bevy_math::prelude::Dir3>| {
                let output: Val<::bevy_math::prelude::Dir3> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir3> = <::bevy_math::prelude::Dir3 as ::core::ops::Neg>::neg(
                                _self.into_inner(),
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
            "new_unchecked",
            |value: Val<::bevy_math::prelude::Vec3>| {
                let output: Val<::bevy_math::prelude::Dir3> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir3> = ::bevy_math::prelude::Dir3::new_unchecked(
                                value.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a [`Dir3`] from a [`Vec3`] that is already normalized.\n # Warning\n `value` must be normalized, i.e its length must be `1.0`.",
            &["value"],
        )
        .register_documented(
            "slerp",
            |
                _self: Val<::bevy_math::prelude::Dir3>,
                rhs: Val<::bevy_math::prelude::Dir3>,
                s: f32|
            {
                let output: Val<::bevy_math::prelude::Dir3> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir3> = ::bevy_math::prelude::Dir3::slerp(
                                _self.into_inner(),
                                rhs.into_inner(),
                                s,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Performs a spherical linear interpolation between `self` and `rhs`\n based on the value `s`.\n This corresponds to interpolating between the two directions at a constant angular velocity.\n When `s == 0.0`, the result will be equal to `self`.\n When `s == 1.0`, the result will be equal to `rhs`.\n # Example\n ```\n # use bevy_math::Dir3;\n # use approx::{assert_relative_eq, RelativeEq};\n #\n let dir1 = Dir3::X;\n let dir2 = Dir3::Y;\n let result1 = dir1.slerp(dir2, 1.0 / 3.0);\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(\n     result1,\n     Dir3::from_xyz(0.75_f32.sqrt(), 0.5, 0.0).unwrap(),\n     epsilon = 0.000001\n );\n let result2 = dir1.slerp(dir2, 0.5);\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(result2, Dir3::from_xyz(0.5_f32.sqrt(), 0.5_f32.sqrt(), 0.0).unwrap());\n ```",
            &["_self", "rhs", "s"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::prelude::Dir3,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_dir_3_a_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::prelude::Dir3A,
    >::new(world)
        .register_documented(
            "as_vec3a",
            |_self: Ref<::bevy_math::prelude::Dir3A>| {
                let output: Val<::bevy_math::prelude::Vec3A> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec3A> = ::bevy_math::prelude::Dir3A::as_vec3a(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the inner [`Vec3A`]",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::prelude::Dir3A>| {
                let output: Val<::bevy_math::prelude::Dir3A> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir3A> = <::bevy_math::prelude::Dir3A as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_math::prelude::Dir3A>,
                other: Ref<::bevy_math::prelude::Dir3A>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::prelude::Dir3A as ::core::cmp::PartialEq<
                            ::bevy_math::prelude::Dir3A,
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
            "fast_renormalize",
            |_self: Val<::bevy_math::prelude::Dir3A>| {
                let output: Val<::bevy_math::prelude::Dir3A> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir3A> = ::bevy_math::prelude::Dir3A::fast_renormalize(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `self` after an approximate normalization, assuming the value is already nearly normalized.\n Useful for preventing numerical error accumulation.\n See [`Dir3::fast_renormalize`] for an example of when such error accumulation might occur.",
            &["_self"],
        )
        .register_documented(
            "from_xyz_unchecked",
            |x: f32, y: f32, z: f32| {
                let output: Val<::bevy_math::prelude::Dir3A> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir3A> = ::bevy_math::prelude::Dir3A::from_xyz_unchecked(
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
            " Create a direction from its `x`, `y`, and `z` components, assuming the resulting vector is normalized.\n # Warning\n The vector produced from `x`, `y`, and `z` must be normalized, i.e its length must be `1.0`.",
            &["x", "y", "z"],
        )
        .register_documented(
            "mul",
            |_self: Val<::bevy_math::prelude::Dir3A>, rhs: f32| {
                let output: Val<::bevy_math::prelude::Vec3A> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec3A> = <::bevy_math::prelude::Dir3A as ::core::ops::Mul<
                            f32,
                        >>::mul(_self.into_inner(), rhs)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "rhs"],
        )
        .register_documented(
            "neg",
            |_self: Val<::bevy_math::prelude::Dir3A>| {
                let output: Val<::bevy_math::prelude::Dir3A> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir3A> = <::bevy_math::prelude::Dir3A as ::core::ops::Neg>::neg(
                                _self.into_inner(),
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
            "new_unchecked",
            |value: Val<::bevy_math::prelude::Vec3A>| {
                let output: Val<::bevy_math::prelude::Dir3A> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir3A> = ::bevy_math::prelude::Dir3A::new_unchecked(
                                value.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a [`Dir3A`] from a [`Vec3A`] that is already normalized.\n # Warning\n `value` must be normalized, i.e its length must be `1.0`.",
            &["value"],
        )
        .register_documented(
            "slerp",
            |
                _self: Val<::bevy_math::prelude::Dir3A>,
                rhs: Val<::bevy_math::prelude::Dir3A>,
                s: f32|
            {
                let output: Val<::bevy_math::prelude::Dir3A> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir3A> = ::bevy_math::prelude::Dir3A::slerp(
                                _self.into_inner(),
                                rhs.into_inner(),
                                s,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Performs a spherical linear interpolation between `self` and `rhs`\n based on the value `s`.\n This corresponds to interpolating between the two directions at a constant angular velocity.\n When `s == 0.0`, the result will be equal to `self`.\n When `s == 1.0`, the result will be equal to `rhs`.\n # Example\n ```\n # use bevy_math::Dir3A;\n # use approx::{assert_relative_eq, RelativeEq};\n #\n let dir1 = Dir3A::X;\n let dir2 = Dir3A::Y;\n let result1 = dir1.slerp(dir2, 1.0 / 3.0);\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(\n     result1,\n     Dir3A::from_xyz(0.75_f32.sqrt(), 0.5, 0.0).unwrap(),\n     epsilon = 0.000001\n );\n let result2 = dir1.slerp(dir2, 0.5);\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(result2, Dir3A::from_xyz(0.5_f32.sqrt(), 0.5_f32.sqrt(), 0.0).unwrap());\n ```",
            &["_self", "rhs", "s"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::prelude::Dir3A,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_i_rect_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::prelude::IRect,
    >::new(world)
        .register_documented(
            "as_rect",
            |_self: Ref<::bevy_math::prelude::IRect>| {
                let output: Val<::bevy_math::prelude::Rect> = {
                    {
                        let output: Val<::bevy_math::prelude::Rect> = ::bevy_math::prelude::IRect::as_rect(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns self as [`Rect`] (f32)",
            &["_self"],
        )
        .register_documented(
            "as_urect",
            |_self: Ref<::bevy_math::prelude::IRect>| {
                let output: Val<::bevy_math::prelude::URect> = {
                    {
                        let output: Val<::bevy_math::prelude::URect> = ::bevy_math::prelude::IRect::as_urect(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns self as [`URect`] (u32)",
            &["_self"],
        )
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_math::prelude::IRect>| {
                let output: () = {
                    {
                        let output: () = <::bevy_math::prelude::IRect as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
            "center",
            |_self: Ref<::bevy_math::prelude::IRect>| {
                let output: Val<::bevy_math::prelude::IVec2> = {
                    {
                        let output: Val<::bevy_math::prelude::IVec2> = ::bevy_math::prelude::IRect::center(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The center point of the rectangle.\n # Rounding Behavior\n If the (min + max) contains odd numbers they will be rounded down to the nearest whole number when calculating the center.\n # Examples\n ```\n # use bevy_math::{IRect, IVec2};\n let r = IRect::new(0, 0, 5, 2); // w=5 h=2\n assert_eq!(r.center(), IVec2::new(2, 1));\n ```",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::prelude::IRect>| {
                let output: Val<::bevy_math::prelude::IRect> = {
                    {
                        let output: Val<::bevy_math::prelude::IRect> = <::bevy_math::prelude::IRect as ::core::clone::Clone>::clone(
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
            "contains",
            |
                _self: Ref<::bevy_math::prelude::IRect>,
                point: Val<::bevy_math::prelude::IVec2>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::prelude::IRect::contains(
                                &_self,
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Check if a point lies within this rectangle, inclusive of its edges.\n # Examples\n ```\n # use bevy_math::IRect;\n let r = IRect::new(0, 0, 5, 1); // w=5 h=1\n assert!(r.contains(r.center()));\n assert!(r.contains(r.min));\n assert!(r.contains(r.max));\n ```",
            &["_self", "point"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_math::prelude::IRect>,
                other: Ref<::bevy_math::prelude::IRect>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::prelude::IRect as ::core::cmp::PartialEq<
                            ::bevy_math::prelude::IRect,
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
            "from_center_half_size",
            |
                origin: Val<::bevy_math::prelude::IVec2>,
                half_size: Val<::bevy_math::prelude::IVec2>|
            {
                let output: Val<::bevy_math::prelude::IRect> = {
                    {
                        let output: Val<::bevy_math::prelude::IRect> = ::bevy_math::prelude::IRect::from_center_half_size(
                                origin.into_inner(),
                                half_size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new rectangle from its center and half-size.\n # Panics\n This method panics if any of the components of the half-size is negative.\n # Examples\n ```\n # use bevy_math::{IRect, IVec2};\n let r = IRect::from_center_half_size(IVec2::ZERO, IVec2::ONE); // w=2 h=2\n assert_eq!(r.min, IVec2::splat(-1));\n assert_eq!(r.max, IVec2::splat(1));\n ```",
            &["origin", "half_size"],
        )
        .register_documented(
            "from_center_size",
            |
                origin: Val<::bevy_math::prelude::IVec2>,
                size: Val<::bevy_math::prelude::IVec2>|
            {
                let output: Val<::bevy_math::prelude::IRect> = {
                    {
                        let output: Val<::bevy_math::prelude::IRect> = ::bevy_math::prelude::IRect::from_center_size(
                                origin.into_inner(),
                                size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new rectangle from its center and size.\n # Rounding Behavior\n If the size contains odd numbers they will be rounded down to the nearest whole number.\n # Panics\n This method panics if any of the components of the size is negative.\n # Examples\n ```\n # use bevy_math::{IRect, IVec2};\n let r = IRect::from_center_size(IVec2::ZERO, IVec2::new(3, 2)); // w=2 h=2\n assert_eq!(r.min, IVec2::splat(-1));\n assert_eq!(r.max, IVec2::splat(1));\n ```",
            &["origin", "size"],
        )
        .register_documented(
            "from_corners",
            |p0: Val<::bevy_math::prelude::IVec2>, p1: Val<::bevy_math::prelude::IVec2>| {
                let output: Val<::bevy_math::prelude::IRect> = {
                    {
                        let output: Val<::bevy_math::prelude::IRect> = ::bevy_math::prelude::IRect::from_corners(
                                p0.into_inner(),
                                p1.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new rectangle from two corner points.\n The two points do not need to be the minimum and/or maximum corners.\n They only need to be two opposite corners.\n # Examples\n ```\n # use bevy_math::{IRect, IVec2};\n // Unit rect from [0,0] to [1,1]\n let r = IRect::from_corners(IVec2::ZERO, IVec2::ONE); // w=1 h=1\n // Same; the points do not need to be ordered\n let r = IRect::from_corners(IVec2::ONE, IVec2::ZERO); // w=1 h=1\n ```",
            &["p0", "p1"],
        )
        .register_documented(
            "half_size",
            |_self: Ref<::bevy_math::prelude::IRect>| {
                let output: Val<::bevy_math::prelude::IVec2> = {
                    {
                        let output: Val<::bevy_math::prelude::IVec2> = ::bevy_math::prelude::IRect::half_size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rectangle half-size.\n # Rounding Behavior\n If the full size contains odd numbers they will be rounded down to the nearest whole number when calculating the half size.\n # Examples\n ```\n # use bevy_math::{IRect, IVec2};\n let r = IRect::new(0, 0, 4, 3); // w=4 h=3\n assert_eq!(r.half_size(), IVec2::new(2, 1));\n ```",
            &["_self"],
        )
        .register_documented(
            "height",
            |_self: Ref<::bevy_math::prelude::IRect>| {
                let output: i32 = {
                    {
                        let output: i32 = ::bevy_math::prelude::IRect::height(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Rectangle height (max.y - min.y).\n # Examples\n ```\n # use bevy_math::IRect;\n let r = IRect::new(0, 0, 5, 1); // w=5 h=1\n assert_eq!(r.height(), 1);\n ```",
            &["_self"],
        )
        .register_documented(
            "inflate",
            |_self: Ref<::bevy_math::prelude::IRect>, expansion: i32| {
                let output: Val<::bevy_math::prelude::IRect> = {
                    {
                        let output: Val<::bevy_math::prelude::IRect> = ::bevy_math::prelude::IRect::inflate(
                                &_self,
                                expansion,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new rectangle by expanding it evenly on all sides.\n A positive expansion value produces a larger rectangle,\n while a negative expansion value produces a smaller rectangle.\n If this would result in zero or negative width or height, [`IRect::EMPTY`] is returned instead.\n # Examples\n ```\n # use bevy_math::{IRect, IVec2};\n let r = IRect::new(0, 0, 5, 1); // w=5 h=1\n let r2 = r.inflate(3); // w=11 h=7\n assert_eq!(r2.min, IVec2::splat(-3));\n assert_eq!(r2.max, IVec2::new(8, 4));\n let r = IRect::new(0, -1, 4, 3); // w=4 h=4\n let r2 = r.inflate(-1); // w=2 h=2\n assert_eq!(r2.min, IVec2::new(1, 0));\n assert_eq!(r2.max, IVec2::new(3, 2));\n ```",
            &["_self", "expansion"],
        )
        .register_documented(
            "intersect",
            |
                _self: Ref<::bevy_math::prelude::IRect>,
                other: Val<::bevy_math::prelude::IRect>|
            {
                let output: Val<::bevy_math::prelude::IRect> = {
                    {
                        let output: Val<::bevy_math::prelude::IRect> = ::bevy_math::prelude::IRect::intersect(
                                &_self,
                                other.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Build a new rectangle formed of the intersection of this rectangle and another rectangle.\n The intersection is the largest rectangle enclosed in both rectangles. If the intersection\n is empty, this method returns an empty rectangle ([`IRect::is_empty()`] returns `true`), but\n the actual values of [`IRect::min`] and [`IRect::max`] are implementation-dependent.\n # Examples\n ```\n # use bevy_math::{IRect, IVec2};\n let r1 = IRect::new(0, 0, 5, 1); // w=5 h=1\n let r2 = IRect::new(1, -1, 3, 3); // w=2 h=4\n let r = r1.intersect(r2);\n assert_eq!(r.min, IVec2::new(1, 0));\n assert_eq!(r.max, IVec2::new(3, 1));\n ```",
            &["_self", "other"],
        )
        .register_documented(
            "is_empty",
            |_self: Ref<::bevy_math::prelude::IRect>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::prelude::IRect::is_empty(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Check if the rectangle is empty.\n # Examples\n ```\n # use bevy_math::{IRect, IVec2};\n let r = IRect::from_corners(IVec2::ZERO, IVec2::new(0, 1)); // w=0 h=1\n assert!(r.is_empty());\n ```",
            &["_self"],
        )
        .register_documented(
            "new",
            |x0: i32, y0: i32, x1: i32, y1: i32| {
                let output: Val<::bevy_math::prelude::IRect> = {
                    {
                        let output: Val<::bevy_math::prelude::IRect> = ::bevy_math::prelude::IRect::new(
                                x0,
                                y0,
                                x1,
                                y1,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new rectangle from two corner points.\n The two points do not need to be the minimum and/or maximum corners.\n They only need to be two opposite corners.\n # Examples\n ```\n # use bevy_math::IRect;\n let r = IRect::new(0, 4, 10, 6); // w=10 h=2\n let r = IRect::new(2, 3, 5, -1); // w=3 h=4\n ```",
            &["x0", "y0", "x1", "y1"],
        )
        .register_documented(
            "size",
            |_self: Ref<::bevy_math::prelude::IRect>| {
                let output: Val<::bevy_math::prelude::IVec2> = {
                    {
                        let output: Val<::bevy_math::prelude::IVec2> = ::bevy_math::prelude::IRect::size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rectangle size.\n # Examples\n ```\n # use bevy_math::{IRect, IVec2};\n let r = IRect::new(0, 0, 5, 1); // w=5 h=1\n assert_eq!(r.size(), IVec2::new(5, 1));\n ```",
            &["_self"],
        )
        .register_documented(
            "union",
            |
                _self: Ref<::bevy_math::prelude::IRect>,
                other: Val<::bevy_math::prelude::IRect>|
            {
                let output: Val<::bevy_math::prelude::IRect> = {
                    {
                        let output: Val<::bevy_math::prelude::IRect> = ::bevy_math::prelude::IRect::union(
                                &_self,
                                other.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Build a new rectangle formed of the union of this rectangle and another rectangle.\n The union is the smallest rectangle enclosing both rectangles.\n # Examples\n ```\n # use bevy_math::{IRect, IVec2};\n let r1 = IRect::new(0, 0, 5, 1); // w=5 h=1\n let r2 = IRect::new(1, -1, 3, 3); // w=2 h=4\n let r = r1.union(r2);\n assert_eq!(r.min, IVec2::new(0, -1));\n assert_eq!(r.max, IVec2::new(5, 3));\n ```",
            &["_self", "other"],
        )
        .register_documented(
            "union_point",
            |
                _self: Ref<::bevy_math::prelude::IRect>,
                other: Val<::bevy_math::prelude::IVec2>|
            {
                let output: Val<::bevy_math::prelude::IRect> = {
                    {
                        let output: Val<::bevy_math::prelude::IRect> = ::bevy_math::prelude::IRect::union_point(
                                &_self,
                                other.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Build a new rectangle formed of the union of this rectangle and a point.\n The union is the smallest rectangle enclosing both the rectangle and the point. If the\n point is already inside the rectangle, this method returns a copy of the rectangle.\n # Examples\n ```\n # use bevy_math::{IRect, IVec2};\n let r = IRect::new(0, 0, 5, 1); // w=5 h=1\n let u = r.union_point(IVec2::new(3, 6));\n assert_eq!(u.min, IVec2::ZERO);\n assert_eq!(u.max, IVec2::new(5, 6));\n ```",
            &["_self", "other"],
        )
        .register_documented(
            "width",
            |_self: Ref<::bevy_math::prelude::IRect>| {
                let output: i32 = {
                    {
                        let output: i32 = ::bevy_math::prelude::IRect::width(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Rectangle width (max.x - min.x).\n # Examples\n ```\n # use bevy_math::IRect;\n let r = IRect::new(0, 0, 5, 1); // w=5 h=1\n assert_eq!(r.width(), 5);\n ```",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::prelude::IRect,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_rect_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::prelude::Rect,
    >::new(world)
        .register_documented(
            "as_irect",
            |_self: Ref<::bevy_math::prelude::Rect>| {
                let output: Val<::bevy_math::prelude::IRect> = {
                    {
                        let output: Val<::bevy_math::prelude::IRect> = ::bevy_math::prelude::Rect::as_irect(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns self as [`IRect`] (i32)",
            &["_self"],
        )
        .register_documented(
            "as_urect",
            |_self: Ref<::bevy_math::prelude::Rect>| {
                let output: Val<::bevy_math::prelude::URect> = {
                    {
                        let output: Val<::bevy_math::prelude::URect> = ::bevy_math::prelude::Rect::as_urect(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns self as [`URect`] (u32)",
            &["_self"],
        )
        .register_documented(
            "center",
            |_self: Ref<::bevy_math::prelude::Rect>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::prelude::Rect::center(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The center point of the rectangle.\n # Examples\n ```\n # use bevy_math::{Rect, Vec2};\n let r = Rect::new(0., 0., 5., 1.); // w=5 h=1\n assert!(r.center().abs_diff_eq(Vec2::new(2.5, 0.5), 1e-5));\n ```",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::prelude::Rect>| {
                let output: Val<::bevy_math::prelude::Rect> = {
                    {
                        let output: Val<::bevy_math::prelude::Rect> = <::bevy_math::prelude::Rect as ::core::clone::Clone>::clone(
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
            "contains",
            |
                _self: Ref<::bevy_math::prelude::Rect>,
                point: Val<::bevy_math::prelude::Vec2>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::prelude::Rect::contains(
                                &_self,
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Check if a point lies within this rectangle, inclusive of its edges.\n # Examples\n ```\n # use bevy_math::Rect;\n let r = Rect::new(0., 0., 5., 1.); // w=5 h=1\n assert!(r.contains(r.center()));\n assert!(r.contains(r.min));\n assert!(r.contains(r.max));\n ```",
            &["_self", "point"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_math::prelude::Rect>,
                other: Ref<::bevy_math::prelude::Rect>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::prelude::Rect as ::core::cmp::PartialEq<
                            ::bevy_math::prelude::Rect,
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
            "from_center_half_size",
            |
                origin: Val<::bevy_math::prelude::Vec2>,
                half_size: Val<::bevy_math::prelude::Vec2>|
            {
                let output: Val<::bevy_math::prelude::Rect> = {
                    {
                        let output: Val<::bevy_math::prelude::Rect> = ::bevy_math::prelude::Rect::from_center_half_size(
                                origin.into_inner(),
                                half_size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new rectangle from its center and half-size.\n # Panics\n This method panics if any of the components of the half-size is negative.\n # Examples\n ```\n # use bevy_math::{Rect, Vec2};\n let r = Rect::from_center_half_size(Vec2::ZERO, Vec2::ONE); // w=2 h=2\n assert!(r.min.abs_diff_eq(Vec2::splat(-1.), 1e-5));\n assert!(r.max.abs_diff_eq(Vec2::splat(1.), 1e-5));\n ```",
            &["origin", "half_size"],
        )
        .register_documented(
            "from_center_size",
            |
                origin: Val<::bevy_math::prelude::Vec2>,
                size: Val<::bevy_math::prelude::Vec2>|
            {
                let output: Val<::bevy_math::prelude::Rect> = {
                    {
                        let output: Val<::bevy_math::prelude::Rect> = ::bevy_math::prelude::Rect::from_center_size(
                                origin.into_inner(),
                                size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new rectangle from its center and size.\n # Panics\n This method panics if any of the components of the size is negative.\n # Examples\n ```\n # use bevy_math::{Rect, Vec2};\n let r = Rect::from_center_size(Vec2::ZERO, Vec2::ONE); // w=1 h=1\n assert!(r.min.abs_diff_eq(Vec2::splat(-0.5), 1e-5));\n assert!(r.max.abs_diff_eq(Vec2::splat(0.5), 1e-5));\n ```",
            &["origin", "size"],
        )
        .register_documented(
            "from_corners",
            |p0: Val<::bevy_math::prelude::Vec2>, p1: Val<::bevy_math::prelude::Vec2>| {
                let output: Val<::bevy_math::prelude::Rect> = {
                    {
                        let output: Val<::bevy_math::prelude::Rect> = ::bevy_math::prelude::Rect::from_corners(
                                p0.into_inner(),
                                p1.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new rectangle from two corner points.\n The two points do not need to be the minimum and/or maximum corners.\n They only need to be two opposite corners.\n # Examples\n ```\n # use bevy_math::{Rect, Vec2};\n // Unit rect from [0,0] to [1,1]\n let r = Rect::from_corners(Vec2::ZERO, Vec2::ONE); // w=1 h=1\n // Same; the points do not need to be ordered\n let r = Rect::from_corners(Vec2::ONE, Vec2::ZERO); // w=1 h=1\n ```",
            &["p0", "p1"],
        )
        .register_documented(
            "half_size",
            |_self: Ref<::bevy_math::prelude::Rect>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::prelude::Rect::half_size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rectangle half-size.\n # Examples\n ```\n # use bevy_math::{Rect, Vec2};\n let r = Rect::new(0., 0., 5., 1.); // w=5 h=1\n assert!(r.half_size().abs_diff_eq(Vec2::new(2.5, 0.5), 1e-5));\n ```",
            &["_self"],
        )
        .register_documented(
            "height",
            |_self: Ref<::bevy_math::prelude::Rect>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::prelude::Rect::height(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Rectangle height (max.y - min.y).\n # Examples\n ```\n # use bevy_math::Rect;\n let r = Rect::new(0., 0., 5., 1.); // w=5 h=1\n assert!((r.height() - 1.).abs() <= 1e-5);\n ```",
            &["_self"],
        )
        .register_documented(
            "inflate",
            |_self: Ref<::bevy_math::prelude::Rect>, expansion: f32| {
                let output: Val<::bevy_math::prelude::Rect> = {
                    {
                        let output: Val<::bevy_math::prelude::Rect> = ::bevy_math::prelude::Rect::inflate(
                                &_self,
                                expansion,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new rectangle by expanding it evenly on all sides.\n A positive expansion value produces a larger rectangle,\n while a negative expansion value produces a smaller rectangle.\n If this would result in zero or negative width or height, [`Rect::EMPTY`] is returned instead.\n # Examples\n ```\n # use bevy_math::{Rect, Vec2};\n let r = Rect::new(0., 0., 5., 1.); // w=5 h=1\n let r2 = r.inflate(3.); // w=11 h=7\n assert!(r2.min.abs_diff_eq(Vec2::splat(-3.), 1e-5));\n assert!(r2.max.abs_diff_eq(Vec2::new(8., 4.), 1e-5));\n let r = Rect::new(0., -1., 6., 7.); // w=6 h=8\n let r2 = r.inflate(-2.); // w=11 h=7\n assert!(r2.min.abs_diff_eq(Vec2::new(2., 1.), 1e-5));\n assert!(r2.max.abs_diff_eq(Vec2::new(4., 5.), 1e-5));\n ```",
            &["_self", "expansion"],
        )
        .register_documented(
            "intersect",
            |
                _self: Ref<::bevy_math::prelude::Rect>,
                other: Val<::bevy_math::prelude::Rect>|
            {
                let output: Val<::bevy_math::prelude::Rect> = {
                    {
                        let output: Val<::bevy_math::prelude::Rect> = ::bevy_math::prelude::Rect::intersect(
                                &_self,
                                other.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Build a new rectangle formed of the intersection of this rectangle and another rectangle.\n The intersection is the largest rectangle enclosed in both rectangles. If the intersection\n is empty, this method returns an empty rectangle ([`Rect::is_empty()`] returns `true`), but\n the actual values of [`Rect::min`] and [`Rect::max`] are implementation-dependent.\n # Examples\n ```\n # use bevy_math::{Rect, Vec2};\n let r1 = Rect::new(0., 0., 5., 1.); // w=5 h=1\n let r2 = Rect::new(1., -1., 3., 3.); // w=2 h=4\n let r = r1.intersect(r2);\n assert!(r.min.abs_diff_eq(Vec2::new(1., 0.), 1e-5));\n assert!(r.max.abs_diff_eq(Vec2::new(3., 1.), 1e-5));\n ```",
            &["_self", "other"],
        )
        .register_documented(
            "is_empty",
            |_self: Ref<::bevy_math::prelude::Rect>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::prelude::Rect::is_empty(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Check if the rectangle is empty.\n # Examples\n ```\n # use bevy_math::{Rect, Vec2};\n let r = Rect::from_corners(Vec2::ZERO, Vec2::new(0., 1.)); // w=0 h=1\n assert!(r.is_empty());\n ```",
            &["_self"],
        )
        .register_documented(
            "new",
            |x0: f32, y0: f32, x1: f32, y1: f32| {
                let output: Val<::bevy_math::prelude::Rect> = {
                    {
                        let output: Val<::bevy_math::prelude::Rect> = ::bevy_math::prelude::Rect::new(
                                x0,
                                y0,
                                x1,
                                y1,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new rectangle from two corner points.\n The two points do not need to be the minimum and/or maximum corners.\n They only need to be two opposite corners.\n # Examples\n ```\n # use bevy_math::Rect;\n let r = Rect::new(0., 4., 10., 6.); // w=10 h=2\n let r = Rect::new(2., 3., 5., -1.); // w=3 h=4\n ```",
            &["x0", "y0", "x1", "y1"],
        )
        .register_documented(
            "normalize",
            |
                _self: Ref<::bevy_math::prelude::Rect>,
                other: Val<::bevy_math::prelude::Rect>|
            {
                let output: Val<::bevy_math::prelude::Rect> = {
                    {
                        let output: Val<::bevy_math::prelude::Rect> = ::bevy_math::prelude::Rect::normalize(
                                &_self,
                                other.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Build a new rectangle from this one with its coordinates expressed\n relative to `other` in a normalized ([0..1] x [0..1]) coordinate system.\n # Examples\n ```\n # use bevy_math::{Rect, Vec2};\n let r = Rect::new(2., 3., 4., 6.);\n let s = Rect::new(0., 0., 10., 10.);\n let n = r.normalize(s);\n assert_eq!(n.min.x, 0.2);\n assert_eq!(n.min.y, 0.3);\n assert_eq!(n.max.x, 0.4);\n assert_eq!(n.max.y, 0.6);\n ```",
            &["_self", "other"],
        )
        .register_documented(
            "size",
            |_self: Ref<::bevy_math::prelude::Rect>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::prelude::Rect::size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rectangle size.\n # Examples\n ```\n # use bevy_math::{Rect, Vec2};\n let r = Rect::new(0., 0., 5., 1.); // w=5 h=1\n assert!(r.size().abs_diff_eq(Vec2::new(5., 1.), 1e-5));\n ```",
            &["_self"],
        )
        .register_documented(
            "union",
            |
                _self: Ref<::bevy_math::prelude::Rect>,
                other: Val<::bevy_math::prelude::Rect>|
            {
                let output: Val<::bevy_math::prelude::Rect> = {
                    {
                        let output: Val<::bevy_math::prelude::Rect> = ::bevy_math::prelude::Rect::union(
                                &_self,
                                other.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Build a new rectangle formed of the union of this rectangle and another rectangle.\n The union is the smallest rectangle enclosing both rectangles.\n # Examples\n ```\n # use bevy_math::{Rect, Vec2};\n let r1 = Rect::new(0., 0., 5., 1.); // w=5 h=1\n let r2 = Rect::new(1., -1., 3., 3.); // w=2 h=4\n let r = r1.union(r2);\n assert!(r.min.abs_diff_eq(Vec2::new(0., -1.), 1e-5));\n assert!(r.max.abs_diff_eq(Vec2::new(5., 3.), 1e-5));\n ```",
            &["_self", "other"],
        )
        .register_documented(
            "union_point",
            |
                _self: Ref<::bevy_math::prelude::Rect>,
                other: Val<::bevy_math::prelude::Vec2>|
            {
                let output: Val<::bevy_math::prelude::Rect> = {
                    {
                        let output: Val<::bevy_math::prelude::Rect> = ::bevy_math::prelude::Rect::union_point(
                                &_self,
                                other.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Build a new rectangle formed of the union of this rectangle and a point.\n The union is the smallest rectangle enclosing both the rectangle and the point. If the\n point is already inside the rectangle, this method returns a copy of the rectangle.\n # Examples\n ```\n # use bevy_math::{Rect, Vec2};\n let r = Rect::new(0., 0., 5., 1.); // w=5 h=1\n let u = r.union_point(Vec2::new(3., 6.));\n assert!(u.min.abs_diff_eq(Vec2::ZERO, 1e-5));\n assert!(u.max.abs_diff_eq(Vec2::new(5., 6.), 1e-5));\n ```",
            &["_self", "other"],
        )
        .register_documented(
            "width",
            |_self: Ref<::bevy_math::prelude::Rect>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::prelude::Rect::width(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Rectangle width (max.x - min.x).\n # Examples\n ```\n # use bevy_math::Rect;\n let r = Rect::new(0., 0., 5., 1.); // w=5 h=1\n assert!((r.width() - 5.).abs() <= 1e-5);\n ```",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::prelude::Rect,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_u_rect_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::prelude::URect,
    >::new(world)
        .register_documented(
            "as_irect",
            |_self: Ref<::bevy_math::prelude::URect>| {
                let output: Val<::bevy_math::prelude::IRect> = {
                    {
                        let output: Val<::bevy_math::prelude::IRect> = ::bevy_math::prelude::URect::as_irect(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns self as [`IRect`] (i32)",
            &["_self"],
        )
        .register_documented(
            "as_rect",
            |_self: Ref<::bevy_math::prelude::URect>| {
                let output: Val<::bevy_math::prelude::Rect> = {
                    {
                        let output: Val<::bevy_math::prelude::Rect> = ::bevy_math::prelude::URect::as_rect(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns self as [`Rect`] (f32)",
            &["_self"],
        )
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_math::prelude::URect>| {
                let output: () = {
                    {
                        let output: () = <::bevy_math::prelude::URect as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
            "center",
            |_self: Ref<::bevy_math::prelude::URect>| {
                let output: Val<::bevy_math::prelude::UVec2> = {
                    {
                        let output: Val<::bevy_math::prelude::UVec2> = ::bevy_math::prelude::URect::center(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " The center point of the rectangle.\n # Rounding Behavior\n If the (min + max) contains odd numbers they will be rounded down to the nearest whole number when calculating the center.\n # Examples\n ```\n # use bevy_math::{URect, UVec2};\n let r = URect::new(0, 0, 4, 2); // w=4 h=2\n assert_eq!(r.center(), UVec2::new(2, 1));\n ```",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::prelude::URect>| {
                let output: Val<::bevy_math::prelude::URect> = {
                    {
                        let output: Val<::bevy_math::prelude::URect> = <::bevy_math::prelude::URect as ::core::clone::Clone>::clone(
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
            "contains",
            |
                _self: Ref<::bevy_math::prelude::URect>,
                point: Val<::bevy_math::prelude::UVec2>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::prelude::URect::contains(
                                &_self,
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Check if a point lies within this rectangle, inclusive of its edges.\n # Examples\n ```\n # use bevy_math::URect;\n let r = URect::new(0, 0, 5, 1); // w=5 h=1\n assert!(r.contains(r.center()));\n assert!(r.contains(r.min));\n assert!(r.contains(r.max));\n ```",
            &["_self", "point"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_math::prelude::URect>,
                other: Ref<::bevy_math::prelude::URect>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::prelude::URect as ::core::cmp::PartialEq<
                            ::bevy_math::prelude::URect,
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
            "from_center_half_size",
            |
                origin: Val<::bevy_math::prelude::UVec2>,
                half_size: Val<::bevy_math::prelude::UVec2>|
            {
                let output: Val<::bevy_math::prelude::URect> = {
                    {
                        let output: Val<::bevy_math::prelude::URect> = ::bevy_math::prelude::URect::from_center_half_size(
                                origin.into_inner(),
                                half_size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new rectangle from its center and half-size.\n # Panics\n This method panics if any of the components of the half-size is negative or if `origin - half_size` results in any negatives.\n # Examples\n ```\n # use bevy_math::{URect, UVec2};\n let r = URect::from_center_half_size(UVec2::ONE, UVec2::ONE); // w=2 h=2\n assert_eq!(r.min, UVec2::splat(0));\n assert_eq!(r.max, UVec2::splat(2));\n ```",
            &["origin", "half_size"],
        )
        .register_documented(
            "from_center_size",
            |
                origin: Val<::bevy_math::prelude::UVec2>,
                size: Val<::bevy_math::prelude::UVec2>|
            {
                let output: Val<::bevy_math::prelude::URect> = {
                    {
                        let output: Val<::bevy_math::prelude::URect> = ::bevy_math::prelude::URect::from_center_size(
                                origin.into_inner(),
                                size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new rectangle from its center and size.\n # Rounding Behavior\n If the size contains odd numbers they will be rounded down to the nearest whole number.\n # Panics\n This method panics if any of the components of the size is negative or if `origin - (size / 2)` results in any negatives.\n # Examples\n ```\n # use bevy_math::{URect, UVec2};\n let r = URect::from_center_size(UVec2::ONE, UVec2::splat(2)); // w=2 h=2\n assert_eq!(r.min, UVec2::splat(0));\n assert_eq!(r.max, UVec2::splat(2));\n ```",
            &["origin", "size"],
        )
        .register_documented(
            "from_corners",
            |p0: Val<::bevy_math::prelude::UVec2>, p1: Val<::bevy_math::prelude::UVec2>| {
                let output: Val<::bevy_math::prelude::URect> = {
                    {
                        let output: Val<::bevy_math::prelude::URect> = ::bevy_math::prelude::URect::from_corners(
                                p0.into_inner(),
                                p1.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new rectangle from two corner points.\n The two points do not need to be the minimum and/or maximum corners.\n They only need to be two opposite corners.\n # Examples\n ```\n # use bevy_math::{URect, UVec2};\n // Unit rect from [0,0] to [1,1]\n let r = URect::from_corners(UVec2::ZERO, UVec2::ONE); // w=1 h=1\n // Same; the points do not need to be ordered\n let r = URect::from_corners(UVec2::ONE, UVec2::ZERO); // w=1 h=1\n ```",
            &["p0", "p1"],
        )
        .register_documented(
            "half_size",
            |_self: Ref<::bevy_math::prelude::URect>| {
                let output: Val<::bevy_math::prelude::UVec2> = {
                    {
                        let output: Val<::bevy_math::prelude::UVec2> = ::bevy_math::prelude::URect::half_size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rectangle half-size.\n # Rounding Behavior\n If the full size contains odd numbers they will be rounded down to the nearest whole number when calculating the half size.\n # Examples\n ```\n # use bevy_math::{URect, UVec2};\n let r = URect::new(0, 0, 4, 2); // w=4 h=2\n assert_eq!(r.half_size(), UVec2::new(2, 1));\n ```",
            &["_self"],
        )
        .register_documented(
            "height",
            |_self: Ref<::bevy_math::prelude::URect>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_math::prelude::URect::height(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Rectangle height (max.y - min.y).\n # Examples\n ```\n # use bevy_math::URect;\n let r = URect::new(0, 0, 5, 1); // w=5 h=1\n assert_eq!(r.height(), 1);\n ```",
            &["_self"],
        )
        .register_documented(
            "inflate",
            |_self: Ref<::bevy_math::prelude::URect>, expansion: i32| {
                let output: Val<::bevy_math::prelude::URect> = {
                    {
                        let output: Val<::bevy_math::prelude::URect> = ::bevy_math::prelude::URect::inflate(
                                &_self,
                                expansion,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new rectangle by expanding it evenly on all sides.\n A positive expansion value produces a larger rectangle,\n while a negative expansion value produces a smaller rectangle.\n If this would result in zero width or height, [`URect::EMPTY`] is returned instead.\n # Examples\n ```\n # use bevy_math::{URect, UVec2};\n let r = URect::new(4, 4, 6, 6); // w=2 h=2\n let r2 = r.inflate(1); // w=4 h=4\n assert_eq!(r2.min, UVec2::splat(3));\n assert_eq!(r2.max, UVec2::splat(7));\n let r = URect::new(4, 4, 8, 8); // w=4 h=4\n let r2 = r.inflate(-1); // w=2 h=2\n assert_eq!(r2.min, UVec2::splat(5));\n assert_eq!(r2.max, UVec2::splat(7));\n ```",
            &["_self", "expansion"],
        )
        .register_documented(
            "intersect",
            |
                _self: Ref<::bevy_math::prelude::URect>,
                other: Val<::bevy_math::prelude::URect>|
            {
                let output: Val<::bevy_math::prelude::URect> = {
                    {
                        let output: Val<::bevy_math::prelude::URect> = ::bevy_math::prelude::URect::intersect(
                                &_self,
                                other.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Build a new rectangle formed of the intersection of this rectangle and another rectangle.\n The intersection is the largest rectangle enclosed in both rectangles. If the intersection\n is empty, this method returns an empty rectangle ([`URect::is_empty()`] returns `true`), but\n the actual values of [`URect::min`] and [`URect::max`] are implementation-dependent.\n # Examples\n ```\n # use bevy_math::{URect, UVec2};\n let r1 = URect::new(0, 0, 2, 2); // w=2 h=2\n let r2 = URect::new(1, 1, 3, 3); // w=2 h=2\n let r = r1.intersect(r2);\n assert_eq!(r.min, UVec2::new(1, 1));\n assert_eq!(r.max, UVec2::new(2, 2));\n ```",
            &["_self", "other"],
        )
        .register_documented(
            "is_empty",
            |_self: Ref<::bevy_math::prelude::URect>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::prelude::URect::is_empty(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Check if the rectangle is empty.\n # Examples\n ```\n # use bevy_math::{URect, UVec2};\n let r = URect::from_corners(UVec2::ZERO, UVec2::new(0, 1)); // w=0 h=1\n assert!(r.is_empty());\n ```",
            &["_self"],
        )
        .register_documented(
            "new",
            |x0: u32, y0: u32, x1: u32, y1: u32| {
                let output: Val<::bevy_math::prelude::URect> = {
                    {
                        let output: Val<::bevy_math::prelude::URect> = ::bevy_math::prelude::URect::new(
                                x0,
                                y0,
                                x1,
                                y1,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new rectangle from two corner points.\n The two points do not need to be the minimum and/or maximum corners.\n They only need to be two opposite corners.\n # Examples\n ```\n # use bevy_math::URect;\n let r = URect::new(0, 4, 10, 6); // w=10 h=2\n let r = URect::new(2, 4, 5, 0); // w=3 h=4\n ```",
            &["x0", "y0", "x1", "y1"],
        )
        .register_documented(
            "size",
            |_self: Ref<::bevy_math::prelude::URect>| {
                let output: Val<::bevy_math::prelude::UVec2> = {
                    {
                        let output: Val<::bevy_math::prelude::UVec2> = ::bevy_math::prelude::URect::size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Rectangle size.\n # Examples\n ```\n # use bevy_math::{URect, UVec2};\n let r = URect::new(0, 0, 5, 1); // w=5 h=1\n assert_eq!(r.size(), UVec2::new(5, 1));\n ```",
            &["_self"],
        )
        .register_documented(
            "union",
            |
                _self: Ref<::bevy_math::prelude::URect>,
                other: Val<::bevy_math::prelude::URect>|
            {
                let output: Val<::bevy_math::prelude::URect> = {
                    {
                        let output: Val<::bevy_math::prelude::URect> = ::bevy_math::prelude::URect::union(
                                &_self,
                                other.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Build a new rectangle formed of the union of this rectangle and another rectangle.\n The union is the smallest rectangle enclosing both rectangles.\n # Examples\n ```\n # use bevy_math::{URect, UVec2};\n let r1 = URect::new(0, 0, 5, 1); // w=5 h=1\n let r2 = URect::new(1, 0, 3, 8); // w=2 h=4\n let r = r1.union(r2);\n assert_eq!(r.min, UVec2::new(0, 0));\n assert_eq!(r.max, UVec2::new(5, 8));\n ```",
            &["_self", "other"],
        )
        .register_documented(
            "union_point",
            |
                _self: Ref<::bevy_math::prelude::URect>,
                other: Val<::bevy_math::prelude::UVec2>|
            {
                let output: Val<::bevy_math::prelude::URect> = {
                    {
                        let output: Val<::bevy_math::prelude::URect> = ::bevy_math::prelude::URect::union_point(
                                &_self,
                                other.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Build a new rectangle formed of the union of this rectangle and a point.\n The union is the smallest rectangle enclosing both the rectangle and the point. If the\n point is already inside the rectangle, this method returns a copy of the rectangle.\n # Examples\n ```\n # use bevy_math::{URect, UVec2};\n let r = URect::new(0, 0, 5, 1); // w=5 h=1\n let u = r.union_point(UVec2::new(3, 6));\n assert_eq!(u.min, UVec2::ZERO);\n assert_eq!(u.max, UVec2::new(5, 6));\n ```",
            &["_self", "other"],
        )
        .register_documented(
            "width",
            |_self: Ref<::bevy_math::prelude::URect>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_math::prelude::URect::width(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Rectangle width (max.x - min.x).\n # Examples\n ```\n # use bevy_math::URect;\n let r = URect::new(0, 0, 5, 1); // w=5 h=1\n assert_eq!(r.width(), 5);\n ```",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::prelude::URect,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_affine_3_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<::bevy_math::Affine3>::new(
        world,
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_math::Affine3, bevy_mod_scripting_bindings::MarkAsGenerated>();
}
pub(crate) fn register_aabb_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::bounding::Aabb2d,
    >::new(world)
        .register_documented(
            "bounding_circle",
            |_self: Ref<::bevy_math::bounding::Aabb2d>| {
                let output: Val<::bevy_math::bounding::BoundingCircle> = {
                    {
                        let output: Val<::bevy_math::bounding::BoundingCircle> = ::bevy_math::bounding::Aabb2d::bounding_circle(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Computes the smallest [`BoundingCircle`] containing this [`Aabb2d`].",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::bounding::Aabb2d>| {
                let output: Val<::bevy_math::bounding::Aabb2d> = {
                    {
                        let output: Val<::bevy_math::bounding::Aabb2d> = <::bevy_math::bounding::Aabb2d as ::core::clone::Clone>::clone(
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
            "closest_point",
            |
                _self: Ref<::bevy_math::bounding::Aabb2d>,
                point: Val<::bevy_math::prelude::Vec2>|
            {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::bounding::Aabb2d::closest_point(
                                &_self,
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Finds the point on the AABB that is closest to the given `point`.\n If the point is outside the AABB, the returned point will be on the perimeter of the AABB.\n Otherwise, it will be inside the AABB and returned as is.",
            &["_self", "point"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_math::bounding::Aabb2d>,
                other: Ref<::bevy_math::bounding::Aabb2d>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::bounding::Aabb2d as ::core::cmp::PartialEq<
                            ::bevy_math::bounding::Aabb2d,
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
            "new",
            |
                center: Val<::bevy_math::prelude::Vec2>,
                half_size: Val<::bevy_math::prelude::Vec2>|
            {
                let output: Val<::bevy_math::bounding::Aabb2d> = {
                    {
                        let output: Val<::bevy_math::bounding::Aabb2d> = ::bevy_math::bounding::Aabb2d::new(
                                center.into_inner(),
                                half_size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Constructs an AABB from its center and half-size.",
            &["center", "half_size"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::bounding::Aabb2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_bounding_circle_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::bounding::BoundingCircle,
    >::new(world)
        .register_documented(
            "aabb_2d",
            |_self: Ref<::bevy_math::bounding::BoundingCircle>| {
                let output: Val<::bevy_math::bounding::Aabb2d> = {
                    {
                        let output: Val<::bevy_math::bounding::Aabb2d> = ::bevy_math::bounding::BoundingCircle::aabb_2d(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Computes the smallest [`Aabb2d`] containing this [`BoundingCircle`].",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::bounding::BoundingCircle>| {
                let output: Val<::bevy_math::bounding::BoundingCircle> = {
                    {
                        let output: Val<::bevy_math::bounding::BoundingCircle> = <::bevy_math::bounding::BoundingCircle as ::core::clone::Clone>::clone(
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
            "closest_point",
            |
                _self: Ref<::bevy_math::bounding::BoundingCircle>,
                point: Val<::bevy_math::prelude::Vec2>|
            {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::bounding::BoundingCircle::closest_point(
                                &_self,
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Finds the point on the bounding circle that is closest to the given `point`.\n If the point is outside the circle, the returned point will be on the perimeter of the circle.\n Otherwise, it will be inside the circle and returned as is.",
            &["_self", "point"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_math::bounding::BoundingCircle>,
                other: Ref<::bevy_math::bounding::BoundingCircle>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::bounding::BoundingCircle as ::core::cmp::PartialEq<
                            ::bevy_math::bounding::BoundingCircle,
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
            "new",
            |center: Val<::bevy_math::prelude::Vec2>, radius: f32| {
                let output: Val<::bevy_math::bounding::BoundingCircle> = {
                    {
                        let output: Val<::bevy_math::bounding::BoundingCircle> = ::bevy_math::bounding::BoundingCircle::new(
                                center.into_inner(),
                                radius,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Constructs a bounding circle from its center and radius.",
            &["center", "radius"],
        )
        .register_documented(
            "radius",
            |_self: Ref<::bevy_math::bounding::BoundingCircle>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::bounding::BoundingCircle::radius(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the radius of the bounding circle",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::bounding::BoundingCircle,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_circle_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Circle,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::Circle>| {
                let output: Val<::bevy_math::primitives::Circle> = {
                    {
                        let output: Val<::bevy_math::primitives::Circle> = <::bevy_math::primitives::Circle as ::core::clone::Clone>::clone(
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
            "closest_point",
            |
                _self: Ref<::bevy_math::primitives::Circle>,
                point: Val<::bevy_math::prelude::Vec2>|
            {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::primitives::Circle::closest_point(
                                &_self,
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Finds the point on the circle that is closest to the given `point`.\n If the point is outside the circle, the returned point will be on the perimeter of the circle.\n Otherwise, it will be inside the circle and returned as is.",
            &["_self", "point"],
        )
        .register_documented(
            "diameter",
            |_self: Ref<::bevy_math::primitives::Circle>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Circle::diameter(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the diameter of the circle",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_math::primitives::Circle>,
                other: Ref<::bevy_math::primitives::Circle>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::Circle as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Circle,
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
            "new",
            |radius: f32| {
                let output: Val<::bevy_math::primitives::Circle> = {
                    {
                        let output: Val<::bevy_math::primitives::Circle> = ::bevy_math::primitives::Circle::new(
                                radius,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`Circle`] from a `radius`",
            &["radius"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Circle,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_annulus_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Annulus,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::Annulus>| {
                let output: Val<::bevy_math::primitives::Annulus> = {
                    {
                        let output: Val<::bevy_math::primitives::Annulus> = <::bevy_math::primitives::Annulus as ::core::clone::Clone>::clone(
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
            "closest_point",
            |
                _self: Ref<::bevy_math::primitives::Annulus>,
                point: Val<::bevy_math::prelude::Vec2>|
            {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::primitives::Annulus::closest_point(
                                &_self,
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Finds the point on the annulus that is closest to the given `point`:\n - If the point is outside of the annulus completely, the returned point will be on the outer perimeter.\n - If the point is inside of the inner circle (hole) of the annulus, the returned point will be on the inner perimeter.\n - Otherwise, the returned point is overlapping the annulus and returned as is.",
            &["_self", "point"],
        )
        .register_documented(
            "diameter",
            |_self: Ref<::bevy_math::primitives::Annulus>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Annulus::diameter(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the diameter of the annulus",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_math::primitives::Annulus>,
                other: Ref<::bevy_math::primitives::Annulus>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::Annulus as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Annulus,
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
            "new",
            |inner_radius: f32, outer_radius: f32| {
                let output: Val<::bevy_math::primitives::Annulus> = {
                    {
                        let output: Val<::bevy_math::primitives::Annulus> = ::bevy_math::primitives::Annulus::new(
                                inner_radius,
                                outer_radius,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`Annulus`] from the radii of the inner and outer circle",
            &["inner_radius", "outer_radius"],
        )
        .register_documented(
            "thickness",
            |_self: Ref<::bevy_math::primitives::Annulus>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Annulus::thickness(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the thickness of the annulus",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Annulus,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_arc_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Arc2d,
    >::new(world)
        .register_documented(
            "angle",
            |_self: Ref<::bevy_math::primitives::Arc2d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Arc2d::angle(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Get the angle of the arc",
            &["_self"],
        )
        .register_documented(
            "apothem",
            |_self: Ref<::bevy_math::primitives::Arc2d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Arc2d::apothem(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Get the length of the apothem of this arc, that is,\n the distance from the center of the circle to the midpoint of the chord, in the direction of the midpoint of the arc.\n Equivalently, the [`radius`](Self::radius) minus the [`sagitta`](Self::sagitta).\n Note that for a [`major`](Self::is_major) arc, the apothem will be negative.",
            &["_self"],
        )
        .register_documented(
            "chord_length",
            |_self: Ref<::bevy_math::primitives::Arc2d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Arc2d::chord_length(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the distance between the endpoints (the length of the chord)",
            &["_self"],
        )
        .register_documented(
            "chord_midpoint",
            |_self: Ref<::bevy_math::primitives::Arc2d>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::primitives::Arc2d::chord_midpoint(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the midpoint of the two endpoints (the midpoint of the chord)",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::Arc2d>| {
                let output: Val<::bevy_math::primitives::Arc2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Arc2d> = <::bevy_math::primitives::Arc2d as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_math::primitives::Arc2d>,
                other: Ref<::bevy_math::primitives::Arc2d>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::Arc2d as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Arc2d,
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
            "from_degrees",
            |radius: f32, angle: f32| {
                let output: Val<::bevy_math::primitives::Arc2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Arc2d> = ::bevy_math::primitives::Arc2d::from_degrees(
                                radius,
                                angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`Arc2d`] from a `radius` and an `angle` in degrees.",
            &["radius", "angle"],
        )
        .register_documented(
            "from_radians",
            |radius: f32, angle: f32| {
                let output: Val<::bevy_math::primitives::Arc2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Arc2d> = ::bevy_math::primitives::Arc2d::from_radians(
                                radius,
                                angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`Arc2d`] from a `radius` and an `angle` in radians",
            &["radius", "angle"],
        )
        .register_documented(
            "from_turns",
            |radius: f32, fraction: f32| {
                let output: Val<::bevy_math::primitives::Arc2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Arc2d> = ::bevy_math::primitives::Arc2d::from_turns(
                                radius,
                                fraction,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`Arc2d`] from a `radius` and a `fraction` of a single turn.\n For instance, `0.5` turns is a semicircle.",
            &["radius", "fraction"],
        )
        .register_documented(
            "half_chord_length",
            |_self: Ref<::bevy_math::primitives::Arc2d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Arc2d::half_chord_length(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get half the distance between the endpoints (half the length of the chord)",
            &["_self"],
        )
        .register_documented(
            "is_major",
            |_self: Ref<::bevy_math::primitives::Arc2d>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::primitives::Arc2d::is_major(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Produces true if the arc is at least half a circle.\n **Note:** This is not the negation of [`is_minor`](Self::is_minor): an exact semicircle is both major and minor.",
            &["_self"],
        )
        .register_documented(
            "is_minor",
            |_self: Ref<::bevy_math::primitives::Arc2d>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::primitives::Arc2d::is_minor(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Produces true if the arc is at most half a circle.\n **Note:** This is not the negation of [`is_major`](Self::is_major): an exact semicircle is both major and minor.",
            &["_self"],
        )
        .register_documented(
            "left_endpoint",
            |_self: Ref<::bevy_math::primitives::Arc2d>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::primitives::Arc2d::left_endpoint(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the left-hand end point of the arc",
            &["_self"],
        )
        .register_documented(
            "length",
            |_self: Ref<::bevy_math::primitives::Arc2d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Arc2d::length(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Get the length of the arc",
            &["_self"],
        )
        .register_documented(
            "midpoint",
            |_self: Ref<::bevy_math::primitives::Arc2d>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::primitives::Arc2d::midpoint(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the midpoint of the arc",
            &["_self"],
        )
        .register_documented(
            "new",
            |radius: f32, half_angle: f32| {
                let output: Val<::bevy_math::primitives::Arc2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Arc2d> = ::bevy_math::primitives::Arc2d::new(
                                radius,
                                half_angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`Arc2d`] from a `radius` and a `half_angle`",
            &["radius", "half_angle"],
        )
        .register_documented(
            "right_endpoint",
            |_self: Ref<::bevy_math::primitives::Arc2d>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::primitives::Arc2d::right_endpoint(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the right-hand end point of the arc",
            &["_self"],
        )
        .register_documented(
            "sagitta",
            |_self: Ref<::bevy_math::primitives::Arc2d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Arc2d::sagitta(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Get the length of the sagitta of this arc, that is,\n the length of the line between the midpoints of the arc and its chord.\n Equivalently, the height of the triangle whose base is the chord and whose apex is the midpoint of the arc.\n The sagitta is also the sum of the [`radius`](Self::radius) and the [`apothem`](Self::apothem).",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Arc2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_capsule_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Capsule2d,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_math::primitives::Capsule2d>| {
            let output: Val<::bevy_math::primitives::Capsule2d> = {
                {
                    let output: Val<::bevy_math::primitives::Capsule2d> =
                        <::bevy_math::primitives::Capsule2d as ::core::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_math::primitives::Capsule2d>,
         other: Ref<::bevy_math::primitives::Capsule2d>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_math::primitives::Capsule2d as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Capsule2d,
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
        "new",
        |radius: f32, length: f32| {
            let output: Val<::bevy_math::primitives::Capsule2d> = {
                {
                    let output: Val<::bevy_math::primitives::Capsule2d> =
                        ::bevy_math::primitives::Capsule2d::new(radius, length).into();
                    output
                }
            };
            output
        },
        " Create a new `Capsule2d` from a radius and length",
        &["radius", "length"],
    )
    .register_documented(
        "to_inner_rectangle",
        |_self: Ref<::bevy_math::primitives::Capsule2d>| {
            let output: Val<::bevy_math::primitives::Rectangle> = {
                {
                    let output: Val<::bevy_math::primitives::Rectangle> =
                        ::bevy_math::primitives::Capsule2d::to_inner_rectangle(&_self).into();
                    output
                }
            };
            output
        },
        " Get the part connecting the semicircular ends of the capsule as a [`Rectangle`]",
        &["_self"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Capsule2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_circular_sector_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::CircularSector,
    >::new(world)
        .register_documented(
            "angle",
            |_self: Ref<::bevy_math::primitives::CircularSector>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::CircularSector::angle(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the angle of the sector",
            &["_self"],
        )
        .register_documented(
            "apothem",
            |_self: Ref<::bevy_math::primitives::CircularSector>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::CircularSector::apothem(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the length of the apothem of this sector\n See [`Arc2d::apothem`]",
            &["_self"],
        )
        .register_documented(
            "arc_length",
            |_self: Ref<::bevy_math::primitives::CircularSector>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::CircularSector::arc_length(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the length of the arc defining the sector",
            &["_self"],
        )
        .register_documented(
            "chord_length",
            |_self: Ref<::bevy_math::primitives::CircularSector>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::CircularSector::chord_length(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the length of the chord defined by the sector\n See [`Arc2d::chord_length`]",
            &["_self"],
        )
        .register_documented(
            "chord_midpoint",
            |_self: Ref<::bevy_math::primitives::CircularSector>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::primitives::CircularSector::chord_midpoint(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the midpoint of the chord defined by the sector\n See [`Arc2d::chord_midpoint`]",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::CircularSector>| {
                let output: Val<::bevy_math::primitives::CircularSector> = {
                    {
                        let output: Val<::bevy_math::primitives::CircularSector> = <::bevy_math::primitives::CircularSector as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_math::primitives::CircularSector>,
                other: Ref<::bevy_math::primitives::CircularSector>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::CircularSector as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::CircularSector,
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
            "from_degrees",
            |radius: f32, angle: f32| {
                let output: Val<::bevy_math::primitives::CircularSector> = {
                    {
                        let output: Val<::bevy_math::primitives::CircularSector> = ::bevy_math::primitives::CircularSector::from_degrees(
                                radius,
                                angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`CircularSector`] from a `radius` and an `angle` in degrees.",
            &["radius", "angle"],
        )
        .register_documented(
            "from_radians",
            |radius: f32, angle: f32| {
                let output: Val<::bevy_math::primitives::CircularSector> = {
                    {
                        let output: Val<::bevy_math::primitives::CircularSector> = ::bevy_math::primitives::CircularSector::from_radians(
                                radius,
                                angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`CircularSector`] from a `radius` and an `angle` in radians.",
            &["radius", "angle"],
        )
        .register_documented(
            "from_turns",
            |radius: f32, fraction: f32| {
                let output: Val<::bevy_math::primitives::CircularSector> = {
                    {
                        let output: Val<::bevy_math::primitives::CircularSector> = ::bevy_math::primitives::CircularSector::from_turns(
                                radius,
                                fraction,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`CircularSector`] from a `radius` and a number of `turns` of a circle.\n For instance, `0.5` turns is a semicircle.",
            &["radius", "fraction"],
        )
        .register_documented(
            "half_angle",
            |_self: Ref<::bevy_math::primitives::CircularSector>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::CircularSector::half_angle(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get half the angle of the sector",
            &["_self"],
        )
        .register_documented(
            "half_chord_length",
            |_self: Ref<::bevy_math::primitives::CircularSector>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::CircularSector::half_chord_length(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get half the length of the chord defined by the sector\n See [`Arc2d::half_chord_length`]",
            &["_self"],
        )
        .register_documented(
            "new",
            |radius: f32, angle: f32| {
                let output: Val<::bevy_math::primitives::CircularSector> = {
                    {
                        let output: Val<::bevy_math::primitives::CircularSector> = ::bevy_math::primitives::CircularSector::new(
                                radius,
                                angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`CircularSector`] from a `radius` and an `angle`",
            &["radius", "angle"],
        )
        .register_documented(
            "radius",
            |_self: Ref<::bevy_math::primitives::CircularSector>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::CircularSector::radius(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the radius of the sector",
            &["_self"],
        )
        .register_documented(
            "sagitta",
            |_self: Ref<::bevy_math::primitives::CircularSector>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::CircularSector::sagitta(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the length of the sagitta of this sector\n See [`Arc2d::sagitta`]",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::CircularSector,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_circular_segment_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::CircularSegment,
    >::new(world)
        .register_documented(
            "angle",
            |_self: Ref<::bevy_math::primitives::CircularSegment>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::CircularSegment::angle(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the angle of the segment",
            &["_self"],
        )
        .register_documented(
            "apothem",
            |_self: Ref<::bevy_math::primitives::CircularSegment>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::CircularSegment::apothem(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the length of the apothem of this segment,\n which is the signed distance between the segment and the center of its circle\n See [`Arc2d::apothem`]",
            &["_self"],
        )
        .register_documented(
            "arc_length",
            |_self: Ref<::bevy_math::primitives::CircularSegment>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::CircularSegment::arc_length(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the length of the arc defining the segment",
            &["_self"],
        )
        .register_documented(
            "chord_length",
            |_self: Ref<::bevy_math::primitives::CircularSegment>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::CircularSegment::chord_length(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the length of the segment's base, also known as its chord",
            &["_self"],
        )
        .register_documented(
            "chord_midpoint",
            |_self: Ref<::bevy_math::primitives::CircularSegment>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::primitives::CircularSegment::chord_midpoint(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the midpoint of the segment's base, also known as its chord",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::CircularSegment>| {
                let output: Val<::bevy_math::primitives::CircularSegment> = {
                    {
                        let output: Val<::bevy_math::primitives::CircularSegment> = <::bevy_math::primitives::CircularSegment as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_math::primitives::CircularSegment>,
                other: Ref<::bevy_math::primitives::CircularSegment>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::CircularSegment as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::CircularSegment,
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
            "from_degrees",
            |radius: f32, angle: f32| {
                let output: Val<::bevy_math::primitives::CircularSegment> = {
                    {
                        let output: Val<::bevy_math::primitives::CircularSegment> = ::bevy_math::primitives::CircularSegment::from_degrees(
                                radius,
                                angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`CircularSegment`] from a `radius` and an `angle` in degrees.",
            &["radius", "angle"],
        )
        .register_documented(
            "from_radians",
            |radius: f32, angle: f32| {
                let output: Val<::bevy_math::primitives::CircularSegment> = {
                    {
                        let output: Val<::bevy_math::primitives::CircularSegment> = ::bevy_math::primitives::CircularSegment::from_radians(
                                radius,
                                angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`CircularSegment`] from a `radius` and an `angle` in radians.",
            &["radius", "angle"],
        )
        .register_documented(
            "from_turns",
            |radius: f32, fraction: f32| {
                let output: Val<::bevy_math::primitives::CircularSegment> = {
                    {
                        let output: Val<::bevy_math::primitives::CircularSegment> = ::bevy_math::primitives::CircularSegment::from_turns(
                                radius,
                                fraction,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`CircularSegment`] from a `radius` and a number of `turns` of a circle.\n For instance, `0.5` turns is a semicircle.",
            &["radius", "fraction"],
        )
        .register_documented(
            "half_angle",
            |_self: Ref<::bevy_math::primitives::CircularSegment>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::CircularSegment::half_angle(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the half-angle of the segment",
            &["_self"],
        )
        .register_documented(
            "half_chord_length",
            |_self: Ref<::bevy_math::primitives::CircularSegment>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::CircularSegment::half_chord_length(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get half the length of the segment's base, also known as its chord",
            &["_self"],
        )
        .register_documented(
            "new",
            |radius: f32, angle: f32| {
                let output: Val<::bevy_math::primitives::CircularSegment> = {
                    {
                        let output: Val<::bevy_math::primitives::CircularSegment> = ::bevy_math::primitives::CircularSegment::new(
                                radius,
                                angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`CircularSegment`] from a `radius`, and an `angle`",
            &["radius", "angle"],
        )
        .register_documented(
            "radius",
            |_self: Ref<::bevy_math::primitives::CircularSegment>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::CircularSegment::radius(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the radius of the segment",
            &["_self"],
        )
        .register_documented(
            "sagitta",
            |_self: Ref<::bevy_math::primitives::CircularSegment>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::CircularSegment::sagitta(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the length of the sagitta of this segment, also known as its height\n See [`Arc2d::sagitta`]",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::CircularSegment,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ellipse_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Ellipse,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::Ellipse>| {
                let output: Val<::bevy_math::primitives::Ellipse> = {
                    {
                        let output: Val<::bevy_math::primitives::Ellipse> = <::bevy_math::primitives::Ellipse as ::core::clone::Clone>::clone(
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
            "eccentricity",
            |_self: Ref<::bevy_math::primitives::Ellipse>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Ellipse::eccentricity(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the [eccentricity](https://en.wikipedia.org/wiki/Eccentricity_(mathematics)) of the ellipse.\n It can be thought of as a measure of how \"stretched\" or elongated the ellipse is.\n The value should be in the range [0, 1), where 0 represents a circle, and 1 represents a parabola.",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_math::primitives::Ellipse>,
                other: Ref<::bevy_math::primitives::Ellipse>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::Ellipse as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Ellipse,
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
            "focal_length",
            |_self: Ref<::bevy_math::primitives::Ellipse>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Ellipse::focal_length(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the focal length of the ellipse. This corresponds to the distance between one of the foci and the center of the ellipse.\n The focal length of an ellipse is related to its eccentricity by `eccentricity = focal_length / semi_major`",
            &["_self"],
        )
        .register_documented(
            "from_size",
            |size: Val<::bevy_math::prelude::Vec2>| {
                let output: Val<::bevy_math::primitives::Ellipse> = {
                    {
                        let output: Val<::bevy_math::primitives::Ellipse> = ::bevy_math::primitives::Ellipse::from_size(
                                size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Ellipse` from a given full size.\n `size.x` is the diameter along the X axis, and `size.y` is the diameter along the Y axis.",
            &["size"],
        )
        .register_documented(
            "new",
            |half_width: f32, half_height: f32| {
                let output: Val<::bevy_math::primitives::Ellipse> = {
                    {
                        let output: Val<::bevy_math::primitives::Ellipse> = ::bevy_math::primitives::Ellipse::new(
                                half_width,
                                half_height,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Ellipse` from half of its width and height.\n This corresponds to the two perpendicular radii defining the ellipse.",
            &["half_width", "half_height"],
        )
        .register_documented(
            "semi_major",
            |_self: Ref<::bevy_math::primitives::Ellipse>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Ellipse::semi_major(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the length of the semi-major axis. This corresponds to the longest radius of the ellipse.",
            &["_self"],
        )
        .register_documented(
            "semi_minor",
            |_self: Ref<::bevy_math::primitives::Ellipse>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Ellipse::semi_minor(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the length of the semi-minor axis. This corresponds to the shortest radius of the ellipse.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Ellipse,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_line_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Line2d,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_math::primitives::Line2d>| {
            let output: Val<::bevy_math::primitives::Line2d> = {
                {
                    let output: Val<::bevy_math::primitives::Line2d> =
                        <::bevy_math::primitives::Line2d as ::core::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_math::primitives::Line2d>,
         other: Ref<::bevy_math::primitives::Line2d>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_math::primitives::Line2d as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Line2d,
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
            ::bevy_math::primitives::Line2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_plane_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Plane2d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::Plane2d>| {
                let output: Val<::bevy_math::primitives::Plane2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Plane2d> = <::bevy_math::primitives::Plane2d as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_math::primitives::Plane2d>,
                other: Ref<::bevy_math::primitives::Plane2d>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::Plane2d as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Plane2d,
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
            "new",
            |normal: Val<::bevy_math::prelude::Vec2>| {
                let output: Val<::bevy_math::primitives::Plane2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Plane2d> = ::bevy_math::primitives::Plane2d::new(
                                normal.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Plane2d` from a normal\n # Panics\n Panics if the given `normal` is zero (or very close to zero), or non-finite.",
            &["normal"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Plane2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_rectangle_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Rectangle,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::Rectangle>| {
                let output: Val<::bevy_math::primitives::Rectangle> = {
                    {
                        let output: Val<::bevy_math::primitives::Rectangle> = <::bevy_math::primitives::Rectangle as ::core::clone::Clone>::clone(
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
            "closest_point",
            |
                _self: Ref<::bevy_math::primitives::Rectangle>,
                point: Val<::bevy_math::prelude::Vec2>|
            {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::primitives::Rectangle::closest_point(
                                &_self,
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Finds the point on the rectangle that is closest to the given `point`.\n If the point is outside the rectangle, the returned point will be on the perimeter of the rectangle.\n Otherwise, it will be inside the rectangle and returned as is.",
            &["_self", "point"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_math::primitives::Rectangle>,
                other: Ref<::bevy_math::primitives::Rectangle>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::Rectangle as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Rectangle,
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
            "from_corners",
            |
                point1: Val<::bevy_math::prelude::Vec2>,
                point2: Val<::bevy_math::prelude::Vec2>|
            {
                let output: Val<::bevy_math::primitives::Rectangle> = {
                    {
                        let output: Val<::bevy_math::primitives::Rectangle> = ::bevy_math::primitives::Rectangle::from_corners(
                                point1.into_inner(),
                                point2.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Rectangle` from two corner points",
            &["point1", "point2"],
        )
        .register_documented(
            "from_length",
            |length: f32| {
                let output: Val<::bevy_math::primitives::Rectangle> = {
                    {
                        let output: Val<::bevy_math::primitives::Rectangle> = ::bevy_math::primitives::Rectangle::from_length(
                                length,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a `Rectangle` from a single length.\n The resulting `Rectangle` will be the same size in every direction.",
            &["length"],
        )
        .register_documented(
            "from_size",
            |size: Val<::bevy_math::prelude::Vec2>| {
                let output: Val<::bevy_math::primitives::Rectangle> = {
                    {
                        let output: Val<::bevy_math::primitives::Rectangle> = ::bevy_math::primitives::Rectangle::from_size(
                                size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Rectangle` from a given full size",
            &["size"],
        )
        .register_documented(
            "new",
            |width: f32, height: f32| {
                let output: Val<::bevy_math::primitives::Rectangle> = {
                    {
                        let output: Val<::bevy_math::primitives::Rectangle> = ::bevy_math::primitives::Rectangle::new(
                                width,
                                height,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Rectangle` from a full width and height",
            &["width", "height"],
        )
        .register_documented(
            "size",
            |_self: Ref<::bevy_math::primitives::Rectangle>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::primitives::Rectangle::size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the size of the rectangle",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Rectangle,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_regular_polygon_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::RegularPolygon,
    >::new(world)
        .register_documented(
            "circumradius",
            |_self: Ref<::bevy_math::primitives::RegularPolygon>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::RegularPolygon::circumradius(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the radius of the circumcircle on which all vertices\n of the regular polygon lie",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::RegularPolygon>| {
                let output: Val<::bevy_math::primitives::RegularPolygon> = {
                    {
                        let output: Val<::bevy_math::primitives::RegularPolygon> = <::bevy_math::primitives::RegularPolygon as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_math::primitives::RegularPolygon>,
                other: Ref<::bevy_math::primitives::RegularPolygon>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::RegularPolygon as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::RegularPolygon,
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
            "external_angle_degrees",
            |_self: Ref<::bevy_math::primitives::RegularPolygon>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::RegularPolygon::external_angle_degrees(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the external angle of the regular polygon in degrees.\n This is the angle formed by two adjacent sides with points\n within the angle being in the exterior of the polygon",
            &["_self"],
        )
        .register_documented(
            "external_angle_radians",
            |_self: Ref<::bevy_math::primitives::RegularPolygon>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::RegularPolygon::external_angle_radians(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the external angle of the regular polygon in radians.\n This is the angle formed by two adjacent sides with points\n within the angle being in the exterior of the polygon",
            &["_self"],
        )
        .register_documented(
            "inradius",
            |_self: Ref<::bevy_math::primitives::RegularPolygon>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::RegularPolygon::inradius(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the inradius or apothem of the regular polygon.\n This is the radius of the largest circle that can\n be drawn within the polygon",
            &["_self"],
        )
        .register_documented(
            "internal_angle_degrees",
            |_self: Ref<::bevy_math::primitives::RegularPolygon>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::RegularPolygon::internal_angle_degrees(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the internal angle of the regular polygon in degrees.\n This is the angle formed by two adjacent sides with points\n within the angle being in the interior of the polygon",
            &["_self"],
        )
        .register_documented(
            "internal_angle_radians",
            |_self: Ref<::bevy_math::primitives::RegularPolygon>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::RegularPolygon::internal_angle_radians(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the internal angle of the regular polygon in radians.\n This is the angle formed by two adjacent sides with points\n within the angle being in the interior of the polygon",
            &["_self"],
        )
        .register_documented(
            "new",
            |circumradius: f32, sides: u32| {
                let output: Val<::bevy_math::primitives::RegularPolygon> = {
                    {
                        let output: Val<::bevy_math::primitives::RegularPolygon> = ::bevy_math::primitives::RegularPolygon::new(
                                circumradius,
                                sides,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `RegularPolygon`\n from the radius of the circumcircle and a number of sides\n # Panics\n Panics if `circumradius` is negative",
            &["circumradius", "sides"],
        )
        .register_documented(
            "side_length",
            |_self: Ref<::bevy_math::primitives::RegularPolygon>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::RegularPolygon::side_length(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the length of one side of the regular polygon",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::RegularPolygon,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_rhombus_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Rhombus,
    >::new(world)
        .register_documented(
            "circumradius",
            |_self: Ref<::bevy_math::primitives::Rhombus>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Rhombus::circumradius(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the radius of the circumcircle on which all vertices\n of the rhombus lie",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::Rhombus>| {
                let output: Val<::bevy_math::primitives::Rhombus> = {
                    {
                        let output: Val<::bevy_math::primitives::Rhombus> = <::bevy_math::primitives::Rhombus as ::core::clone::Clone>::clone(
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
            "closest_point",
            |
                _self: Ref<::bevy_math::primitives::Rhombus>,
                point: Val<::bevy_math::prelude::Vec2>|
            {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::primitives::Rhombus::closest_point(
                                &_self,
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Finds the point on the rhombus that is closest to the given `point`.\n If the point is outside the rhombus, the returned point will be on the perimeter of the rhombus.\n Otherwise, it will be inside the rhombus and returned as is.",
            &["_self", "point"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_math::primitives::Rhombus>,
                other: Ref<::bevy_math::primitives::Rhombus>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::Rhombus as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Rhombus,
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
            "from_inradius",
            |inradius: f32| {
                let output: Val<::bevy_math::primitives::Rhombus> = {
                    {
                        let output: Val<::bevy_math::primitives::Rhombus> = ::bevy_math::primitives::Rhombus::from_inradius(
                                inradius,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Rhombus` from a given inradius with all inner angles equal.",
            &["inradius"],
        )
        .register_documented(
            "from_side",
            |side: f32| {
                let output: Val<::bevy_math::primitives::Rhombus> = {
                    {
                        let output: Val<::bevy_math::primitives::Rhombus> = ::bevy_math::primitives::Rhombus::from_side(
                                side,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Rhombus` from a side length with all inner angles equal.",
            &["side"],
        )
        .register_documented(
            "inradius",
            |_self: Ref<::bevy_math::primitives::Rhombus>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Rhombus::inradius(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the radius of the largest circle that can\n be drawn within the rhombus",
            &["_self"],
        )
        .register_documented(
            "new",
            |horizontal_diagonal: f32, vertical_diagonal: f32| {
                let output: Val<::bevy_math::primitives::Rhombus> = {
                    {
                        let output: Val<::bevy_math::primitives::Rhombus> = ::bevy_math::primitives::Rhombus::new(
                                horizontal_diagonal,
                                vertical_diagonal,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Rhombus` from a vertical and horizontal diagonal sizes.",
            &["horizontal_diagonal", "vertical_diagonal"],
        )
        .register_documented(
            "side",
            |_self: Ref<::bevy_math::primitives::Rhombus>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Rhombus::side(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Get the length of each side of the rhombus",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Rhombus,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_segment_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Segment2d,
    >::new(world)
        .register_documented(
            "center",
            |_self: Ref<::bevy_math::primitives::Segment2d>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::primitives::Segment2d::center(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the midpoint between the two endpoints of the line segment.",
            &["_self"],
        )
        .register_documented(
            "centered",
            |_self: Ref<::bevy_math::primitives::Segment2d>| {
                let output: Val<::bevy_math::primitives::Segment2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment2d> = ::bevy_math::primitives::Segment2d::centered(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the segment with its center at the origin, keeping the same direction and length.",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::Segment2d>| {
                let output: Val<::bevy_math::primitives::Segment2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment2d> = <::bevy_math::primitives::Segment2d as ::core::clone::Clone>::clone(
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
            "direction",
            |_self: Ref<::bevy_math::primitives::Segment2d>| {
                let output: Val<::bevy_math::prelude::Dir2> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir2> = ::bevy_math::primitives::Segment2d::direction(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the normalized direction pointing from the first endpoint to the second endpoint.\n For the non-panicking version, see [`Segment2d::try_direction`].\n # Panics\n Panics if a valid direction could not be computed, for example when the endpoints are coincident, NaN, or infinite.",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_math::primitives::Segment2d>,
                other: Ref<::bevy_math::primitives::Segment2d>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::Segment2d as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Segment2d,
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
            "from_direction_and_length",
            |direction: Val<::bevy_math::prelude::Dir2>, length: f32| {
                let output: Val<::bevy_math::primitives::Segment2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment2d> = ::bevy_math::primitives::Segment2d::from_direction_and_length(
                                direction.into_inner(),
                                length,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Segment2d` centered at the origin with the given direction and length.\n The endpoints will be at `-direction * length / 2.0` and `direction * length / 2.0`.",
            &["direction", "length"],
        )
        .register_documented(
            "from_ray_and_length",
            |ray: Val<::bevy_math::Ray2d>, length: f32| {
                let output: Val<::bevy_math::primitives::Segment2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment2d> = ::bevy_math::primitives::Segment2d::from_ray_and_length(
                                ray.into_inner(),
                                length,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Segment2d` starting from the origin of the given `ray`,\n going in the direction of the ray for the given `length`.\n The endpoints will be at `ray.origin` and `ray.origin + length * ray.direction`.",
            &["ray", "length"],
        )
        .register_documented(
            "from_scaled_direction",
            |scaled_direction: Val<::bevy_math::prelude::Vec2>| {
                let output: Val<::bevy_math::primitives::Segment2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment2d> = ::bevy_math::primitives::Segment2d::from_scaled_direction(
                                scaled_direction.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Segment2d` centered at the origin from a vector representing\n the direction and length of the line segment.\n The endpoints will be at `-scaled_direction / 2.0` and `scaled_direction / 2.0`.",
            &["scaled_direction"],
        )
        .register_documented(
            "left_normal",
            |_self: Ref<::bevy_math::primitives::Segment2d>| {
                let output: Val<::bevy_math::prelude::Dir2> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir2> = ::bevy_math::primitives::Segment2d::left_normal(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the normalized counterclockwise normal on the left-hand side of the line segment.\n For the non-panicking version, see [`Segment2d::try_left_normal`].\n # Panics\n Panics if a valid normal could not be computed, for example when the endpoints are coincident, NaN, or infinite.",
            &["_self"],
        )
        .register_documented(
            "length",
            |_self: Ref<::bevy_math::primitives::Segment2d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Segment2d::length(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the length of the line segment.",
            &["_self"],
        )
        .register_documented(
            "length_squared",
            |_self: Ref<::bevy_math::primitives::Segment2d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Segment2d::length_squared(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the squared length of the line segment.",
            &["_self"],
        )
        .register_documented(
            "new",
            |
                point1: Val<::bevy_math::prelude::Vec2>,
                point2: Val<::bevy_math::prelude::Vec2>|
            {
                let output: Val<::bevy_math::primitives::Segment2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment2d> = ::bevy_math::primitives::Segment2d::new(
                                point1.into_inner(),
                                point2.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Segment2d` from its endpoints.",
            &["point1", "point2"],
        )
        .register_documented(
            "point1",
            |_self: Ref<::bevy_math::primitives::Segment2d>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::primitives::Segment2d::point1(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the position of the first endpoint of the line segment.",
            &["_self"],
        )
        .register_documented(
            "point2",
            |_self: Ref<::bevy_math::primitives::Segment2d>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::primitives::Segment2d::point2(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the position of the second endpoint of the line segment.",
            &["_self"],
        )
        .register_documented(
            "resized",
            |_self: Ref<::bevy_math::primitives::Segment2d>, length: f32| {
                let output: Val<::bevy_math::primitives::Segment2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment2d> = ::bevy_math::primitives::Segment2d::resized(
                                &_self,
                                length,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the segment with a new length, keeping the same direction and center.",
            &["_self", "length"],
        )
        .register_documented(
            "reverse",
            |mut _self: Mut<::bevy_math::primitives::Segment2d>| {
                let output: () = {
                    {
                        let output: () = ::bevy_math::primitives::Segment2d::reverse(
                                &mut _self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Reverses the direction of the line segment by swapping the endpoints.",
            &["_self"],
        )
        .register_documented(
            "reversed",
            |_self: Val<::bevy_math::primitives::Segment2d>| {
                let output: Val<::bevy_math::primitives::Segment2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment2d> = ::bevy_math::primitives::Segment2d::reversed(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the line segment with its direction reversed by swapping the endpoints.",
            &["_self"],
        )
        .register_documented(
            "right_normal",
            |_self: Ref<::bevy_math::primitives::Segment2d>| {
                let output: Val<::bevy_math::prelude::Dir2> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir2> = ::bevy_math::primitives::Segment2d::right_normal(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the normalized clockwise normal on the right-hand side of the line segment.\n For the non-panicking version, see [`Segment2d::try_right_normal`].\n # Panics\n Panics if a valid normal could not be computed, for example when the endpoints are coincident, NaN, or infinite.",
            &["_self"],
        )
        .register_documented(
            "rotated",
            |
                _self: Ref<::bevy_math::primitives::Segment2d>,
                rotation: Val<::bevy_math::Rot2>|
            {
                let output: Val<::bevy_math::primitives::Segment2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment2d> = ::bevy_math::primitives::Segment2d::rotated(
                                &_self,
                                rotation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the segment rotated around the origin by the given rotation.",
            &["_self", "rotation"],
        )
        .register_documented(
            "rotated_around",
            |
                _self: Ref<::bevy_math::primitives::Segment2d>,
                rotation: Val<::bevy_math::Rot2>,
                point: Val<::bevy_math::prelude::Vec2>|
            {
                let output: Val<::bevy_math::primitives::Segment2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment2d> = ::bevy_math::primitives::Segment2d::rotated_around(
                                &_self,
                                rotation.into_inner(),
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the segment rotated around the given point by the given rotation.",
            &["_self", "rotation", "point"],
        )
        .register_documented(
            "rotated_around_center",
            |
                _self: Ref<::bevy_math::primitives::Segment2d>,
                rotation: Val<::bevy_math::Rot2>|
            {
                let output: Val<::bevy_math::primitives::Segment2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment2d> = ::bevy_math::primitives::Segment2d::rotated_around_center(
                                &_self,
                                rotation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the segment rotated around its own center.",
            &["_self", "rotation"],
        )
        .register_documented(
            "scaled_direction",
            |_self: Ref<::bevy_math::primitives::Segment2d>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::primitives::Segment2d::scaled_direction(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the vector from the first endpoint to the second endpoint.",
            &["_self"],
        )
        .register_documented(
            "scaled_left_normal",
            |_self: Ref<::bevy_math::primitives::Segment2d>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::primitives::Segment2d::scaled_left_normal(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the non-normalized counterclockwise normal on the left-hand side of the line segment.\n The length of the normal is the distance between the endpoints.",
            &["_self"],
        )
        .register_documented(
            "scaled_right_normal",
            |_self: Ref<::bevy_math::primitives::Segment2d>| {
                let output: Val<::bevy_math::prelude::Vec2> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec2> = ::bevy_math::primitives::Segment2d::scaled_right_normal(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the non-normalized clockwise normal on the right-hand side of the line segment.\n The length of the normal is the distance between the endpoints.",
            &["_self"],
        )
        .register_documented(
            "translated",
            |
                _self: Ref<::bevy_math::primitives::Segment2d>,
                translation: Val<::bevy_math::prelude::Vec2>|
            {
                let output: Val<::bevy_math::primitives::Segment2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment2d> = ::bevy_math::primitives::Segment2d::translated(
                                &_self,
                                translation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the segment translated by the given vector.",
            &["_self", "translation"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Segment2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_triangle_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Triangle2d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::Triangle2d>| {
                let output: Val<::bevy_math::primitives::Triangle2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Triangle2d> = <::bevy_math::primitives::Triangle2d as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_math::primitives::Triangle2d>,
                other: Ref<::bevy_math::primitives::Triangle2d>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::Triangle2d as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Triangle2d,
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
            "is_acute",
            |_self: Ref<::bevy_math::primitives::Triangle2d>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::primitives::Triangle2d::is_acute(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Checks if the triangle is acute, meaning all angles are less than 90 degrees",
            &["_self"],
        )
        .register_documented(
            "is_degenerate",
            |_self: Ref<::bevy_math::primitives::Triangle2d>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::primitives::Triangle2d::is_degenerate(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Checks if the triangle is degenerate, meaning it has zero area.\n A triangle is degenerate if the cross product of the vectors `ab` and `ac` has a length less than `10e-7`.\n This indicates that the three vertices are collinear or nearly collinear.",
            &["_self"],
        )
        .register_documented(
            "is_obtuse",
            |_self: Ref<::bevy_math::primitives::Triangle2d>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::primitives::Triangle2d::is_obtuse(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Checks if the triangle is obtuse, meaning one angle is greater than 90 degrees",
            &["_self"],
        )
        .register_documented(
            "new",
            |
                a: Val<::bevy_math::prelude::Vec2>,
                b: Val<::bevy_math::prelude::Vec2>,
                c: Val<::bevy_math::prelude::Vec2>|
            {
                let output: Val<::bevy_math::primitives::Triangle2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Triangle2d> = ::bevy_math::primitives::Triangle2d::new(
                                a.into_inner(),
                                b.into_inner(),
                                c.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Triangle2d` from points `a`, `b`, and `c`",
            &["a", "b", "c"],
        )
        .register_documented(
            "reverse",
            |mut _self: Mut<::bevy_math::primitives::Triangle2d>| {
                let output: () = {
                    {
                        let output: () = ::bevy_math::primitives::Triangle2d::reverse(
                                &mut _self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Reverse the [`WindingOrder`] of the triangle\n by swapping the first and last vertices.",
            &["_self"],
        )
        .register_documented(
            "reversed",
            |_self: Val<::bevy_math::primitives::Triangle2d>| {
                let output: Val<::bevy_math::primitives::Triangle2d> = {
                    {
                        let output: Val<::bevy_math::primitives::Triangle2d> = ::bevy_math::primitives::Triangle2d::reversed(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " This triangle but reversed.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Triangle2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_aabb_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::bounding::Aabb3d,
    >::new(world)
    .register_documented(
        "bounding_sphere",
        |_self: Ref<::bevy_math::bounding::Aabb3d>| {
            let output: Val<::bevy_math::bounding::BoundingSphere> = {
                {
                    let output: Val<::bevy_math::bounding::BoundingSphere> =
                        ::bevy_math::bounding::Aabb3d::bounding_sphere(&_self).into();
                    output
                }
            };
            output
        },
        " Computes the smallest [`BoundingSphere`] containing this [`Aabb3d`].",
        &["_self"],
    )
    .register_documented(
        "clone",
        |_self: Ref<::bevy_math::bounding::Aabb3d>| {
            let output: Val<::bevy_math::bounding::Aabb3d> = {
                {
                    let output: Val<::bevy_math::bounding::Aabb3d> =
                        <::bevy_math::bounding::Aabb3d as ::core::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_math::bounding::Aabb3d>, other: Ref<::bevy_math::bounding::Aabb3d>| {
            let output: bool = {
                {
                    let output: bool = <::bevy_math::bounding::Aabb3d as ::core::cmp::PartialEq<
                        ::bevy_math::bounding::Aabb3d,
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
            ::bevy_math::bounding::Aabb3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_bounding_sphere_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::bounding::BoundingSphere,
    >::new(world)
    .register_documented(
        "aabb_3d",
        |_self: Ref<::bevy_math::bounding::BoundingSphere>| {
            let output: Val<::bevy_math::bounding::Aabb3d> = {
                {
                    let output: Val<::bevy_math::bounding::Aabb3d> =
                        ::bevy_math::bounding::BoundingSphere::aabb_3d(&_self).into();
                    output
                }
            };
            output
        },
        " Computes the smallest [`Aabb3d`] containing this [`BoundingSphere`].",
        &["_self"],
    )
    .register_documented(
        "clone",
        |_self: Ref<::bevy_math::bounding::BoundingSphere>| {
            let output: Val<::bevy_math::bounding::BoundingSphere> = {
                {
                    let output: Val<::bevy_math::bounding::BoundingSphere> =
                        <::bevy_math::bounding::BoundingSphere as ::core::clone::Clone>::clone(
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
        |_self: Ref<::bevy_math::bounding::BoundingSphere>,
         other: Ref<::bevy_math::bounding::BoundingSphere>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_math::bounding::BoundingSphere as ::core::cmp::PartialEq<
                            ::bevy_math::bounding::BoundingSphere,
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
        "radius",
        |_self: Ref<::bevy_math::bounding::BoundingSphere>| {
            let output: f32 = {
                {
                    let output: f32 = ::bevy_math::bounding::BoundingSphere::radius(&_self).into();
                    output
                }
            };
            output
        },
        " Get the radius of the bounding sphere",
        &["_self"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::bounding::BoundingSphere,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_sphere_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Sphere,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::Sphere>| {
                let output: Val<::bevy_math::primitives::Sphere> = {
                    {
                        let output: Val<::bevy_math::primitives::Sphere> = <::bevy_math::primitives::Sphere as ::core::clone::Clone>::clone(
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
            "closest_point",
            |
                _self: Ref<::bevy_math::primitives::Sphere>,
                point: Val<::bevy_math::prelude::Vec3>|
            {
                let output: Val<::bevy_math::prelude::Vec3> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec3> = ::bevy_math::primitives::Sphere::closest_point(
                                &_self,
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Finds the point on the sphere that is closest to the given `point`.\n If the point is outside the sphere, the returned point will be on the surface of the sphere.\n Otherwise, it will be inside the sphere and returned as is.",
            &["_self", "point"],
        )
        .register_documented(
            "diameter",
            |_self: Ref<::bevy_math::primitives::Sphere>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Sphere::diameter(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the diameter of the sphere",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_math::primitives::Sphere>,
                other: Ref<::bevy_math::primitives::Sphere>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::Sphere as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Sphere,
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
            "new",
            |radius: f32| {
                let output: Val<::bevy_math::primitives::Sphere> = {
                    {
                        let output: Val<::bevy_math::primitives::Sphere> = ::bevy_math::primitives::Sphere::new(
                                radius,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`Sphere`] from a `radius`",
            &["radius"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Sphere,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cuboid_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Cuboid,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::Cuboid>| {
                let output: Val<::bevy_math::primitives::Cuboid> = {
                    {
                        let output: Val<::bevy_math::primitives::Cuboid> = <::bevy_math::primitives::Cuboid as ::core::clone::Clone>::clone(
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
            "closest_point",
            |
                _self: Ref<::bevy_math::primitives::Cuboid>,
                point: Val<::bevy_math::prelude::Vec3>|
            {
                let output: Val<::bevy_math::prelude::Vec3> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec3> = ::bevy_math::primitives::Cuboid::closest_point(
                                &_self,
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Finds the point on the cuboid that is closest to the given `point`.\n If the point is outside the cuboid, the returned point will be on the surface of the cuboid.\n Otherwise, it will be inside the cuboid and returned as is.",
            &["_self", "point"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_math::primitives::Cuboid>,
                other: Ref<::bevy_math::primitives::Cuboid>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::Cuboid as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Cuboid,
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
            "from_corners",
            |
                point1: Val<::bevy_math::prelude::Vec3>,
                point2: Val<::bevy_math::prelude::Vec3>|
            {
                let output: Val<::bevy_math::primitives::Cuboid> = {
                    {
                        let output: Val<::bevy_math::primitives::Cuboid> = ::bevy_math::primitives::Cuboid::from_corners(
                                point1.into_inner(),
                                point2.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Cuboid` from two corner points",
            &["point1", "point2"],
        )
        .register_documented(
            "from_length",
            |length: f32| {
                let output: Val<::bevy_math::primitives::Cuboid> = {
                    {
                        let output: Val<::bevy_math::primitives::Cuboid> = ::bevy_math::primitives::Cuboid::from_length(
                                length,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a `Cuboid` from a single length.\n The resulting `Cuboid` will be the same size in every direction.",
            &["length"],
        )
        .register_documented(
            "from_size",
            |size: Val<::bevy_math::prelude::Vec3>| {
                let output: Val<::bevy_math::primitives::Cuboid> = {
                    {
                        let output: Val<::bevy_math::primitives::Cuboid> = ::bevy_math::primitives::Cuboid::from_size(
                                size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Cuboid` from a given full size",
            &["size"],
        )
        .register_documented(
            "new",
            |x_length: f32, y_length: f32, z_length: f32| {
                let output: Val<::bevy_math::primitives::Cuboid> = {
                    {
                        let output: Val<::bevy_math::primitives::Cuboid> = ::bevy_math::primitives::Cuboid::new(
                                x_length,
                                y_length,
                                z_length,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Cuboid` from a full x, y, and z length",
            &["x_length", "y_length", "z_length"],
        )
        .register_documented(
            "size",
            |_self: Ref<::bevy_math::primitives::Cuboid>| {
                let output: Val<::bevy_math::prelude::Vec3> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec3> = ::bevy_math::primitives::Cuboid::size(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the size of the cuboid",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Cuboid,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cylinder_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Cylinder,
    >::new(world)
    .register_documented(
        "base",
        |_self: Ref<::bevy_math::primitives::Cylinder>| {
            let output: Val<::bevy_math::primitives::Circle> = {
                {
                    let output: Val<::bevy_math::primitives::Circle> =
                        ::bevy_math::primitives::Cylinder::base(&_self).into();
                    output
                }
            };
            output
        },
        " Get the base of the cylinder as a [`Circle`]",
        &["_self"],
    )
    .register_documented(
        "base_area",
        |_self: Ref<::bevy_math::primitives::Cylinder>| {
            let output: f32 = {
                {
                    let output: f32 = ::bevy_math::primitives::Cylinder::base_area(&_self).into();
                    output
                }
            };
            output
        },
        " Get the surface area of one base of the cylinder",
        &["_self"],
    )
    .register_documented(
        "clone",
        |_self: Ref<::bevy_math::primitives::Cylinder>| {
            let output: Val<::bevy_math::primitives::Cylinder> = {
                {
                    let output: Val<::bevy_math::primitives::Cylinder> =
                        <::bevy_math::primitives::Cylinder as ::core::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_math::primitives::Cylinder>,
         other: Ref<::bevy_math::primitives::Cylinder>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_math::primitives::Cylinder as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Cylinder,
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
        "lateral_area",
        |_self: Ref<::bevy_math::primitives::Cylinder>| {
            let output: f32 = {
                {
                    let output: f32 =
                        ::bevy_math::primitives::Cylinder::lateral_area(&_self).into();
                    output
                }
            };
            output
        },
        " Get the surface area of the side of the cylinder,\n also known as the lateral area",
        &["_self"],
    )
    .register_documented(
        "new",
        |radius: f32, height: f32| {
            let output: Val<::bevy_math::primitives::Cylinder> = {
                {
                    let output: Val<::bevy_math::primitives::Cylinder> =
                        ::bevy_math::primitives::Cylinder::new(radius, height).into();
                    output
                }
            };
            output
        },
        " Create a new `Cylinder` from a radius and full height",
        &["radius", "height"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Cylinder,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_capsule_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Capsule3d,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_math::primitives::Capsule3d>| {
            let output: Val<::bevy_math::primitives::Capsule3d> = {
                {
                    let output: Val<::bevy_math::primitives::Capsule3d> =
                        <::bevy_math::primitives::Capsule3d as ::core::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_math::primitives::Capsule3d>,
         other: Ref<::bevy_math::primitives::Capsule3d>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_math::primitives::Capsule3d as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Capsule3d,
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
        "new",
        |radius: f32, length: f32| {
            let output: Val<::bevy_math::primitives::Capsule3d> = {
                {
                    let output: Val<::bevy_math::primitives::Capsule3d> =
                        ::bevy_math::primitives::Capsule3d::new(radius, length).into();
                    output
                }
            };
            output
        },
        " Create a new `Capsule3d` from a radius and length",
        &["radius", "length"],
    )
    .register_documented(
        "to_cylinder",
        |_self: Ref<::bevy_math::primitives::Capsule3d>| {
            let output: Val<::bevy_math::primitives::Cylinder> = {
                {
                    let output: Val<::bevy_math::primitives::Cylinder> =
                        ::bevy_math::primitives::Capsule3d::to_cylinder(&_self).into();
                    output
                }
            };
            output
        },
        " Get the part connecting the hemispherical ends\n of the capsule as a [`Cylinder`]",
        &["_self"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Capsule3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cone_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Cone,
    >::new(world)
        .register_documented(
            "base",
            |_self: Ref<::bevy_math::primitives::Cone>| {
                let output: Val<::bevy_math::primitives::Circle> = {
                    {
                        let output: Val<::bevy_math::primitives::Circle> = ::bevy_math::primitives::Cone::base(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the base of the cone as a [`Circle`]",
            &["_self"],
        )
        .register_documented(
            "base_area",
            |_self: Ref<::bevy_math::primitives::Cone>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Cone::base_area(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the surface area of the base of the cone",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::Cone>| {
                let output: Val<::bevy_math::primitives::Cone> = {
                    {
                        let output: Val<::bevy_math::primitives::Cone> = <::bevy_math::primitives::Cone as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_math::primitives::Cone>,
                other: Ref<::bevy_math::primitives::Cone>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::Cone as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Cone,
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
            "lateral_area",
            |_self: Ref<::bevy_math::primitives::Cone>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Cone::lateral_area(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the surface area of the side of the cone,\n also known as the lateral area",
            &["_self"],
        )
        .register_documented(
            "new",
            |radius: f32, height: f32| {
                let output: Val<::bevy_math::primitives::Cone> = {
                    {
                        let output: Val<::bevy_math::primitives::Cone> = ::bevy_math::primitives::Cone::new(
                                radius,
                                height,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`Cone`] from a radius and height.",
            &["radius", "height"],
        )
        .register_documented(
            "slant_height",
            |_self: Ref<::bevy_math::primitives::Cone>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Cone::slant_height(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the slant height of the cone, the length of the line segment\n connecting a point on the base to the apex",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Cone,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_conical_frustum_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::ConicalFrustum,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_math::primitives::ConicalFrustum>| {
            let output: Val<::bevy_math::primitives::ConicalFrustum> = {
                {
                    let output: Val<::bevy_math::primitives::ConicalFrustum> =
                        <::bevy_math::primitives::ConicalFrustum as ::core::clone::Clone>::clone(
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
        |_self: Ref<::bevy_math::primitives::ConicalFrustum>,
         other: Ref<::bevy_math::primitives::ConicalFrustum>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_math::primitives::ConicalFrustum as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::ConicalFrustum,
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
            ::bevy_math::primitives::ConicalFrustum,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_infinite_plane_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::InfinitePlane3d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::InfinitePlane3d>| {
                let output: Val<::bevy_math::primitives::InfinitePlane3d> = {
                    {
                        let output: Val<::bevy_math::primitives::InfinitePlane3d> = <::bevy_math::primitives::InfinitePlane3d as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_math::primitives::InfinitePlane3d>,
                other: Ref<::bevy_math::primitives::InfinitePlane3d>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::InfinitePlane3d as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::InfinitePlane3d,
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
            "isometry_from_xy",
            |
                _self: Ref<::bevy_math::primitives::InfinitePlane3d>,
                origin: Val<::bevy_math::prelude::Vec3>|
            {
                let output: Val<::bevy_math::Isometry3d> = {
                    {
                        let output: Val<::bevy_math::Isometry3d> = ::bevy_math::primitives::InfinitePlane3d::isometry_from_xy(
                                &_self,
                                origin.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Computes an [`Isometry3d`] which transforms points from the XY-plane to this plane with the\n given `origin`.\n ## Guarantees\n * the transformation is a [congruence] meaning it will preserve all distances and angles of\n   the transformed geometry\n * uses the least rotation possible to transform the geometry\n * if two geometries are transformed with the same isometry, then the relations between\n   them, like distances, are also preserved\n * compared to projections, the transformation is lossless (up to floating point errors)\n   reversible\n ## Non-Guarantees\n * the rotation used is generally not unique\n * the orientation of the transformed geometry in the XY plane might be arbitrary, to\n   enforce some kind of alignment the user has to use an extra transformation ontop of this\n   one\n See [`isometries_xy`] for example usescases.\n [congruence]: https://en.wikipedia.org/wiki/Congruence_(geometry)\n [`isometries_xy`]: `InfinitePlane3d::isometries_xy`",
            &["_self", "origin"],
        )
        .register_documented(
            "isometry_into_xy",
            |
                _self: Ref<::bevy_math::primitives::InfinitePlane3d>,
                origin: Val<::bevy_math::prelude::Vec3>|
            {
                let output: Val<::bevy_math::Isometry3d> = {
                    {
                        let output: Val<::bevy_math::Isometry3d> = ::bevy_math::primitives::InfinitePlane3d::isometry_into_xy(
                                &_self,
                                origin.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Computes an [`Isometry3d`] which transforms points from the plane in 3D space with the given\n `origin` to the XY-plane.\n ## Guarantees\n * the transformation is a [congruence] meaning it will preserve all distances and angles of\n   the transformed geometry\n * uses the least rotation possible to transform the geometry\n * if two geometries are transformed with the same isometry, then the relations between\n   them, like distances, are also preserved\n * compared to projections, the transformation is lossless (up to floating point errors)\n   reversible\n ## Non-Guarantees\n * the rotation used is generally not unique\n * the orientation of the transformed geometry in the XY plane might be arbitrary, to\n   enforce some kind of alignment the user has to use an extra transformation ontop of this\n   one\n See [`isometries_xy`] for example usescases.\n [congruence]: https://en.wikipedia.org/wiki/Congruence_(geometry)\n [`isometries_xy`]: `InfinitePlane3d::isometries_xy`",
            &["_self", "origin"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::InfinitePlane3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_line_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Line3d,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_math::primitives::Line3d>| {
            let output: Val<::bevy_math::primitives::Line3d> = {
                {
                    let output: Val<::bevy_math::primitives::Line3d> =
                        <::bevy_math::primitives::Line3d as ::core::clone::Clone>::clone(&_self)
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
        |_self: Ref<::bevy_math::primitives::Line3d>,
         other: Ref<::bevy_math::primitives::Line3d>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_math::primitives::Line3d as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Line3d,
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
            ::bevy_math::primitives::Line3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_segment_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Segment3d,
    >::new(world)
        .register_documented(
            "center",
            |_self: Ref<::bevy_math::primitives::Segment3d>| {
                let output: Val<::bevy_math::prelude::Vec3> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec3> = ::bevy_math::primitives::Segment3d::center(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the midpoint between the two endpoints of the line segment.",
            &["_self"],
        )
        .register_documented(
            "centered",
            |_self: Ref<::bevy_math::primitives::Segment3d>| {
                let output: Val<::bevy_math::primitives::Segment3d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment3d> = ::bevy_math::primitives::Segment3d::centered(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the segment with its center at the origin, keeping the same direction and length.",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::Segment3d>| {
                let output: Val<::bevy_math::primitives::Segment3d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment3d> = <::bevy_math::primitives::Segment3d as ::core::clone::Clone>::clone(
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
            "direction",
            |_self: Ref<::bevy_math::primitives::Segment3d>| {
                let output: Val<::bevy_math::prelude::Dir3> = {
                    {
                        let output: Val<::bevy_math::prelude::Dir3> = ::bevy_math::primitives::Segment3d::direction(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the normalized direction pointing from the first endpoint to the second endpoint.\n For the non-panicking version, see [`Segment3d::try_direction`].\n # Panics\n Panics if a valid direction could not be computed, for example when the endpoints are coincident, NaN, or infinite.",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_math::primitives::Segment3d>,
                other: Ref<::bevy_math::primitives::Segment3d>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::Segment3d as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Segment3d,
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
            "from_direction_and_length",
            |direction: Val<::bevy_math::prelude::Dir3>, length: f32| {
                let output: Val<::bevy_math::primitives::Segment3d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment3d> = ::bevy_math::primitives::Segment3d::from_direction_and_length(
                                direction.into_inner(),
                                length,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Segment3d` centered at the origin with the given direction and length.\n The endpoints will be at `-direction * length / 2.0` and `direction * length / 2.0`.",
            &["direction", "length"],
        )
        .register_documented(
            "from_ray_and_length",
            |ray: Val<::bevy_math::Ray3d>, length: f32| {
                let output: Val<::bevy_math::primitives::Segment3d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment3d> = ::bevy_math::primitives::Segment3d::from_ray_and_length(
                                ray.into_inner(),
                                length,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Segment3d` starting from the origin of the given `ray`,\n going in the direction of the ray for the given `length`.\n The endpoints will be at `ray.origin` and `ray.origin + length * ray.direction`.",
            &["ray", "length"],
        )
        .register_documented(
            "from_scaled_direction",
            |scaled_direction: Val<::bevy_math::prelude::Vec3>| {
                let output: Val<::bevy_math::primitives::Segment3d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment3d> = ::bevy_math::primitives::Segment3d::from_scaled_direction(
                                scaled_direction.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Segment3d` centered at the origin from a vector representing\n the direction and length of the line segment.\n The endpoints will be at `-scaled_direction / 2.0` and `scaled_direction / 2.0`.",
            &["scaled_direction"],
        )
        .register_documented(
            "length",
            |_self: Ref<::bevy_math::primitives::Segment3d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Segment3d::length(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the length of the line segment.",
            &["_self"],
        )
        .register_documented(
            "length_squared",
            |_self: Ref<::bevy_math::primitives::Segment3d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Segment3d::length_squared(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the squared length of the line segment.",
            &["_self"],
        )
        .register_documented(
            "new",
            |
                point1: Val<::bevy_math::prelude::Vec3>,
                point2: Val<::bevy_math::prelude::Vec3>|
            {
                let output: Val<::bevy_math::primitives::Segment3d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment3d> = ::bevy_math::primitives::Segment3d::new(
                                point1.into_inner(),
                                point2.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Segment3d` from its endpoints.",
            &["point1", "point2"],
        )
        .register_documented(
            "point1",
            |_self: Ref<::bevy_math::primitives::Segment3d>| {
                let output: Val<::bevy_math::prelude::Vec3> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec3> = ::bevy_math::primitives::Segment3d::point1(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the position of the first endpoint of the line segment.",
            &["_self"],
        )
        .register_documented(
            "point2",
            |_self: Ref<::bevy_math::primitives::Segment3d>| {
                let output: Val<::bevy_math::prelude::Vec3> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec3> = ::bevy_math::primitives::Segment3d::point2(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the position of the second endpoint of the line segment.",
            &["_self"],
        )
        .register_documented(
            "resized",
            |_self: Ref<::bevy_math::primitives::Segment3d>, length: f32| {
                let output: Val<::bevy_math::primitives::Segment3d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment3d> = ::bevy_math::primitives::Segment3d::resized(
                                &_self,
                                length,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the segment with a new length, keeping the same direction and center.",
            &["_self", "length"],
        )
        .register_documented(
            "reverse",
            |mut _self: Mut<::bevy_math::primitives::Segment3d>| {
                let output: () = {
                    {
                        let output: () = ::bevy_math::primitives::Segment3d::reverse(
                                &mut _self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Reverses the direction of the line segment by swapping the endpoints.",
            &["_self"],
        )
        .register_documented(
            "reversed",
            |_self: Val<::bevy_math::primitives::Segment3d>| {
                let output: Val<::bevy_math::primitives::Segment3d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment3d> = ::bevy_math::primitives::Segment3d::reversed(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the line segment with its direction reversed by swapping the endpoints.",
            &["_self"],
        )
        .register_documented(
            "rotated",
            |
                _self: Ref<::bevy_math::primitives::Segment3d>,
                rotation: Val<::bevy_math::prelude::Quat>|
            {
                let output: Val<::bevy_math::primitives::Segment3d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment3d> = ::bevy_math::primitives::Segment3d::rotated(
                                &_self,
                                rotation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the segment rotated around the origin by the given rotation.",
            &["_self", "rotation"],
        )
        .register_documented(
            "rotated_around",
            |
                _self: Ref<::bevy_math::primitives::Segment3d>,
                rotation: Val<::bevy_math::prelude::Quat>,
                point: Val<::bevy_math::prelude::Vec3>|
            {
                let output: Val<::bevy_math::primitives::Segment3d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment3d> = ::bevy_math::primitives::Segment3d::rotated_around(
                                &_self,
                                rotation.into_inner(),
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the segment rotated around the given point by the given rotation.",
            &["_self", "rotation", "point"],
        )
        .register_documented(
            "rotated_around_center",
            |
                _self: Ref<::bevy_math::primitives::Segment3d>,
                rotation: Val<::bevy_math::prelude::Quat>|
            {
                let output: Val<::bevy_math::primitives::Segment3d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment3d> = ::bevy_math::primitives::Segment3d::rotated_around_center(
                                &_self,
                                rotation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the segment rotated around its own center.",
            &["_self", "rotation"],
        )
        .register_documented(
            "scaled_direction",
            |_self: Ref<::bevy_math::primitives::Segment3d>| {
                let output: Val<::bevy_math::prelude::Vec3> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec3> = ::bevy_math::primitives::Segment3d::scaled_direction(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the vector from the first endpoint to the second endpoint.",
            &["_self"],
        )
        .register_documented(
            "translated",
            |
                _self: Ref<::bevy_math::primitives::Segment3d>,
                translation: Val<::bevy_math::prelude::Vec3>|
            {
                let output: Val<::bevy_math::primitives::Segment3d> = {
                    {
                        let output: Val<::bevy_math::primitives::Segment3d> = ::bevy_math::primitives::Segment3d::translated(
                                &_self,
                                translation.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Compute the segment translated by the given vector.",
            &["_self", "translation"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Segment3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_torus_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Torus,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::Torus>| {
                let output: Val<::bevy_math::primitives::Torus> = {
                    {
                        let output: Val<::bevy_math::primitives::Torus> = <::bevy_math::primitives::Torus as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_math::primitives::Torus>,
                other: Ref<::bevy_math::primitives::Torus>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::Torus as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Torus,
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
            "inner_radius",
            |_self: Ref<::bevy_math::primitives::Torus>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Torus::inner_radius(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the inner radius of the torus.\n For a ring torus, this corresponds to the radius of the hole,\n or `major_radius - minor_radius`",
            &["_self"],
        )
        .register_documented(
            "new",
            |inner_radius: f32, outer_radius: f32| {
                let output: Val<::bevy_math::primitives::Torus> = {
                    {
                        let output: Val<::bevy_math::primitives::Torus> = ::bevy_math::primitives::Torus::new(
                                inner_radius,
                                outer_radius,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Torus` from an inner and outer radius.\n The inner radius is the radius of the hole, and the outer radius\n is the radius of the entire object",
            &["inner_radius", "outer_radius"],
        )
        .register_documented(
            "outer_radius",
            |_self: Ref<::bevy_math::primitives::Torus>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Torus::outer_radius(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the outer radius of the torus.\n This corresponds to the overall radius of the entire object,\n or `major_radius + minor_radius`",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Torus,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_triangle_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Triangle3d,
    >::new(world)
        .register_documented(
            "centroid",
            |_self: Ref<::bevy_math::primitives::Triangle3d>| {
                let output: Val<::bevy_math::prelude::Vec3> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec3> = ::bevy_math::primitives::Triangle3d::centroid(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the centroid of the triangle.\n This function finds the geometric center of the triangle by averaging the vertices:\n `centroid = (a + b + c) / 3`.",
            &["_self"],
        )
        .register_documented(
            "circumcenter",
            |_self: Ref<::bevy_math::primitives::Triangle3d>| {
                let output: Val<::bevy_math::prelude::Vec3> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec3> = ::bevy_math::primitives::Triangle3d::circumcenter(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the circumcenter of the triangle.",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::Triangle3d>| {
                let output: Val<::bevy_math::primitives::Triangle3d> = {
                    {
                        let output: Val<::bevy_math::primitives::Triangle3d> = <::bevy_math::primitives::Triangle3d as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_math::primitives::Triangle3d>,
                other: Ref<::bevy_math::primitives::Triangle3d>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::Triangle3d as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Triangle3d,
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
            "is_acute",
            |_self: Ref<::bevy_math::primitives::Triangle3d>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::primitives::Triangle3d::is_acute(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Checks if the triangle is acute, meaning all angles are less than 90 degrees",
            &["_self"],
        )
        .register_documented(
            "is_degenerate",
            |_self: Ref<::bevy_math::primitives::Triangle3d>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::primitives::Triangle3d::is_degenerate(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Checks if the triangle is degenerate, meaning it has zero area.\n A triangle is degenerate if the cross product of the vectors `ab` and `ac` has a length less than `10e-7`.\n This indicates that the three vertices are collinear or nearly collinear.",
            &["_self"],
        )
        .register_documented(
            "is_obtuse",
            |_self: Ref<::bevy_math::primitives::Triangle3d>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::primitives::Triangle3d::is_obtuse(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Checks if the triangle is obtuse, meaning one angle is greater than 90 degrees",
            &["_self"],
        )
        .register_documented(
            "new",
            |
                a: Val<::bevy_math::prelude::Vec3>,
                b: Val<::bevy_math::prelude::Vec3>,
                c: Val<::bevy_math::prelude::Vec3>|
            {
                let output: Val<::bevy_math::primitives::Triangle3d> = {
                    {
                        let output: Val<::bevy_math::primitives::Triangle3d> = ::bevy_math::primitives::Triangle3d::new(
                                a.into_inner(),
                                b.into_inner(),
                                c.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`Triangle3d`] from points `a`, `b`, and `c`.",
            &["a", "b", "c"],
        )
        .register_documented(
            "reverse",
            |mut _self: Mut<::bevy_math::primitives::Triangle3d>| {
                let output: () = {
                    {
                        let output: () = ::bevy_math::primitives::Triangle3d::reverse(
                                &mut _self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Reverse the triangle by swapping the first and last vertices.",
            &["_self"],
        )
        .register_documented(
            "reversed",
            |_self: Val<::bevy_math::primitives::Triangle3d>| {
                let output: Val<::bevy_math::primitives::Triangle3d> = {
                    {
                        let output: Val<::bevy_math::primitives::Triangle3d> = ::bevy_math::primitives::Triangle3d::reversed(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " This triangle but reversed.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Triangle3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ray_cast_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::bounding::RayCast2d,
    >::new(world)
    .register_documented(
        "aabb_intersection_at",
        |_self: Ref<::bevy_math::bounding::RayCast2d>, aabb: Ref<::bevy_math::bounding::Aabb2d>| {
            let output: ::core::option::Option<f32> = {
                {
                    let output: ::core::option::Option<f32> =
                        ::bevy_math::bounding::RayCast2d::aabb_intersection_at(&_self, &aabb)
                            .into();
                    output
                }
            };
            output
        },
        " Get the distance of an intersection with an [`Aabb2d`], if any.",
        &["_self", "aabb"],
    )
    .register_documented(
        "circle_intersection_at",
        |_self: Ref<::bevy_math::bounding::RayCast2d>,
         circle: Ref<::bevy_math::bounding::BoundingCircle>| {
            let output: ::core::option::Option<f32> = {
                {
                    let output: ::core::option::Option<f32> =
                        ::bevy_math::bounding::RayCast2d::circle_intersection_at(&_self, &circle)
                            .into();
                    output
                }
            };
            output
        },
        " Get the distance of an intersection with a [`BoundingCircle`], if any.",
        &["_self", "circle"],
    )
    .register_documented(
        "clone",
        |_self: Ref<::bevy_math::bounding::RayCast2d>| {
            let output: Val<::bevy_math::bounding::RayCast2d> = {
                {
                    let output: Val<::bevy_math::bounding::RayCast2d> =
                        <::bevy_math::bounding::RayCast2d as ::core::clone::Clone>::clone(&_self)
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
        "direction_recip",
        |_self: Ref<::bevy_math::bounding::RayCast2d>| {
            let output: Val<::bevy_math::prelude::Vec2> = {
                {
                    let output: Val<::bevy_math::prelude::Vec2> =
                        ::bevy_math::bounding::RayCast2d::direction_recip(&_self).into();
                    output
                }
            };
            output
        },
        " Get the cached multiplicative inverse of the direction of the ray.",
        &["_self"],
    )
    .register_documented(
        "from_ray",
        |ray: Val<::bevy_math::Ray2d>, max: f32| {
            let output: Val<::bevy_math::bounding::RayCast2d> = {
                {
                    let output: Val<::bevy_math::bounding::RayCast2d> =
                        ::bevy_math::bounding::RayCast2d::from_ray(ray.into_inner(), max).into();
                    output
                }
            };
            output
        },
        " Construct a [`RayCast2d`] from a [`Ray2d`] and max distance.",
        &["ray", "max"],
    )
    .register_documented(
        "new",
        |origin: Val<::bevy_math::prelude::Vec2>,
         direction: Val<::bevy_math::prelude::Dir2>,
         max: f32| {
            let output: Val<::bevy_math::bounding::RayCast2d> = {
                {
                    let output: Val<::bevy_math::bounding::RayCast2d> =
                        ::bevy_math::bounding::RayCast2d::new(
                            origin.into_inner(),
                            direction.into_inner(),
                            max,
                        )
                        .into();
                    output
                }
            };
            output
        },
        " Construct a [`RayCast2d`] from an origin, [`Dir2`], and max distance.",
        &["origin", "direction", "max"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::bounding::RayCast2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_aabb_cast_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::bounding::AabbCast2d,
    >::new(world)
    .register_documented(
        "aabb_collision_at",
        |_self: Ref<::bevy_math::bounding::AabbCast2d>,
         aabb: Val<::bevy_math::bounding::Aabb2d>| {
            let output: ::core::option::Option<f32> = {
                {
                    let output: ::core::option::Option<f32> =
                        ::bevy_math::bounding::AabbCast2d::aabb_collision_at(
                            &_self,
                            aabb.into_inner(),
                        )
                        .into();
                    output
                }
            };
            output
        },
        " Get the distance at which the [`Aabb2d`]s collide, if at all.",
        &["_self", "aabb"],
    )
    .register_documented(
        "clone",
        |_self: Ref<::bevy_math::bounding::AabbCast2d>| {
            let output: Val<::bevy_math::bounding::AabbCast2d> = {
                {
                    let output: Val<::bevy_math::bounding::AabbCast2d> =
                        <::bevy_math::bounding::AabbCast2d as ::core::clone::Clone>::clone(&_self)
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
        "from_ray",
        |aabb: Val<::bevy_math::bounding::Aabb2d>, ray: Val<::bevy_math::Ray2d>, max: f32| {
            let output: Val<::bevy_math::bounding::AabbCast2d> = {
                {
                    let output: Val<::bevy_math::bounding::AabbCast2d> =
                        ::bevy_math::bounding::AabbCast2d::from_ray(
                            aabb.into_inner(),
                            ray.into_inner(),
                            max,
                        )
                        .into();
                    output
                }
            };
            output
        },
        " Construct an [`AabbCast2d`] from an [`Aabb2d`], [`Ray2d`], and max distance.",
        &["aabb", "ray", "max"],
    )
    .register_documented(
        "new",
        |aabb: Val<::bevy_math::bounding::Aabb2d>,
         origin: Val<::bevy_math::prelude::Vec2>,
         direction: Val<::bevy_math::prelude::Dir2>,
         max: f32| {
            let output: Val<::bevy_math::bounding::AabbCast2d> = {
                {
                    let output: Val<::bevy_math::bounding::AabbCast2d> =
                        ::bevy_math::bounding::AabbCast2d::new(
                            aabb.into_inner(),
                            origin.into_inner(),
                            direction.into_inner(),
                            max,
                        )
                        .into();
                    output
                }
            };
            output
        },
        " Construct an [`AabbCast2d`] from an [`Aabb2d`], origin, [`Dir2`], and max distance.",
        &["aabb", "origin", "direction", "max"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::bounding::AabbCast2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_bounding_circle_cast_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::bounding::BoundingCircleCast,
    >::new(world)
        .register_documented(
            "circle_collision_at",
            |
                _self: Ref<::bevy_math::bounding::BoundingCircleCast>,
                circle: Val<::bevy_math::bounding::BoundingCircle>|
            {
                let output: ::core::option::Option<f32> = {
                    {
                        let output: ::core::option::Option<f32> = ::bevy_math::bounding::BoundingCircleCast::circle_collision_at(
                                &_self,
                                circle.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the distance at which the [`BoundingCircle`]s collide, if at all.",
            &["_self", "circle"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::bounding::BoundingCircleCast>| {
                let output: Val<::bevy_math::bounding::BoundingCircleCast> = {
                    {
                        let output: Val<::bevy_math::bounding::BoundingCircleCast> = <::bevy_math::bounding::BoundingCircleCast as ::core::clone::Clone>::clone(
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
            "from_ray",
            |
                circle: Val<::bevy_math::bounding::BoundingCircle>,
                ray: Val<::bevy_math::Ray2d>,
                max: f32|
            {
                let output: Val<::bevy_math::bounding::BoundingCircleCast> = {
                    {
                        let output: Val<::bevy_math::bounding::BoundingCircleCast> = ::bevy_math::bounding::BoundingCircleCast::from_ray(
                                circle.into_inner(),
                                ray.into_inner(),
                                max,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a [`BoundingCircleCast`] from a [`BoundingCircle`], [`Ray2d`], and max distance.",
            &["circle", "ray", "max"],
        )
        .register_documented(
            "new",
            |
                circle: Val<::bevy_math::bounding::BoundingCircle>,
                origin: Val<::bevy_math::prelude::Vec2>,
                direction: Val<::bevy_math::prelude::Dir2>,
                max: f32|
            {
                let output: Val<::bevy_math::bounding::BoundingCircleCast> = {
                    {
                        let output: Val<::bevy_math::bounding::BoundingCircleCast> = ::bevy_math::bounding::BoundingCircleCast::new(
                                circle.into_inner(),
                                origin.into_inner(),
                                direction.into_inner(),
                                max,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a [`BoundingCircleCast`] from a [`BoundingCircle`], origin, [`Dir2`], and max distance.",
            &["circle", "origin", "direction", "max"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::bounding::BoundingCircleCast,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ray_cast_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::bounding::RayCast3d,
    >::new(world)
    .register_documented(
        "aabb_intersection_at",
        |_self: Ref<::bevy_math::bounding::RayCast3d>, aabb: Ref<::bevy_math::bounding::Aabb3d>| {
            let output: ::core::option::Option<f32> = {
                {
                    let output: ::core::option::Option<f32> =
                        ::bevy_math::bounding::RayCast3d::aabb_intersection_at(&_self, &aabb)
                            .into();
                    output
                }
            };
            output
        },
        " Get the distance of an intersection with an [`Aabb3d`], if any.",
        &["_self", "aabb"],
    )
    .register_documented(
        "clone",
        |_self: Ref<::bevy_math::bounding::RayCast3d>| {
            let output: Val<::bevy_math::bounding::RayCast3d> = {
                {
                    let output: Val<::bevy_math::bounding::RayCast3d> =
                        <::bevy_math::bounding::RayCast3d as ::core::clone::Clone>::clone(&_self)
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
        "direction_recip",
        |_self: Ref<::bevy_math::bounding::RayCast3d>| {
            let output: Val<::bevy_math::prelude::Vec3A> = {
                {
                    let output: Val<::bevy_math::prelude::Vec3A> =
                        ::bevy_math::bounding::RayCast3d::direction_recip(&_self).into();
                    output
                }
            };
            output
        },
        " Get the cached multiplicative inverse of the direction of the ray.",
        &["_self"],
    )
    .register_documented(
        "from_ray",
        |ray: Val<::bevy_math::Ray3d>, max: f32| {
            let output: Val<::bevy_math::bounding::RayCast3d> = {
                {
                    let output: Val<::bevy_math::bounding::RayCast3d> =
                        ::bevy_math::bounding::RayCast3d::from_ray(ray.into_inner(), max).into();
                    output
                }
            };
            output
        },
        " Construct a [`RayCast3d`] from a [`Ray3d`] and max distance.",
        &["ray", "max"],
    )
    .register_documented(
        "sphere_intersection_at",
        |_self: Ref<::bevy_math::bounding::RayCast3d>,
         sphere: Ref<::bevy_math::bounding::BoundingSphere>| {
            let output: ::core::option::Option<f32> = {
                {
                    let output: ::core::option::Option<f32> =
                        ::bevy_math::bounding::RayCast3d::sphere_intersection_at(&_self, &sphere)
                            .into();
                    output
                }
            };
            output
        },
        " Get the distance of an intersection with a [`BoundingSphere`], if any.",
        &["_self", "sphere"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::bounding::RayCast3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_aabb_cast_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::bounding::AabbCast3d,
    >::new(world)
    .register_documented(
        "aabb_collision_at",
        |_self: Ref<::bevy_math::bounding::AabbCast3d>,
         aabb: Val<::bevy_math::bounding::Aabb3d>| {
            let output: ::core::option::Option<f32> = {
                {
                    let output: ::core::option::Option<f32> =
                        ::bevy_math::bounding::AabbCast3d::aabb_collision_at(
                            &_self,
                            aabb.into_inner(),
                        )
                        .into();
                    output
                }
            };
            output
        },
        " Get the distance at which the [`Aabb3d`]s collide, if at all.",
        &["_self", "aabb"],
    )
    .register_documented(
        "clone",
        |_self: Ref<::bevy_math::bounding::AabbCast3d>| {
            let output: Val<::bevy_math::bounding::AabbCast3d> = {
                {
                    let output: Val<::bevy_math::bounding::AabbCast3d> =
                        <::bevy_math::bounding::AabbCast3d as ::core::clone::Clone>::clone(&_self)
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
        "from_ray",
        |aabb: Val<::bevy_math::bounding::Aabb3d>, ray: Val<::bevy_math::Ray3d>, max: f32| {
            let output: Val<::bevy_math::bounding::AabbCast3d> = {
                {
                    let output: Val<::bevy_math::bounding::AabbCast3d> =
                        ::bevy_math::bounding::AabbCast3d::from_ray(
                            aabb.into_inner(),
                            ray.into_inner(),
                            max,
                        )
                        .into();
                    output
                }
            };
            output
        },
        " Construct an [`AabbCast3d`] from an [`Aabb3d`], [`Ray3d`], and max distance.",
        &["aabb", "ray", "max"],
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::bounding::AabbCast3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_bounding_sphere_cast_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::bounding::BoundingSphereCast,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::bounding::BoundingSphereCast>| {
                let output: Val<::bevy_math::bounding::BoundingSphereCast> = {
                    {
                        let output: Val<::bevy_math::bounding::BoundingSphereCast> = <::bevy_math::bounding::BoundingSphereCast as ::core::clone::Clone>::clone(
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
            "from_ray",
            |
                sphere: Val<::bevy_math::bounding::BoundingSphere>,
                ray: Val<::bevy_math::Ray3d>,
                max: f32|
            {
                let output: Val<::bevy_math::bounding::BoundingSphereCast> = {
                    {
                        let output: Val<::bevy_math::bounding::BoundingSphereCast> = ::bevy_math::bounding::BoundingSphereCast::from_ray(
                                sphere.into_inner(),
                                ray.into_inner(),
                                max,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Construct a [`BoundingSphereCast`] from a [`BoundingSphere`], [`Ray3d`], and max distance.",
            &["sphere", "ray", "max"],
        )
        .register_documented(
            "sphere_collision_at",
            |
                _self: Ref<::bevy_math::bounding::BoundingSphereCast>,
                sphere: Val<::bevy_math::bounding::BoundingSphere>|
            {
                let output: ::core::option::Option<f32> = {
                    {
                        let output: ::core::option::Option<f32> = ::bevy_math::bounding::BoundingSphereCast::sphere_collision_at(
                                &_self,
                                sphere.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the distance at which the [`BoundingSphere`]s collide, if at all.",
            &["_self", "sphere"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::bounding::BoundingSphereCast,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_interval_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::curve::interval::Interval,
    >::new(world)
        .register_documented(
            "clamp",
            |_self: Val<::bevy_math::curve::interval::Interval>, value: f32| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::curve::interval::Interval::clamp(
                                _self.into_inner(),
                                value,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Clamp the given `value` to lie within this interval.",
            &["_self", "value"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::curve::interval::Interval>| {
                let output: Val<::bevy_math::curve::interval::Interval> = {
                    {
                        let output: Val<::bevy_math::curve::interval::Interval> = <::bevy_math::curve::interval::Interval as ::core::clone::Clone>::clone(
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
            "contains",
            |_self: Val<::bevy_math::curve::interval::Interval>, item: f32| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::curve::interval::Interval::contains(
                                _self.into_inner(),
                                item,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if `item` is contained in this interval.",
            &["_self", "item"],
        )
        .register_documented(
            "contains_interval",
            |
                _self: Val<::bevy_math::curve::interval::Interval>,
                other: Val<::bevy_math::curve::interval::Interval>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::curve::interval::Interval::contains_interval(
                                _self.into_inner(),
                                other.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if the other interval is contained in this interval.\n This is non-strict: each interval will contain itself.",
            &["_self", "other"],
        )
        .register_documented(
            "end",
            |_self: Val<::bevy_math::curve::interval::Interval>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::curve::interval::Interval::end(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the end of this interval.",
            &["_self"],
        )
        .register_documented(
            "eq",
            |
                _self: Ref<::bevy_math::curve::interval::Interval>,
                other: Ref<::bevy_math::curve::interval::Interval>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::curve::interval::Interval as ::core::cmp::PartialEq<
                            ::bevy_math::curve::interval::Interval,
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
            "has_finite_end",
            |_self: Val<::bevy_math::curve::interval::Interval>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::curve::interval::Interval::has_finite_end(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if this interval has a finite end.",
            &["_self"],
        )
        .register_documented(
            "has_finite_start",
            |_self: Val<::bevy_math::curve::interval::Interval>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::curve::interval::Interval::has_finite_start(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if this interval has a finite start.",
            &["_self"],
        )
        .register_documented(
            "is_bounded",
            |_self: Val<::bevy_math::curve::interval::Interval>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::curve::interval::Interval::is_bounded(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `true` if this interval is bounded — that is, if both its start and end are finite.\n Equivalently, an interval is bounded if its length is finite.",
            &["_self"],
        )
        .register_documented(
            "length",
            |_self: Val<::bevy_math::curve::interval::Interval>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::curve::interval::Interval::length(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the length of this interval. Note that the result may be infinite (`f32::INFINITY`).",
            &["_self"],
        )
        .register_documented(
            "start",
            |_self: Val<::bevy_math::curve::interval::Interval>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::curve::interval::Interval::start(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the start of this interval.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::curve::interval::Interval,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_float_ord_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::FloatOrd,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::FloatOrd>| {
                let output: Val<::bevy_math::FloatOrd> = {
                    {
                        let output: Val<::bevy_math::FloatOrd> = <::bevy_math::FloatOrd as ::core::clone::Clone>::clone(
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
            |_self: Ref<::bevy_math::FloatOrd>, other: Ref<::bevy_math::FloatOrd>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::FloatOrd as ::core::cmp::PartialEq<
                            ::bevy_math::FloatOrd,
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
            "ge",
            |_self: Ref<::bevy_math::FloatOrd>, other: Ref<::bevy_math::FloatOrd>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::FloatOrd as ::core::cmp::PartialOrd<
                            ::bevy_math::FloatOrd,
                        >>::ge(&_self, &other)
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
            "gt",
            |_self: Ref<::bevy_math::FloatOrd>, other: Ref<::bevy_math::FloatOrd>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::FloatOrd as ::core::cmp::PartialOrd<
                            ::bevy_math::FloatOrd,
                        >>::gt(&_self, &other)
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
            "le",
            |_self: Ref<::bevy_math::FloatOrd>, other: Ref<::bevy_math::FloatOrd>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::FloatOrd as ::core::cmp::PartialOrd<
                            ::bevy_math::FloatOrd,
                        >>::le(&_self, &other)
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
            "lt",
            |_self: Ref<::bevy_math::FloatOrd>, other: Ref<::bevy_math::FloatOrd>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::FloatOrd as ::core::cmp::PartialOrd<
                            ::bevy_math::FloatOrd,
                        >>::lt(&_self, &other)
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
            "neg",
            |_self: Val<::bevy_math::FloatOrd>| {
                let output: Val<::bevy_math::FloatOrd> = {
                    {
                        let output: Val<::bevy_math::FloatOrd> = <::bevy_math::FloatOrd as ::core::ops::Neg>::neg(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_math::FloatOrd, bevy_mod_scripting_bindings::MarkAsGenerated>(
        );
}
pub(crate) fn register_plane_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Plane3d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::Plane3d>| {
                let output: Val<::bevy_math::primitives::Plane3d> = {
                    {
                        let output: Val<::bevy_math::primitives::Plane3d> = <::bevy_math::primitives::Plane3d as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_math::primitives::Plane3d>,
                other: Ref<::bevy_math::primitives::Plane3d>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::Plane3d as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Plane3d,
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
            "new",
            |
                normal: Val<::bevy_math::prelude::Vec3>,
                half_size: Val<::bevy_math::prelude::Vec2>|
            {
                let output: Val<::bevy_math::primitives::Plane3d> = {
                    {
                        let output: Val<::bevy_math::primitives::Plane3d> = ::bevy_math::primitives::Plane3d::new(
                                normal.into_inner(),
                                half_size.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Plane3d` from a normal and a half size\n # Panics\n Panics if the given `normal` is zero (or very close to zero), or non-finite.",
            &["normal", "half_size"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Plane3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_tetrahedron_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::primitives::Tetrahedron,
    >::new(world)
        .register_documented(
            "centroid",
            |_self: Ref<::bevy_math::primitives::Tetrahedron>| {
                let output: Val<::bevy_math::prelude::Vec3> = {
                    {
                        let output: Val<::bevy_math::prelude::Vec3> = ::bevy_math::primitives::Tetrahedron::centroid(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the centroid of the tetrahedron.\n This function finds the geometric center of the tetrahedron\n by averaging the vertices: `centroid = (a + b + c + d) / 4`.",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: Ref<::bevy_math::primitives::Tetrahedron>| {
                let output: Val<::bevy_math::primitives::Tetrahedron> = {
                    {
                        let output: Val<::bevy_math::primitives::Tetrahedron> = <::bevy_math::primitives::Tetrahedron as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_math::primitives::Tetrahedron>,
                other: Ref<::bevy_math::primitives::Tetrahedron>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::primitives::Tetrahedron as ::core::cmp::PartialEq<
                            ::bevy_math::primitives::Tetrahedron,
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
            "new",
            |
                a: Val<::bevy_math::prelude::Vec3>,
                b: Val<::bevy_math::prelude::Vec3>,
                c: Val<::bevy_math::prelude::Vec3>,
                d: Val<::bevy_math::prelude::Vec3>|
            {
                let output: Val<::bevy_math::primitives::Tetrahedron> = {
                    {
                        let output: Val<::bevy_math::primitives::Tetrahedron> = ::bevy_math::primitives::Tetrahedron::new(
                                a.into_inner(),
                                b.into_inner(),
                                c.into_inner(),
                                d.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`Tetrahedron`] from points `a`, `b`, `c` and `d`.",
            &["a", "b", "c", "d"],
        )
        .register_documented(
            "signed_volume",
            |_self: Ref<::bevy_math::primitives::Tetrahedron>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::primitives::Tetrahedron::signed_volume(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the signed volume of the tetrahedron.\n If it's negative, the normal vector of the face defined by\n the first three points using the right-hand rule points\n away from the fourth vertex.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::primitives::Tetrahedron,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ease_function_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::curve::easing::EaseFunction,
    >::new(world)
    .register_documented(
        "clone",
        |_self: Ref<::bevy_math::curve::easing::EaseFunction>| {
            let output: Val<::bevy_math::curve::easing::EaseFunction> = {
                {
                    let output: Val<::bevy_math::curve::easing::EaseFunction> =
                        <::bevy_math::curve::easing::EaseFunction as ::core::clone::Clone>::clone(
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
        |_self: Ref<::bevy_math::curve::easing::EaseFunction>,
         other: Ref<::bevy_math::curve::easing::EaseFunction>| {
            let output: bool = {
                {
                    let output: bool =
                        <::bevy_math::curve::easing::EaseFunction as ::core::cmp::PartialEq<
                            ::bevy_math::curve::easing::EaseFunction,
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
            ::bevy_math::curve::easing::EaseFunction,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_jump_at_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::curve::easing::JumpAt,
    >::new(world)
        .register_documented(
            "assert_receiver_is_total_eq",
            |_self: Ref<::bevy_math::curve::easing::JumpAt>| {
                let output: () = {
                    {
                        let output: () = <::bevy_math::curve::easing::JumpAt as ::core::cmp::Eq>::assert_receiver_is_total_eq(
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
            |_self: Ref<::bevy_math::curve::easing::JumpAt>| {
                let output: Val<::bevy_math::curve::easing::JumpAt> = {
                    {
                        let output: Val<::bevy_math::curve::easing::JumpAt> = <::bevy_math::curve::easing::JumpAt as ::core::clone::Clone>::clone(
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
                _self: Ref<::bevy_math::curve::easing::JumpAt>,
                other: Ref<::bevy_math::curve::easing::JumpAt>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::curve::easing::JumpAt as ::core::cmp::PartialEq<
                            ::bevy_math::curve::easing::JumpAt,
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
            ::bevy_math::curve::easing::JumpAt,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyMathScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_aspect_ratio_functions(&mut world);
        register_compass_octant_functions(&mut world);
        register_compass_quadrant_functions(&mut world);
        register_isometry_2_d_functions(&mut world);
        register_isometry_3_d_functions(&mut world);
        register_ray_2_d_functions(&mut world);
        register_ray_3_d_functions(&mut world);
        register_rot_2_functions(&mut world);
        register_dir_2_functions(&mut world);
        register_dir_3_functions(&mut world);
        register_dir_3_a_functions(&mut world);
        register_i_rect_functions(&mut world);
        register_rect_functions(&mut world);
        register_u_rect_functions(&mut world);
        register_affine_3_functions(&mut world);
        register_aabb_2_d_functions(&mut world);
        register_bounding_circle_functions(&mut world);
        register_circle_functions(&mut world);
        register_annulus_functions(&mut world);
        register_arc_2_d_functions(&mut world);
        register_capsule_2_d_functions(&mut world);
        register_circular_sector_functions(&mut world);
        register_circular_segment_functions(&mut world);
        register_ellipse_functions(&mut world);
        register_line_2_d_functions(&mut world);
        register_plane_2_d_functions(&mut world);
        register_rectangle_functions(&mut world);
        register_regular_polygon_functions(&mut world);
        register_rhombus_functions(&mut world);
        register_segment_2_d_functions(&mut world);
        register_triangle_2_d_functions(&mut world);
        register_aabb_3_d_functions(&mut world);
        register_bounding_sphere_functions(&mut world);
        register_sphere_functions(&mut world);
        register_cuboid_functions(&mut world);
        register_cylinder_functions(&mut world);
        register_capsule_3_d_functions(&mut world);
        register_cone_functions(&mut world);
        register_conical_frustum_functions(&mut world);
        register_infinite_plane_3_d_functions(&mut world);
        register_line_3_d_functions(&mut world);
        register_segment_3_d_functions(&mut world);
        register_torus_functions(&mut world);
        register_triangle_3_d_functions(&mut world);
        register_ray_cast_2_d_functions(&mut world);
        register_aabb_cast_2_d_functions(&mut world);
        register_bounding_circle_cast_functions(&mut world);
        register_ray_cast_3_d_functions(&mut world);
        register_aabb_cast_3_d_functions(&mut world);
        register_bounding_sphere_cast_functions(&mut world);
        register_interval_functions(&mut world);
        register_float_ord_functions(&mut world);
        register_plane_3_d_functions(&mut world);
        register_tetrahedron_functions(&mut world);
        register_ease_function_functions(&mut world);
        register_jump_at_functions(&mut world);
    }
}
