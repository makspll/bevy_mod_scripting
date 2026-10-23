//! Script bindings for bevy cameras, projections and visibility.

use bevy_app::{App, Plugin};
// `script_bindings` expects these in scope
use bevy_camera::{
    Camera, OrthographicProjection,
    visibility::{InheritedVisibility, RenderLayers, ViewVisibility, Visibility},
};
use bevy_ecs::{reflect::AppTypeRegistry, world::World};
use bevy_math::{Rect, UVec2, Vec2, Vec3};
use bevy_mod_scripting_bindings::function::from::{M, R, V};
use bevy_mod_scripting_derive::script_bindings;
use bevy_shape::Ray3d;
use bevy_transform::components::GlobalTransform;

/// Registers the camera script bindings.
#[derive(Default)]
pub struct BevyCameraBindingsPlugin;

impl Plugin for BevyCameraBindingsPlugin {
    fn finish(&self, app: &mut App) {
        let registry = app
            .world()
            .resource::<bevy_ecs::reflect::AppTypeRegistry>()
            .clone();
        bevy_mod_scripting_bindings::set_crate_script_namespaces(
            &mut registry.write(),
            &["bevy_camera"],
            "camera",
        );
    }

    fn build(&self, app: &mut App) {
        use bevy_camera::visibility::VisibilitySystems;
        use bevy_mod_scripting_bindings::ScriptSystemSetAppExt;
        app.register_script_system_set(
            "camera",
            "CameraUpdateSystems",
            bevy_camera::CameraUpdateSystems,
        );
        for (name, set) in [
            ("CalculateBounds", VisibilitySystems::CalculateBounds),
            ("UpdateFrusta", VisibilitySystems::UpdateFrusta),
            (
                "VisibilityPropagate",
                VisibilitySystems::VisibilityPropagate,
            ),
            ("CheckVisibility", VisibilitySystems::CheckVisibility),
            (
                "MarkNewlyHiddenEntitiesInvisible",
                VisibilitySystems::MarkNewlyHiddenEntitiesInvisible,
            ),
        ] {
            app.register_script_system_set("camera.VisibilitySystems", name, set);
        }
        let world = app.world_mut();
        register_camera_functions(world);
        register_orthographic_projection_functions(world);
        register_visibility_functions(world);
        register_inherited_visibility_functions(world);
        register_view_visibility_functions(world);
        register_render_layers_functions(world);
    }
}

#[script_bindings(
    remote,
    name = "camera_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "camera",
    core
)]
impl Camera {
    /// A ray from the camera through a point on its viewport, or nil if the point cannot be converted.
    fn viewport_to_world(
        _self: R<Camera>,
        camera_transform: R<GlobalTransform>,
        viewport_position: R<Vec2>,
    ) -> Option<V<Ray3d>> {
        _self
            .viewport_to_world(&camera_transform, *viewport_position)
            .ok()
            .map(V)
    }

    /// The point in the 2D world seen through a point on the viewport, or nil if the point cannot be converted.
    fn viewport_to_world_2d(
        _self: R<Camera>,
        camera_transform: R<GlobalTransform>,
        viewport_position: R<Vec2>,
    ) -> Option<V<Vec2>> {
        _self
            .viewport_to_world_2d(&camera_transform, *viewport_position)
            .ok()
            .map(V)
    }

    /// The position on the viewport of a point in the world, or nil if the point cannot be converted.
    fn world_to_viewport(
        _self: R<Camera>,
        camera_transform: R<GlobalTransform>,
        world_position: R<Vec3>,
    ) -> Option<V<Vec2>> {
        _self
            .world_to_viewport(&camera_transform, *world_position)
            .ok()
            .map(V)
    }

    /// The size of the viewport in logical pixels, or nil if the camera has no target yet.
    fn logical_viewport_size(_self: R<Camera>) -> Option<V<Vec2>> {
        _self.logical_viewport_size().map(V)
    }

    /// The rectangle of the viewport in logical pixels, or nil if the camera has no target yet.
    fn logical_viewport_rect(_self: R<Camera>) -> Option<V<Rect>> {
        _self.logical_viewport_rect().map(V)
    }

    /// The size of the viewport in physical pixels, or nil if the camera has no target yet.
    fn physical_viewport_size(_self: R<Camera>) -> Option<V<UVec2>> {
        _self.physical_viewport_size().map(V)
    }
}

#[script_bindings(
    remote,
    name = "orthographic_projection_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "camera",
    core
)]
impl OrthographicProjection {
    /// The default projection for 2D cameras.
    fn default_2d() -> V<OrthographicProjection> {
        V(OrthographicProjection::default_2d())
    }

    /// The default projection for 3D cameras.
    fn default_3d() -> V<OrthographicProjection> {
        V(OrthographicProjection::default_3d())
    }
}

#[script_bindings(
    remote,
    name = "visibility_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "camera",
    core
)]
impl Visibility {
    /// Switches between inherited and visible. A hidden entity stays hidden.
    fn toggle_inherited_visible(mut _self: M<Visibility>) {
        _self.toggle_inherited_visible();
    }

    /// Switches between visible and hidden. An inherited entity stays inherited.
    fn toggle_visible_hidden(mut _self: M<Visibility>) {
        _self.toggle_visible_hidden();
    }
}

#[script_bindings(
    remote,
    name = "inherited_visibility_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "camera",
    core
)]
impl InheritedVisibility {
    /// Whether the entity is visible, taking its parents into account.
    fn get(_self: R<InheritedVisibility>) -> bool {
        (*_self).get()
    }
}

#[script_bindings(
    remote,
    name = "view_visibility_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "camera",
    core
)]
impl ViewVisibility {
    /// Whether the entity is visible to a camera this frame.
    fn get(_self: R<ViewVisibility>) -> bool {
        (*_self).get()
    }
}

#[script_bindings(
    remote,
    name = "render_layers_functions",
    bms_bindings_path = "bevy_mod_scripting_bindings",
    namespace = "camera",
    core
)]
impl RenderLayers {
    /// Render layers containing only the given layer.
    fn layer(n: usize) -> V<RenderLayers> {
        // `RenderLayers::layer` is const and panics for large layers, so build it from `none` instead
        V(RenderLayers::none().with(n))
    }

    /// Render layers containing no layers, so nothing is rendered with them.
    fn none() -> V<RenderLayers> {
        V(RenderLayers::none())
    }

    /// A copy of these render layers that also contains the given layer.
    fn with(_self: R<RenderLayers>, n: usize) -> V<RenderLayers> {
        V(_self.clone().with(n))
    }

    /// A copy of these render layers without the given layer.
    fn without(_self: R<RenderLayers>, n: usize) -> V<RenderLayers> {
        V(_self.clone().without(n))
    }

    /// Whether these render layers share at least one layer with another set.
    fn intersects(_self: R<RenderLayers>, other: R<RenderLayers>) -> bool {
        _self.intersects(&other)
    }

    /// The numbers of all layers in these render layers.
    fn iter(_self: R<RenderLayers>) -> Vec<usize> {
        _self.iter().collect()
    }
}
