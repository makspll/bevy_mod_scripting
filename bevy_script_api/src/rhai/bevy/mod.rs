use bevy::prelude::Entity;
use bevy_mod_scripting_core::{prelude::*, world::WorldPointer};

use bevy_mod_scripting_rhai::{
    prelude::*,
    rhai::{self, CustomType},
};
use rhai::plugin::*;

use crate::{
    common::bevy::{ScriptTypeRegistration, ScriptWorld},
    ReflectedValue,
};

use super::{RegisterForeignRhaiType, ToDynamic};

#[allow(deprecated)]
impl CustomType for ScriptTypeRegistration {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder
            .with_name("TypeRegistration")
            .with_fn("short_name", |self_: &mut Self| {
                ImmutableString::from(self_.short_name())
            })
            .with_fn("type_name", |self_: &mut Self| self_.type_name())
            .with_fn("to_string", |self_: &mut Self| self_.to_string())
            .with_fn("to_debug", |self_: &mut Self| format!("{:?}", self_));
    }
}

#[allow(deprecated)]
impl CustomType for ScriptWorld {
    fn build(mut builder: rhai::TypeBuilder<Self>) {
        builder
            .with_name("World")
            .with_fn("get_type_by_name", |self_: ScriptWorld, type_name: &str| {
                self_
                    .get_type_by_name(type_name)
                    .map(Dynamic::from)
                    .unwrap_or_default()
            })
            .with_fn("get_children", |self_: ScriptWorld, parent: Entity| {
                self_
                    .get_children(parent)
                    .into_iter()
                    .map(Dynamic::from)
                    .collect::<Vec<_>>()
            })
            .with_fn(
                "add_default_component",
                |self_: ScriptWorld, entity: Entity, type_registration: ScriptTypeRegistration| {
                    self_
                        .add_default_component(entity, type_registration)
                        .map_err(|e| {
                            Box::new(EvalAltResult::ErrorRuntime(
                                Dynamic::from(e.to_string()),
                                Position::NONE,
                            ))
                        })
                        .and_then(|ok| ok.to_dynamic())
                },
            )
            .with_fn(
                "get_component",
                |self_: ScriptWorld, entity: Entity, comp_type: ScriptTypeRegistration| {
                    let component = self_.get_component(entity, comp_type).map_err(|e| {
                        Box::new(EvalAltResult::ErrorRuntime(
                            e.to_string().into(),
                            Position::NONE,
                        ))
                    })?;
                    if let Some(c) = component {
                        c.to_dynamic()
                    } else {
                        Ok(Default::default())
                    }
                },
            )
            .with_fn("to_string", |self_: &mut ScriptWorld| self_.to_string())
            .with_fn("to_debug", |self_: &mut ScriptWorld| format!("{:?}", self_));
    }
}

pub struct RhaiBevyAPIProvider;

impl APIProvider for RhaiBevyAPIProvider {
    type APITarget = Engine;
    type ScriptContext = RhaiContext;
    type DocTarget = RhaiDocFragment;

    fn attach_api(&mut self, engine: &mut Self::APITarget) -> Result<(), ScriptError> {
        engine.build_type::<ReflectedValue>();
        engine.build_type::<ScriptTypeRegistration>();
        engine.build_type::<ScriptWorld>();
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
        app.register_foreign_rhai_type::<String>();
    }
}
