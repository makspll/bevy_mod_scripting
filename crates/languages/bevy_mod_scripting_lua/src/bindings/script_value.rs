use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_bindings::{
    LocationContext, error::InteropError, function::script_function::FunctionCallContext,
    script_value::ScriptValue,
};
use bevy_platform::collections::HashMap;
use mlua::{FromLua, FromLuaMulti, IntoLua, IntoLuaMulti, MultiValue, Value, Variadic};

use crate::{IntoMluaError, LuaContextAppData};

use super::reference::LuaReflectReference;

#[derive(Debug, Clone)]
/// A wrapper around a [`ScriptValue`] that implements [`FromLua`] and [`IntoLua`]
pub struct LuaScriptValue(pub ScriptValue);

impl LuaScriptValue {
    /// converts self into either a table of values or a single value depending on if it's a [`ScriptValue::MultipleResult`].
    /// The value returned will either be a [`mlua::Value::Table`] or any other [`mlua::Value`]
    fn normalize_into_lua_value(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        let is_table = matches!(&self.0, ScriptValue::MultipleValue(_));
        let multi = self.into_lua_multi(lua)?;
        let mut iter = multi.into_iter();
        if is_table && let Some(only) = iter.next() {
            Ok(only)
        } else {
            Ok(mlua::Value::Table(lua.create_sequence_from(iter)?))
        }
    }

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
                    let varargs = args.into_iter().collect::<ScriptValue>();
                    match f.call::<LuaScriptValue>(LuaScriptValue(varargs)) {
                        Ok(v) => v.0,
                        Err(e) => ScriptValue::Error(InteropError::external(Box::new(e))),
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
                .create_function(move |lua, args: LuaScriptValue| {
                    let loc = {
                        profiling::scope!("function call context");
                        lua.inspect_stack(1).map(|debug| LocationContext {
                            line: debug.curr_line().try_into().unwrap_or_default(),
                            col: None,
                            script_name: lua.app_data_ref::<LuaContextAppData>().and_then(|v| {
                                v.last_loaded_script_name.as_ref().map(|n| n.to_string())
                            }),
                        })
                    };
                    let out = function
                        .call(
                            args.0.into_iter().map(Into::into),
                            FunctionCallContext::new_with_location(Language::Lua, loc),
                        )
                        .map_err(IntoMluaError::to_lua_error)?;

                    Ok(LuaScriptValue::from(out))
                })?
                .into_lua(lua)?,
            ScriptValue::FunctionMut(function) => lua
                .create_function(move |lua, args: LuaScriptValue| {
                    let loc = {
                        profiling::scope!("function call context");
                        lua.inspect_stack(1).map(|debug| LocationContext {
                            line: debug.curr_line().try_into().unwrap_or_default(),
                            col: None,
                            script_name: lua.app_data_ref::<LuaContextAppData>().and_then(|v| {
                                v.last_loaded_script_name.as_ref().map(|n| n.to_string())
                            }),
                        })
                    };
                    let out = function
                        .call(
                            args.0.into_iter().map(Into::into),
                            FunctionCallContext::new_with_location(Language::Lua, loc),
                        )
                        .map_err(IntoMluaError::to_lua_error)?;

                    Ok(LuaScriptValue::from(out))
                })?
                .into_lua(lua)?,
            ScriptValue::List(vec) => {
                LuaScriptValue(ScriptValue::MultipleValue(vec)).normalize_into_lua_value(lua)?
            }
            ScriptValue::MultipleValue(vec) => {
                // normally `MultipleValue` will be converted at a higher level via `into_lua_multi`, but if nested levels of this come up, we will hit this path
                // in which case we probably want to convert to a table, for example if we have [ScriptValue::Number, ScriptValue::Tuple], we will want: {1: 1, 2: {1: ..}} etc.
                LuaScriptValue(ScriptValue::MultipleValue(vec)).normalize_into_lua_value(lua)?
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
impl FromLuaMulti for LuaScriptValue {
    fn from_lua_multi(
        tuple: mlua::MultiValue,
        lua: &mlua::Lua,
    ) -> std::result::Result<Self, mlua::Error> {
        // multiple values get converted into well, MultipleValue variants.
        // note that from_lua, may still produce nested multiple values
        // that is to be interpted by bindings code directly, i.e. we can support `my_binding(args: Variadic<ScriptValue>)`, by expecting a multi value
        let vals = tuple
            .into_iter()
            .map(|v| LuaScriptValue::from_lua(v, lua).map(ScriptValue::from))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(LuaScriptValue(ScriptValue::MultipleValue(vals)))
    }
}

/// The context for calling a function from Lua
pub const LUA_CALLER_CONTEXT: FunctionCallContext = FunctionCallContext::new(Language::Lua);
#[profiling::all_functions]
impl IntoLuaMulti for LuaScriptValue {
    fn into_lua_multi(self, lua: &mlua::Lua) -> std::result::Result<mlua::MultiValue, mlua::Error> {
        let vals = self.0.into_iter();
        let vals = vals
            .into_iter()
            .map(LuaScriptValue)
            .map(|v| v.into_lua(lua))
            .collect::<Result<Vec<_>, _>>()?;

        Ok(MultiValue::from_vec(vals))
    }
}
