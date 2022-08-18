use std::{
    ops::{Deref, DerefMut},
    sync::Arc,
};

use bevy::reflect::{TypeRegistration, TypeRegistry};
use bevy_mod_scripting_core::{prelude::*, world::WorldPointer};
use bevy_mod_scripting_rhai::{prelude::*, rhai};
use rhai::plugin::*;

use crate::common::bevy::{ScriptTypeRegistration, ScriptWorld};
use crate::rhai::ToDynamic;

use super::{base_rhai_plugin, RegisterForeignRhaiType};

#[export_module]
#[allow(dead_code)]
pub(crate) mod bevy_plugin {
    use crate::common::bevy::ScriptWorld;
    use bevy::prelude::Entity;

    pub mod type_registration {
        pub type TypeRegistration = ScriptTypeRegistration;

        #[rhai_fn(global)]
        pub fn short_name(self_: &mut TypeRegistration) -> ImmutableString {
            self_.short_name().into()
        }

        #[rhai_fn(global)]
        pub fn type_name(self_: &mut TypeRegistration) -> ImmutableString {
            self_.type_name().into()
        }

        #[rhai_fn(global)]
        pub fn to_string(self_: &mut TypeRegistration) -> String {
            self_.to_string()
        }

        #[rhai_fn(global)]
        pub fn to_debug(self_: &mut TypeRegistration) -> String {
            format!("{:?}", self_)
        }
    }

    pub mod world {
        pub type World = ScriptWorld;

        #[rhai_fn(global)]
        pub fn get_type_by_name(self_: World, type_name: &str) -> Dynamic {
            self_
                .get_type_by_name(type_name)
                .map(Dynamic::from)
                .unwrap_or_default()
        }

        #[rhai_fn(global)]
        pub fn get_children(self_: World, parent: Entity) -> Vec<Dynamic> {
            self_
                .get_children(parent)
                .into_iter()
                .map(Dynamic::from)
                .collect()
        }

        #[rhai_fn(global, return_raw)]
        pub fn add_default_component(
            ctx: NativeCallContext,
            self_: World,
            entity: Entity,
            type_registration: super::type_registration::TypeRegistration,
        ) -> Result<Dynamic, Box<EvalAltResult>> {
            self_
                .add_default_component(entity, type_registration)
                .map_err(|e| {
                    Box::new(EvalAltResult::ErrorRuntime(
                        Dynamic::from(e.to_string()),
                        Position::NONE,
                    ))
                })
                .and_then(|ok| ok.to_dynamic(ctx))
        }

        #[rhai_fn(global, return_raw)]
        pub fn get_component(
            ctx: NativeCallContext,
            self_: World,
            entity: Entity,
            comp_type: super::type_registration::TypeRegistration,
        ) -> Result<Dynamic, Box<EvalAltResult>> {
            let component = self_.get_component(entity, comp_type).map_err(|e| {
                Box::new(EvalAltResult::ErrorRuntime(
                    e.to_string().into(),
                    Position::NONE,
                ))
            })?;

            if let Some(c) = component {
                c.to_dynamic(ctx)
            } else {
                Ok(Default::default())
            }
        }

        #[rhai_fn(global)]
        pub fn to_string(self_: &mut World) -> String {
            self_.to_string()
        }

        #[rhai_fn(global)]
        pub fn to_debug(self_: &mut World) -> String {
            format!("{:?}", self_)
        }
    }
}

pub struct RhaiBevyAPIProvider;

impl APIProvider for RhaiBevyAPIProvider {
    type APITarget = Engine;
    type ScriptContext = RhaiContext;
    type DocTarget = RhaiDocFragment;

    fn attach_api(&mut self, engine: &mut Self::APITarget) -> Result<(), ScriptError> {
        engine.register_static_module("bevy", exported_module!(bevy_plugin).into());
        engine.register_static_module("base", exported_module!(base_rhai_plugin).into());
        Ok(())
    }

    fn setup_script_runtime(
        &mut self,
        world_ptr: WorldPointer,
        _script_data: &ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), ScriptError> {
        ctx.scope.set_value("world", ScriptWorld::new(world_ptr));
        Ok(())
    }

    fn setup_script(
        &mut self,
        script_data: &ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), ScriptError> {
        ctx.scope.set_value("entity", script_data.entity);
        Ok(())
    }

    fn get_doc_fragment(&self) -> Option<Self::DocTarget> {
        None
    }

    fn register_with_app(&self, app: &mut bevy::prelude::App) {
        app.register_foreign_rhai_type::<bool>();
        app.register_foreign_rhai_type::<f32>();
        app.register_foreign_rhai_type::<f64>();
        app.register_foreign_rhai_type::<i8>();
        app.register_foreign_rhai_type::<i16>();
        app.register_foreign_rhai_type::<i32>();
        app.register_foreign_rhai_type::<i64>();
        app.register_foreign_rhai_type::<i128>();
        app.register_foreign_rhai_type::<isize>();
        app.register_foreign_rhai_type::<u8>();
        app.register_foreign_rhai_type::<u16>();
        app.register_foreign_rhai_type::<u32>();
        app.register_foreign_rhai_type::<u64>();
        app.register_foreign_rhai_type::<u128>();
        app.register_foreign_rhai_type::<usize>();
    }
}
