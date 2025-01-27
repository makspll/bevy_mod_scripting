use super::reference::LuaReflectReference;
use bevy_mod_scripting_core::bindings::{
    function::script_function::FunctionCallContext, script_value::ScriptValue,
};
use mlua::{FromLua, IntoLua, Value, Variadic};
use std::{
    collections::HashMap,
    ops::{Deref, DerefMut},
};

#[derive(Debug, Clone)]
pub struct LuaScriptValue(pub ScriptValue);

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

impl From<LuaScriptValue> for ScriptValue {
    fn from(value: LuaScriptValue) -> Self {
        value.0
    }
}

impl FromLua for LuaScriptValue {
    fn from_lua(value: mlua::Value, _lua: &mlua::Lua) -> mlua::Result<Self> {
        Ok(match value {
            Value::Nil => ScriptValue::Unit,
            Value::Boolean(b) => ScriptValue::Bool(b),
            // Value::LightUserData(light_user_data) => todo!(),
            #[cfg(not(feature = "luau"))]
            Value::Integer(i) => ScriptValue::Integer(i),
            #[cfg(feature = "luau")]
            Value::Integer(i) => ScriptValue::Integer(i as i64),
            Value::Number(n) => ScriptValue::Float(n),
            Value::String(s) => ScriptValue::String(s.to_str()?.to_owned().into()),
            Value::Table(table) => {
                let mut vec = Vec::with_capacity(table.len()? as usize);
                for i in table.sequence_values() {
                    let v: LuaScriptValue = i?;
                    vec.push(v.into());
                }
                ScriptValue::List(vec)
            }
            // Value::Thread(thread) => todo!(),
            Value::UserData(ud) => {
                let ud = ud.borrow::<LuaReflectReference>().map_err(|e| {
                    mlua::Error::FromLuaConversionError {
                        from: "UserData",
                        to: "LuaReflectReference".to_owned(),
                        message: Some(e.to_string()),
                    }
                })?;
                ScriptValue::Reference(ud.clone().into())
            }
            _ => {
                return Err(mlua::Error::FromLuaConversionError {
                    from: value.type_name(),
                    to: "ScriptValue".to_owned(),
                    message: Some("unsupported value type".to_owned()),
                })
            }
        }
        .into())
    }
}

pub const LUA_CALLER_CONTEXT: FunctionCallContext = FunctionCallContext {
    convert_to_0_indexed: true,
};

impl IntoLua for LuaScriptValue {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        Ok(match self.0 {
            ScriptValue::Unit => Value::Nil,
            ScriptValue::Bool(b) => Value::Boolean(b),
            #[cfg(not(feature = "luau"))]
            ScriptValue::Integer(i) => Value::Integer(i),
            #[cfg(feature = "luau")]
            ScriptValue::Integer(i) => Value::Integer(i as i32),
            ScriptValue::Float(f) => Value::Number(f),
            ScriptValue::String(s) => Value::String(lua.create_string(s.as_ref())?),
            ScriptValue::Reference(r) => LuaReflectReference::from(r).into_lua(lua)?,
            ScriptValue::Error(script_error) => return Err(mlua::Error::external(script_error)),
            ScriptValue::Function(function) => lua
                .create_function(move |_lua, args: Variadic<LuaScriptValue>| {
                    let out =
                        function.call(args.into_iter().map(Into::into), LUA_CALLER_CONTEXT)?;

                    Ok(LuaScriptValue::from(out))
                })?
                .into_lua(lua)?,
            ScriptValue::FunctionMut(function) => lua
                .create_function(move |_lua, args: Variadic<LuaScriptValue>| {
                    let out =
                        function.call(args.into_iter().map(Into::into), LUA_CALLER_CONTEXT)?;

                    Ok(LuaScriptValue::from(out))
                })?
                .into_lua(lua)?,
            ScriptValue::List(vec) => {
                let table = lua.create_table_from(
                    vec.into_iter()
                        .enumerate()
                        .map(|(k, v)| (k + 1, LuaScriptValue::from(v))),
                )?;
                Value::Table(table)
            }
            ScriptValue::Map(map) => {
                let hashmap: HashMap<String, Value> = map
                    .into_iter()
                    .map(|(k, v)| Ok((k, LuaScriptValue::from(v).into_lua(lua)?)))
                    .collect::<Result<_, mlua::Error>>()?;
                hashmap.into_lua(lua)?
            }
        })
    }
}
