use std::any::TypeId;

use bevy_mod_scripting_bindings::ReflectReference;
use mlua::{FromLua, IntoLua, MetaMethod, UserData, UserDataFields};

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

/// A type itself, i.e. what [`bevy_mod_scripting_bindings::ScriptValue::Type`] is in Lua. Indexing it reaches the type's static members, i.e. `Entity.from_raw(..)`.
#[derive(Debug, Clone, Copy, PartialEq, mlua::FromLua)]
pub struct LuaTypeRef(pub TypeId);

impl UserData for LuaTypeRef {
    fn add_fields<F: UserDataFields<Self>>(f: &mut F) {
        // Safety: the function follows the Lua C API, see `type_ref_index`
        f.add_meta_field_with(MetaMethod::Index, |lua| unsafe {
            lua.create_c_function(super::native::type_ref_index)
        });
    }
}
