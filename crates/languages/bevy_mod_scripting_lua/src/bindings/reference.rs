use std::any::TypeId;

use bevy_mod_scripting_bindings::{
    ReflectReference, ScriptValue, WorldGuard, dispatch, error::InteropError,
};
use bevy_mod_scripting_world::ThreadWorldContainer;
use mlua::{FromLua, IntoLua, MetaMethod, UserData, UserDataMethods};

use crate::IntoMluaError;

use super::script_value::{LUA_CALLER_CONTEXT, LuaScriptValue, registered_function_into_lua};

/// A [`ReflectReference`] in Lua: a userdata with BMS' metatable of references, see `super::native::reference_at`.
/// Acts as a lua reflection interface. Any value which is registered in the type registry can be interacted with using this type.
#[derive(Debug, Clone, PartialEq)]
pub struct LuaReflectReference(pub ReflectReference);

impl AsRef<ReflectReference> for LuaReflectReference {
    fn as_ref(&self) -> &ReflectReference {
        &self.0
    }
}

impl From<LuaReflectReference> for ReflectReference {
    fn from(value: LuaReflectReference) -> Self {
        value.0
    }
}

impl From<ReflectReference> for LuaReflectReference {
    fn from(value: ReflectReference) -> Self {
        Self(value)
    }
}

impl IntoLua for LuaReflectReference {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        super::native::reference_into_lua(lua, self.0)
    }
}

impl FromLua for LuaReflectReference {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        match &value {
            mlua::Value::UserData(ud) => super::native::reference_value(lua, ud),
            _ => None,
        }
        .map(Self)
        .ok_or_else(|| mlua::Error::FromLuaConversionError {
            from: value.type_name(),
            to: "LuaReflectReference".to_owned(),
            message: None,
        })
    }
}

/// Runs a [`dispatch`] function with the world of the running script.
fn with_world<O>(
    f: impl FnOnce(WorldGuard<'static>) -> Result<O, InteropError>,
) -> mlua::Result<O> {
    let world = ThreadWorldContainer::try_get_context()
        .map_err(IntoMluaError::to_lua_error)?
        .world;
    f(world).map_err(IntoMluaError::to_lua_error)
}

/// A type itself, i.e. what [`bevy_mod_scripting_bindings::ScriptValue::Type`] is in Lua. Indexing it reaches the type's static members, i.e. `Entity.from_raw(..)`.
#[derive(Debug, Clone, Copy, PartialEq, mlua::FromLua)]
pub struct LuaTypeRef(pub TypeId);

impl UserData for LuaTypeRef {
    fn add_methods<T: UserDataMethods<Self>>(m: &mut T) {
        m.add_meta_function(
            MetaMethod::Index,
            |lua, (self_, key): (LuaTypeRef, LuaScriptValue)| {
                profiling::function_scope!("MetaMethod::Index");
                let member = with_world(|world| {
                    dispatch::static_index(world, self_.0, key.into(), LUA_CALLER_CONTEXT)
                })?;
                match member {
                    // functions of types live in the registry (bound ones are cached there too): their Lua functions
                    // are made once, not on every `Type.name`
                    ScriptValue::Function(function) => registered_function_into_lua(lua, function),
                    member => LuaScriptValue(member).into_lua(lua),
                }
            },
        );
    }
}
