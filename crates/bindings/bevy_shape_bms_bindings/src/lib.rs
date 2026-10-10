
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
pub struct BevyShapeScriptingPlugin;
pub(crate) fn register_ray_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Ray2d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Ray2d>| {
                let output: V<::bevy_shape::Ray2d> = {
                    {
                        let output: V<::bevy_shape::Ray2d> = <::bevy_shape::Ray2d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Ray2d>, other: R<::bevy_shape::Ray2d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Ray2d as ::core::cmp::PartialEq<
                            ::bevy_shape::Ray2d,
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
            |_self: R<::bevy_shape::Ray2d>, distance: f32| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::Ray2d::get_point(
                                &_self,
                                distance,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the point at a given distance along the ray.",
            &["_self", "distance"],
        )
        .register_documented(
            "intersect_plane",
            |
                _self: R<::bevy_shape::Ray2d>,
                plane_origin: V<::bevy_math::Vec2>,
                plane: V<::bevy_shape::Plane2d>|
            {
                let output: ::core::option::Option<f32> = {
                    {
                        let output: ::core::option::Option<f32> = ::bevy_shape::Ray2d::intersect_plane(
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
            " Returns the distance to a plane if the ray intersects it.\n Use [`Ray2d::plane_intersection_point`] to get the intersection point directly.",
            &["_self", "plane_origin", "plane"],
        )
        .register_documented(
            "new",
            |origin: V<::bevy_math::Vec2>, direction: V<::bevy_math::Dir2>| {
                let output: V<::bevy_shape::Ray2d> = {
                    {
                        let output: V<::bevy_shape::Ray2d> = ::bevy_shape::Ray2d::new(
                                origin.into_inner(),
                                direction.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new `Ray2d` from a given origin and direction",
            &["origin", "direction"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_shape::Ray2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ray_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Ray3d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Ray3d>| {
                let output: V<::bevy_shape::Ray3d> = {
                    {
                        let output: V<::bevy_shape::Ray3d> = <::bevy_shape::Ray3d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Ray3d>, other: R<::bevy_shape::Ray3d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Ray3d as ::core::cmp::PartialEq<
                            ::bevy_shape::Ray3d,
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
            |_self: R<::bevy_shape::Ray3d>, distance: f32| {
                let output: V<::bevy_math::Vec3> = {
                    {
                        let output: V<::bevy_math::Vec3> = ::bevy_shape::Ray3d::get_point(
                                &_self,
                                distance,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the point at a given distance along the ray",
            &["_self", "distance"],
        )
        .register_documented(
            "intersect_plane",
            |
                _self: R<::bevy_shape::Ray3d>,
                plane_origin: V<::bevy_math::Vec3>,
                plane: V<::bevy_shape::InfinitePlane3d>|
            {
                let output: ::core::option::Option<f32> = {
                    {
                        let output: ::core::option::Option<f32> = ::bevy_shape::Ray3d::intersect_plane(
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
            " Returns the distance to a plane if the ray intersects it\n Use [`Ray3d::plane_intersection_point`] to get the intersection point directly.",
            &["_self", "plane_origin", "plane"],
        )
        .register_documented(
            "new",
            |origin: V<::bevy_math::Vec3>, direction: V<::bevy_math::Dir3>| {
                let output: V<::bevy_shape::Ray3d> = {
                    {
                        let output: V<::bevy_shape::Ray3d> = ::bevy_shape::Ray3d::new(
                                origin.into_inner(),
                                direction.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Creates a new `Ray3d` from a given origin and direction",
            &["origin", "direction"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_shape::Ray3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_circle_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Circle,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Circle>| {
                let output: V<::bevy_shape::Circle> = {
                    {
                        let output: V<::bevy_shape::Circle> = <::bevy_shape::Circle as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Circle>, point: V<::bevy_math::Vec2>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::Circle::closest_point(
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
            |_self: R<::bevy_shape::Circle>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Circle::diameter(&_self).into();
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
            |_self: R<::bevy_shape::Circle>, other: R<::bevy_shape::Circle>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Circle as ::core::cmp::PartialEq<
                            ::bevy_shape::Circle,
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
                let output: V<::bevy_shape::Circle> = {
                    {
                        let output: V<::bevy_shape::Circle> = ::bevy_shape::Circle::new(
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
            ::bevy_shape::Circle,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_arc_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Arc2d,
    >::new(world)
        .register_documented(
            "angle",
            |_self: R<::bevy_shape::Arc2d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Arc2d::angle(&_self).into();
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
            |_self: R<::bevy_shape::Arc2d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Arc2d::apothem(&_self).into();
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
            |_self: R<::bevy_shape::Arc2d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Arc2d::chord_length(&_self)
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
            |_self: R<::bevy_shape::Arc2d>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::Arc2d::chord_midpoint(
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
            |_self: R<::bevy_shape::Arc2d>| {
                let output: V<::bevy_shape::Arc2d> = {
                    {
                        let output: V<::bevy_shape::Arc2d> = <::bevy_shape::Arc2d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Arc2d>, other: R<::bevy_shape::Arc2d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Arc2d as ::core::cmp::PartialEq<
                            ::bevy_shape::Arc2d,
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
                let output: V<::bevy_shape::Arc2d> = {
                    {
                        let output: V<::bevy_shape::Arc2d> = ::bevy_shape::Arc2d::from_degrees(
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
                let output: V<::bevy_shape::Arc2d> = {
                    {
                        let output: V<::bevy_shape::Arc2d> = ::bevy_shape::Arc2d::from_radians(
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
                let output: V<::bevy_shape::Arc2d> = {
                    {
                        let output: V<::bevy_shape::Arc2d> = ::bevy_shape::Arc2d::from_turns(
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
            |_self: R<::bevy_shape::Arc2d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Arc2d::half_chord_length(&_self)
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
            |_self: R<::bevy_shape::Arc2d>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_shape::Arc2d::is_major(&_self).into();
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
            |_self: R<::bevy_shape::Arc2d>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_shape::Arc2d::is_minor(&_self).into();
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
            |_self: R<::bevy_shape::Arc2d>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::Arc2d::left_endpoint(
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
            |_self: R<::bevy_shape::Arc2d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Arc2d::length(&_self).into();
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
            |_self: R<::bevy_shape::Arc2d>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::Arc2d::midpoint(
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
                let output: V<::bevy_shape::Arc2d> = {
                    {
                        let output: V<::bevy_shape::Arc2d> = ::bevy_shape::Arc2d::new(
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
            |_self: R<::bevy_shape::Arc2d>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::Arc2d::right_endpoint(
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
            |_self: R<::bevy_shape::Arc2d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Arc2d::sagitta(&_self).into();
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
            ::bevy_shape::Arc2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_circular_sector_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::CircularSector,
    >::new(world)
        .register_documented(
            "angle",
            |_self: R<::bevy_shape::CircularSector>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::CircularSector::angle(&_self)
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
            |_self: R<::bevy_shape::CircularSector>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::CircularSector::apothem(&_self)
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
            |_self: R<::bevy_shape::CircularSector>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::CircularSector::arc_length(
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
            |_self: R<::bevy_shape::CircularSector>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::CircularSector::chord_length(
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
            |_self: R<::bevy_shape::CircularSector>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::CircularSector::chord_midpoint(
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
            |_self: R<::bevy_shape::CircularSector>| {
                let output: V<::bevy_shape::CircularSector> = {
                    {
                        let output: V<::bevy_shape::CircularSector> = <::bevy_shape::CircularSector as ::core::clone::Clone>::clone(
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
                _self: R<::bevy_shape::CircularSector>,
                other: R<::bevy_shape::CircularSector>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::CircularSector as ::core::cmp::PartialEq<
                            ::bevy_shape::CircularSector,
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
                let output: V<::bevy_shape::CircularSector> = {
                    {
                        let output: V<::bevy_shape::CircularSector> = ::bevy_shape::CircularSector::from_degrees(
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
                let output: V<::bevy_shape::CircularSector> = {
                    {
                        let output: V<::bevy_shape::CircularSector> = ::bevy_shape::CircularSector::from_radians(
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
                let output: V<::bevy_shape::CircularSector> = {
                    {
                        let output: V<::bevy_shape::CircularSector> = ::bevy_shape::CircularSector::from_turns(
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
            |_self: R<::bevy_shape::CircularSector>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::CircularSector::half_angle(
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
            |_self: R<::bevy_shape::CircularSector>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::CircularSector::half_chord_length(
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
                let output: V<::bevy_shape::CircularSector> = {
                    {
                        let output: V<::bevy_shape::CircularSector> = ::bevy_shape::CircularSector::new(
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
            |_self: R<::bevy_shape::CircularSector>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::CircularSector::radius(&_self)
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
            |_self: R<::bevy_shape::CircularSector>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::CircularSector::sagitta(&_self)
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
            ::bevy_shape::CircularSector,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_circular_segment_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::CircularSegment,
    >::new(world)
        .register_documented(
            "angle",
            |_self: R<::bevy_shape::CircularSegment>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::CircularSegment::angle(&_self)
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
            |_self: R<::bevy_shape::CircularSegment>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::CircularSegment::apothem(&_self)
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
            |_self: R<::bevy_shape::CircularSegment>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::CircularSegment::arc_length(
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
            |_self: R<::bevy_shape::CircularSegment>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::CircularSegment::chord_length(
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
            |_self: R<::bevy_shape::CircularSegment>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::CircularSegment::chord_midpoint(
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
            |_self: R<::bevy_shape::CircularSegment>| {
                let output: V<::bevy_shape::CircularSegment> = {
                    {
                        let output: V<::bevy_shape::CircularSegment> = <::bevy_shape::CircularSegment as ::core::clone::Clone>::clone(
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
                _self: R<::bevy_shape::CircularSegment>,
                other: R<::bevy_shape::CircularSegment>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::CircularSegment as ::core::cmp::PartialEq<
                            ::bevy_shape::CircularSegment,
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
                let output: V<::bevy_shape::CircularSegment> = {
                    {
                        let output: V<::bevy_shape::CircularSegment> = ::bevy_shape::CircularSegment::from_degrees(
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
                let output: V<::bevy_shape::CircularSegment> = {
                    {
                        let output: V<::bevy_shape::CircularSegment> = ::bevy_shape::CircularSegment::from_radians(
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
                let output: V<::bevy_shape::CircularSegment> = {
                    {
                        let output: V<::bevy_shape::CircularSegment> = ::bevy_shape::CircularSegment::from_turns(
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
            |_self: R<::bevy_shape::CircularSegment>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::CircularSegment::half_angle(
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
            |_self: R<::bevy_shape::CircularSegment>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::CircularSegment::half_chord_length(
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
            |radius: f32, half_angle: f32| {
                let output: V<::bevy_shape::CircularSegment> = {
                    {
                        let output: V<::bevy_shape::CircularSegment> = ::bevy_shape::CircularSegment::new(
                                radius,
                                half_angle,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new [`CircularSegment`] from a `radius`, and a `half_angle` in radians.",
            &["radius", "half_angle"],
        )
        .register_documented(
            "radius",
            |_self: R<::bevy_shape::CircularSegment>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::CircularSegment::radius(&_self)
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
            |_self: R<::bevy_shape::CircularSegment>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::CircularSegment::sagitta(&_self)
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
            ::bevy_shape::CircularSegment,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ellipse_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Ellipse,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Ellipse>| {
                let output: V<::bevy_shape::Ellipse> = {
                    {
                        let output: V<::bevy_shape::Ellipse> = <::bevy_shape::Ellipse as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Ellipse>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Ellipse::eccentricity(&_self)
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
            |_self: R<::bevy_shape::Ellipse>, other: R<::bevy_shape::Ellipse>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Ellipse as ::core::cmp::PartialEq<
                            ::bevy_shape::Ellipse,
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
            |_self: R<::bevy_shape::Ellipse>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Ellipse::focal_length(&_self)
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
            |size: V<::bevy_math::Vec2>| {
                let output: V<::bevy_shape::Ellipse> = {
                    {
                        let output: V<::bevy_shape::Ellipse> = ::bevy_shape::Ellipse::from_size(
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
                let output: V<::bevy_shape::Ellipse> = {
                    {
                        let output: V<::bevy_shape::Ellipse> = ::bevy_shape::Ellipse::new(
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
            |_self: R<::bevy_shape::Ellipse>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Ellipse::semi_major(&_self)
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
            |_self: R<::bevy_shape::Ellipse>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Ellipse::semi_minor(&_self)
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
            ::bevy_shape::Ellipse,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_annulus_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Annulus,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Annulus>| {
                let output: V<::bevy_shape::Annulus> = {
                    {
                        let output: V<::bevy_shape::Annulus> = <::bevy_shape::Annulus as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Annulus>, point: V<::bevy_math::Vec2>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::Annulus::closest_point(
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
            |_self: R<::bevy_shape::Annulus>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Annulus::diameter(&_self).into();
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
            |_self: R<::bevy_shape::Annulus>, other: R<::bevy_shape::Annulus>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Annulus as ::core::cmp::PartialEq<
                            ::bevy_shape::Annulus,
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
                let output: V<::bevy_shape::Annulus> = {
                    {
                        let output: V<::bevy_shape::Annulus> = ::bevy_shape::Annulus::new(
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
            |_self: R<::bevy_shape::Annulus>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Annulus::thickness(&_self)
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
            ::bevy_shape::Annulus,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_rhombus_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Rhombus,
    >::new(world)
        .register_documented(
            "circumradius",
            |_self: R<::bevy_shape::Rhombus>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Rhombus::circumradius(&_self)
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
            |_self: R<::bevy_shape::Rhombus>| {
                let output: V<::bevy_shape::Rhombus> = {
                    {
                        let output: V<::bevy_shape::Rhombus> = <::bevy_shape::Rhombus as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Rhombus>, point: V<::bevy_math::Vec2>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::Rhombus::closest_point(
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
            |_self: R<::bevy_shape::Rhombus>, other: R<::bevy_shape::Rhombus>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Rhombus as ::core::cmp::PartialEq<
                            ::bevy_shape::Rhombus,
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
                let output: V<::bevy_shape::Rhombus> = {
                    {
                        let output: V<::bevy_shape::Rhombus> = ::bevy_shape::Rhombus::from_inradius(
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
                let output: V<::bevy_shape::Rhombus> = {
                    {
                        let output: V<::bevy_shape::Rhombus> = ::bevy_shape::Rhombus::from_side(
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
            |_self: R<::bevy_shape::Rhombus>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Rhombus::inradius(&_self).into();
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
                let output: V<::bevy_shape::Rhombus> = {
                    {
                        let output: V<::bevy_shape::Rhombus> = ::bevy_shape::Rhombus::new(
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
            |_self: R<::bevy_shape::Rhombus>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Rhombus::side(&_self).into();
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
            ::bevy_shape::Rhombus,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_plane_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Plane2d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Plane2d>| {
                let output: V<::bevy_shape::Plane2d> = {
                    {
                        let output: V<::bevy_shape::Plane2d> = <::bevy_shape::Plane2d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Plane2d>, other: R<::bevy_shape::Plane2d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Plane2d as ::core::cmp::PartialEq<
                            ::bevy_shape::Plane2d,
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
            |normal: V<::bevy_math::Vec2>| {
                let output: V<::bevy_shape::Plane2d> = {
                    {
                        let output: V<::bevy_shape::Plane2d> = ::bevy_shape::Plane2d::new(
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
            ::bevy_shape::Plane2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_line_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Line2d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Line2d>| {
                let output: V<::bevy_shape::Line2d> = {
                    {
                        let output: V<::bevy_shape::Line2d> = <::bevy_shape::Line2d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Line2d>, other: R<::bevy_shape::Line2d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Line2d as ::core::cmp::PartialEq<
                            ::bevy_shape::Line2d,
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
            ::bevy_shape::Line2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_segment_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Segment2d,
    >::new(world)
        .register_documented(
            "center",
            |_self: R<::bevy_shape::Segment2d>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::Segment2d::center(
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
            |_self: R<::bevy_shape::Segment2d>| {
                let output: V<::bevy_shape::Segment2d> = {
                    {
                        let output: V<::bevy_shape::Segment2d> = ::bevy_shape::Segment2d::centered(
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
            |_self: R<::bevy_shape::Segment2d>| {
                let output: V<::bevy_shape::Segment2d> = {
                    {
                        let output: V<::bevy_shape::Segment2d> = <::bevy_shape::Segment2d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Segment2d>, point: V<::bevy_math::Vec2>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::Segment2d::closest_point(
                                &_self,
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the point on the [`Segment2d`] that is closest to the specified `point`.",
            &["_self", "point"],
        )
        .register_documented(
            "direction",
            |_self: R<::bevy_shape::Segment2d>| {
                let output: V<::bevy_math::Dir2> = {
                    {
                        let output: V<::bevy_math::Dir2> = ::bevy_shape::Segment2d::direction(
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
            |_self: R<::bevy_shape::Segment2d>, other: R<::bevy_shape::Segment2d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Segment2d as ::core::cmp::PartialEq<
                            ::bevy_shape::Segment2d,
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
            |direction: V<::bevy_math::Dir2>, length: f32| {
                let output: V<::bevy_shape::Segment2d> = {
                    {
                        let output: V<::bevy_shape::Segment2d> = ::bevy_shape::Segment2d::from_direction_and_length(
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
            |ray: V<::bevy_shape::Ray2d>, length: f32| {
                let output: V<::bevy_shape::Segment2d> = {
                    {
                        let output: V<::bevy_shape::Segment2d> = ::bevy_shape::Segment2d::from_ray_and_length(
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
            |scaled_direction: V<::bevy_math::Vec2>| {
                let output: V<::bevy_shape::Segment2d> = {
                    {
                        let output: V<::bevy_shape::Segment2d> = ::bevy_shape::Segment2d::from_scaled_direction(
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
            |_self: R<::bevy_shape::Segment2d>| {
                let output: V<::bevy_math::Dir2> = {
                    {
                        let output: V<::bevy_math::Dir2> = ::bevy_shape::Segment2d::left_normal(
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
            |_self: R<::bevy_shape::Segment2d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Segment2d::length(&_self).into();
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
            |_self: R<::bevy_shape::Segment2d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Segment2d::length_squared(&_self)
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
            |point1: V<::bevy_math::Vec2>, point2: V<::bevy_math::Vec2>| {
                let output: V<::bevy_shape::Segment2d> = {
                    {
                        let output: V<::bevy_shape::Segment2d> = ::bevy_shape::Segment2d::new(
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
            |_self: R<::bevy_shape::Segment2d>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::Segment2d::point1(
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
            |_self: R<::bevy_shape::Segment2d>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::Segment2d::point2(
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
            |_self: R<::bevy_shape::Segment2d>, length: f32| {
                let output: V<::bevy_shape::Segment2d> = {
                    {
                        let output: V<::bevy_shape::Segment2d> = ::bevy_shape::Segment2d::resized(
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
            |mut _self: M<::bevy_shape::Segment2d>| {
                let output: () = {
                    {
                        let output: () = ::bevy_shape::Segment2d::reverse(&mut _self)
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
            |_self: V<::bevy_shape::Segment2d>| {
                let output: V<::bevy_shape::Segment2d> = {
                    {
                        let output: V<::bevy_shape::Segment2d> = ::bevy_shape::Segment2d::reversed(
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
            |_self: R<::bevy_shape::Segment2d>| {
                let output: V<::bevy_math::Dir2> = {
                    {
                        let output: V<::bevy_math::Dir2> = ::bevy_shape::Segment2d::right_normal(
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
            |_self: R<::bevy_shape::Segment2d>, rotation: V<::bevy_math::Rot2>| {
                let output: V<::bevy_shape::Segment2d> = {
                    {
                        let output: V<::bevy_shape::Segment2d> = ::bevy_shape::Segment2d::rotated(
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
                _self: R<::bevy_shape::Segment2d>,
                rotation: V<::bevy_math::Rot2>,
                point: V<::bevy_math::Vec2>|
            {
                let output: V<::bevy_shape::Segment2d> = {
                    {
                        let output: V<::bevy_shape::Segment2d> = ::bevy_shape::Segment2d::rotated_around(
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
            |_self: R<::bevy_shape::Segment2d>, rotation: V<::bevy_math::Rot2>| {
                let output: V<::bevy_shape::Segment2d> = {
                    {
                        let output: V<::bevy_shape::Segment2d> = ::bevy_shape::Segment2d::rotated_around_center(
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
            |_self: R<::bevy_shape::Segment2d>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::Segment2d::scaled_direction(
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
            |_self: R<::bevy_shape::Segment2d>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::Segment2d::scaled_left_normal(
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
            |_self: R<::bevy_shape::Segment2d>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::Segment2d::scaled_right_normal(
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
            |_self: R<::bevy_shape::Segment2d>, translation: V<::bevy_math::Vec2>| {
                let output: V<::bevy_shape::Segment2d> = {
                    {
                        let output: V<::bevy_shape::Segment2d> = ::bevy_shape::Segment2d::translated(
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
            ::bevy_shape::Segment2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_polyline_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Polyline2d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Polyline2d>| {
                let output: V<::bevy_shape::Polyline2d> = {
                    {
                        let output: V<::bevy_shape::Polyline2d> = <::bevy_shape::Polyline2d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Polyline2d>, other: R<::bevy_shape::Polyline2d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Polyline2d as ::core::cmp::PartialEq<
                            ::bevy_shape::Polyline2d,
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
            "with_subdivisions",
            |
                start: V<::bevy_math::Vec2>,
                end: V<::bevy_math::Vec2>,
                subdivisions: usize|
            {
                let output: V<::bevy_shape::Polyline2d> = {
                    {
                        let output: V<::bevy_shape::Polyline2d> = ::bevy_shape::Polyline2d::with_subdivisions(
                                start.into_inner(),
                                end.into_inner(),
                                subdivisions,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Polyline2d` from two endpoints with subdivision points.\n `subdivisions = 0` creates a simple line with just start and end points.\n `subdivisions = 1` adds one point in the middle, creating 2 segments, etc.",
            &["start", "end", "subdivisions"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_shape::Polyline2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_triangle_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Triangle2d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Triangle2d>| {
                let output: V<::bevy_shape::Triangle2d> = {
                    {
                        let output: V<::bevy_shape::Triangle2d> = <::bevy_shape::Triangle2d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Triangle2d>, other: R<::bevy_shape::Triangle2d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Triangle2d as ::core::cmp::PartialEq<
                            ::bevy_shape::Triangle2d,
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
            |_self: R<::bevy_shape::Triangle2d>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_shape::Triangle2d::is_acute(&_self)
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
            |_self: R<::bevy_shape::Triangle2d>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_shape::Triangle2d::is_degenerate(
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
            |_self: R<::bevy_shape::Triangle2d>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_shape::Triangle2d::is_obtuse(&_self)
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
            |a: V<::bevy_math::Vec2>, b: V<::bevy_math::Vec2>, c: V<::bevy_math::Vec2>| {
                let output: V<::bevy_shape::Triangle2d> = {
                    {
                        let output: V<::bevy_shape::Triangle2d> = ::bevy_shape::Triangle2d::new(
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
            |mut _self: M<::bevy_shape::Triangle2d>| {
                let output: () = {
                    {
                        let output: () = ::bevy_shape::Triangle2d::reverse(&mut _self)
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
            |_self: V<::bevy_shape::Triangle2d>| {
                let output: V<::bevy_shape::Triangle2d> = {
                    {
                        let output: V<::bevy_shape::Triangle2d> = ::bevy_shape::Triangle2d::reversed(
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
            ::bevy_shape::Triangle2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_rectangle_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Rectangle,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Rectangle>| {
                let output: V<::bevy_shape::Rectangle> = {
                    {
                        let output: V<::bevy_shape::Rectangle> = <::bevy_shape::Rectangle as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Rectangle>, point: V<::bevy_math::Vec2>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::Rectangle::closest_point(
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
            |_self: R<::bevy_shape::Rectangle>, other: R<::bevy_shape::Rectangle>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Rectangle as ::core::cmp::PartialEq<
                            ::bevy_shape::Rectangle,
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
            |point1: V<::bevy_math::Vec2>, point2: V<::bevy_math::Vec2>| {
                let output: V<::bevy_shape::Rectangle> = {
                    {
                        let output: V<::bevy_shape::Rectangle> = ::bevy_shape::Rectangle::from_corners(
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
                let output: V<::bevy_shape::Rectangle> = {
                    {
                        let output: V<::bevy_shape::Rectangle> = ::bevy_shape::Rectangle::from_length(
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
            |size: V<::bevy_math::Vec2>| {
                let output: V<::bevy_shape::Rectangle> = {
                    {
                        let output: V<::bevy_shape::Rectangle> = ::bevy_shape::Rectangle::from_size(
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
                let output: V<::bevy_shape::Rectangle> = {
                    {
                        let output: V<::bevy_shape::Rectangle> = ::bevy_shape::Rectangle::new(
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
            |_self: R<::bevy_shape::Rectangle>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::Rectangle::size(
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
            ::bevy_shape::Rectangle,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_polygon_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Polygon,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Polygon>| {
                let output: V<::bevy_shape::Polygon> = {
                    {
                        let output: V<::bevy_shape::Polygon> = <::bevy_shape::Polygon as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Polygon>, other: R<::bevy_shape::Polygon>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Polygon as ::core::cmp::PartialEq<
                            ::bevy_shape::Polygon,
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
            "is_simple",
            |_self: R<::bevy_shape::Polygon>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_shape::Polygon::is_simple(&_self)
                            .into();
                        output
                    }
                };
                output
            },
            " Tests if the polygon is simple.\n A polygon is simple if it is not self intersecting and not self tangent.\n As such, no two edges of the polygon may cross each other and each vertex must not lie on another edge.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_shape::Polygon,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_convex_polygon_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::ConvexPolygon,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::ConvexPolygon>| {
                let output: V<::bevy_shape::ConvexPolygon> = {
                    {
                        let output: V<::bevy_shape::ConvexPolygon> = <::bevy_shape::ConvexPolygon as ::core::clone::Clone>::clone(
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
                _self: R<::bevy_shape::ConvexPolygon>,
                other: R<::bevy_shape::ConvexPolygon>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::ConvexPolygon as ::core::cmp::PartialEq<
                            ::bevy_shape::ConvexPolygon,
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
            ::bevy_shape::ConvexPolygon,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_regular_polygon_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::RegularPolygon,
    >::new(world)
        .register_documented(
            "circumradius",
            |_self: R<::bevy_shape::RegularPolygon>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::RegularPolygon::circumradius(
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
            |_self: R<::bevy_shape::RegularPolygon>| {
                let output: V<::bevy_shape::RegularPolygon> = {
                    {
                        let output: V<::bevy_shape::RegularPolygon> = <::bevy_shape::RegularPolygon as ::core::clone::Clone>::clone(
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
                _self: R<::bevy_shape::RegularPolygon>,
                other: R<::bevy_shape::RegularPolygon>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::RegularPolygon as ::core::cmp::PartialEq<
                            ::bevy_shape::RegularPolygon,
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
            |_self: R<::bevy_shape::RegularPolygon>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::RegularPolygon::external_angle_degrees(
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
            |_self: R<::bevy_shape::RegularPolygon>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::RegularPolygon::external_angle_radians(
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
            |_self: R<::bevy_shape::RegularPolygon>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::RegularPolygon::inradius(&_self)
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
            |_self: R<::bevy_shape::RegularPolygon>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::RegularPolygon::internal_angle_degrees(
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
            |_self: R<::bevy_shape::RegularPolygon>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::RegularPolygon::internal_angle_radians(
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
                let output: V<::bevy_shape::RegularPolygon> = {
                    {
                        let output: V<::bevy_shape::RegularPolygon> = ::bevy_shape::RegularPolygon::new(
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
            |_self: R<::bevy_shape::RegularPolygon>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::RegularPolygon::side_length(
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
            ::bevy_shape::RegularPolygon,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_capsule_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Capsule2d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Capsule2d>| {
                let output: V<::bevy_shape::Capsule2d> = {
                    {
                        let output: V<::bevy_shape::Capsule2d> = <::bevy_shape::Capsule2d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Capsule2d>, other: R<::bevy_shape::Capsule2d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Capsule2d as ::core::cmp::PartialEq<
                            ::bevy_shape::Capsule2d,
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
                let output: V<::bevy_shape::Capsule2d> = {
                    {
                        let output: V<::bevy_shape::Capsule2d> = ::bevy_shape::Capsule2d::new(
                                radius,
                                length,
                            )
                            .into();
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
            |_self: R<::bevy_shape::Capsule2d>| {
                let output: V<::bevy_shape::Rectangle> = {
                    {
                        let output: V<::bevy_shape::Rectangle> = ::bevy_shape::Capsule2d::to_inner_rectangle(
                                &_self,
                            )
                            .into();
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
            ::bevy_shape::Capsule2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_sphere_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Sphere,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Sphere>| {
                let output: V<::bevy_shape::Sphere> = {
                    {
                        let output: V<::bevy_shape::Sphere> = <::bevy_shape::Sphere as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Sphere>, point: V<::bevy_math::Vec3>| {
                let output: V<::bevy_math::Vec3> = {
                    {
                        let output: V<::bevy_math::Vec3> = ::bevy_shape::Sphere::closest_point(
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
            |_self: R<::bevy_shape::Sphere>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Sphere::diameter(&_self).into();
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
            |_self: R<::bevy_shape::Sphere>, other: R<::bevy_shape::Sphere>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Sphere as ::core::cmp::PartialEq<
                            ::bevy_shape::Sphere,
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
                let output: V<::bevy_shape::Sphere> = {
                    {
                        let output: V<::bevy_shape::Sphere> = ::bevy_shape::Sphere::new(
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
            ::bevy_shape::Sphere,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_plane_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Plane3d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Plane3d>| {
                let output: V<::bevy_shape::Plane3d> = {
                    {
                        let output: V<::bevy_shape::Plane3d> = <::bevy_shape::Plane3d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Plane3d>, other: R<::bevy_shape::Plane3d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Plane3d as ::core::cmp::PartialEq<
                            ::bevy_shape::Plane3d,
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
            |normal: V<::bevy_math::Vec3>, half_size: V<::bevy_math::Vec2>| {
                let output: V<::bevy_shape::Plane3d> = {
                    {
                        let output: V<::bevy_shape::Plane3d> = ::bevy_shape::Plane3d::new(
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
            ::bevy_shape::Plane3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_infinite_plane_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::InfinitePlane3d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::InfinitePlane3d>| {
                let output: V<::bevy_shape::InfinitePlane3d> = {
                    {
                        let output: V<::bevy_shape::InfinitePlane3d> = <::bevy_shape::InfinitePlane3d as ::core::clone::Clone>::clone(
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
                _self: R<::bevy_shape::InfinitePlane3d>,
                other: R<::bevy_shape::InfinitePlane3d>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::InfinitePlane3d as ::core::cmp::PartialEq<
                            ::bevy_shape::InfinitePlane3d,
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
            |_self: R<::bevy_shape::InfinitePlane3d>, origin: V<::bevy_math::Vec3>| {
                let output: V<::bevy_math::Isometry3d> = {
                    {
                        let output: V<::bevy_math::Isometry3d> = ::bevy_shape::InfinitePlane3d::isometry_from_xy(
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
            |_self: R<::bevy_shape::InfinitePlane3d>, origin: V<::bevy_math::Vec3>| {
                let output: V<::bevy_math::Isometry3d> = {
                    {
                        let output: V<::bevy_math::Isometry3d> = ::bevy_shape::InfinitePlane3d::isometry_into_xy(
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
            ::bevy_shape::InfinitePlane3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_line_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Line3d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Line3d>| {
                let output: V<::bevy_shape::Line3d> = {
                    {
                        let output: V<::bevy_shape::Line3d> = <::bevy_shape::Line3d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Line3d>, other: R<::bevy_shape::Line3d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Line3d as ::core::cmp::PartialEq<
                            ::bevy_shape::Line3d,
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
            ::bevy_shape::Line3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_segment_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Segment3d,
    >::new(world)
        .register_documented(
            "center",
            |_self: R<::bevy_shape::Segment3d>| {
                let output: V<::bevy_math::Vec3> = {
                    {
                        let output: V<::bevy_math::Vec3> = ::bevy_shape::Segment3d::center(
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
            |_self: R<::bevy_shape::Segment3d>| {
                let output: V<::bevy_shape::Segment3d> = {
                    {
                        let output: V<::bevy_shape::Segment3d> = ::bevy_shape::Segment3d::centered(
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
            |_self: R<::bevy_shape::Segment3d>| {
                let output: V<::bevy_shape::Segment3d> = {
                    {
                        let output: V<::bevy_shape::Segment3d> = <::bevy_shape::Segment3d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Segment3d>, point: V<::bevy_math::Vec3>| {
                let output: V<::bevy_math::Vec3> = {
                    {
                        let output: V<::bevy_math::Vec3> = ::bevy_shape::Segment3d::closest_point(
                                &_self,
                                point.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the point on the [`Segment3d`] that is closest to the specified `point`.",
            &["_self", "point"],
        )
        .register_documented(
            "direction",
            |_self: R<::bevy_shape::Segment3d>| {
                let output: V<::bevy_math::Dir3> = {
                    {
                        let output: V<::bevy_math::Dir3> = ::bevy_shape::Segment3d::direction(
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
            |_self: R<::bevy_shape::Segment3d>, other: R<::bevy_shape::Segment3d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Segment3d as ::core::cmp::PartialEq<
                            ::bevy_shape::Segment3d,
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
            |direction: V<::bevy_math::Dir3>, length: f32| {
                let output: V<::bevy_shape::Segment3d> = {
                    {
                        let output: V<::bevy_shape::Segment3d> = ::bevy_shape::Segment3d::from_direction_and_length(
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
            |ray: V<::bevy_shape::Ray3d>, length: f32| {
                let output: V<::bevy_shape::Segment3d> = {
                    {
                        let output: V<::bevy_shape::Segment3d> = ::bevy_shape::Segment3d::from_ray_and_length(
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
            |scaled_direction: V<::bevy_math::Vec3>| {
                let output: V<::bevy_shape::Segment3d> = {
                    {
                        let output: V<::bevy_shape::Segment3d> = ::bevy_shape::Segment3d::from_scaled_direction(
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
            |_self: R<::bevy_shape::Segment3d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Segment3d::length(&_self).into();
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
            |_self: R<::bevy_shape::Segment3d>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Segment3d::length_squared(&_self)
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
            |point1: V<::bevy_math::Vec3>, point2: V<::bevy_math::Vec3>| {
                let output: V<::bevy_shape::Segment3d> = {
                    {
                        let output: V<::bevy_shape::Segment3d> = ::bevy_shape::Segment3d::new(
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
            |_self: R<::bevy_shape::Segment3d>| {
                let output: V<::bevy_math::Vec3> = {
                    {
                        let output: V<::bevy_math::Vec3> = ::bevy_shape::Segment3d::point1(
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
            |_self: R<::bevy_shape::Segment3d>| {
                let output: V<::bevy_math::Vec3> = {
                    {
                        let output: V<::bevy_math::Vec3> = ::bevy_shape::Segment3d::point2(
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
            |_self: R<::bevy_shape::Segment3d>, length: f32| {
                let output: V<::bevy_shape::Segment3d> = {
                    {
                        let output: V<::bevy_shape::Segment3d> = ::bevy_shape::Segment3d::resized(
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
            |mut _self: M<::bevy_shape::Segment3d>| {
                let output: () = {
                    {
                        let output: () = ::bevy_shape::Segment3d::reverse(&mut _self)
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
            |_self: V<::bevy_shape::Segment3d>| {
                let output: V<::bevy_shape::Segment3d> = {
                    {
                        let output: V<::bevy_shape::Segment3d> = ::bevy_shape::Segment3d::reversed(
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
            |_self: R<::bevy_shape::Segment3d>, rotation: V<::bevy_math::Quat>| {
                let output: V<::bevy_shape::Segment3d> = {
                    {
                        let output: V<::bevy_shape::Segment3d> = ::bevy_shape::Segment3d::rotated(
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
                _self: R<::bevy_shape::Segment3d>,
                rotation: V<::bevy_math::Quat>,
                point: V<::bevy_math::Vec3>|
            {
                let output: V<::bevy_shape::Segment3d> = {
                    {
                        let output: V<::bevy_shape::Segment3d> = ::bevy_shape::Segment3d::rotated_around(
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
            |_self: R<::bevy_shape::Segment3d>, rotation: V<::bevy_math::Quat>| {
                let output: V<::bevy_shape::Segment3d> = {
                    {
                        let output: V<::bevy_shape::Segment3d> = ::bevy_shape::Segment3d::rotated_around_center(
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
            |_self: R<::bevy_shape::Segment3d>| {
                let output: V<::bevy_math::Vec3> = {
                    {
                        let output: V<::bevy_math::Vec3> = ::bevy_shape::Segment3d::scaled_direction(
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
            |_self: R<::bevy_shape::Segment3d>, translation: V<::bevy_math::Vec3>| {
                let output: V<::bevy_shape::Segment3d> = {
                    {
                        let output: V<::bevy_shape::Segment3d> = ::bevy_shape::Segment3d::translated(
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
            ::bevy_shape::Segment3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_polyline_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Polyline3d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Polyline3d>| {
                let output: V<::bevy_shape::Polyline3d> = {
                    {
                        let output: V<::bevy_shape::Polyline3d> = <::bevy_shape::Polyline3d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Polyline3d>, other: R<::bevy_shape::Polyline3d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Polyline3d as ::core::cmp::PartialEq<
                            ::bevy_shape::Polyline3d,
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
            "with_subdivisions",
            |
                start: V<::bevy_math::Vec3>,
                end: V<::bevy_math::Vec3>,
                subdivisions: usize|
            {
                let output: V<::bevy_shape::Polyline3d> = {
                    {
                        let output: V<::bevy_shape::Polyline3d> = ::bevy_shape::Polyline3d::with_subdivisions(
                                start.into_inner(),
                                end.into_inner(),
                                subdivisions,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Create a new `Polyline3d` from two endpoints with subdivision points.\n `subdivisions = 0` creates a simple line with just start and end points.\n `subdivisions = 1` adds one point in the middle, creating 2 segments, etc.",
            &["start", "end", "subdivisions"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_shape::Polyline3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cuboid_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Cuboid,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Cuboid>| {
                let output: V<::bevy_shape::Cuboid> = {
                    {
                        let output: V<::bevy_shape::Cuboid> = <::bevy_shape::Cuboid as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Cuboid>, point: V<::bevy_math::Vec3>| {
                let output: V<::bevy_math::Vec3> = {
                    {
                        let output: V<::bevy_math::Vec3> = ::bevy_shape::Cuboid::closest_point(
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
            |_self: R<::bevy_shape::Cuboid>, other: R<::bevy_shape::Cuboid>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Cuboid as ::core::cmp::PartialEq<
                            ::bevy_shape::Cuboid,
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
            |point1: V<::bevy_math::Vec3>, point2: V<::bevy_math::Vec3>| {
                let output: V<::bevy_shape::Cuboid> = {
                    {
                        let output: V<::bevy_shape::Cuboid> = ::bevy_shape::Cuboid::from_corners(
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
                let output: V<::bevy_shape::Cuboid> = {
                    {
                        let output: V<::bevy_shape::Cuboid> = ::bevy_shape::Cuboid::from_length(
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
            |size: V<::bevy_math::Vec3>| {
                let output: V<::bevy_shape::Cuboid> = {
                    {
                        let output: V<::bevy_shape::Cuboid> = ::bevy_shape::Cuboid::from_size(
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
                let output: V<::bevy_shape::Cuboid> = {
                    {
                        let output: V<::bevy_shape::Cuboid> = ::bevy_shape::Cuboid::new(
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
            |_self: R<::bevy_shape::Cuboid>| {
                let output: V<::bevy_math::Vec3> = {
                    {
                        let output: V<::bevy_math::Vec3> = ::bevy_shape::Cuboid::size(
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
            ::bevy_shape::Cuboid,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cylinder_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Cylinder,
    >::new(world)
        .register_documented(
            "base",
            |_self: R<::bevy_shape::Cylinder>| {
                let output: V<::bevy_shape::Circle> = {
                    {
                        let output: V<::bevy_shape::Circle> = ::bevy_shape::Cylinder::base(
                                &_self,
                            )
                            .into();
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
            |_self: R<::bevy_shape::Cylinder>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Cylinder::base_area(&_self)
                            .into();
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
            |_self: R<::bevy_shape::Cylinder>| {
                let output: V<::bevy_shape::Cylinder> = {
                    {
                        let output: V<::bevy_shape::Cylinder> = <::bevy_shape::Cylinder as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Cylinder>, other: R<::bevy_shape::Cylinder>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Cylinder as ::core::cmp::PartialEq<
                            ::bevy_shape::Cylinder,
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
            |_self: R<::bevy_shape::Cylinder>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Cylinder::lateral_area(&_self)
                            .into();
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
                let output: V<::bevy_shape::Cylinder> = {
                    {
                        let output: V<::bevy_shape::Cylinder> = ::bevy_shape::Cylinder::new(
                                radius,
                                height,
                            )
                            .into();
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
            ::bevy_shape::Cylinder,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_capsule_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Capsule3d,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Capsule3d>| {
                let output: V<::bevy_shape::Capsule3d> = {
                    {
                        let output: V<::bevy_shape::Capsule3d> = <::bevy_shape::Capsule3d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Capsule3d>, other: R<::bevy_shape::Capsule3d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Capsule3d as ::core::cmp::PartialEq<
                            ::bevy_shape::Capsule3d,
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
                let output: V<::bevy_shape::Capsule3d> = {
                    {
                        let output: V<::bevy_shape::Capsule3d> = ::bevy_shape::Capsule3d::new(
                                radius,
                                length,
                            )
                            .into();
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
            |_self: R<::bevy_shape::Capsule3d>| {
                let output: V<::bevy_shape::Cylinder> = {
                    {
                        let output: V<::bevy_shape::Cylinder> = ::bevy_shape::Capsule3d::to_cylinder(
                                &_self,
                            )
                            .into();
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
            ::bevy_shape::Capsule3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_cone_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Cone,
    >::new(world)
        .register_documented(
            "base",
            |_self: R<::bevy_shape::Cone>| {
                let output: V<::bevy_shape::Circle> = {
                    {
                        let output: V<::bevy_shape::Circle> = ::bevy_shape::Cone::base(
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
            |_self: R<::bevy_shape::Cone>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Cone::base_area(&_self).into();
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
            |_self: R<::bevy_shape::Cone>| {
                let output: V<::bevy_shape::Cone> = {
                    {
                        let output: V<::bevy_shape::Cone> = <::bevy_shape::Cone as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Cone>, other: R<::bevy_shape::Cone>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Cone as ::core::cmp::PartialEq<
                            ::bevy_shape::Cone,
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
            |_self: R<::bevy_shape::Cone>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Cone::lateral_area(&_self)
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
                let output: V<::bevy_shape::Cone> = {
                    {
                        let output: V<::bevy_shape::Cone> = ::bevy_shape::Cone::new(
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
            |_self: R<::bevy_shape::Cone>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Cone::slant_height(&_self)
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
            ::bevy_shape::Cone,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_conical_frustum_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::ConicalFrustum,
    >::new(world)
        .register_documented(
            "bottom_base",
            |_self: R<::bevy_shape::ConicalFrustum>| {
                let output: V<::bevy_shape::Circle> = {
                    {
                        let output: V<::bevy_shape::Circle> = ::bevy_shape::ConicalFrustum::bottom_base(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the bottom base of the conical frustum as a [`Circle`]",
            &["_self"],
        )
        .register_documented(
            "bottom_base_area",
            |_self: R<::bevy_shape::ConicalFrustum>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::ConicalFrustum::bottom_base_area(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the surface area of the bottom base of the conical frustum",
            &["_self"],
        )
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::ConicalFrustum>| {
                let output: V<::bevy_shape::ConicalFrustum> = {
                    {
                        let output: V<::bevy_shape::ConicalFrustum> = <::bevy_shape::ConicalFrustum as ::core::clone::Clone>::clone(
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
                _self: R<::bevy_shape::ConicalFrustum>,
                other: R<::bevy_shape::ConicalFrustum>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::ConicalFrustum as ::core::cmp::PartialEq<
                            ::bevy_shape::ConicalFrustum,
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
            |_self: R<::bevy_shape::ConicalFrustum>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::ConicalFrustum::lateral_area(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the surface area of the side of the conical frustum,\n also known as the lateral area",
            &["_self"],
        )
        .register_documented(
            "slant_height",
            |_self: R<::bevy_shape::ConicalFrustum>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::ConicalFrustum::slant_height(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the slant height of the conical frustum, the length of the line segment\n connecting a point on the base to the closest point on the top",
            &["_self"],
        )
        .register_documented(
            "top_base",
            |_self: R<::bevy_shape::ConicalFrustum>| {
                let output: V<::bevy_shape::Circle> = {
                    {
                        let output: V<::bevy_shape::Circle> = ::bevy_shape::ConicalFrustum::top_base(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the top base of the conical frustum as a [`Circle`]",
            &["_self"],
        )
        .register_documented(
            "top_base_area",
            |_self: R<::bevy_shape::ConicalFrustum>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::ConicalFrustum::top_base_area(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Get the surface area of the top base of the conical frustum",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_shape::ConicalFrustum,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_torus_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Torus,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::Torus>| {
                let output: V<::bevy_shape::Torus> = {
                    {
                        let output: V<::bevy_shape::Torus> = <::bevy_shape::Torus as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Torus>, other: R<::bevy_shape::Torus>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Torus as ::core::cmp::PartialEq<
                            ::bevy_shape::Torus,
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
            |_self: R<::bevy_shape::Torus>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Torus::inner_radius(&_self)
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
                let output: V<::bevy_shape::Torus> = {
                    {
                        let output: V<::bevy_shape::Torus> = ::bevy_shape::Torus::new(
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
            |_self: R<::bevy_shape::Torus>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Torus::outer_radius(&_self)
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
            ::bevy_shape::Torus,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_triangle_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Triangle3d,
    >::new(world)
        .register_documented(
            "centroid",
            |_self: R<::bevy_shape::Triangle3d>| {
                let output: V<::bevy_math::Vec3> = {
                    {
                        let output: V<::bevy_math::Vec3> = ::bevy_shape::Triangle3d::centroid(
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
            |_self: R<::bevy_shape::Triangle3d>| {
                let output: V<::bevy_math::Vec3> = {
                    {
                        let output: V<::bevy_math::Vec3> = ::bevy_shape::Triangle3d::circumcenter(
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
            |_self: R<::bevy_shape::Triangle3d>| {
                let output: V<::bevy_shape::Triangle3d> = {
                    {
                        let output: V<::bevy_shape::Triangle3d> = <::bevy_shape::Triangle3d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Triangle3d>, other: R<::bevy_shape::Triangle3d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Triangle3d as ::core::cmp::PartialEq<
                            ::bevy_shape::Triangle3d,
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
            |_self: R<::bevy_shape::Triangle3d>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_shape::Triangle3d::is_acute(&_self)
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
            |_self: R<::bevy_shape::Triangle3d>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_shape::Triangle3d::is_degenerate(
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
            |_self: R<::bevy_shape::Triangle3d>| {
                let output: bool = {
                    {
                        let output: bool = ::bevy_shape::Triangle3d::is_obtuse(&_self)
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
            |a: V<::bevy_math::Vec3>, b: V<::bevy_math::Vec3>, c: V<::bevy_math::Vec3>| {
                let output: V<::bevy_shape::Triangle3d> = {
                    {
                        let output: V<::bevy_shape::Triangle3d> = ::bevy_shape::Triangle3d::new(
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
            |mut _self: M<::bevy_shape::Triangle3d>| {
                let output: () = {
                    {
                        let output: () = ::bevy_shape::Triangle3d::reverse(&mut _self)
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
            |_self: V<::bevy_shape::Triangle3d>| {
                let output: V<::bevy_shape::Triangle3d> = {
                    {
                        let output: V<::bevy_shape::Triangle3d> = ::bevy_shape::Triangle3d::reversed(
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
            ::bevy_shape::Triangle3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_tetrahedron_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Tetrahedron,
    >::new(world)
        .register_documented(
            "centroid",
            |_self: R<::bevy_shape::Tetrahedron>| {
                let output: V<::bevy_math::Vec3> = {
                    {
                        let output: V<::bevy_math::Vec3> = ::bevy_shape::Tetrahedron::centroid(
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
            |_self: R<::bevy_shape::Tetrahedron>| {
                let output: V<::bevy_shape::Tetrahedron> = {
                    {
                        let output: V<::bevy_shape::Tetrahedron> = <::bevy_shape::Tetrahedron as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Tetrahedron>, other: R<::bevy_shape::Tetrahedron>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Tetrahedron as ::core::cmp::PartialEq<
                            ::bevy_shape::Tetrahedron,
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
                a: V<::bevy_math::Vec3>,
                b: V<::bevy_math::Vec3>,
                c: V<::bevy_math::Vec3>,
                d: V<::bevy_math::Vec3>|
            {
                let output: V<::bevy_shape::Tetrahedron> = {
                    {
                        let output: V<::bevy_shape::Tetrahedron> = ::bevy_shape::Tetrahedron::new(
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
            |_self: R<::bevy_shape::Tetrahedron>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::Tetrahedron::signed_volume(
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
            ::bevy_shape::Tetrahedron,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_half_space_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::HalfSpace,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::HalfSpace>| {
                let output: V<::bevy_shape::HalfSpace> = {
                    {
                        let output: V<::bevy_shape::HalfSpace> = <::bevy_shape::HalfSpace as ::core::clone::Clone>::clone(
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
            "d",
            |_self: R<::bevy_shape::HalfSpace>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::HalfSpace::d(&_self).into();
                        output
                    }
                };
                output
            },
            " Returns the signed distance from the bisecting plane to the origin along\n the plane's unit normal vector.",
            &["_self"],
        )
        .register_documented(
            "eq",
            |_self: R<::bevy_shape::HalfSpace>, other: R<::bevy_shape::HalfSpace>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::HalfSpace as ::core::cmp::PartialEq<
                            ::bevy_shape::HalfSpace,
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
            |normal_d: V<::bevy_math::Vec4>| {
                let output: V<::bevy_shape::HalfSpace> = {
                    {
                        let output: V<::bevy_shape::HalfSpace> = ::bevy_shape::HalfSpace::new(
                                normal_d.into_inner(),
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Constructs a `HalfSpace` from a 4D vector whose first 3 components\n represent the bisecting plane's unit normal, and the last component is\n the signed distance along the normal from the plane to the origin.\n The constructor ensures the normal vector is normalized and the distance is appropriately scaled.",
            &["normal_d"],
        )
        .register_documented(
            "normal",
            |_self: R<::bevy_shape::HalfSpace>| {
                let output: V<::bevy_math::Vec3A> = {
                    {
                        let output: V<::bevy_math::Vec3A> = ::bevy_shape::HalfSpace::normal(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the unit normal vector of the bisecting plane that characterizes the `HalfSpace`.",
            &["_self"],
        )
        .register_documented(
            "normal_d",
            |_self: R<::bevy_shape::HalfSpace>| {
                let output: V<::bevy_math::Vec4> = {
                    {
                        let output: V<::bevy_math::Vec4> = ::bevy_shape::HalfSpace::normal_d(
                                &_self,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns the bisecting plane's unit normal vector and the signed distance\n from the plane to the origin.",
            &["_self"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_shape::HalfSpace,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_view_frustum_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::ViewFrustum,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::ViewFrustum>| {
                let output: V<::bevy_shape::ViewFrustum> = {
                    {
                        let output: V<::bevy_shape::ViewFrustum> = <::bevy_shape::ViewFrustum as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::ViewFrustum>, other: R<::bevy_shape::ViewFrustum>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::ViewFrustum as ::core::cmp::PartialEq<
                            ::bevy_shape::ViewFrustum,
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
            "from_clip_from_world",
            |clip_from_world: R<::bevy_math::Mat4>| {
                let output: V<::bevy_shape::ViewFrustum> = {
                    {
                        let output: V<::bevy_shape::ViewFrustum> = ::bevy_shape::ViewFrustum::from_clip_from_world(
                                &clip_from_world,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns a view frustum derived from `clip_from_world`.",
            &["clip_from_world"],
        )
        .register_documented(
            "from_clip_from_world_custom_far",
            |
                clip_from_world: R<::bevy_math::Mat4>,
                view_translation: R<::bevy_math::Vec3>,
                view_backward: R<::bevy_math::Vec3>,
                far: f32|
            {
                let output: V<::bevy_shape::ViewFrustum> = {
                    {
                        let output: V<::bevy_shape::ViewFrustum> = ::bevy_shape::ViewFrustum::from_clip_from_world_custom_far(
                                &clip_from_world,
                                &view_translation,
                                &view_backward,
                                far,
                            )
                            .into();
                        output
                    }
                };
                output
            },
            " Returns a view frustum derived from `clip_from_world`,\n but with a custom far plane.",
            &["clip_from_world", "view_translation", "view_backward", "far"],
        );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_shape::ViewFrustum,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_aabb_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Aabb2d,
    >::new(world)
        .register_documented(
            "bounding_circle",
            |_self: R<::bevy_shape::Aabb2d>| {
                let output: V<::bevy_shape::BoundingCircle> = {
                    {
                        let output: V<::bevy_shape::BoundingCircle> = ::bevy_shape::Aabb2d::bounding_circle(
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
            |_self: R<::bevy_shape::Aabb2d>| {
                let output: V<::bevy_shape::Aabb2d> = {
                    {
                        let output: V<::bevy_shape::Aabb2d> = <::bevy_shape::Aabb2d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Aabb2d>, point: V<::bevy_math::Vec2>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::Aabb2d::closest_point(
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
            |_self: R<::bevy_shape::Aabb2d>, other: R<::bevy_shape::Aabb2d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Aabb2d as ::core::cmp::PartialEq<
                            ::bevy_shape::Aabb2d,
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
            |center: V<::bevy_math::Vec2>, half_size: V<::bevy_math::Vec2>| {
                let output: V<::bevy_shape::Aabb2d> = {
                    {
                        let output: V<::bevy_shape::Aabb2d> = ::bevy_shape::Aabb2d::new(
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
            ::bevy_shape::Aabb2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_bounding_circle_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::BoundingCircle,
    >::new(world)
        .register_documented(
            "aabb_2d",
            |_self: R<::bevy_shape::BoundingCircle>| {
                let output: V<::bevy_shape::Aabb2d> = {
                    {
                        let output: V<::bevy_shape::Aabb2d> = ::bevy_shape::BoundingCircle::aabb_2d(
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
            |_self: R<::bevy_shape::BoundingCircle>| {
                let output: V<::bevy_shape::BoundingCircle> = {
                    {
                        let output: V<::bevy_shape::BoundingCircle> = <::bevy_shape::BoundingCircle as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::BoundingCircle>, point: V<::bevy_math::Vec2>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::BoundingCircle::closest_point(
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
                _self: R<::bevy_shape::BoundingCircle>,
                other: R<::bevy_shape::BoundingCircle>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::BoundingCircle as ::core::cmp::PartialEq<
                            ::bevy_shape::BoundingCircle,
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
            |center: V<::bevy_math::Vec2>, radius: f32| {
                let output: V<::bevy_shape::BoundingCircle> = {
                    {
                        let output: V<::bevy_shape::BoundingCircle> = ::bevy_shape::BoundingCircle::new(
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
            |_self: R<::bevy_shape::BoundingCircle>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::BoundingCircle::radius(&_self)
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
            ::bevy_shape::BoundingCircle,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_aabb_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::Aabb3d,
    >::new(world)
        .register_documented(
            "bounding_sphere",
            |_self: R<::bevy_shape::Aabb3d>| {
                let output: V<::bevy_shape::BoundingSphere> = {
                    {
                        let output: V<::bevy_shape::BoundingSphere> = ::bevy_shape::Aabb3d::bounding_sphere(
                                &_self,
                            )
                            .into();
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
            |_self: R<::bevy_shape::Aabb3d>| {
                let output: V<::bevy_shape::Aabb3d> = {
                    {
                        let output: V<::bevy_shape::Aabb3d> = <::bevy_shape::Aabb3d as ::core::clone::Clone>::clone(
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
            |_self: R<::bevy_shape::Aabb3d>, other: R<::bevy_shape::Aabb3d>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::Aabb3d as ::core::cmp::PartialEq<
                            ::bevy_shape::Aabb3d,
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
            ::bevy_shape::Aabb3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_bounding_sphere_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::BoundingSphere,
    >::new(world)
        .register_documented(
            "aabb_3d",
            |_self: R<::bevy_shape::BoundingSphere>| {
                let output: V<::bevy_shape::Aabb3d> = {
                    {
                        let output: V<::bevy_shape::Aabb3d> = ::bevy_shape::BoundingSphere::aabb_3d(
                                &_self,
                            )
                            .into();
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
            |_self: R<::bevy_shape::BoundingSphere>| {
                let output: V<::bevy_shape::BoundingSphere> = {
                    {
                        let output: V<::bevy_shape::BoundingSphere> = <::bevy_shape::BoundingSphere as ::core::clone::Clone>::clone(
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
                _self: R<::bevy_shape::BoundingSphere>,
                other: R<::bevy_shape::BoundingSphere>|
            {
                let output: bool = {
                    {
                        let output: bool = <::bevy_shape::BoundingSphere as ::core::cmp::PartialEq<
                            ::bevy_shape::BoundingSphere,
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
            |_self: R<::bevy_shape::BoundingSphere>| {
                let output: f32 = {
                    {
                        let output: f32 = ::bevy_shape::BoundingSphere::radius(&_self)
                            .into();
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
            ::bevy_shape::BoundingSphere,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ray_cast_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::RayCast2d,
    >::new(world)
        .register_documented(
            "aabb_intersection_at",
            |_self: R<::bevy_shape::RayCast2d>, aabb: R<::bevy_shape::Aabb2d>| {
                let output: ::core::option::Option<f32> = {
                    {
                        let output: ::core::option::Option<f32> = ::bevy_shape::RayCast2d::aabb_intersection_at(
                                &_self,
                                &aabb,
                            )
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
            |_self: R<::bevy_shape::RayCast2d>, circle: R<::bevy_shape::BoundingCircle>| {
                let output: ::core::option::Option<f32> = {
                    {
                        let output: ::core::option::Option<f32> = ::bevy_shape::RayCast2d::circle_intersection_at(
                                &_self,
                                &circle,
                            )
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
            |_self: R<::bevy_shape::RayCast2d>| {
                let output: V<::bevy_shape::RayCast2d> = {
                    {
                        let output: V<::bevy_shape::RayCast2d> = <::bevy_shape::RayCast2d as ::core::clone::Clone>::clone(
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
            "direction_recip",
            |_self: R<::bevy_shape::RayCast2d>| {
                let output: V<::bevy_math::Vec2> = {
                    {
                        let output: V<::bevy_math::Vec2> = ::bevy_shape::RayCast2d::direction_recip(
                                &_self,
                            )
                            .into();
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
            |ray: V<::bevy_shape::Ray2d>, max: f32| {
                let output: V<::bevy_shape::RayCast2d> = {
                    {
                        let output: V<::bevy_shape::RayCast2d> = ::bevy_shape::RayCast2d::from_ray(
                                ray.into_inner(),
                                max,
                            )
                            .into();
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
            |origin: V<::bevy_math::Vec2>, direction: V<::bevy_math::Dir2>, max: f32| {
                let output: V<::bevy_shape::RayCast2d> = {
                    {
                        let output: V<::bevy_shape::RayCast2d> = ::bevy_shape::RayCast2d::new(
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
            ::bevy_shape::RayCast2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_aabb_cast_2_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::AabbCast2d,
    >::new(world)
        .register_documented(
            "aabb_collision_at",
            |_self: R<::bevy_shape::AabbCast2d>, aabb: V<::bevy_shape::Aabb2d>| {
                let output: ::core::option::Option<f32> = {
                    {
                        let output: ::core::option::Option<f32> = ::bevy_shape::AabbCast2d::aabb_collision_at(
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
            |_self: R<::bevy_shape::AabbCast2d>| {
                let output: V<::bevy_shape::AabbCast2d> = {
                    {
                        let output: V<::bevy_shape::AabbCast2d> = <::bevy_shape::AabbCast2d as ::core::clone::Clone>::clone(
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
            |aabb: V<::bevy_shape::Aabb2d>, ray: V<::bevy_shape::Ray2d>, max: f32| {
                let output: V<::bevy_shape::AabbCast2d> = {
                    {
                        let output: V<::bevy_shape::AabbCast2d> = ::bevy_shape::AabbCast2d::from_ray(
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
            |
                aabb: V<::bevy_shape::Aabb2d>,
                origin: V<::bevy_math::Vec2>,
                direction: V<::bevy_math::Dir2>,
                max: f32|
            {
                let output: V<::bevy_shape::AabbCast2d> = {
                    {
                        let output: V<::bevy_shape::AabbCast2d> = ::bevy_shape::AabbCast2d::new(
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
            ::bevy_shape::AabbCast2d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_bounding_circle_cast_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::BoundingCircleCast,
    >::new(world)
        .register_documented(
            "circle_collision_at",
            |
                _self: R<::bevy_shape::BoundingCircleCast>,
                circle: V<::bevy_shape::BoundingCircle>|
            {
                let output: ::core::option::Option<f32> = {
                    {
                        let output: ::core::option::Option<f32> = ::bevy_shape::BoundingCircleCast::circle_collision_at(
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
            |_self: R<::bevy_shape::BoundingCircleCast>| {
                let output: V<::bevy_shape::BoundingCircleCast> = {
                    {
                        let output: V<::bevy_shape::BoundingCircleCast> = <::bevy_shape::BoundingCircleCast as ::core::clone::Clone>::clone(
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
                circle: V<::bevy_shape::BoundingCircle>,
                ray: V<::bevy_shape::Ray2d>,
                max: f32|
            {
                let output: V<::bevy_shape::BoundingCircleCast> = {
                    {
                        let output: V<::bevy_shape::BoundingCircleCast> = ::bevy_shape::BoundingCircleCast::from_ray(
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
                circle: V<::bevy_shape::BoundingCircle>,
                origin: V<::bevy_math::Vec2>,
                direction: V<::bevy_math::Dir2>,
                max: f32|
            {
                let output: V<::bevy_shape::BoundingCircleCast> = {
                    {
                        let output: V<::bevy_shape::BoundingCircleCast> = ::bevy_shape::BoundingCircleCast::new(
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
            ::bevy_shape::BoundingCircleCast,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_ray_cast_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::RayCast3d,
    >::new(world)
        .register_documented(
            "aabb_intersection_at",
            |_self: R<::bevy_shape::RayCast3d>, aabb: R<::bevy_shape::Aabb3d>| {
                let output: ::core::option::Option<f32> = {
                    {
                        let output: ::core::option::Option<f32> = ::bevy_shape::RayCast3d::aabb_intersection_at(
                                &_self,
                                &aabb,
                            )
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
            |_self: R<::bevy_shape::RayCast3d>| {
                let output: V<::bevy_shape::RayCast3d> = {
                    {
                        let output: V<::bevy_shape::RayCast3d> = <::bevy_shape::RayCast3d as ::core::clone::Clone>::clone(
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
            "direction_recip",
            |_self: R<::bevy_shape::RayCast3d>| {
                let output: V<::bevy_math::Vec3A> = {
                    {
                        let output: V<::bevy_math::Vec3A> = ::bevy_shape::RayCast3d::direction_recip(
                                &_self,
                            )
                            .into();
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
            |ray: V<::bevy_shape::Ray3d>, max: f32| {
                let output: V<::bevy_shape::RayCast3d> = {
                    {
                        let output: V<::bevy_shape::RayCast3d> = ::bevy_shape::RayCast3d::from_ray(
                                ray.into_inner(),
                                max,
                            )
                            .into();
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
            |_self: R<::bevy_shape::RayCast3d>, sphere: R<::bevy_shape::BoundingSphere>| {
                let output: ::core::option::Option<f32> = {
                    {
                        let output: ::core::option::Option<f32> = ::bevy_shape::RayCast3d::sphere_intersection_at(
                                &_self,
                                &sphere,
                            )
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
            ::bevy_shape::RayCast3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_aabb_cast_3_d_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::AabbCast3d,
    >::new(world)
        .register_documented(
            "aabb_collision_at",
            |_self: R<::bevy_shape::AabbCast3d>, aabb: V<::bevy_shape::Aabb3d>| {
                let output: ::core::option::Option<f32> = {
                    {
                        let output: ::core::option::Option<f32> = ::bevy_shape::AabbCast3d::aabb_collision_at(
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
            |_self: R<::bevy_shape::AabbCast3d>| {
                let output: V<::bevy_shape::AabbCast3d> = {
                    {
                        let output: V<::bevy_shape::AabbCast3d> = <::bevy_shape::AabbCast3d as ::core::clone::Clone>::clone(
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
            |aabb: V<::bevy_shape::Aabb3d>, ray: V<::bevy_shape::Ray3d>, max: f32| {
                let output: V<::bevy_shape::AabbCast3d> = {
                    {
                        let output: V<::bevy_shape::AabbCast3d> = ::bevy_shape::AabbCast3d::from_ray(
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
            ::bevy_shape::AabbCast3d,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
pub(crate) fn register_bounding_sphere_cast_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_shape::BoundingSphereCast,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_shape::BoundingSphereCast>| {
                let output: V<::bevy_shape::BoundingSphereCast> = {
                    {
                        let output: V<::bevy_shape::BoundingSphereCast> = <::bevy_shape::BoundingSphereCast as ::core::clone::Clone>::clone(
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
                sphere: V<::bevy_shape::BoundingSphere>,
                ray: V<::bevy_shape::Ray3d>,
                max: f32|
            {
                let output: V<::bevy_shape::BoundingSphereCast> = {
                    {
                        let output: V<::bevy_shape::BoundingSphereCast> = ::bevy_shape::BoundingSphereCast::from_ray(
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
                _self: R<::bevy_shape::BoundingSphereCast>,
                sphere: V<::bevy_shape::BoundingSphere>|
            {
                let output: ::core::option::Option<f32> = {
                    {
                        let output: ::core::option::Option<f32> = ::bevy_shape::BoundingSphereCast::sphere_collision_at(
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
            ::bevy_shape::BoundingSphereCast,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyShapeScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_ray_2_d_functions(&mut world);
        register_ray_3_d_functions(&mut world);
        register_circle_functions(&mut world);
        register_arc_2_d_functions(&mut world);
        register_circular_sector_functions(&mut world);
        register_circular_segment_functions(&mut world);
        register_ellipse_functions(&mut world);
        register_annulus_functions(&mut world);
        register_rhombus_functions(&mut world);
        register_plane_2_d_functions(&mut world);
        register_line_2_d_functions(&mut world);
        register_segment_2_d_functions(&mut world);
        register_polyline_2_d_functions(&mut world);
        register_triangle_2_d_functions(&mut world);
        register_rectangle_functions(&mut world);
        register_polygon_functions(&mut world);
        register_convex_polygon_functions(&mut world);
        register_regular_polygon_functions(&mut world);
        register_capsule_2_d_functions(&mut world);
        register_sphere_functions(&mut world);
        register_plane_3_d_functions(&mut world);
        register_infinite_plane_3_d_functions(&mut world);
        register_line_3_d_functions(&mut world);
        register_segment_3_d_functions(&mut world);
        register_polyline_3_d_functions(&mut world);
        register_cuboid_functions(&mut world);
        register_cylinder_functions(&mut world);
        register_capsule_3_d_functions(&mut world);
        register_cone_functions(&mut world);
        register_conical_frustum_functions(&mut world);
        register_torus_functions(&mut world);
        register_triangle_3_d_functions(&mut world);
        register_tetrahedron_functions(&mut world);
        register_half_space_functions(&mut world);
        register_view_frustum_functions(&mut world);
        register_aabb_2_d_functions(&mut world);
        register_bounding_circle_functions(&mut world);
        register_aabb_3_d_functions(&mut world);
        register_bounding_sphere_functions(&mut world);
        register_ray_cast_2_d_functions(&mut world);
        register_aabb_cast_2_d_functions(&mut world);
        register_bounding_circle_cast_functions(&mut world);
        register_ray_cast_3_d_functions(&mut world);
        register_aabb_cast_3_d_functions(&mut world);
        register_bounding_sphere_cast_functions(&mut world);
    }
}
