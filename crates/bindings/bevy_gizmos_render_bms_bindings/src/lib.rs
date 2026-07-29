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
pub struct BevyGizmosRenderScriptingPlugin;
pub(crate) fn register_line_gizmo_entities_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_gizmos_render::LineGizmoEntities,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_gizmos_render::LineGizmoEntities>| {
            let output: V<::bevy_gizmos_render::LineGizmoEntities> = {
                {
                    let output: V<::bevy_gizmos_render::LineGizmoEntities> =
                        <::bevy_gizmos_render::LineGizmoEntities as ::std::clone::Clone>::clone(
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
    );
    let registry = world.get_resource_or_init::<AppTypeRegistry>();
    let mut registry = registry.write();
    registry
        .register_type_data::<
            ::bevy_gizmos_render::LineGizmoEntities,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyGizmosRenderScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_line_gizmo_entities_functions(&mut world);
    }
}
