use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_bindings::{
    LocationContext, VariadicTuple, error::InteropError,
    function::script_function::FunctionCallContext, script_value::ScriptValue,
};
use bevy_platform::collections::HashMap;
use mlua::{FromLua, FromLuaMulti, IntoLua, IntoLuaMulti, MultiValue, Value};

use crate::{IntoMluaError, LuaContextAppData};

use super::reference::LuaReflectReference;

/// A wrapper around many [`ScriptValue`]'s, used where the potential to return/receive many values separately arises.
pub struct MultiLuaScriptValue(pub VecDeque<ScriptValue>);

impl MultiLuaScriptValue {
    /// Coalesces the many values as interpreted by lua to a single script value from the bms framework
    pub fn into_script_value(mut self) -> ScriptValue {
        if self.0.is_empty() {
            ScriptValue::Unit
        } else if let Some(first) = self.0.pop_front()
            && self.0.len() == 1
        {
            first
        } else {
            ScriptValue::Tuple(VariadicTuple(self.0))
        }
    }

    /// Interprets a bms framework value as potentially multiple values in lua
    pub fn from_script_value(value: ScriptValue) -> Self {
        if let ScriptValue::Tuple(VariadicTuple(tuple)) = value {
            Self(tuple)
        } else {
            MultiLuaScriptValue(VecDeque::from_iter([value]))
        }
    }
}

impl FromLuaMulti for MultiLuaScriptValue {
    fn from_lua_multi(values: mlua::MultiValue, lua: &mlua::Lua) -> mlua::Result<Self> {
        let mut vals = VecDeque::with_capacity(values.len());
        for val in values {
            let script_val = LuaScriptValue::from_lua(val, lua)?;
            vals.push_back(script_val.into());
        }
        Ok(MultiLuaScriptValue(vals))
    }
}

impl IntoLuaMulti for MultiLuaScriptValue {
    fn into_lua_multi(self, lua: &mlua::Lua) -> mlua::Result<mlua::MultiValue> {
        let mut vals = MultiValue::with_capacity(self.0.len());
        let mut left = self.0.len();
        for val in self.0 {
            left -= 1;
            // tuples can get destructured into many vals if they are the last in the collection
            match val {
                ScriptValue::Tuple(VariadicTuple(tuple)) if left == 0 => {
                    let mut many_vals = MultiLuaScriptValue(tuple).into_lua_multi(lua)?;
                    vals.append(&mut many_vals);
                }
                _ => {
                    let mlua_val = LuaScriptValue(val).into_lua(lua)?;
                    vals.push_back(mlua_val);
                }
            }
        }
        Ok(vals)
    }
}

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
                    match f.call::<MultiLuaScriptValue>(MultiLuaScriptValue(args)) {
                        Ok(v) => v.into_script_value(),
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
                            let mut vec = VecDeque::with_capacity(table.len()? as usize);
                            vec.push_back(v.into());
                            for pair in iter {
                                vec.push_back(pair?.1.into());
                            }
                            return Ok(LuaScriptValue::from(ScriptValue::List(vec)));
                        }
                    }
                    None => return Ok(LuaScriptValue::from(ScriptValue::List(Default::default()))),
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
                .create_function(move |lua, args: MultiLuaScriptValue| {
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
                            args.0,
                            FunctionCallContext::new_with_location(Language::Lua, loc),
                        )
                        .map_err(IntoMluaError::to_lua_error)?;

                    Ok(MultiLuaScriptValue::from_script_value(out))
                })?
                .into_lua(lua)?,
            ScriptValue::FunctionMut(function) => lua
                .create_function(move |lua, args: MultiLuaScriptValue| {
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
                            args.0,
                            FunctionCallContext::new_with_location(Language::Lua, loc),
                        )
                        .map_err(IntoMluaError::to_lua_error)?;

                    Ok(MultiLuaScriptValue::from_script_value(out))
                })?
                .into_lua(lua)?,
            // when we encounter a tuple here, we can't convert to many args, so just return a table
            ScriptValue::List(vec) | ScriptValue::Tuple(VariadicTuple(vec)) => {
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
