#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
extern crate std;

use bevy_app::{App, Plugin};
use bevy_ecs::prelude::*;
use bevy_mod_scripting_bindings::{
    ReflectReference,
    function::{
        from::{M, R, V},
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
            |_self: R<::bevy_math::AspectRatio>| {
                let output: V<::bevy_math::AspectRatio> = {
                    {
                        let output: V<::bevy_math::AspectRatio> = <::bevy_math::AspectRatio as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_math::AspectRatio>, other: R<::bevy_math::AspectRatio>| {
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
            |_self: R<::bevy_math::AspectRatio>| {
                let output: V<::bevy_math::AspectRatio> = {
                    {
                        let output: V<::bevy_math::AspectRatio> = ::bevy_math::AspectRatio::inverse(
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
            |_self: R<::bevy_math::AspectRatio>| {
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
            |_self: R<::bevy_math::AspectRatio>| {
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
            |_self: R<::bevy_math::AspectRatio>| {
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
            |_self: R<::bevy_math::AspectRatio>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::AspectRatio::ratio(&_self).into();
                        output
                    }
                };
                output
            },
            " Returns the aspect ratio as an `f32` value.",
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
            "clone",
            |_self: R<::bevy_math::CompassOctant>| {
                let output: V<::bevy_math::CompassOctant> = {
                    {
                        let output: V<::bevy_math::CompassOctant> = <::bevy_math::CompassOctant as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_math::CompassOctant>, other: R<::bevy_math::CompassOctant>| {
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
            "is_in_direction",
            |
                _self: V<::bevy_math::CompassOctant>,
                origin: V<::bevy_math::Vec2>,
                candidate: V<::bevy_math::Vec2>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::CompassOctant::is_in_direction(
                                _self.into_inner(),
                                origin.into_inner(),
                                candidate.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Checks if a point is in the direction represented by this [`CompassOctant`] from an origin.\n This uses a cone-based check: the vector from origin to the candidate point\n must have a positive dot product with the direction vector.\n Uses standard mathematical coordinates where Y increases upward.\n # Arguments\n * `origin` - The starting position\n * `candidate` - The target position to check\n # Returns\n `true` if the candidate is generally in the direction of this octant from the origin.\n # Example\n ```\n use bevy_math::{CompassOctant, Vec2};\n let origin = Vec2::new(0.0, 0.0);\n let north_point = Vec2::new(0.0, 10.0);  // Above origin (Y+ = up)\n let east_point = Vec2::new(10.0, 0.0);   // Right of origin\n assert!(CompassOctant::North.is_in_direction(origin, north_point));\n assert!(!CompassOctant::North.is_in_direction(origin, east_point));\n ```",
            &["_self", "origin", "candidate"],
        )
        .register_documented(
            "opposite",
            |_self: R<::bevy_math::CompassOctant>| {
                let output: V<::bevy_math::CompassOctant> = {
                    {
                        let output: V<::bevy_math::CompassOctant> = ::bevy_math::CompassOctant::opposite(
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
            |_self: V<::bevy_math::CompassOctant>| {
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
            "clone",
            |_self: R<::bevy_math::CompassQuadrant>| {
                let output: V<::bevy_math::CompassQuadrant> = {
                    {
                        let output: V<::bevy_math::CompassQuadrant> = <::bevy_math::CompassQuadrant as ::core::clone::Clone>::clone(
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
                _self: R<::bevy_math::CompassQuadrant>,
                other: R<::bevy_math::CompassQuadrant>|
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
            "is_in_direction",
            |
                _self: V<::bevy_math::CompassQuadrant>,
                origin: V<::bevy_math::Vec2>,
                candidate: V<::bevy_math::Vec2>|
            {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::CompassQuadrant::is_in_direction(
                                _self.into_inner(),
                                origin.into_inner(),
                                candidate.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Checks if a point is in the direction represented by this [`CompassQuadrant`] from an origin.\n This uses a cone-based check: the vector from origin to the candidate point\n must have a positive dot product with the direction vector.\n Uses standard mathematical coordinates where Y increases upward.\n # Arguments\n * `origin` - The starting position\n * `candidate` - The target position to check\n # Returns\n `true` if the candidate is generally in the direction of this quadrant from the origin.\n # Example\n ```\n use bevy_math::{CompassQuadrant, Vec2};\n let origin = Vec2::new(0.0, 0.0);\n let north_point = Vec2::new(0.0, 10.0);  // Above origin (Y+ = up)\n let east_point = Vec2::new(10.0, 0.0);   // Right of origin\n assert!(CompassQuadrant::North.is_in_direction(origin, north_point));\n assert!(!CompassQuadrant::North.is_in_direction(origin, east_point));\n ```",
            &["_self", "origin", "candidate"],
        )
        .register_documented(
            "opposite",
            |_self: R<::bevy_math::CompassQuadrant>| {
                let output: V<::bevy_math::CompassQuadrant> = {
                    {
                        let output: V<::bevy_math::CompassQuadrant> = ::bevy_math::CompassQuadrant::opposite(
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
            |_self: V<::bevy_math::CompassQuadrant>| {
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
            |_self: R<::bevy_math::Isometry2d>| {
                let output: V<::bevy_math::Isometry2d> = {
                    {
                        let output: V<::bevy_math::Isometry2d> = <::bevy_math::Isometry2d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_math::Isometry2d>, other: R<::bevy_math::Isometry2d>| {
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
            |rotation: V<::bevy_math::Rot2>| {
                let output: V<::bevy_math::Isometry2d> = {
                    {
                        let output: V<::bevy_math::Isometry2d> = ::bevy_math::Isometry2d::from_rotation(
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
            |translation: V<::bevy_math::Vec2>| {
                let output: V<::bevy_math::Isometry2d> = {
                    {
                        let output: V<::bevy_math::Isometry2d> = ::bevy_math::Isometry2d::from_translation(
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
                let output: V<::bevy_math::Isometry2d> = {
                    {
                        let output: V<::bevy_math::Isometry2d> = ::bevy_math::Isometry2d::from_xy(
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
            |_self: R<::bevy_math::Isometry2d>| {
                let output: V<::bevy_math::Isometry2d> = {
                    {
                        let output: V<::bevy_math::Isometry2d> = ::bevy_math::Isometry2d::inverse(
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
            |_self: R<::bevy_math::Isometry2d>, rhs: V<::bevy_math::Isometry2d>| {
                let output: V<::bevy_math::Isometry2d> = {
                    {
                        let output: V<::bevy_math::Isometry2d> = ::bevy_math::Isometry2d::inverse_mul(
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
            |_self: R<::bevy_math::Isometry2d>, point: V<::bevy_math::Vec2>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_math::Isometry2d::inverse_transform_point(
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
            "new",
            |translation: V<::bevy_math::Vec2>, rotation: V<::bevy_math::Rot2>| {
                let output: V<::bevy_math::Isometry2d> = {
                    {
                        let output: V<::bevy_math::Isometry2d> = ::bevy_math::Isometry2d::new(
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
            |_self: R<::bevy_math::Isometry2d>, point: V<::bevy_math::Vec2>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_math::Isometry2d::transform_point(
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
            |_self: R<::bevy_math::Isometry3d>| {
                let output: V<::bevy_math::Isometry3d> = {
                    {
                        let output: V<::bevy_math::Isometry3d> = <::bevy_math::Isometry3d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_math::Isometry3d>, other: R<::bevy_math::Isometry3d>| {
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
            |rotation: V<::bevy_math::Quat>| {
                let output: V<::bevy_math::Isometry3d> = {
                    {
                        let output: V<::bevy_math::Isometry3d> = ::bevy_math::Isometry3d::from_rotation(
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
                let output: V<::bevy_math::Isometry3d> = {
                    {
                        let output: V<::bevy_math::Isometry3d> = ::bevy_math::Isometry3d::from_xyz(
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
            |_self: R<::bevy_math::Isometry3d>| {
                let output: V<::bevy_math::Isometry3d> = {
                    {
                        let output: V<::bevy_math::Isometry3d> = ::bevy_math::Isometry3d::inverse(
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
            |_self: R<::bevy_math::Isometry3d>, rhs: V<::bevy_math::Isometry3d>| {
                let output: V<::bevy_math::Isometry3d> = {
                    {
                        let output: V<::bevy_math::Isometry3d> = ::bevy_math::Isometry3d::inverse_mul(
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
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_math::Isometry3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_rot_2_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::Rot2,
    >::new(world)
        .register_documented(
            "angle_to",
            |_self: V<::bevy_math::Rot2>, other: V<::bevy_math::Rot2>| {
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
            |_self: V<::bevy_math::Rot2>| {
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
            " Returns a corresponding rotation angle in degrees in the `(-180, 180]` range.",
            &["_self"],
        )
        .register_documented(
            "as_radians",
            |_self: V<::bevy_math::Rot2>| {
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
            " Returns a corresponding rotation angle in radians in the `(-pi, pi]` range.",
            &["_self"],
        )
        .register_documented(
            "as_turn_fraction",
            |_self: V<::bevy_math::Rot2>| {
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
            " Returns a corresponding rotation angle as a fraction of a full 360 degree turn in the `(-0.5, 0.5]` range.",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: R<::bevy_math::Rot2>| {
                let output: V<::bevy_math::Rot2> = {
                    {
                        let output: V<::bevy_math::Rot2> = <::bevy_math::Rot2 as ::core::clone::Clone>::clone(
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
                let output: V<::bevy_math::Rot2> = {
                    {
                        let output: V<::bevy_math::Rot2> = ::bevy_math::Rot2::degrees(
                                degrees,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a [`Rot2`] from a counterclockwise angle in degrees.\n A negative argument corresponds to a clockwise rotation.\n # Note\n Angles larger than or equal to 360° (in either direction) loop around to smaller rotations, since a full rotation returns an object to its starting orientation.\n # Example\n ```\n # use bevy_math::Rot2;\n # use approx::{assert_relative_eq, assert_abs_diff_eq};\n let rot1 = Rot2::degrees(270.0);\n let rot2 = Rot2::degrees(-90.0);\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(rot1, rot2);\n let rot3 = Rot2::degrees(180.0);\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(rot1 * rot1, rot3);\n // A rotation by 365° and 5° are the same\n #[cfg(feature = \"approx\")]\n assert_abs_diff_eq!(Rot2::degrees(365.0), Rot2::degrees(5.0), epsilon = 2e-7);\n ```",
            &["degrees"],
        )
        .register_documented(
            "eq",
            |_self: R<::bevy_math::Rot2>, other: R<::bevy_math::Rot2>| {
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
            |_self: V<::bevy_math::Rot2>| {
                let output: V<::bevy_math::Rot2> = {
                    {
                        let output: V<::bevy_math::Rot2> = ::bevy_math::Rot2::fast_renormalize(
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
                let output: V<::bevy_math::Rot2> = {
                    {
                        let output: V<::bevy_math::Rot2> = ::bevy_math::Rot2::from_sin_cos(
                                sin,
                                cos,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a [`Rot2`] from the sine and cosine of an angle.\n The rotation is only valid if `sin * sin + cos * cos == 1.0`.\n # Panics\n Panics if `sin * sin + cos * cos != 1.0` when the `glam_assert` feature is enabled.",
            &["sin", "cos"],
        )
        .register_documented(
            "inverse",
            |_self: V<::bevy_math::Rot2>| {
                let output: V<::bevy_math::Rot2> = {
                    {
                        let output: V<::bevy_math::Rot2> = ::bevy_math::Rot2::inverse(
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
            |_self: V<::bevy_math::Rot2>| {
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
            |_self: V<::bevy_math::Rot2>| {
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
            |_self: V<::bevy_math::Rot2>| {
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
            |_self: V<::bevy_math::Rot2>| {
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
            |_self: V<::bevy_math::Rot2>| {
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
            |_self: V<::bevy_math::Rot2>| {
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
            |_self: V<::bevy_math::Rot2>| {
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
            "nlerp",
            |_self: V<::bevy_math::Rot2>, end: V<::bevy_math::Rot2>, s: f32| {
                let output: V<::bevy_math::Rot2> = {
                    {
                        let output: V<::bevy_math::Rot2> = ::bevy_math::Rot2::nlerp(
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
            |_self: V<::bevy_math::Rot2>| {
                let output: V<::bevy_math::Rot2> = {
                    {
                        let output: V<::bevy_math::Rot2> = ::bevy_math::Rot2::normalize(
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
                let output: V<::bevy_math::Rot2> = {
                    {
                        let output: V<::bevy_math::Rot2> = ::bevy_math::Rot2::radians(
                                radians,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a [`Rot2`] from a counterclockwise angle in radians.\n A negative argument corresponds to a clockwise rotation.\n # Note\n Angles larger than or equal to 2π (in either direction) loop around to smaller rotations, since a full rotation returns an object to its starting orientation.\n # Example\n ```\n # use bevy_math::Rot2;\n # use approx::assert_relative_eq;\n # use std::f32::consts::{FRAC_PI_2, PI};\n let rot1 = Rot2::radians(3.0 * FRAC_PI_2);\n let rot2 = Rot2::radians(-FRAC_PI_2);\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(rot1, rot2);\n let rot3 = Rot2::radians(PI);\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(rot1 * rot1, rot3);\n // A rotation by 3π and 1π are the same\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(Rot2::radians(3.0 * PI), Rot2::radians(PI));\n ```",
            &["radians"],
        )
        .register_documented(
            "sin_cos",
            |_self: V<::bevy_math::Rot2>| {
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
            " Returns the sine and cosine of the rotation angle.",
            &["_self"],
        )
        .register_documented(
            "slerp",
            |_self: V<::bevy_math::Rot2>, end: V<::bevy_math::Rot2>, s: f32| {
                let output: V<::bevy_math::Rot2> = {
                    {
                        let output: V<::bevy_math::Rot2> = ::bevy_math::Rot2::slerp(
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
                let output: V<::bevy_math::Rot2> = {
                    {
                        let output: V<::bevy_math::Rot2> = ::bevy_math::Rot2::turn_fraction(
                                fraction,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a [`Rot2`] from a counterclockwise fraction of a full turn of 360 degrees.\n A negative argument corresponds to a clockwise rotation.\n # Note\n Angles larger than or equal to 1 turn (in either direction) loop around to smaller rotations, since a full rotation returns an object to its starting orientation.\n # Example\n ```\n # use bevy_math::Rot2;\n # use approx::assert_relative_eq;\n let rot1 = Rot2::turn_fraction(0.75);\n let rot2 = Rot2::turn_fraction(-0.25);\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(rot1, rot2);\n let rot3 = Rot2::turn_fraction(0.5);\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(rot1 * rot1, rot3);\n // A rotation by 1.5 turns and 0.5 turns are the same\n #[cfg(feature = \"approx\")]\n assert_relative_eq!(Rot2::turn_fraction(1.5), Rot2::turn_fraction(0.5));\n ```",
            &["fraction"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_math::Rot2, bevy_mod_scripting_bindings::MarkAsGenerated>();
}
pub(crate) fn register_dir_2_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::Dir2,
    >::new(world)
        .register_documented(
            "as_vec2",
            |_self: R<::bevy_math::Dir2>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_math::Dir2::as_vec2(
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
            |_self: R<::bevy_math::Dir2>| {
                let output: V<::bevy_math::Dir2> = {
                    {
                        let output: V<::bevy_math::Dir2> = <::bevy_math::Dir2 as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_math::Dir2>, other: R<::bevy_math::Dir2>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::Dir2 as ::core::cmp::PartialEq<
                            ::bevy_math::Dir2,
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
            |_self: V<::bevy_math::Dir2>| {
                let output: V<::bevy_math::Dir2> = {
                    {
                        let output: V<::bevy_math::Dir2> = ::bevy_math::Dir2::fast_renormalize(
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
            "from_angle",
            |angle: f32| {
                let output: V<::bevy_math::Dir2> = {
                    {
                        let output: V<::bevy_math::Dir2> = ::bevy_math::Dir2::from_angle(
                                angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a 2D direction containing `[angle.cos(), angle.sin()]`.",
            &["angle"],
        )
        .register_documented(
            "from_xy_unchecked",
            |x: f32, y: f32| {
                let output: V<::bevy_math::Dir2> = {
                    {
                        let output: V<::bevy_math::Dir2> = ::bevy_math::Dir2::from_xy_unchecked(
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
            "new_unchecked",
            |value: V<::bevy_math::Vec2>| {
                let output: V<::bevy_math::Dir2> = {
                    {
                        let output: V<::bevy_math::Dir2> = ::bevy_math::Dir2::new_unchecked(
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
            "perpendicular",
            |_self: V<::bevy_math::Dir2>| {
                let output: V<::bevy_math::Dir2> = {
                    {
                        let output: V<::bevy_math::Dir2> = ::bevy_math::Dir2::perpendicular(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the perpendicular vector rotated to 90 degrees counterclockwise.",
            &["_self"],
        )
        .register_documented(
            "rotation_from",
            |_self: V<::bevy_math::Dir2>, other: V<::bevy_math::Dir2>| {
                let output: V<::bevy_math::Rot2> = {
                    {
                        let output: V<::bevy_math::Rot2> = ::bevy_math::Dir2::rotation_from(
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
            |_self: V<::bevy_math::Dir2>| {
                let output: V<::bevy_math::Rot2> = {
                    {
                        let output: V<::bevy_math::Rot2> = ::bevy_math::Dir2::rotation_from_x(
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
            |_self: V<::bevy_math::Dir2>| {
                let output: V<::bevy_math::Rot2> = {
                    {
                        let output: V<::bevy_math::Rot2> = ::bevy_math::Dir2::rotation_from_y(
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
            |_self: V<::bevy_math::Dir2>, other: V<::bevy_math::Dir2>| {
                let output: V<::bevy_math::Rot2> = {
                    {
                        let output: V<::bevy_math::Rot2> = ::bevy_math::Dir2::rotation_to(
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
            |_self: V<::bevy_math::Dir2>| {
                let output: V<::bevy_math::Rot2> = {
                    {
                        let output: V<::bevy_math::Rot2> = ::bevy_math::Dir2::rotation_to_x(
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
            |_self: V<::bevy_math::Dir2>| {
                let output: V<::bevy_math::Rot2> = {
                    {
                        let output: V<::bevy_math::Rot2> = ::bevy_math::Dir2::rotation_to_y(
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
            |_self: V<::bevy_math::Dir2>, rhs: V<::bevy_math::Dir2>, s: f32| {
                let output: V<::bevy_math::Dir2> = {
                    {
                        let output: V<::bevy_math::Dir2> = ::bevy_math::Dir2::slerp(
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
        .register_type_data::<::bevy_math::Dir2, bevy_mod_scripting_bindings::MarkAsGenerated>();
}
pub(crate) fn register_dir_3_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::Dir3,
    >::new(world)
        .register_documented(
            "as_vec3",
            |_self: R<::bevy_math::Dir3>| {
                let output: V<::bevy_math::Vec3> = {
                    {
                        let output: V<::bevy_math::Vec3> = ::bevy_math::Dir3::as_vec3(
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
            |_self: R<::bevy_math::Dir3>| {
                let output: V<::bevy_math::Dir3> = {
                    {
                        let output: V<::bevy_math::Dir3> = <::bevy_math::Dir3 as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_math::Dir3>, other: R<::bevy_math::Dir3>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::Dir3 as ::core::cmp::PartialEq<
                            ::bevy_math::Dir3,
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
            |_self: V<::bevy_math::Dir3>| {
                let output: V<::bevy_math::Dir3> = {
                    {
                        let output: V<::bevy_math::Dir3> = ::bevy_math::Dir3::fast_renormalize(
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
                let output: V<::bevy_math::Dir3> = {
                    {
                        let output: V<::bevy_math::Dir3> = ::bevy_math::Dir3::from_xyz_unchecked(
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
            "new_unchecked",
            |value: V<::bevy_math::Vec3>| {
                let output: V<::bevy_math::Dir3> = {
                    {
                        let output: V<::bevy_math::Dir3> = ::bevy_math::Dir3::new_unchecked(
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
            |_self: V<::bevy_math::Dir3>, rhs: V<::bevy_math::Dir3>, s: f32| {
                let output: V<::bevy_math::Dir3> = {
                    {
                        let output: V<::bevy_math::Dir3> = ::bevy_math::Dir3::slerp(
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
        .register_type_data::<::bevy_math::Dir3, bevy_mod_scripting_bindings::MarkAsGenerated>();
}
pub(crate) fn register_dir_3_a_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::Dir3A,
    >::new(world)
        .register_documented(
            "as_vec3a",
            |_self: R<::bevy_math::Dir3A>| {
                let output: V<::bevy_math::Vec3A> = {
                    {
                        let output: V<::bevy_math::Vec3A> = ::bevy_math::Dir3A::as_vec3a(
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
            |_self: R<::bevy_math::Dir3A>| {
                let output: V<::bevy_math::Dir3A> = {
                    {
                        let output: V<::bevy_math::Dir3A> = <::bevy_math::Dir3A as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_math::Dir3A>, other: R<::bevy_math::Dir3A>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::Dir3A as ::core::cmp::PartialEq<
                            ::bevy_math::Dir3A,
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
            |_self: V<::bevy_math::Dir3A>| {
                let output: V<::bevy_math::Dir3A> = {
                    {
                        let output: V<::bevy_math::Dir3A> = ::bevy_math::Dir3A::fast_renormalize(
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
                let output: V<::bevy_math::Dir3A> = {
                    {
                        let output: V<::bevy_math::Dir3A> = ::bevy_math::Dir3A::from_xyz_unchecked(
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
            "new_unchecked",
            |value: V<::bevy_math::Vec3A>| {
                let output: V<::bevy_math::Dir3A> = {
                    {
                        let output: V<::bevy_math::Dir3A> = ::bevy_math::Dir3A::new_unchecked(
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
            |_self: V<::bevy_math::Dir3A>, rhs: V<::bevy_math::Dir3A>, s: f32| {
                let output: V<::bevy_math::Dir3A> = {
                    {
                        let output: V<::bevy_math::Dir3A> = ::bevy_math::Dir3A::slerp(
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
        .register_type_data::<::bevy_math::Dir3A, bevy_mod_scripting_bindings::MarkAsGenerated>();
}
pub(crate) fn register_i_rect_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::IRect,
    >::new(world)
        .register_documented(
            "as_rect",
            |_self: R<::bevy_math::IRect>| {
                let output: V<::bevy_math::Rect> = {
                    {
                        let output: V<::bevy_math::Rect> = ::bevy_math::IRect::as_rect(
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
            |_self: R<::bevy_math::IRect>| {
                let output: V<::bevy_math::URect> = {
                    {
                        let output: V<::bevy_math::URect> = ::bevy_math::IRect::as_urect(
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
            |_self: R<::bevy_math::IRect>| {
                let output: V<::bevy_math::IVec2> = {
                    {
                        let output: V<::bevy_math::IVec2> = ::bevy_math::IRect::center(
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
            |_self: R<::bevy_math::IRect>| {
                let output: V<::bevy_math::IRect> = {
                    {
                        let output: V<::bevy_math::IRect> = <::bevy_math::IRect as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_math::IRect>, point: V<::bevy_math::IVec2>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::IRect::contains(
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
            |_self: R<::bevy_math::IRect>, other: R<::bevy_math::IRect>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::IRect as ::core::cmp::PartialEq<
                            ::bevy_math::IRect,
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
            |origin: V<::bevy_math::IVec2>, half_size: V<::bevy_math::IVec2>| {
                let output: V<::bevy_math::IRect> = {
                    {
                        let output: V<::bevy_math::IRect> = ::bevy_math::IRect::from_center_half_size(
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
            |origin: V<::bevy_math::IVec2>, size: V<::bevy_math::IVec2>| {
                let output: V<::bevy_math::IRect> = {
                    {
                        let output: V<::bevy_math::IRect> = ::bevy_math::IRect::from_center_size(
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
            |p0: V<::bevy_math::IVec2>, p1: V<::bevy_math::IVec2>| {
                let output: V<::bevy_math::IRect> = {
                    {
                        let output: V<::bevy_math::IRect> = ::bevy_math::IRect::from_corners(
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
            |_self: R<::bevy_math::IRect>| {
                let output: V<::bevy_math::IVec2> = {
                    {
                        let output: V<::bevy_math::IVec2> = ::bevy_math::IRect::half_size(
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
            |_self: R<::bevy_math::IRect>| {
                let output: i32 = {
                    {
                        let output: i32 = ::bevy_math::IRect::height(&_self).into();
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
            |_self: R<::bevy_math::IRect>, expansion: i32| {
                let output: V<::bevy_math::IRect> = {
                    {
                        let output: V<::bevy_math::IRect> = ::bevy_math::IRect::inflate(
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
            |_self: R<::bevy_math::IRect>, other: V<::bevy_math::IRect>| {
                let output: V<::bevy_math::IRect> = {
                    {
                        let output: V<::bevy_math::IRect> = ::bevy_math::IRect::intersect(
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
            |_self: R<::bevy_math::IRect>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::IRect::is_empty(&_self).into();
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
                let output: V<::bevy_math::IRect> = {
                    {
                        let output: V<::bevy_math::IRect> = ::bevy_math::IRect::new(
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
            |_self: R<::bevy_math::IRect>| {
                let output: V<::bevy_math::IVec2> = {
                    {
                        let output: V<::bevy_math::IVec2> = ::bevy_math::IRect::size(
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
            |_self: R<::bevy_math::IRect>, other: V<::bevy_math::IRect>| {
                let output: V<::bevy_math::IRect> = {
                    {
                        let output: V<::bevy_math::IRect> = ::bevy_math::IRect::union(
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
            |_self: R<::bevy_math::IRect>, other: V<::bevy_math::IVec2>| {
                let output: V<::bevy_math::IRect> = {
                    {
                        let output: V<::bevy_math::IRect> = ::bevy_math::IRect::union_point(
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
            |_self: R<::bevy_math::IRect>| {
                let output: i32 = {
                    {
                        let output: i32 = ::bevy_math::IRect::width(&_self).into();
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
        .register_type_data::<::bevy_math::IRect, bevy_mod_scripting_bindings::MarkAsGenerated>();
}
pub(crate) fn register_rect_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::Rect,
    >::new(world)
        .register_documented(
            "area",
            |_self: R<::bevy_math::Rect>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::Rect::area(&_self).into();
                        output
                    }
                };
                output
            },
            " Return the area of this rectangle.\n # Examples\n ```\n # use bevy_math::Rect;\n let r = Rect::new(0., 0., 10., 10.); // w=10 h=10\n assert_eq!(r.area(), 100.0);\n ```",
            &["_self"],
        )
        .register_documented(
            "as_irect",
            |_self: R<::bevy_math::Rect>| {
                let output: V<::bevy_math::IRect> = {
                    {
                        let output: V<::bevy_math::IRect> = ::bevy_math::Rect::as_irect(
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
            |_self: R<::bevy_math::Rect>| {
                let output: V<::bevy_math::URect> = {
                    {
                        let output: V<::bevy_math::URect> = ::bevy_math::Rect::as_urect(
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
            |_self: R<::bevy_math::Rect>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_math::Rect::center(
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
            "clamp_point",
            |_self: R<::bevy_math::Rect>, point: V<::bevy_math::Vec2>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_math::Rect::clamp_point(
                                &_self,
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Clamps a point to lie within this rectangle.\n # Examples\n ```\n # use bevy_math::{Rect, Vec2};\n let r = Rect::new(0., 0., 4., 5.);\n assert_eq!(r.clamp_point(Vec2::new(-1., 6.)), Vec2::new(0., 5.));\n assert_eq!(r.clamp_point(Vec2::ONE), Vec2::ONE);\n ```",
            &["_self", "point"],
        )
        .register_documented(
            "clone",
            |_self: R<::bevy_math::Rect>| {
                let output: V<::bevy_math::Rect> = {
                    {
                        let output: V<::bevy_math::Rect> = <::bevy_math::Rect as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_math::Rect>, point: V<::bevy_math::Vec2>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::Rect::contains(
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
            |_self: R<::bevy_math::Rect>, other: R<::bevy_math::Rect>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::Rect as ::core::cmp::PartialEq<
                            ::bevy_math::Rect,
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
            |origin: V<::bevy_math::Vec2>, half_size: V<::bevy_math::Vec2>| {
                let output: V<::bevy_math::Rect> = {
                    {
                        let output: V<::bevy_math::Rect> = ::bevy_math::Rect::from_center_half_size(
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
            |origin: V<::bevy_math::Vec2>, size: V<::bevy_math::Vec2>| {
                let output: V<::bevy_math::Rect> = {
                    {
                        let output: V<::bevy_math::Rect> = ::bevy_math::Rect::from_center_size(
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
            |p0: V<::bevy_math::Vec2>, p1: V<::bevy_math::Vec2>| {
                let output: V<::bevy_math::Rect> = {
                    {
                        let output: V<::bevy_math::Rect> = ::bevy_math::Rect::from_corners(
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
            |_self: R<::bevy_math::Rect>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_math::Rect::half_size(
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
            |_self: R<::bevy_math::Rect>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::Rect::height(&_self).into();
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
            |_self: R<::bevy_math::Rect>, expansion: f32| {
                let output: V<::bevy_math::Rect> = {
                    {
                        let output: V<::bevy_math::Rect> = ::bevy_math::Rect::inflate(
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
            |_self: R<::bevy_math::Rect>, other: V<::bevy_math::Rect>| {
                let output: V<::bevy_math::Rect> = {
                    {
                        let output: V<::bevy_math::Rect> = ::bevy_math::Rect::intersect(
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
            |_self: R<::bevy_math::Rect>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::Rect::is_empty(&_self).into();
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
                let output: V<::bevy_math::Rect> = {
                    {
                        let output: V<::bevy_math::Rect> = ::bevy_math::Rect::new(
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
            |_self: R<::bevy_math::Rect>, other: V<::bevy_math::Rect>| {
                let output: V<::bevy_math::Rect> = {
                    {
                        let output: V<::bevy_math::Rect> = ::bevy_math::Rect::normalize(
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
            |_self: R<::bevy_math::Rect>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_math::Rect::size(
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
            "translate",
            |_self: R<::bevy_math::Rect>, offset: V<::bevy_math::Vec2>| {
                let output: V<::bevy_math::Rect> = {
                    {
                        let output: V<::bevy_math::Rect> = ::bevy_math::Rect::translate(
                                &_self,
                                offset.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the rectangle translated by the given offset.\n # Examples\n ```\n # use bevy_math::{Rect, Vec2};\n let r = Rect::new(0., 0., 5., 1.); // w=5 h=1\n let r2 = r.translate(Vec2::new(2., -3.));\n assert!(r2.min.abs_diff_eq(Vec2::new(2., -3.), 1e-5));\n assert!(r2.max.abs_diff_eq(Vec2::new(7., -2.), 1e-5));\n ```",
            &["_self", "offset"],
        )
        .register_documented(
            "union",
            |_self: R<::bevy_math::Rect>, other: V<::bevy_math::Rect>| {
                let output: V<::bevy_math::Rect> = {
                    {
                        let output: V<::bevy_math::Rect> = ::bevy_math::Rect::union(
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
            |_self: R<::bevy_math::Rect>, other: V<::bevy_math::Vec2>| {
                let output: V<::bevy_math::Rect> = {
                    {
                        let output: V<::bevy_math::Rect> = ::bevy_math::Rect::union_point(
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
            |_self: R<::bevy_math::Rect>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_math::Rect::width(&_self).into();
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
        .register_type_data::<::bevy_math::Rect, bevy_mod_scripting_bindings::MarkAsGenerated>();
}
pub(crate) fn register_u_rect_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::URect,
    >::new(world)
        .register_documented(
            "as_irect",
            |_self: R<::bevy_math::URect>| {
                let output: V<::bevy_math::IRect> = {
                    {
                        let output: V<::bevy_math::IRect> = ::bevy_math::URect::as_irect(
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
            |_self: R<::bevy_math::URect>| {
                let output: V<::bevy_math::Rect> = {
                    {
                        let output: V<::bevy_math::Rect> = ::bevy_math::URect::as_rect(
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
            "center",
            |_self: R<::bevy_math::URect>| {
                let output: V<::bevy_math::UVec2> = {
                    {
                        let output: V<::bevy_math::UVec2> = ::bevy_math::URect::center(
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
            |_self: R<::bevy_math::URect>| {
                let output: V<::bevy_math::URect> = {
                    {
                        let output: V<::bevy_math::URect> = <::bevy_math::URect as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_math::URect>, point: V<::bevy_math::UVec2>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::URect::contains(
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
            |_self: R<::bevy_math::URect>, other: R<::bevy_math::URect>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::URect as ::core::cmp::PartialEq<
                            ::bevy_math::URect,
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
            |origin: V<::bevy_math::UVec2>, half_size: V<::bevy_math::UVec2>| {
                let output: V<::bevy_math::URect> = {
                    {
                        let output: V<::bevy_math::URect> = ::bevy_math::URect::from_center_half_size(
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
            |origin: V<::bevy_math::UVec2>, size: V<::bevy_math::UVec2>| {
                let output: V<::bevy_math::URect> = {
                    {
                        let output: V<::bevy_math::URect> = ::bevy_math::URect::from_center_size(
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
            |p0: V<::bevy_math::UVec2>, p1: V<::bevy_math::UVec2>| {
                let output: V<::bevy_math::URect> = {
                    {
                        let output: V<::bevy_math::URect> = ::bevy_math::URect::from_corners(
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
            |_self: R<::bevy_math::URect>| {
                let output: V<::bevy_math::UVec2> = {
                    {
                        let output: V<::bevy_math::UVec2> = ::bevy_math::URect::half_size(
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
            |_self: R<::bevy_math::URect>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_math::URect::height(&_self).into();
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
            |_self: R<::bevy_math::URect>, expansion: i32| {
                let output: V<::bevy_math::URect> = {
                    {
                        let output: V<::bevy_math::URect> = ::bevy_math::URect::inflate(
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
            |_self: R<::bevy_math::URect>, other: V<::bevy_math::URect>| {
                let output: V<::bevy_math::URect> = {
                    {
                        let output: V<::bevy_math::URect> = ::bevy_math::URect::intersect(
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
            |_self: R<::bevy_math::URect>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_math::URect::is_empty(&_self).into();
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
                let output: V<::bevy_math::URect> = {
                    {
                        let output: V<::bevy_math::URect> = ::bevy_math::URect::new(
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
            |_self: R<::bevy_math::URect>| {
                let output: V<::bevy_math::UVec2> = {
                    {
                        let output: V<::bevy_math::UVec2> = ::bevy_math::URect::size(
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
            |_self: R<::bevy_math::URect>, other: V<::bevy_math::URect>| {
                let output: V<::bevy_math::URect> = {
                    {
                        let output: V<::bevy_math::URect> = ::bevy_math::URect::union(
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
            |_self: R<::bevy_math::URect>, other: V<::bevy_math::UVec2>| {
                let output: V<::bevy_math::URect> = {
                    {
                        let output: V<::bevy_math::URect> = ::bevy_math::URect::union_point(
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
            |_self: R<::bevy_math::URect>| {
                let output: u32 = {
                    {
                        let output: u32 = ::bevy_math::URect::width(&_self).into();
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
        .register_type_data::<::bevy_math::URect, bevy_mod_scripting_bindings::MarkAsGenerated>();
}
pub(crate) fn register_dir_4_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::Dir4,
    >::new(world)
        .register_documented(
            "as_vec4",
            |_self: R<::bevy_math::Dir4>| {
                let output: V<::bevy_math::Vec4> = {
                    {
                        let output: V<::bevy_math::Vec4> = ::bevy_math::Dir4::as_vec4(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the inner [`Vec4`]",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: R<::bevy_math::Dir4>| {
                let output: V<::bevy_math::Dir4> = {
                    {
                        let output: V<::bevy_math::Dir4> = <::bevy_math::Dir4 as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_math::Dir4>, other: R<::bevy_math::Dir4>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_math::Dir4 as ::core::cmp::PartialEq<
                            ::bevy_math::Dir4,
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
            |_self: V<::bevy_math::Dir4>| {
                let output: V<::bevy_math::Dir4> = {
                    {
                        let output: V<::bevy_math::Dir4> = ::bevy_math::Dir4::fast_renormalize(
                                _self.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns `self` after an approximate normalization, assuming the value is already nearly normalized.\n Useful for preventing numerical error accumulation.",
            &["_self"],
        )
        .register_documented(
            "from_xyzw_unchecked",
            |x: f32, y: f32, z: f32, w: f32| {
                let output: V<::bevy_math::Dir4> = {
                    {
                        let output: V<::bevy_math::Dir4> = ::bevy_math::Dir4::from_xyzw_unchecked(
                                x,
                                y,
                                z,
                                w,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a direction from its `x`, `y`, `z`, and `w` components, assuming the resulting vector is normalized.\n # Warning\n The vector produced from `x`, `y`, `z`, and `w` must be normalized, i.e its length must be `1.0`.",
            &["x", "y", "z", "w"],
        )
        .register_documented(
            "new_unchecked",
            |value: V<::bevy_math::Vec4>| {
                let output: V<::bevy_math::Dir4> = {
                    {
                        let output: V<::bevy_math::Dir4> = ::bevy_math::Dir4::new_unchecked(
                                value.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a [`Dir4`] from a [`Vec4`] that is already normalized.\n # Warning\n `value` must be normalized, i.e its length must be `1.0`.",
            &["value"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_math::Dir4, bevy_mod_scripting_bindings::MarkAsGenerated>();
}
pub(crate) fn register_float_ord_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_math::FloatOrd,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_math::FloatOrd>| {
                let output: V<::bevy_math::FloatOrd> = {
                    {
                        let output: V<::bevy_math::FloatOrd> = <::bevy_math::FloatOrd as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_math::FloatOrd>, other: R<::bevy_math::FloatOrd>| {
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
            |_self: R<::bevy_math::FloatOrd>, other: R<::bevy_math::FloatOrd>| {
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
            |_self: R<::bevy_math::FloatOrd>, other: R<::bevy_math::FloatOrd>| {
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
            |_self: R<::bevy_math::FloatOrd>, other: R<::bevy_math::FloatOrd>| {
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
            |_self: R<::bevy_math::FloatOrd>, other: R<::bevy_math::FloatOrd>| {
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
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<::bevy_math::FloatOrd, bevy_mod_scripting_bindings::MarkAsGenerated>(
        );
}
impl Plugin for BevyMathScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_aspect_ratio_functions(&mut world);
        register_compass_octant_functions(&mut world);
        register_compass_quadrant_functions(&mut world);
        register_isometry_2_d_functions(&mut world);
        register_isometry_3_d_functions(&mut world);
        register_rot_2_functions(&mut world);
        register_dir_2_functions(&mut world);
        register_dir_3_functions(&mut world);
        register_dir_3_a_functions(&mut world);
        register_i_rect_functions(&mut world);
        register_rect_functions(&mut world);
        register_u_rect_functions(&mut world);
        register_dir_4_functions(&mut world);
        register_float_ord_functions(&mut world);
    }
}
