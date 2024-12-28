// @generated by cargo bevy-api-gen generate, modify the templates not this file
#![allow(clippy::all)]
#![allow(unused, deprecated, dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]
use super::bevy_ecs::*;
use super::bevy_reflect::*;
use bevy_mod_scripting_core::{
    AddContextInitializer, StoreDocumentation,
    bindings::{ReflectReference, function::from::{Ref, Mut, Val}},
};
use crate::*;
pub struct BevyCoreScriptingPlugin;
impl bevy::app::Plugin for BevyCoreScriptingPlugin {
    fn build(&self, app: &mut bevy::prelude::App) {
        let mut world = app.world_mut();
        NamespaceBuilder::<::bevy::core::prelude::Name>::new(world)
            .overwrite_script_function(
                "eq",
                |
                    _self: Ref<bevy::core::prelude::Name>,
                    other: Ref<bevy::core::prelude::Name>|
                {
                    let output: bool = ::bevy::core::prelude::Name::eq(
                            _self.into(),
                            other.into(),
                        )
                        .into();
                    output
                },
            )
            .overwrite_script_function(
                "clone",
                |_self: Ref<bevy::core::prelude::Name>| {
                    let output: Val<bevy::core::prelude::Name> = ::bevy::core::prelude::Name::clone(
                            _self.into(),
                        )
                        .into();
                    output
                },
            );
    }
}
