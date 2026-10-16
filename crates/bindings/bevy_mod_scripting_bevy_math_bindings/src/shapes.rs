//! Geometric shapes from `bevy_shape`: circles, rectangles, spheres, planes and more.

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_math::{Dir3, Vec2, Vec3};
use bevy_mod_scripting_bindings::{
    InteropError,
    function::from::{R, V},
};
use bevy_mod_scripting_derive::script_bindings;
use bevy_shape::{
    Annulus, Capsule3d, Circle, Cuboid, Cylinder, Plane3d, Rectangle, RegularPolygon, Sphere, Torus,
};

#[script_bindings(
    remote,
    name = "circle_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Circle {
    /// A circle with the given radius.
    fn new(radius: f32) -> V<Circle> {
        V(Circle::new(radius))
    }
}

#[script_bindings(
    remote,
    name = "sphere_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Sphere {
    /// A sphere with the given radius.
    fn new(radius: f32) -> V<Sphere> {
        V(Sphere::new(radius))
    }
}

#[script_bindings(
    remote,
    name = "rectangle_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Rectangle {
    /// A rectangle with the given width and height.
    fn new(width: f32, height: f32) -> V<Rectangle> {
        V(Rectangle::new(width, height))
    }

    /// A rectangle with the given size as a [`Vec2`](bevy_math::Vec2).
    fn from_size(size: R<Vec2>) -> V<Rectangle> {
        V(Rectangle::from_size(*size))
    }
}

#[script_bindings(
    remote,
    name = "cuboid_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Cuboid {
    /// A box with the given length along the x, y and z axes.
    fn new(x_length: f32, y_length: f32, z_length: f32) -> V<Cuboid> {
        V(Cuboid::new(x_length, y_length, z_length))
    }

    /// A box with the given size as a [`Vec3`](bevy_math::Vec3).
    fn from_size(size: R<Vec3>) -> V<Cuboid> {
        V(Cuboid::from_size(*size))
    }
}

#[script_bindings(
    remote,
    name = "capsule3d_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Capsule3d {
    /// A capsule with the given radius and length of its middle section.
    fn new(radius: f32, length: f32) -> V<Capsule3d> {
        V(Capsule3d::new(radius, length))
    }
}

#[script_bindings(
    remote,
    name = "cylinder_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Cylinder {
    /// A cylinder with the given radius and height.
    fn new(radius: f32, height: f32) -> V<Cylinder> {
        V(Cylinder::new(radius, height))
    }
}

#[script_bindings(
    remote,
    name = "plane3d_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Plane3d {
    /// A plane facing `normal` with the given half size. Errors if the normal is zero or not finite.
    fn new(normal: R<Vec3>, half_size: R<Vec2>) -> Result<V<Plane3d>, InteropError> {
        let normal =
            Dir3::new(*normal).map_err(|_| InteropError::str("plane normal must not be zero"))?;
        Ok(V(Plane3d {
            normal,
            half_size: *half_size,
        }))
    }
}

#[script_bindings(
    remote,
    name = "annulus_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Annulus {
    /// A ring with the given inner and outer radii.
    fn new(inner_radius: f32, outer_radius: f32) -> V<Annulus> {
        V(Annulus::new(inner_radius, outer_radius))
    }
}

#[script_bindings(
    remote,
    name = "regular_polygon_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl RegularPolygon {
    /// A regular polygon with the given number of sides, all corners at `circumradius` from the center.
    fn new(circumradius: f32, sides: u32) -> V<RegularPolygon> {
        V(RegularPolygon::new(circumradius, sides))
    }
}

#[script_bindings(
    remote,
    name = "torus_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Torus {
    /// A donut shape with the given inner and outer radii of its tube.
    fn new(inner_radius: f32, outer_radius: f32) -> V<Torus> {
        V(Torus::new(inner_radius, outer_radius))
    }
}

/// Registers the functions of shapes.
pub(crate) fn register(world: &mut World) {
    register_circle_functions(world);
    register_sphere_functions(world);
    register_rectangle_functions(world);
    register_cuboid_functions(world);
    register_capsule3d_functions(world);
    register_cylinder_functions(world);
    register_plane3d_functions(world);
    register_annulus_functions(world);
    register_regular_polygon_functions(world);
    register_torus_functions(world);
}
