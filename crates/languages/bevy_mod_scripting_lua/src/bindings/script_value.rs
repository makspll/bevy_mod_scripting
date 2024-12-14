use std::ops::{Deref, DerefMut};

use bevy_mod_scripting_core::bindings::{script_val::ScriptValue, ReflectBase};
use mlua::{FromLua, Value};

use super::reference::LuaReflectReference;

#[derive(Debug, Clone)]
pub struct LuaScriptValue(ScriptValue);

impl Deref for LuaScriptValue {
    type Target = ScriptValue;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl DerefMut for LuaScriptValue {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}

impl From<ScriptValue> for LuaScriptValue {
    fn from(value: ScriptValue) -> Self {
        LuaScriptValue(value)
    }
}

impl<'lua> FromLua<'lua> for LuaScriptValue {
    fn from_lua(value: mlua::Value<'lua>, lua: &'lua mlua::Lua) -> mlua::Result<Self> {
        Ok(match value {
            Value::Nil => ScriptValue::Unit,
            Value::Boolean(b) => ScriptValue::Bool(b),
            // Value::LightUserData(light_user_data) => todo!(),
            Value::Integer(i) => ScriptValue::Integer(i),
            Value::Number(n) => ScriptValue::Float(n),
            Value::String(s) => ScriptValue::String(s.to_str()?.to_owned().into()),
            // Value::Table(table) => todo!(),
            // Value::Function(function) => todo!(),
            // Value::Thread(thread) => todo!(),
            Value::UserData(ud) => {
                let ud = ud.take::<LuaReflectReference>()?;
                if ud.0.base.base_id == ReflectBase::World {
                    ScriptValue::World
                } else {
                    ScriptValue::Reference(ud.into())
                }
            }
            // Value::Error(error) => todo!(),
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: value.type_name(),
                    to: "ScriptValue",
                    message: Some("unsupported value type".to_owned()),
                })
            }
        }
        .into())
    }
}

impl From<LuaScriptValue> for ScriptValue {
    fn from(value: LuaScriptValue) -> Self {
        value.0
    }
}
