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
pub struct BevySceneScriptingPlugin;
pub(crate) fn register_scene_component_info_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_scene::SceneComponentInfo,
    >::new(world)
    .register_documented(
        "clone",
        |_self: R<::bevy_scene::SceneComponentInfo>| {
            let output: V<::bevy_scene::SceneComponentInfo> = {
                {
                    let output: V<::bevy_scene::SceneComponentInfo> =
                        <::bevy_scene::SceneComponentInfo as ::std::clone::Clone>::clone(&_self)
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
            ::bevy_scene::SceneComponentInfo,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevySceneScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_scene_component_info_functions(&mut world);
    }
}
