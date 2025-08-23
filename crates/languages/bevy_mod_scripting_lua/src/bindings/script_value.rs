use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

use bevy_mod_scripting_core::{
    asset::Language,
    bindings::{function::script_function::FunctionCallContext, script_value::ScriptValue},
    error::InteropError,
};
use bevy_platform::collections::HashMap;
use mlua::{FromLua, IntoLua, Value, Variadic};

use super::reference::LuaReflectReference;

#[derive(Debug, Clone)]
/// A wrapper around a [`ScriptValue`] that implements [`FromLua`] and [`IntoLua`]
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
#[profiling::all_functions]
impl FromLua for LuaScriptValue {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
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
            Value::Function(f) => ScriptValue::Function(
                (move |_context: FunctionCallContext, args: VecDeque<ScriptValue>| {
                    println!("Lua function called with args: {args:?}");
                    match f.call::<LuaScriptValue>(
                        args.into_iter()
                            .map(LuaScriptValue)
                            .collect::<Variadic<_>>(),
                    ) {
                        Ok(v) => v.0,
                        Err(e) => ScriptValue::Error(InteropError::external_error(Box::new(e))),
                    }
                })
                .into(),
            ),
            Value::Table(table) => {
                // check the key types, if strings then it's a map
                let mut iter = table.pairs::<Value, LuaScriptValue>();

                match iter.next() {
                    Some(v) => {
                        let (k, v) = v?;
                        // if the key is a string, then it's a map
                        if k.is_string() {
                            let mut map = HashMap::new();
                            map.insert(k.to_string()?, v.into());
                            for pair in iter {
                                let (k, v) = pair?;
                                let str_: String = String::from_lua(k, lua)?;
                                map.insert(str_, v.into());
                            }
                            return Ok(LuaScriptValue::from(ScriptValue::Map(map)));
                        } else {
                            // if the key is an integer, then it's a list
                            let mut vec = Vec::with_capacity(table.len()? as usize);
                            vec.push(v.into());
                            for pair in iter {
                                vec.push(pair?.1.into());
                            }
                            return Ok(LuaScriptValue::from(ScriptValue::List(vec)));
                        }
                    }
                    None => return Ok(LuaScriptValue::from(ScriptValue::List(vec![]))),
                }
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
                });
            }
        }
        .into())
    }
}

/// The context for calling a function from Lua
pub const LUA_CALLER_CONTEXT: FunctionCallContext = FunctionCallContext::new(Language::Lua);
#[profiling::all_functions]
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
                let hashmap: std::collections::HashMap<String, Value> = map
                    .into_iter()
                    .map(|(k, v)| Ok((k, LuaScriptValue::from(v).into_lua(lua)?)))
                    .collect::<Result<_, mlua::Error>>()?;
                hashmap.into_lua(lua)?
            }
        })
    }
}
