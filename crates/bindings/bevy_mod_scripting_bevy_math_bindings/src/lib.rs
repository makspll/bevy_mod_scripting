//! Script bindings for bevy math: vectors, quaternions, matrices, directions, rectangles, rays, shapes and easing.

use bevy_app::{App, Plugin};
use bevy_mod_scripting_bindings::ReflectScriptValueType;

mod dir;
mod ease;
mod ivec;
mod mat;
mod quat;
mod ray;
mod rect;
mod rot2;
mod shapes;
mod vec;

/// Registers the math script bindings.
#[derive(Default)]
pub struct BevyMathBindingsPlugin;

impl Plugin for BevyMathBindingsPlugin {
    fn finish(&self, app: &mut App) {
        let registry = app
            .world()
            .resource::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        let mut registry = registry.write();
        bevy_mod_scripting_bindings::set_crate_script_namespaces(
            &mut registry,
            &["glam", "bevy_math", "bevy_curve", "bevy_shape"],
            "math",
        );
        register_value_types(&mut registry);
    }

    fn build(&self, app: &mut App) {
        let world = app.world_mut();
        vec::register(world);
        ivec::register(world);
        quat::register(world);
        mat::register(world);
        dir::register(world);
        rot2::register(world);
        rect::register(world);
        ray::register(world);
        shapes::register(world);
        ease::register(world);
    }
}

/// Vectors, quaternions and matrices are copied into scripts, see [`ReflectScriptValueType`].
fn register_value_types(registry: &mut bevy_reflect::TypeRegistry) {
    use bevy_math::*;
    macro_rules! value_types {
        ($($ty:ty),*) => {
            $(if let Some(registration) = registry.get_mut(std::any::TypeId::of::<$ty>()) {
                registration.insert(<ReflectScriptValueType as bevy_reflect::CreateTypeData<$ty>>::create_type_data(()));
            })*
        };
    }
    value_types!(
        Vec2, Vec3, Vec3A, Vec4, DVec2, DVec3, DVec4, IVec2, IVec3, IVec4, UVec2, UVec3, UVec4,
        Quat, DQuat, Mat2, Mat3, Mat3A, Mat4, DMat2, DMat3, DMat4
    );
}
