//! World transforms: [`GlobalTransform`](bevy_transform::components::GlobalTransform).

// `script_bindings` expects these in scope
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_math::{Mat4, Quat, Vec3};
use bevy_mod_scripting_bindings::function::from::{R, Union, V};
use bevy_mod_scripting_derive::script_bindings;
use bevy_transform::components::{GlobalTransform, Transform};

#[script_bindings(
    remote,
    name = "global_transform_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    core
)]
impl GlobalTransform {
    /// A transform at the position `x`, `y`, `z`, with no rotation and a scale of `1`.
    fn from_xyz(x: f32, y: f32, z: f32) -> V<GlobalTransform> {
        V(GlobalTransform::from_xyz(x, y, z))
    }

    /// The identity transform, which maps every point to itself.
    fn identity() -> V<GlobalTransform> {
        V(GlobalTransform::IDENTITY)
    }

    /// The position in world space.
    fn translation(_self: R<GlobalTransform>) -> V<Vec3> {
        V(_self.translation())
    }

    /// The rotation in world space.
    fn rotation(_self: R<GlobalTransform>) -> V<Quat> {
        V(_self.rotation())
    }

    /// The scale in world space.
    fn scale(_self: R<GlobalTransform>) -> V<Vec3> {
        V(_self.scale())
    }

    /// The direction the transform faces, `-z` in local space.
    fn forward(_self: R<GlobalTransform>) -> V<Vec3> {
        V(_self.forward().as_vec3())
    }

    /// The direction opposite to [`forward`](Self::forward), `+z` in local space.
    fn back(_self: R<GlobalTransform>) -> V<Vec3> {
        V(_self.back().as_vec3())
    }

    /// The direction to the local left, `-x` in local space.
    fn left(_self: R<GlobalTransform>) -> V<Vec3> {
        V(_self.left().as_vec3())
    }

    /// The direction to the local right, `+x` in local space.
    fn right(_self: R<GlobalTransform>) -> V<Vec3> {
        V(_self.right().as_vec3())
    }

    /// The local up direction, `+y`.
    fn up(_self: R<GlobalTransform>) -> V<Vec3> {
        V(_self.up().as_vec3())
    }

    /// The local down direction, `-y`.
    fn down(_self: R<GlobalTransform>) -> V<Vec3> {
        V(_self.down().as_vec3())
    }

    /// Applies the transform to a point in local space, giving its world space position.
    fn transform_point(_self: R<GlobalTransform>, point: R<Vec3>) -> V<Vec3> {
        V(_self.transform_point(*point))
    }

    /// The transform as a [`Transform`](bevy_transform::components::Transform), with the same position, rotation and scale.
    fn compute_transform(_self: R<GlobalTransform>) -> V<Transform> {
        V(_self.compute_transform())
    }

    /// The matrix equivalent to this transform.
    fn to_matrix(_self: R<GlobalTransform>) -> V<Mat4> {
        V(_self.to_matrix())
    }

    /// The local [`Transform`](bevy_transform::components::Transform) that gives this world transform when parented to `parent`.
    fn reparented_to(_self: R<GlobalTransform>, parent: R<GlobalTransform>) -> V<Transform> {
        V(_self.reparented_to(&parent))
    }

    /// Composes two transforms when given a global transform, or applies a local transform to this one.
    fn mul(
        _self: R<GlobalTransform>,
        rhs: Union<R<GlobalTransform>, R<Transform>>,
    ) -> V<GlobalTransform> {
        V(match rhs.into_left() {
            Ok(g) => *_self * *g,
            Err(t) => *_self * *t,
        })
    }

    /// Whether both transforms have exactly the same components.
    fn eq(_self: R<GlobalTransform>, other: R<GlobalTransform>) -> bool {
        *_self == *other
    }
}

/// Registers the functions of [`GlobalTransform`].
pub(crate) fn register(world: &mut World) {
    register_global_transform_functions(world);
}
