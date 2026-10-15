//! Script bindings for bevy gizmos: immediate mode debug drawing.

// `script_bindings` expects these in scope
use bevy_app::{App, Plugin};
use bevy_color::Color;
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_gizmos::prelude::{GizmoConfigStore, gizmo};
use bevy_math::{Isometry2d, Isometry3d, Vec2, Vec3};
use bevy_mod_scripting_bindings::{
    InteropError, WorldExtensions,
    function::{
        from::{R, V},
        script_function::FunctionCallContext,
    },
};
use bevy_mod_scripting_derive::script_bindings;
use bevy_reflect::Reflect;
use bevy_transform::components::Transform;

/// Immediate mode debug drawing: lines and shapes drawn this frame.
///
/// Every function draws straight away, the shapes are shown for the current frame only.
/// Needs the gizmo plugin, without it the functions fail with an error.
///
/// ```multi-lang-example,lua
/// local Color = import_namespace("color").Color
/// local Gizmos = import_namespace("gizmos").Gizmos
/// local Vec3 = import_namespace("math").Vec3
/// Gizmos.line(Vec3.zero(), Vec3.new(1, 0, 0), Color.white())
/// ```
///
/// ```multi-lang-example,rhai
/// let Color = import_namespace.call("color").Color;
/// let Gizmos = import_namespace.call("gizmos").Gizmos;
/// let Vec3 = import_namespace.call("math").Vec3;
/// Gizmos.line.call(Vec3.zero.call(), Vec3.new_.call(1.0, 0.0, 0.0), Color.white.call());
/// ```
#[derive(Reflect, Default)]
pub struct Gizmos;

/// Fails if the gizmo plugin is missing: the global gizmo buffer is only drawn by it, so drawing without it would only grow the buffer.
fn ensure_gizmos_available(ctxt: &FunctionCallContext) -> Result<(), InteropError> {
    match ctxt.world()?.with_resource(|_: &GizmoConfigStore| ()) {
        Ok(()) => Ok(()),
        Err(
            InteropError::MissingResource { .. }
            | InteropError::UnregisteredComponentOrResourceType { .. },
        ) => Err(InteropError::str(
            "gizmos are not available, add the GizmoPlugin",
        )),
        Err(error) => Err(error),
    }
}

#[script_bindings(
    remote,
    name = "gizmos_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "gizmos",
    core
)]
impl Gizmos {
    /// Draws a line between two points.
    fn line(
        ctxt: FunctionCallContext,
        start: R<Vec3>,
        end: R<Vec3>,
        color: R<Color>,
    ) -> Result<(), InteropError> {
        ensure_gizmos_available(&ctxt)?;
        gizmo().line(*start, *end, *color);
        Ok(())
    }

    /// Draws a line between two points in 2D.
    fn line_2d(
        ctxt: FunctionCallContext,
        start: R<Vec2>,
        end: R<Vec2>,
        color: R<Color>,
    ) -> Result<(), InteropError> {
        ensure_gizmos_available(&ctxt)?;
        gizmo().line_2d(*start, *end, *color);
        Ok(())
    }

    /// Draws a line from the origin along the vector.
    fn ray(
        ctxt: FunctionCallContext,
        origin: R<Vec3>,
        vector: R<Vec3>,
        color: R<Color>,
    ) -> Result<(), InteropError> {
        ensure_gizmos_available(&ctxt)?;
        gizmo().ray(*origin, *vector, *color);
        Ok(())
    }

    /// Draws a line from the origin along the vector in 2D.
    fn ray_2d(
        ctxt: FunctionCallContext,
        origin: R<Vec2>,
        vector: R<Vec2>,
        color: R<Color>,
    ) -> Result<(), InteropError> {
        ensure_gizmos_available(&ctxt)?;
        gizmo().ray_2d(*origin, *vector, *color);
        Ok(())
    }

    /// Draws an arrow from the start to the end point.
    fn arrow(
        ctxt: FunctionCallContext,
        start: R<Vec3>,
        end: R<Vec3>,
        color: R<Color>,
    ) -> Result<(), InteropError> {
        ensure_gizmos_available(&ctxt)?;
        gizmo().arrow(*start, *end, *color);
        Ok(())
    }

    /// Draws an arrow from the start to the end point in 2D.
    fn arrow_2d(
        ctxt: FunctionCallContext,
        start: R<Vec2>,
        end: R<Vec2>,
        color: R<Color>,
    ) -> Result<(), InteropError> {
        ensure_gizmos_available(&ctxt)?;
        gizmo().arrow_2d(*start, *end, *color);
        Ok(())
    }

