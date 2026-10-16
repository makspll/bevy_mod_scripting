//! Script bindings for bevy rendering: meshes, materials and images.

use bevy_app::{App, Plugin};
use bevy_color::Color;
use bevy_image::Image;
use bevy_math::{UVec2, Vec2};
use bevy_mesh::Mesh;
use bevy_pbr::StandardMaterial;
use bevy_reflect::PartialReflect;
// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::{
    InteropError, ReflectReference,
    function::{
        from::{R, V},
        script_function::FunctionCallContext,
    },
};
use bevy_mod_scripting_derive::script_bindings;
use bevy_shape::{
    Annulus, Capsule2d, Capsule3d, Circle, Cone, Cuboid, Cylinder, Plane3d, Rectangle,
    RegularPolygon, Sphere, Torus, Triangle2d,
};

/// Registers the render script bindings.
#[derive(Default)]
pub struct BevyRenderBindingsPlugin;

impl Plugin for BevyRenderBindingsPlugin {
    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        register_mesh_functions(world);
        register_standard_material_functions(world);
        register_image_functions(world);
    }
}

/// The mesh of a reflected shape, or `None` if the value is not one of the shapes that have a mesh.
fn mesh_from_shape(value: &dyn PartialReflect) -> Option<Mesh> {
    // one `downcast` per shape type, the first match wins
    macro_rules! mesh_of {
        ($($shape:ty),* $(,)?) => {
            None $(.or_else(|| value.try_downcast_ref::<$shape>().map(|shape| Mesh::from(*shape))))*
        };
    }
    mesh_of!(
        Cuboid,
        Sphere,
        Plane3d,
        Cylinder,
        Capsule3d,
        Cone,
        Torus,
        Circle,
        Rectangle,
        RegularPolygon,
        Capsule2d,
        Annulus,
        Triangle2d,
    )
}

#[script_bindings(
    remote,
    name = "mesh_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Mesh {
    /// A mesh for a shape such as a cuboid or a sphere. Errors if the value is not a shape.
    fn from_shape(
        ctxt: FunctionCallContext,
        shape: ReflectReference,
    ) -> Result<V<Mesh>, InteropError> {
        shape
            .with_reflect(ctxt.world()?, mesh_from_shape)?
            .map(V)
            .ok_or_else(|| InteropError::str("not a shape that can be turned into a mesh"))
    }

    /// The number of vertices in the mesh.
    fn count_vertices(_self: R<Mesh>) -> usize {
        _self.count_vertices()
    }
}

#[script_bindings(
    remote,
    name = "standard_material_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl StandardMaterial {
    /// A material with the given color as its base color and default settings for everything else.
    fn from_color(color: R<Color>) -> V<StandardMaterial> {
        V(StandardMaterial::from_color(*color))
    }
}

#[script_bindings(
    remote,
    name = "image_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl Image {
    /// The width of the image in pixels.
    fn width(_self: R<Image>) -> u32 {
        _self.width()
    }

    /// The height of the image in pixels.
    fn height(_self: R<Image>) -> u32 {
        _self.height()
    }

    /// The size of the image in pixels.
    fn size(_self: R<Image>) -> V<UVec2> {
        V(_self.size())
    }

    /// The size of the image in pixels, as decimal numbers.
    fn size_f32(_self: R<Image>) -> V<Vec2> {
        V(_self.size_f32())
    }
}
