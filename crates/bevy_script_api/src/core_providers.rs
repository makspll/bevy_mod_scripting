use crate::lua::RegisterForeignLuaType;

pub struct LuaCoreBevyAPIProvider;

#[derive(Default)]
pub(crate) struct CoreBevyGlobals;

crate::impl_tealr_generic!(pub(crate) struct T);

impl bevy_mod_scripting_lua::tealr::mlu::ExportInstances for CoreBevyGlobals {
    fn add_instances<'lua, T: bevy_mod_scripting_lua::tealr::mlu::InstanceCollector<'lua>>(
        self,
        instances: &mut T,
    ) -> bevy_mod_scripting_lua::tealr::mlu::mlua::Result<()> {
        instances.add_instance(
            "world",
            crate::lua::util::DummyTypeName::<crate::lua::bevy::LuaWorld>::new,
        )?;
        instances.add_instance(
            "script",
            crate::lua::util::DummyTypeName::<crate::lua::bevy::LuaScriptData>::new,
        )?;
        instances.add_instance(
            "entity",
            crate::lua::util::DummyTypeName::<crate::providers::bevy_ecs::LuaEntity>::new,
        )?;
        Ok(())
    }
}

impl bevy_mod_scripting_core::hosts::APIProvider for LuaCoreBevyAPIProvider {
    type APITarget = std::sync::Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type ScriptContext = std::sync::Mutex<bevy_mod_scripting_lua::tealr::mlu::mlua::Lua>;
    type DocTarget = bevy_mod_scripting_lua::docs::LuaDocFragment;

    fn attach_api(
        &mut self,
        ctx: &mut Self::APITarget,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        let ctx = ctx
            .get_mut()
            .expect("Unable to acquire lock on Lua context");
        bevy_mod_scripting_lua::tealr::mlu::set_global_env(CoreBevyGlobals, ctx)
            .map_err(|e| bevy_mod_scripting_core::error::ScriptError::Other(e.to_string()))
    }

    fn get_doc_fragment(&self) -> Option<Self::DocTarget> {
        Some(bevy_mod_scripting_lua::docs::LuaDocFragment::new(
            "CoreBevyAPI",
            |tw| {
                tw
			.document_global_instance::<CoreBevyGlobals>().expect("Something went wrong documenting globals")            
            .process_type::<crate::lua::bevy::LuaWorld>()
			.process_type::<bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<crate::lua::bevy::LuaWorld>>()
			.process_type::<crate::lua::bevy::LuaScriptData>()
			.process_type::<bevy_mod_scripting_lua::tealr::mlu::UserDataProxy<crate::lua::bevy::LuaScriptData>>()
			.process_type::<crate::lua::bevy::LuaTypeRegistration>()
			.process_type::<crate::lua::std::LuaVec<T>>()
            },
        ))
    }

    fn setup_script(
        &mut self,
        script_data: &bevy_mod_scripting_core::hosts::ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        let ctx = ctx.get_mut().expect("Could not get context");
        let globals = ctx.globals();
        globals
            .set(
                "entity",
                crate::providers::bevy_ecs::LuaEntity::new(script_data.entity),
            )
            .map_err(bevy_mod_scripting_core::error::ScriptError::new_other)?;
        globals
            .set::<_, crate::lua::bevy::LuaScriptData>("script", script_data.into())
            .map_err(bevy_mod_scripting_core::error::ScriptError::new_other)?;

        Ok(())
    }

    fn setup_script_runtime(
        &mut self,
        world_ptr: bevy_mod_scripting_core::world::WorldPointer,
        _script_data: &bevy_mod_scripting_core::hosts::ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        let ctx = ctx.get_mut().expect("Could not get context");
        let globals = ctx.globals();
        globals
            .set("world", crate::lua::bevy::LuaWorld::new(world_ptr))
            .map_err(bevy_mod_scripting_core::error::ScriptError::new_other)
    }

    fn register_with_app(&self, app: &mut bevy::app::App) {
        app.register_foreign_lua_type::<usize>();
        app.register_foreign_lua_type::<isize>();
        app.register_foreign_lua_type::<f32>();
        app.register_foreign_lua_type::<f64>();
        app.register_foreign_lua_type::<u128>();
        app.register_foreign_lua_type::<u64>();
        app.register_foreign_lua_type::<u32>();
        app.register_foreign_lua_type::<u16>();
        app.register_foreign_lua_type::<u8>();
        app.register_foreign_lua_type::<i128>();
        app.register_foreign_lua_type::<i64>();
        app.register_foreign_lua_type::<i32>();
        app.register_foreign_lua_type::<i16>();
        app.register_foreign_lua_type::<i8>();
        app.register_foreign_lua_type::<String>();
        app.register_foreign_lua_type::<bool>();
    }
}