    /// Draws a circle around the position, lying in the XY plane so it faces +Z.
    fn circle(
        ctxt: FunctionCallContext,
        position: R<Vec3>,
        radius: f32,
        color: R<Color>,
    ) -> Result<(), InteropError> {
        ensure_gizmos_available(&ctxt)?;
        gizmo().circle(Isometry3d::from_translation(*position), radius, *color);
        Ok(())
    }

    /// Draws a circle around the position in 2D.
    fn circle_2d(
        ctxt: FunctionCallContext,
        position: R<Vec2>,
        radius: f32,
        color: R<Color>,
    ) -> Result<(), InteropError> {
        ensure_gizmos_available(&ctxt)?;
        gizmo().circle_2d(Isometry2d::from_translation(*position), radius, *color);
        Ok(())
    }

    /// Draws a wireframe sphere around the position.
    fn sphere(
        ctxt: FunctionCallContext,
        position: R<Vec3>,
        radius: f32,
        color: R<Color>,
    ) -> Result<(), InteropError> {
        ensure_gizmos_available(&ctxt)?;
        gizmo().sphere(Isometry3d::from_translation(*position), radius, *color);
        Ok(())
    }

    /// Draws a rectangle of the given size centred on the position, lying in the XY plane.
    fn rect(
        ctxt: FunctionCallContext,
        position: R<Vec3>,
        size: R<Vec2>,
        color: R<Color>,
    ) -> Result<(), InteropError> {
        ensure_gizmos_available(&ctxt)?;
        gizmo().rect(Isometry3d::from_translation(*position), *size, *color);
        Ok(())
    }

    /// Draws a rectangle of the given size centred on the position in 2D.
    fn rect_2d(
        ctxt: FunctionCallContext,
        position: R<Vec2>,
        size: R<Vec2>,
        color: R<Color>,
    ) -> Result<(), InteropError> {
        ensure_gizmos_available(&ctxt)?;
        gizmo().rect_2d(Isometry2d::from_translation(*position), *size, *color);
        Ok(())
    }

    /// Draws a unit cube, with the transform applied to it.
    fn cuboid(
        ctxt: FunctionCallContext,
        transform: R<Transform>,
        color: R<Color>,
    ) -> Result<(), InteropError> {
        ensure_gizmos_available(&ctxt)?;
        gizmo().cube(*transform, *color);
        Ok(())
    }

    /// Draws the red, green and blue axes of the transform, scaled by the length.
    fn axes(
        ctxt: FunctionCallContext,
        transform: R<Transform>,
        length: f32,
    ) -> Result<(), InteropError> {
        ensure_gizmos_available(&ctxt)?;
        gizmo().axes(*transform, length);
        Ok(())
    }

    /// Draws a cross of the given half size along the three axes, centred on the position.
    fn cross(
        ctxt: FunctionCallContext,
        position: R<Vec3>,
        half_size: f32,
        color: R<Color>,
    ) -> Result<(), InteropError> {
        ensure_gizmos_available(&ctxt)?;
        gizmo().cross(Isometry3d::from_translation(*position), half_size, *color);
        Ok(())
    }

    /// Draws connected lines through the points, in order.
    fn linestrip(
        ctxt: FunctionCallContext,
        points: Vec<V<Vec3>>,
        color: R<Color>,
    ) -> Result<(), InteropError> {
        ensure_gizmos_available(&ctxt)?;
        gizmo().linestrip(points.into_iter().map(|point| point.0), *color);
        Ok(())
    }

    /// Draws connected lines through the points in 2D, in order.
    fn linestrip_2d(
        ctxt: FunctionCallContext,
        points: Vec<V<Vec2>>,
        color: R<Color>,
    ) -> Result<(), InteropError> {
        ensure_gizmos_available(&ctxt)?;
        gizmo().linestrip_2d(points.into_iter().map(|point| point.0), *color);
        Ok(())
    }
}

/// Registers the gizmo functions.
pub(crate) fn register(world: &mut World) {
    register_gizmos_functions(world);
}

/// Registers the gizmos script bindings.
#[derive(Default)]
pub struct BevyGizmosBindingsPlugin;

impl Plugin for BevyGizmosBindingsPlugin {
    fn finish(&self, app: &mut App) {
        let registry = app
            .world()
            .resource::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        bevy_mod_scripting_bindings::set_crate_script_namespaces(
            &mut registry.write(),
            &["bevy_gizmos"],
            "gizmos",
        );
    }

    fn build(&self, app: &mut App) {
        // the marker type has no fields, scripts only see it once registered
        app.register_type::<Gizmos>();
        register(app.world_mut());
    }
}
