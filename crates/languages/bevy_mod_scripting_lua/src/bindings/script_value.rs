use std::{
    cell::Cell,
    collections::VecDeque,
    ops::{Deref, DerefMut},
    os::raw::c_int,
    sync::Arc,
};

use bevy_mod_scripting_asset::Language;
use bevy_mod_scripting_bindings::{
    LocationContext, VariadicTuple,
    error::InteropError,
    function::script_function::{DynamicScriptFunction, FunctionCallContext},
    script_value::ScriptValue,
};
use bevy_platform::collections::HashMap;
use mlua::{FromLua, FromLuaMulti, IntoLua, IntoLuaMulti, MultiValue, Value, ffi, state::RawLua};

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

    /// Converts function arguments straight off the stack, see [`LuaScriptValue::from_stack`].
    unsafe fn from_stack_multi(nvals: c_int, lua: &RawLua) -> mlua::Result<Self> {
        let mut vals = VecDeque::with_capacity(nvals.max(0) as usize);
        for i in 0..nvals {
            // Safety: the caller guarantees the `nvals` top values are on the stack
            vals.push_back(unsafe { LuaScriptValue::from_stack(-nvals + i, lua) }?.0);
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

/// Moves the values of a table found so far at the keys `1..` into a map, keyed by the keys as strings,
/// as Lua turns numbers into strings.
fn slots_into_map(slots: &mut Vec<Option<ScriptValue>>) -> HashMap<String, ScriptValue> {
    slots
        .drain(..)
        .enumerate()
        .filter_map(|(i, v)| v.map(|v| ((i + 1).to_string(), v)))
        .collect()
}

impl Drop for TableDepth {
    fn drop(&mut self) {
        TABLE_DEPTH.with(|depth| depth.set(depth.get() - 1));
    }
}

#[profiling::all_functions]
impl FromLua for LuaScriptValue {
    fn from_lua(value: mlua::Value, lua: &mlua::Lua) -> mlua::Result<Self> {
        let Value::Table(table) = value else {
            return non_table_value(value).map(LuaScriptValue);
        };
        // released when the table is converted, nested tables are converted inside `pairs`
        let _depth = TableDepth::enter()?;
        // NOTE: lua tables keep integer keys in either an array part (iterated by `pairs` in
        // key order) or a hash part (iterated in arbitrary order), and `t[i]`/`#t` can't tell
        // which. Never infer list order from `pairs`, place each value by its key instead.
        let mut builder = TableBuilder::new(table.raw_len());
        table.for_each::<Value, LuaScriptValue>(|k, v| {
            let slot = match &k {
                Value::Integer(i) => Some(*i),
                _ => None,
            };
            builder.insert(slot, v.0, || String::from_lua(k, lua))
        })?;
        Ok(LuaScriptValue(builder.finish()))
    }

    /// Reads values straight off the stack: strings and tables are converted without creating Lua references
    /// for them, which dominated converting tables.
    unsafe fn from_stack(idx: c_int, lua: &RawLua) -> mlua::Result<Self> {
        let state = lua.state();
        // Safety: the caller guarantees `idx` is a valid stack index
        unsafe {
            let idx = ffi::lua_absindex(state, idx);
            match ffi::lua_type(state, idx) {
                ffi::LUA_TSTRING => Ok(LuaScriptValue(ScriptValue::String(
                    String::from_stack(idx, lua)?.into(),
                ))),
                ffi::LUA_TTABLE => table_from_stack(idx, lua).map(LuaScriptValue),
                _ => non_table_value(Value::from_stack(idx, lua)?).map(LuaScriptValue),
            }
        }
    }
}

/// Converts any value but a table.
fn non_table_value(value: Value) -> mlua::Result<ScriptValue> {
    Ok(match value {
        Value::Nil => ScriptValue::Unit,
        Value::Boolean(b) => ScriptValue::Bool(b),
        // Value::LightUserData(light_user_data) => todo!(),
        Value::Integer(i) => ScriptValue::Integer(i),
        Value::Number(n) => ScriptValue::Float(n),
        Value::String(s) => ScriptValue::String(s.to_str()?.to_owned().into()),
        Value::Function(f) => {
            // kept next to the function calling it, to hand it back to Lua as it is, see `IntoLua`
            let f = Arc::new(f);
            let called = f.clone();
            ScriptValue::Function(
                DynamicScriptFunction::from(
                    move |_context: FunctionCallContext, args: VecDeque<ScriptValue>| {
                        match called.call::<MultiLuaScriptValue>(MultiLuaScriptValue(args)) {
                            Ok(v) => v.into_script_value(),
                            Err(e) => ScriptValue::Error(InteropError::external(Box::new(e))),
                        }
                    },
                )
                .with_script_function(f),
            )
        }
        // Value::Thread(thread) => todo!(),
        Value::UserData(ud) => {
            if let Ok(type_ref) = ud.borrow::<LuaTypeRef>() {
                return Ok(ScriptValue::Type(type_ref.0));
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
    })
}

/// Builds the script value of a table in one pass: values of the keys `1..=len` go to their slot,
/// until another key shows the table isn't a sequence, then everything goes to a map.
struct TableBuilder {
    len: usize,
    slots: Vec<Option<ScriptValue>>,
    map: Option<HashMap<String, ScriptValue>>,
    count: usize,
}

impl TableBuilder {
    fn new(len: usize) -> Self {
        Self {
            len,
            slots: Vec::new(),
            map: None,
            count: 0,
        }
    }

    /// Adds an entry, `integer_key` if the key is an integer, `string_key` turns the key into a map key.
    fn insert(
        &mut self,
        integer_key: Option<i64>,
        value: ScriptValue,
        string_key: impl FnOnce() -> mlua::Result<String>,
    ) -> mlua::Result<()> {
        self.count += 1;
        let map = match &mut self.map {
            Some(map) => map,
            None => match integer_key {
                Some(i) if (1..=self.len as i64).contains(&i) => {
                    if self.slots.is_empty() {
                        self.slots.resize_with(self.len, || None);
                    }
                    self.slots[i as usize - 1] = Some(value);
                    return Ok(());
                }
                _ => self.map.insert(slots_into_map(&mut self.slots)),
            },
        };
        map.insert(string_key()?, value);
        Ok(())
    }

    fn finish(mut self) -> ScriptValue {
        match self.map {
            Some(map) => ScriptValue::Map(map),
            // a proper sequence has exactly the keys 1..=len
            None if self.count == self.len => {
                ScriptValue::List(self.slots.into_iter().flatten().collect())
            }
            // only integer keys, but with holes
            None => ScriptValue::Map(slots_into_map(&mut self.slots)),
        }
    }
}

/// The value at an absolute stack index, passed to a protected call without creating a Lua reference for it.
#[cfg(any(feature = "lua51", feature = "luajit"))]
struct StackValue(c_int);

#[cfg(any(feature = "lua51", feature = "luajit"))]
impl IntoLua for StackValue {
    fn into_lua(self, lua: &mlua::Lua) -> mlua::Result<Value> {
        // Safety: the index is valid, see `push_into_stack`
        lua.exec_raw_lua(|lua| unsafe { Value::from_stack(self.0, lua) })
    }

    unsafe fn push_into_stack(self, lua: &RawLua) -> mlua::Result<()> {
        // Safety: made with a valid index, the caller guarantees room for the copy
        unsafe { ffi::lua_pushvalue(lua.state(), self.0) };
        Ok(())
    }
}

/// Converts the table at the absolute stack index `idx`, iterating it with the raw API.
///
/// # Safety
/// `idx` must be an absolute index of a table on the stack. The stack is left as it was.
unsafe fn table_from_stack(idx: c_int, lua: &RawLua) -> mlua::Result<ScriptValue> {
    let _depth = TableDepth::enter()?;
    let state = lua.state();
    // Safety: `idx` is a table, every push below is popped before returning
    unsafe {
        // the key, the value, and a nested conversion's own key and value
        if ffi::lua_checkstack(state, 4) == 0 {
            return Err(mlua::Error::StackError);
        }
        let mut builder = TableBuilder::new(ffi::lua_rawlen(state, idx) as usize);
        // `lua_next` raises an error if the table changed under it, which a finalizer can do: converting
        // an entry may allocate (i.e. a number key turned into a string) and so run a collection.
        // Iterating in a protected call keeps that error from jumping over the Rust frames here.
        #[cfg(not(feature = "luau"))]
        iterate_protected(idx, &mut builder, lua)?;
        // as `Table::for_each` does on Luau: `lua_rawiter` pushes each key and value, and raises no errors
        #[cfg(feature = "luau")]
        {
            let mut index = ffi::lua_rawiter(state, idx, 0);
            while index >= 0 {
                let entry = entry_from_stack(&mut builder, lua);
                ffi::lua_pop(state, 2);
                entry?;
                index = ffi::lua_rawiter(state, idx, index);
            }
        }
        Ok(builder.finish())
    }
}

/// What [`iterate_table`] works on, passed through Lua as a pointer.
#[cfg(not(feature = "luau"))]
struct TableIteration<'a> {
    builder: &'a mut TableBuilder,
    lua: &'a RawLua,
    /// The error or panic which stopped the iteration, raised once the protected call has returned
    stopped: std::thread::Result<mlua::Result<()>>,
}

/// Adds the entries of the table at index 1 to the [`TableIteration`] at index 2. Only `lua_next` raises errors here.
#[cfg(not(feature = "luau"))]
unsafe extern "C-unwind" fn iterate_table(state: *mut ffi::lua_State) -> c_int {
    use std::panic::{AssertUnwindSafe, catch_unwind};
    // Safety: called by `iterate_protected` only, which passes the table and a live `TableIteration`
    unsafe {
        let iteration = ffi::lua_touserdata(state, 2) as *mut TableIteration;
        ffi::lua_pushnil(state);
        while ffi::lua_next(state, 1) != 0 {
            let iteration = &mut *iteration;
            match catch_unwind(AssertUnwindSafe(|| {
                entry_from_stack(iteration.builder, iteration.lua)
            })) {
                // the value, `lua_next` continues from the key
                Ok(Ok(())) => ffi::lua_pop(state, 1),
                stopped => {
                    iteration.stopped = stopped;
                    break;
                }
            }
        }
        0
    }
}

/// Adds the entries of the table at the absolute stack index `idx` to the builder, iterating in a protected call.
///
/// # Safety
/// `idx` must be an absolute index of a table on the stack, with room for 3 more values. The stack is left as it was.
#[cfg(not(feature = "luau"))]
unsafe fn iterate_protected(
    idx: c_int,
    builder: &mut TableBuilder,
    lua: &RawLua,
) -> mlua::Result<()> {
    let state = lua.state();
    let mut iteration = TableIteration {
        builder,
        lua,
        stopped: Ok(Ok(())),
    };
    let iteration_ptr = &mut iteration as *mut TableIteration as *mut std::ffi::c_void;
    // Safety: forwarded from the caller, `iteration` outlives the call
    unsafe {
        // pushing a function allocates on these, which can raise an error too: mlua's protected call handles that
        #[cfg(any(feature = "lua51", feature = "luajit"))]
        mlua::Lua::get_or_init_from_ptr(state).exec_raw::<()>(StackValue(idx), |state| {
            ffi::lua_pushlightuserdata(state, iteration_ptr);
            iterate_table(state);
            ffi::lua_settop(state, 0);
        })?;
        // a function without upvalues is a plain value here, nothing below allocates
        #[cfg(not(any(feature = "lua51", feature = "luajit")))]
        {
            ffi::lua_pushcfunction(state, iterate_table);
            ffi::lua_pushvalue(state, idx);
            ffi::lua_pushlightuserdata(state, iteration_ptr);
            if ffi::lua_pcall(state, 2, 0, 0) != ffi::LUA_OK {
                // only strings are read in place
                let message = if ffi::lua_type(state, -1) == ffi::LUA_TSTRING {
                    let mut size = 0;
                    let data = ffi::lua_tolstring(state, -1, &mut size);
                    let bytes = std::slice::from_raw_parts(data as *const u8, size);
                    String::from_utf8_lossy(bytes).into_owned()
                } else {
                    "error while iterating a table".to_owned()
                };
                ffi::lua_pop(state, 1);
                return Err(mlua::Error::RuntimeError(message));
            }
        }
    }
    iteration
        .stopped
        .unwrap_or_else(|panic| std::panic::resume_unwind(panic))
}

/// Adds the key at -2 and the value at -1 of the stack to the table being converted.
///
/// # Safety
/// The two top stack values must be a table's key and value. They are left on the stack.
unsafe fn entry_from_stack(builder: &mut TableBuilder, lua: &RawLua) -> mlua::Result<()> {
    let state = lua.state();
    // Safety: forwarded from the caller
    unsafe {
        let key = ffi::lua_absindex(state, -2);
        let value = LuaScriptValue::from_stack(-1, lua)?.0;
        let integer_key = match ffi::lua_type(state, key) {
            ffi::LUA_TSTRING => None,
            // numbers are read without references, others are rare
            _ => match Value::from_stack(key, lua)? {
                Value::Integer(i) => Some(i),
                _ => None,
            },
        };
        // strings are read in place, numbers are turned into strings on a copy, so the key the
        // iteration continues from is never changed
        builder.insert(integer_key, value, || String::from_stack(key, lua))
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
            ScriptValue::Function(function) => match function.script_function::<mlua::Function>() {
                // a function of this Lua state is handed back as it is: the script gets the function it
                // passed in, and calls it without going through Rust
                Some(f) if *f.weak_lua() == lua.weak() => Value::Function(f.clone()),
                _ => lua
                    .create_function(move |lua, args: MultiLuaScriptValue| {
                        call_from_lua(lua, |context| function.call(args.0, context))
                    })?
                    .into_lua(lua)?,
            },
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
    use std::sync::{Arc, Mutex};

    use super::*;

    /// A comparable form of a value: maps sorted by key.
    fn canonical(value: &ScriptValue) -> String {
        match value {
            ScriptValue::Map(map) => {
                let mut entries = map
                    .iter()
                    .map(|(k, v)| format!("{k}={}", canonical(v)))
                    .collect::<Vec<_>>();
                entries.sort();
                format!("map{{{}}}", entries.join(","))
            }
            ScriptValue::List(list) => {
                let items = list.iter().map(canonical).collect::<Vec<_>>();
                format!("list[{}]", items.join(","))
            }
            ScriptValue::Unit => "nil".to_owned(),
            ScriptValue::Bool(b) => b.to_string(),
            ScriptValue::Integer(i) => format!("{i}i"),
            ScriptValue::Float(f) => format!("{f}f"),
            ScriptValue::String(s) => format!("{s:?}"),
            other => format!("{other:?}"),
        }
    }

    /// A Lua function storing what its arguments convert to, as function arguments are read off the stack.
    fn capture(lua: &mlua::Lua) -> Arc<Mutex<Vec<mlua::Result<VecDeque<ScriptValue>>>>> {
        let seen = Arc::new(Mutex::new(Vec::new()));
        let store = seen.clone();
        let function = lua
            .create_function(move |lua, args: mlua::MultiValue| {
                // converted like the arguments of registered functions
                let converted = unsafe {
                    lua.exec_raw_lua(|raw| {
                        let n = args.len() as c_int;
                        for arg in args {
                            arg.push_into_stack(raw)?;
                        }
                        let converted = MultiLuaScriptValue::from_stack_multi(n, raw);
                        ffi::lua_pop(raw.state(), n);
                        converted
                    })
                };
                store.lock().unwrap().push(converted.map(|v| v.0));
                Ok(())
            })
            .unwrap();
        lua.globals().set("capture", function).unwrap();
        seen
    }

    #[test]
    fn cyclic_and_too_deep_tables_are_errors() {
        let lua = mlua::Lua::new();
        let seen = capture(&lua);
        let (cyclic, deep, ok): (Value, Value, Value) = lua
            .load(
                r#"
                local cyclic = {}; cyclic.t = cyclic
                local function nest(n) local t = {} for _ = 1, n do t = { t } end return t end
                capture(cyclic); capture(nest(1000)); capture(nest(100))
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
        // the same read off the stack
        {
            let seen = seen.lock().unwrap();
            for result in &seen[..2] {
                let err = result.as_ref().unwrap_err();
                assert!(err.to_string().contains("is the table cyclic?"), "{err}");
            }
            assert!(seen[2].is_ok());
        }
        // a finalizer changing the table while it is converted is an error too, never a jump over the conversion
        let changed: bool = lua
            .load(
                r#"
                local t = {}
                for i = 1, 20000 do t[i + 0.5] = i end
                local armed, changed, stop = false, false, false
                -- a finalizer which runs again in every collection until it changed the table
                local function arm()
                    local function finalizer()
                        if armed then
                            armed, changed = false, true
                            for k in pairs(t) do t[k] = nil end
                            for i = 1, 60000 do t[i + 100000.5] = i end
                        elseif not stop then
                            arm()
                        end
                    end
                    if newproxy then
                        getmetatable(newproxy(true)).__gc = finalizer
                    else
                        setmetatable({}, { __gc = finalizer })
                    end
                end
                arm()
                -- collect all the time, number keys allocate strings
                pcall(collectgarbage, "setpause", 1)
                pcall(collectgarbage, "setstepmul", 1000)
                armed = true
                capture(t)
                armed, stop = false, true
                return changed
                "#,
            )
            .eval()
            .unwrap();
        // Luau has no finalizers
        assert_eq!(changed, cfg!(not(feature = "luau")));
        if changed {
            let seen = seen.lock().unwrap();
            let err = seen[3].as_ref().unwrap_err();
            assert!(err.to_string().contains("invalid key to 'next'"), "{err}");
        }
        // the depth is released after errors as well
        assert_eq!(TABLE_DEPTH.with(Cell::get), 0);
    }

    #[test]
    fn values_convert_the_same_off_the_stack() {
        let lua = mlua::Lua::new();
        let seen = capture(&lua);
        let values: Vec<Value> = lua
            .load(
                r#"
                local values = {
                    1, 2.5, "text", true,
                    {}, { 1, 2, 3 }, { a = 1, b = "x" }, { 1, 2, nil, 4 }, { [1] = "a", b = 2 },
                    { [2.5] = "f", [3] = "i", [-1] = "n" }, { { 1 }, { k = { 2, "deep" } } },
                    { [3] = "c", [1] = "a", [2] = "b" },
                }
                for _, v in ipairs(values) do capture(v) end
                return values
                "#,
            )
            .eval()
            .unwrap();
        let seen = seen.lock().unwrap();
        assert_eq!(seen.len(), values.len());
        for (value, from_stack) in values.into_iter().zip(seen.iter()) {
            let from_value = LuaScriptValue::from_lua(value, &lua).unwrap().0;
            let from_stack = from_stack.as_ref().unwrap();
            assert_eq!(from_stack.len(), 1);
            assert_eq!(canonical(&from_stack[0]), canonical(&from_value));
        }
        // functions come back to their Lua state as themselves, and wrapped to another
        let function = Value::Function(lua.create_function(|_, ()| Ok(())).unwrap());
        let converted = LuaScriptValue::from_lua(function.clone(), &lua).unwrap().0;
        let back = LuaScriptValue(converted.clone()).into_lua(&lua).unwrap();
        assert_eq!(back, function);
        let other = mlua::Lua::new();
        let wrapped = LuaScriptValue(converted).into_lua(&other).unwrap();
        assert!(wrapped.is_function());
    }
}
