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
pub struct BevyExtractScriptingPlugin;
pub(crate) fn register_main_entity_functions(world: &mut World) {
    bevy_mod_scripting_bindings::function::namespace::NamespaceBuilder::<
        ::bevy_extract::MainEntity,
    >::new(world)
        .register_documented(
            "clone",
            |_self: R<::bevy_extract::MainEntity>| {
                let output: V<::bevy_extract::MainEntity> = {
                    {
                        let output: V<::bevy_extract::MainEntity> = <::bevy_extract::MainEntity as ::std::clone::Clone>::clone(
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
        )
        .register_documented(
            "eq",
            |_self: R<::bevy_extract::MainEntity>, other: R<::bevy_extract::MainEntity>| {
                let output: bool = {
                    {
                        let output: bool = <::bevy_extract::MainEntity as ::std::cmp::PartialEq<
                            ::bevy_extract::MainEntity,
                        >>::eq(&_self, &other)
                            .into();
                        output
                    }
                };
                output
            },
            "",
            &["_self", "other"],
        )
        .register_documented(
            "id",
            |_self: R<::bevy_extract::MainEntity>| {
                let output: V<::bevy_ecs::entity::Entity> = {
                    {
                        let output: V<::bevy_ecs::entity::Entity> = ::bevy_extract::MainEntity::id(
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
            ::bevy_extract::MainEntity,
            bevy_mod_scripting_bindings::MarkAsGenerated,
        >();
}
impl Plugin for BevyExtractScriptingPlugin {
    fn build(&self, app: &mut App) {
        let mut world = app.world_mut();
        register_main_entity_functions(&mut world);
    }
}
