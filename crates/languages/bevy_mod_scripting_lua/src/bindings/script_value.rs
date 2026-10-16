use std::{
    collections::VecDeque,
    ops::{Deref, DerefMut},
};

use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_bindings::{
    LocationContext, VariadicTuple,
    error::InteropError,
    function::script_function::{DynamicScriptFunction, FunctionCallContext},
    script_value::ScriptValue,
};
use bevy_platform::collections::HashMap;
use mlua::{FromLua, FromLuaMulti, IntoLua, IntoLuaMulti, MultiValue, Value};

use crate::{IntoMluaError, LuaContextAppData};

use super::reference::{LuaReflectReference, LuaTypeRef};

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
            Value::Integer(i) => ScriptValue::Integer(i),
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
                // NOTE: lua tables keep integer keys in either an array part (iterated by `pairs` in
                // key order) or a hash part (iterated in arbitrary order), and `t[i]`/`#t` can't tell
                // which. Never infer list order from `pairs`, place each value by its key instead.
                let len = table.raw_len();
                let entries = table
                    .pairs::<Value, LuaScriptValue>()
                    .collect::<Result<Vec<_>, _>>()?;

                if entries.is_empty() {
                    return Ok(LuaScriptValue::from(ScriptValue::List(Default::default())));
                }

                // a proper sequence has exactly the keys 1..=len
                let is_sequence = entries.len() == len
                    && entries.iter().all(
                        |(k, _)| matches!(k, Value::Integer(i) if (1..=len as i64).contains(i)),
                    );

                if is_sequence {
                    let mut slots: Vec<Option<ScriptValue>> = vec![None; len];
                    for (k, v) in entries {
                        if let Value::Integer(i) = k {
                            slots[i as usize - 1] = Some(v.into());
                        }
                    }
                    let vec = slots.into_iter().flatten().collect::<VecDeque<_>>();
                    return Ok(LuaScriptValue::from(ScriptValue::List(vec)));
                } else {
                    // anything else (string keys, holes, mixed) is treated as a map
                    let mut map = HashMap::new();
                    for (k, v) in entries {
                        let str_: String = String::from_lua(k, lua)?;
                        map.insert(str_, v.into());
                    }
                    return Ok(LuaScriptValue::from(ScriptValue::Map(map)));
                }
            }
            // Value::Thread(thread) => todo!(),
            Value::UserData(ud) => {
                if let Ok(type_ref) = ud.borrow::<LuaTypeRef>() {
                    return Ok(LuaScriptValue(ScriptValue::Type(type_ref.0)));
                }
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

/// Calls a function from Lua, the location of the call is only looked up for errors.
fn call_from_lua(
    lua: &mlua::Lua,
    call: impl FnOnce(FunctionCallContext) -> Result<ScriptValue, InteropError>,
) -> mlua::Result<MultiLuaScriptValue> {
    match call(LUA_CALLER_CONTEXT) {
        Ok(out) => Ok(MultiLuaScriptValue::from_script_value(out)),
        Err(mut error) => {
            if let InteropError::FunctionInteropError { context, .. } = &mut error
                && let Some(context) = &mut **context
            {
                profiling::scope!("function call context");
                context.set_location(lua.inspect_stack(1, |debug| LocationContext {
                    line: debug.current_line().unwrap_or_default() as u32,
                    col: None,
                    script_name: lua
                        .app_data_ref::<LuaContextAppData>()
                        .and_then(|v| v.last_loaded_script_name.as_ref().map(|n| n.to_string())),
                }));
            }
            Err(error.to_lua_error())
        }
    }
}

/// The Lua functions created for registered functions, so `v:method()` doesn't create one on every access.
///
/// Keeps the function alive with its Lua function, so its [`DynamicScriptFunction::identity`] can't be reused.
#[derive(Default)]
struct LuaFunctionCache(HashMap<usize, (DynamicScriptFunction, mlua::Function)>);

/// Converts a registered function to a Lua function, reusing the one created the last time.
///
/// Only for functions which live in the registry, others would fill the cache.
pub(crate) fn registered_function_into_lua(
    lua: &mlua::Lua,
    function: DynamicScriptFunction,
) -> mlua::Result<Value> {
    let key = function.identity();
    if let Some(cache) = lua.app_data_ref::<LuaFunctionCache>()
        && let Some((_, lua_function)) = cache.0.get(&key)
    {
        return Ok(Value::Function(lua_function.clone()));
    }
    let lua_function = lua.create_function({
        let function = function.clone();
        move |lua, args: MultiLuaScriptValue| {
            call_from_lua(lua, |context| function.call(args.0, context))
        }
    })?;
    if lua.app_data_ref::<LuaFunctionCache>().is_none() {
        lua.set_app_data(LuaFunctionCache::default());
    }
    if let Some(mut cache) = lua.app_data_mut::<LuaFunctionCache>() {
        cache.0.insert(key, (function, lua_function.clone()));
    }
    Ok(Value::Function(lua_function))
}
#[profiling::all_functions]
impl IntoLua for LuaScriptValue {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<mlua::Value> {
        Ok(match self.0 {
            ScriptValue::Unit => Value::Nil,
            ScriptValue::Bool(b) => Value::Boolean(b),
            ScriptValue::Integer(i) => Value::Integer(i),
            ScriptValue::Float(f) => Value::Number(f),
            ScriptValue::String(s) => Value::String(lua.create_string(s.as_ref())?),
            ScriptValue::Reference(r) => LuaReflectReference::from(r).into_lua(lua)?,
            ScriptValue::Type(type_id) => LuaTypeRef(type_id).into_lua(lua)?,
            ScriptValue::Error(script_error) => return Err(mlua::Error::external(script_error)),
            ScriptValue::Function(function) => lua
                .create_function(move |lua, args: MultiLuaScriptValue| {
                    call_from_lua(lua, |context| function.call(args.0, context))
                })?
                .into_lua(lua)?,
            ScriptValue::FunctionMut(function) => lua
                .create_function(move |lua, args: MultiLuaScriptValue| {
                    call_from_lua(lua, |context| function.call(args.0, context))
                })?
                .into_lua(lua)?,
            // when we encounter a tuple here, we can't convert to many args, so just return a table
            ScriptValue::List(vec) | ScriptValue::Tuple(VariadicTuple(vec)) => {
                // NOTE: `create_table_from` with integer keys puts them in the table's hash part, where
                // `pairs` yields them in arbitrary order. `create_sequence_from` fills the array part,
                // so the list keeps its order when it is later iterated or converted back.
                let table = lua.create_sequence_from(vec.into_iter().map(LuaScriptValue::from))?;
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
