//! Script bindings for bevy rendering: meshes, materials and images.

use bevy_app::{App, Plugin};
use bevy_color::Color;
use bevy_image::Image;
use bevy_math::{UVec2, Vec2};
use bevy_mesh::Mesh;
use bevy_pbr::StandardMaterial;
use bevy_reflect::{PartialReflect, TypeRegistry, Typed};
// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_mod_scripting_bindings::{
    GetTypeDependencies, InteropError, ReflectReference, ScriptValue, ThroughTypeInfo,
    TypedThrough, WorldGuard,
    function::{
        arg_meta::ArgMeta,
        from::{FromScript, R, V},
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
    fn finish(&self, app: &mut App) {
        let registry = app
            .world()
            .resource::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        bevy_mod_scripting_bindings::set_crate_script_namespaces(
            &mut registry.write(),
            &[
                "bevy_render",
                "bevy_light",
                "bevy_mesh",
                "bevy_pbr",
                "bevy_image",
                "bevy_core_pipeline",
                "bevy_post_process",
                "bevy_anti_alias",
                "bevy_material",
            ],
            "render",
        );
    }

    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        register_mesh_functions(world);
        register_standard_material_functions(world);
        register_image_functions(world);
    }
}

/// Calls the given macro with the shapes that have a mesh, so [`mesh_from_shape`] and [`MeshShape`] share one list.
macro_rules! with_mesh_shapes {
    ($callback:ident) => {
        $callback!(
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
    };
}

/// The mesh of a reflected shape, or `None` if the value is not one of the shapes that have a mesh.
fn mesh_from_shape(value: &dyn PartialReflect) -> Option<Mesh> {
    // one `downcast` per shape type, the first match wins
    macro_rules! mesh_of {
        ($($shape:ty),* $(,)?) => {
            None $(.or_else(|| value.try_downcast_ref::<$shape>().map(|shape| Mesh::from(*shape))))*
        };
    }
    with_mesh_shapes!(mesh_of)
}

/// One of the shapes that have a mesh, passed as a reference: only its documentation differs from [`ReflectReference`],
/// which is the union of the shapes, i.e. `math.Cuboid | math.Sphere | ...` in Lua.
struct MeshShape(ReflectReference);

impl FromScript for MeshShape {
    type This<'w> = Self;

    fn from_script(value: ScriptValue, world: WorldGuard) -> Result<Self, InteropError> {
        ReflectReference::from_script(value, world).map(MeshShape)
    }
}

impl ArgMeta for MeshShape {}

impl GetTypeDependencies for MeshShape {
    type Underlying = ReflectReference;

    fn register_type_dependencies(registry: &mut TypeRegistry) {
        macro_rules! register {
            ($($shape:ty),* $(,)?) => {
                { $(registry.register::<$shape>();)* }
            };
        }
        with_mesh_shapes!(register)
    }
}

impl TypedThrough for MeshShape {
    fn through_type_info() -> ThroughTypeInfo {
        macro_rules! union {
            ($($shape:ty),* $(,)?) => {
                ThroughTypeInfo::Union(vec![$(ThroughTypeInfo::Ref(<$shape as Typed>::type_info())),*])
            };
        }
        with_mesh_shapes!(union)
    }
}

#[script_bindings(
    remote,
    name = "mesh_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "render",
    core
)]
impl Mesh {
    /// A mesh for a shape such as a cuboid or a sphere. Errors if the value is not a shape.
    fn from_shape(ctxt: FunctionCallContext, shape: MeshShape) -> Result<V<Mesh>, InteropError> {
        shape
            .0
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
    namespace = "render",
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
    namespace = "render",
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
