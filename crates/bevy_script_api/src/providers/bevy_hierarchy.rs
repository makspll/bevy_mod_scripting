#![allow(clippy::all, unused_imports, deprecated, dead_code)]
// @generated by cargo bevy-api-gen generate, modify the templates not this file

use super::bevy_ecs::*;

use super::bevy_reflect::*;

use super::bevy_core::*;

extern crate self as bevy_script_api;
use crate::lua::RegisterForeignLuaType;

/// Contains references to the child entities of this entity.

/// Each child must contain a [`Parent`] component that points back to this entity.

/// This component rarely needs to be created manually,

/// consider using higher level utilities like [`BuildChildren::with_children`]

/// which are safer and easier to use.

/// See [`HierarchyQueryExt`] for hierarchy related methods on [`Query`].

/// [`HierarchyQueryExt`]: crate::query_extension::HierarchyQueryExt

/// [`Query`]: bevy_ecs::system::Query

/// [`Parent`]: crate::components::parent::Parent

/// [`BuildChildren::with_children`]: crate::child_builder::BuildChildren::with_children

#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
derive(debug,),
remote="bevy::hierarchy::prelude::Children",
functions[r#"
/// Swaps the child at `a_index` with the child at `b_index`.

    #[lua(kind = "MutatingMethod")]
    fn swap(&mut self, a_index: usize, b_index: usize) -> ();

"#]
)]

pub struct Children();

/// Holds a reference to the parent entity of this entity.

/// This component should only be present on entities that actually have a parent entity.

/// Parent entity must have this entity stored in its [`Children`] component.

/// It is hard to set up parent/child relationships manually,

/// consider using higher level utilities like [`BuildChildren::with_children`].

/// See [`HierarchyQueryExt`] for hierarchy related methods on [`Query`].

/// [`HierarchyQueryExt`]: crate::query_extension::HierarchyQueryExt

/// [`Query`]: bevy_ecs::system::Query

/// [`Children`]: super::children::Children

/// [`BuildChildren::with_children`]: crate::child_builder::BuildChildren::with_children

#[derive(bevy_mod_scripting_lua_derive::LuaProxy)]
#[proxy(
derive(debug,),
remote="bevy::hierarchy::prelude::Parent",
functions[r#"
/// Gets the [`Entity`] ID of the parent.

    #[lua(kind = "Method", output(proxy))]
    fn get(&self) -> bevy::ecs::entity::Entity;

"#,
			r#"

    #[lua(as_trait = "std::cmp::Eq", kind = "Method")]
    fn assert_receiver_is_total_eq(&self) -> ();

"#,
			r#"

    #[lua(as_trait = "std::cmp::PartialEq", kind = "Function", composite = "eq")]
    fn eq(&self, #[proxy] other: &components::parent::Parent) -> bool;

"#]
)]

pub struct Parent();

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

pub struct BevyHierarchyAPIProvider;

impl bevy_mod_scripting_core::hosts::APIProvider for BevyHierarchyAPIProvider {
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
            "BevyHierarchyAPI",
            |tw| {
                tw.document_global_instance::<Globals>()
                    .expect("Something went wrong documenting globals")
                    .process_type::<LuaChildren>()
                    .process_type::<LuaParent>()
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
        app.register_foreign_lua_type::<bevy::hierarchy::prelude::Children>();

        app.register_foreign_lua_type::<bevy::hierarchy::prelude::Parent>();
    }
}
