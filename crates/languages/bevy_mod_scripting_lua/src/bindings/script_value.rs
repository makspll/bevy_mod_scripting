use std::{
    cell::Cell,
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
        } else if self.0.len() == 1
            && let Some(first) = self.0.pop_front()
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
/// How deeply tables may nest when converted to a [`ScriptValue`], deeper (i.e. cyclic) tables are an error instead of a stack overflow.
pub const MAX_TABLE_DEPTH: usize = 128;

thread_local! {
    static TABLE_DEPTH: Cell<usize> = const { Cell::new(0) };
}

/// Counts a table being converted on this thread, see [`MAX_TABLE_DEPTH`].
struct TableDepth;

impl TableDepth {
    fn enter() -> mlua::Result<Self> {
        TABLE_DEPTH.with(|depth| {
            if depth.get() >= MAX_TABLE_DEPTH {
                return Err(mlua::Error::FromLuaConversionError {
                    from: "table",
                    to: "ScriptValue".to_owned(),
                    message: Some(format!(
                        "tables nest deeper than {MAX_TABLE_DEPTH} levels, is the table cyclic?"
                    )),
                });
            }
            depth.set(depth.get() + 1);
            Ok(TableDepth)
        })
    }
}

impl Drop for TableDepth {
    fn drop(&mut self) {
        TABLE_DEPTH.with(|depth| depth.set(depth.get() - 1));
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
                // released when the table is converted, nested tables are converted inside `pairs`
                let _depth = TableDepth::enter()?;
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

/// The value returned by a function called from Lua, a tuple is returned as multiple values.
struct LuaFunctionReturn(ScriptValue);

impl IntoLuaMulti for LuaFunctionReturn {
    fn into_lua_multi(self, lua: &mlua::Lua) -> mlua::Result<MultiValue> {
        match self.0 {
            ScriptValue::Tuple(VariadicTuple(tuple)) => {
                MultiLuaScriptValue(tuple).into_lua_multi(lua)
            }
            value => LuaScriptValue(value).into_lua_multi(lua),
        }
    }
}

/// Calls a function from Lua, the location of the call is only looked up for errors.
fn call_from_lua(
    lua: &mlua::Lua,
    call: impl FnOnce(FunctionCallContext) -> Result<ScriptValue, InteropError>,
) -> mlua::Result<LuaFunctionReturn> {
    match call(LUA_CALLER_CONTEXT) {
        Ok(out) => Ok(LuaFunctionReturn(out)),
        Err(mut error) => {
            if let InteropError::FunctionInteropError { context, .. } = &mut error
                && let Some(context) = &mut **context
            {
                profiling::scope!("function call context");
                context.set_location(lua.inspect_stack(1, |debug| {
                    LocationContext {
                        line: debug.current_line().unwrap_or_default() as u32,
                        col: None,
                        script_name: lua.app_data_ref::<LuaContextAppData>().and_then(|v| {
                            v.last_loaded_script_name.as_ref().map(|n| n.to_string())
                        }),
                    }
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
pub(crate) struct LuaFunctionCache(HashMap<usize, (DynamicScriptFunction, mlua::Function)>);

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
    // the cache is created with the context, inserting app data here could panic if any is borrowed
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
                // avoid using into_lua of the hashmap type as that requires an intermediate throwaway map
                Value::Table(lua.create_table_from(
                    map.into_iter().map(|(k, v)| (k, LuaScriptValue::from(v))),
                )?)
            }
        })
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cyclic_and_too_deep_tables_are_errors() {
        let lua = mlua::Lua::new();
        let (cyclic, deep, ok): (Value, Value, Value) = lua
            .load(
                r#"
                local cyclic = {}; cyclic.t = cyclic
                local function nest(n) local t = {} for _ = 1, n do t = { t } end return t end
                return cyclic, nest(1000), nest(100)
                "#,
            )
            .eval()
            .unwrap();
        for table in [cyclic, deep] {
            let err = LuaScriptValue::from_lua(table, &lua).unwrap_err();
            assert!(err.to_string().contains("is the table cyclic?"), "{err}");
        }
        assert!(LuaScriptValue::from_lua(ok, &lua).is_ok());
        // the depth is released after errors as well
        assert_eq!(TABLE_DEPTH.with(Cell::get), 0);
    }
}
