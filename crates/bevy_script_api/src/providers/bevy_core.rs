#![allow(clippy::all, unused_imports, deprecated, dead_code)]
// @generated by cargo bevy-api-gen generate, modify the templates not this file

use super::bevy_ecs::*;

use super::bevy_reflect::*;

extern crate self as bevy_script_api;

/// Component used to identify an entity. Stores a hash for faster comparisons.

/// The hash is eagerly re-computed upon each update to the name.

/// [`Name`] should not be treated as a globally unique identifier for entities,

/// as multiple entities can have the same name.  [`Entity`] should be

/// used instead as the default unique identifier.

#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
derive(clone,debug),
remote="bevy::core::prelude::Name",
functions[r#"
/// Gets the name of the entity as a `&str`.

    #[lua(kind = "Method")]
    fn as_str(&self) -> &str;

"#,
			r#"

    #[lua(kind = "Method")]
    fn update_hash(&mut self) -> ();

"#,
			r#"

    #[lua(as_trait = "std::cmp::PartialEq", kind = "Method")]
    fn eq(&self, #[proxy] other: &name::Name) -> bool;

"#,
			r#"

    #[lua(as_trait = "std::clone::Clone", kind = "Method", output(proxy))]
    fn clone(&self) -> bevy::core::prelude::Name;

"#]
)]

pub struct LuaName {}

crate::impl_tealr_generic!(pub(crate) struct T);

#[derive(Default)]
pub(crate) struct Globals;

impl bevy_mod_scripting_lua::tealr::mlu::ExportInstances for Globals {
    fn add_instances<'lua, T: bevy_mod_scripting_lua::tealr::mlu::InstanceCollector<'lua>>(
        self,
        instances: &mut T,
    ) -> bevy_mod_scripting_lua::tealr::mlu::mlua::Result<()> {
        Ok(())
    }
}

pub struct BevyCoreAPIProvider;

impl bevy_mod_scripting_core::hosts::APIProvider for BevyCoreAPIProvider {
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
        bevy_mod_scripting_lua::tealr::mlu::set_global_env(Globals, ctx)
            .map_err(|e| bevy_mod_scripting_core::error::ScriptError::Other(e.to_string()))
    }

    fn get_doc_fragment(&self) -> Option<Self::DocTarget> {
        Some(bevy_mod_scripting_lua::docs::LuaDocFragment::new(
            "BevyCoreAPI",
            |tw| {
                tw.document_global_instance::<Globals>()
                    .expect("Something went wrong documenting globals")
                    .process_type::<LuaName>()
            },
        ))
    }

    fn setup_script(
        &mut self,
        script_data: &bevy_mod_scripting_core::hosts::ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        Ok(())
    }

    fn setup_script_runtime(
        &mut self,
        world_ptr: bevy_mod_scripting_core::world::WorldPointer,
        _script_data: &bevy_mod_scripting_core::hosts::ScriptData,
        ctx: &mut Self::ScriptContext,
    ) -> Result<(), bevy_mod_scripting_core::error::ScriptError> {
        Ok(())
    }

    fn register_with_app(&self, app: &mut bevy::app::App) {
        app.register_foreign_lua_type::<bevy::core::prelude::Name>();
    }
}
