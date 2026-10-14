//! Rays and their intersections with planes: [`Ray2d`] and [`Ray3d`].

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_math::{Dir2, Dir3, Vec2, Vec3};
use bevy_mod_scripting_bindings::{
    InteropError,
    function::from::{R, V},
};
use bevy_mod_scripting_derive::script_bindings;
use bevy_shape::{InfinitePlane3d, Plane2d, Ray2d, Ray3d};

#[script_bindings(
    remote,
    name = "ray2d_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Ray2d {
    /// A ray from `origin` in the direction of `direction`. Errors if the direction is zero or not finite.
    fn new(origin: R<Vec2>, direction: R<Vec2>) -> Result<V<Ray2d>, InteropError> {
        let direction = Dir2::new(*direction)
            .map_err(|_| InteropError::str("ray direction must not be zero"))?;
        Ok(V(Ray2d::new(*origin, direction)))
    }

    /// The point at `distance` along the ray from its origin.
    fn get_point(_self: R<Ray2d>, distance: f32) -> V<Vec2> {
        V(_self.get_point(distance))
    }

    /// The distance along the ray to the line through `plane_origin` with normal `plane_normal`, or nil if the ray never reaches it. Errors if the normal is zero or not finite.
    fn intersect_plane(
        _self: R<Ray2d>,
        plane_origin: R<Vec2>,
        plane_normal: R<Vec2>,
    ) -> Result<Option<f32>, InteropError> {
        let normal = Dir2::new(*plane_normal)
            .map_err(|_| InteropError::str("plane normal must not be zero"))?;
        Ok(_self.intersect_plane(*plane_origin, Plane2d { normal }))
    }
}

#[script_bindings(
    remote,
    name = "ray3d_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Ray3d {
    /// A ray from `origin` in the direction of `direction`. Errors if the direction is zero or not finite.
    fn new(origin: R<Vec3>, direction: R<Vec3>) -> Result<V<Ray3d>, InteropError> {
        let direction = Dir3::new(*direction)
            .map_err(|_| InteropError::str("ray direction must not be zero"))?;
        Ok(V(Ray3d::new(*origin, direction)))
    }

    /// The point at `distance` along the ray from its origin.
    fn get_point(_self: R<Ray3d>, distance: f32) -> V<Vec3> {
        V(_self.get_point(distance))
    }

    /// The distance along the ray to the plane through `plane_origin` with normal `plane_normal`, or nil if the ray never reaches it. Errors if the normal is zero or not finite.
    fn intersect_plane(
        _self: R<Ray3d>,
        plane_origin: R<Vec3>,
        plane_normal: R<Vec3>,
    ) -> Result<Option<f32>, InteropError> {
        let normal = Dir3::new(*plane_normal)
            .map_err(|_| InteropError::str("plane normal must not be zero"))?;
        Ok(_self.intersect_plane(*plane_origin, InfinitePlane3d::new(normal)))
    }
}

/// Registers the functions of rays.
pub(crate) fn register(world: &mut World) {
    register_ray2d_functions(world);
    register_ray3d_functions(world);
}
