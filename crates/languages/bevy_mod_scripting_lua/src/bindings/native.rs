//! Typed calls and native values in Lua, see [`bevy_mod_scripting_bindings::native`].
//!
//! - Functions with a typed entry are raw C closures ([`native_function`]) reading their arguments straight off the stack
//!   ([`LuaArgs`]) and pushing their results ([`LuaReturn`]). Arguments stay on the stack for the whole call, so the values
//!   borrowed from them can't be collected.
//! - Native values are raw userdata holding a [`NativeStorage`], with one metatable per type, made on first use: fields are read
//!   and written with the type's typed accessors, methods are the functions registered on the type, everything else goes
//!   through a reference to a copy of the value (the slow path).
//!
//! Errors are raised the way mlua raises them, with nothing left to drop in the frames they skip.
#![allow(unsafe_code)]

use std::{
    any::TypeId,
    borrow::Cow,
    ffi::c_void,
    os::raw::{c_char, c_int},
};

use bevy_mod_scripting_bindings::{
    DynamicScriptFunction, InteropError, ScriptValue, WorldExtensions, dispatch,
    native::{ArgSource, NativeStorage, NativeVTable, ReturnSink},
};
use bevy_mod_scripting_bindings_domain::ScriptOperatorNames;
use bevy_mod_scripting_world::ThreadWorldContainer;
use mlua::{FromLua, IntoLua, IntoLuaMulti, Lua, ffi, state::RawLua};

use super::script_value::{
    LUA_CALLER_CONTEXT, LuaFunctionReturn, LuaScriptValue, add_call_location,
};
use crate::{IntoInteropError, IntoMluaError};

/// Its address keys the "native" marker in the metatables of native values
static NATIVE_MARKER: u8 = 0;
/// Its address keys the metatable of function holders in the registry
static HOLDER_METATABLE: u8 = 0;

fn key(p: &'static u8) -> *mut c_void {
    p as *const u8 as *mut c_void
}

/// `t[p]` where `t` is at `idx`, pushed.
unsafe fn raw_get_p(l: *mut ffi::lua_State, idx: c_int, p: *mut c_void) -> c_int {
    unsafe {
        let idx = ffi::lua_absindex(l, idx);
        ffi::lua_pushlightuserdata(l, p);
        ffi::lua_rawget(l, idx)
    }
}

/// `t[p] = v` where `t` is at `idx` and `v` on top, popped.
unsafe fn raw_set_p(l: *mut ffi::lua_State, idx: c_int, p: *mut c_void) {
    unsafe {
        let idx = ffi::lua_absindex(l, idx);
        ffi::lua_pushlightuserdata(l, p);
        ffi::lua_insert(l, -2);
        ffi::lua_rawset(l, idx);
    }
}

/// The base of the native value at `idx`, if it is one: a userdata whose metatable is marked native.
pub(crate) unsafe fn native_base(l: *mut ffi::lua_State, idx: c_int) -> Option<*mut u8> {
    unsafe {
        if ffi::lua_type(l, idx) != ffi::LUA_TUSERDATA {
            return None;
        }
        let base = ffi::lua_touserdata(l, idx) as *mut u8;
        if base.is_null() || ffi::lua_getmetatable(l, idx) == 0 {
            return None;
        }
        let native = raw_get_p(l, -1, key(&NATIVE_MARKER)) != ffi::LUA_TNIL;
        ffi::lua_pop(l, 2);
        native.then_some(base)
    }
}

/// Runs `f` with mlua's view of the state and the stack it works on, which may be another thread than `l`'s.
unsafe fn with_raw<R>(l: *mut ffi::lua_State, f: impl FnOnce(&Lua, &RawLua) -> R) -> R {
    // Safety: every state BMS calls into is made by mlua, which outlives the call
    let lua = unsafe { Lua::get_or_init_from_ptr(l) };
    lua.exec_raw_lua(|raw| f(lua, raw))
}

/// The value at `idx` of `l` as a script value, the slow path.
unsafe fn script_value_at(l: *mut ffi::lua_State, idx: c_int) -> Result<ScriptValue, InteropError> {
    unsafe {
        if let Some(base) = native_base(l, idx) {
            return NativeStorage::to_script_value(base);
        }
        let idx = ffi::lua_absindex(l, idx);
        with_raw(l, |_, raw| {
            let state = raw.state();
            if state == l {
                return LuaScriptValue::from_stack(idx, raw);
            }
            ffi::lua_pushvalue(l, idx);
            ffi::lua_xmove(l, state, 1);
            let value = LuaScriptValue::from_stack(-1, raw);
            ffi::lua_pop(state, 1);
            value
        })
        .map(|v| v.0)
        .map_err(IntoInteropError::to_bms_error)
    }
}

/// Pushes values made by mlua onto `l`, returns how many.
unsafe fn push_with_mlua(
    l: *mut ffi::lua_State,
    push: impl FnOnce(&Lua, &RawLua) -> mlua::Result<c_int>,
) -> mlua::Result<c_int> {
    unsafe {
        with_raw(l, |lua, raw| {
            let state = raw.state();
            let n = push(lua, raw)?;
            if state != l {
                ffi::lua_xmove(state, l, n);
            }
            Ok(n)
        })
    }
}

/// The arguments of a typed call: the stack of the C function from `first`.
pub(crate) struct LuaArgs {
    l: *mut ffi::lua_State,
    first: c_int,
    len: usize,
}

impl LuaArgs {
    fn idx(&self, i: usize) -> c_int {
        self.first + i as c_int
    }
}

// Safety: native bases are only handed out for userdata marked native, made by `LuaReturn::native`, which stay on the stack
unsafe impl ArgSource for LuaArgs {
    fn len(&self) -> usize {
        self.len
    }

    fn number(&self, i: usize) -> Option<f64> {
        // Safety: the index is on the stack
        unsafe {
            (ffi::lua_type(self.l, self.idx(i)) == ffi::LUA_TNUMBER)
                .then(|| ffi::lua_tonumber(self.l, self.idx(i)))
        }
    }

    fn integer(&self, i: usize) -> Option<i64> {
        #[cfg(any(feature = "lua54", feature = "lua53", feature = "luau"))]
        // Safety: the index is on the stack
        unsafe {
            (ffi::lua_isinteger(self.l, self.idx(i)) != 0)
                .then(|| ffi::lua_tointeger(self.l, self.idx(i)) as i64)
        }
        #[cfg(not(any(feature = "lua54", feature = "lua53", feature = "luau")))]
        {
            let _ = i;
            None
        }
    }

    fn boolean(&self, i: usize) -> Option<bool> {
        // Safety: the index is on the stack
        unsafe {
            (ffi::lua_type(self.l, self.idx(i)) == ffi::LUA_TBOOLEAN)
                .then(|| ffi::lua_toboolean(self.l, self.idx(i)) != 0)
        }
    }

    fn native(&self, i: usize) -> Option<*mut u8> {
        // Safety: the index is on the stack
        unsafe { native_base(self.l, self.idx(i)) }
    }

    fn script_value(&self, i: usize) -> Result<ScriptValue, InteropError> {
        // Safety: the index is on the stack
        unsafe { script_value_at(self.l, self.idx(i)) }
    }
}

/// The results of a typed call, pushed onto the stack.
pub(crate) struct LuaReturn {
    l: *mut ffi::lua_State,
    n: c_int,
}

impl ReturnSink for LuaReturn {
    fn unit(&mut self) {}

    fn number(&mut self, n: f64) {
        // Safety: C functions have room for a few values
        unsafe { ffi::lua_pushnumber(self.l, n) };
        self.n += 1;
    }

    fn integer(&mut self, i: i64) {
        // Safety: see above
        unsafe { ffi::lua_pushinteger(self.l, i as ffi::lua_Integer) };
        self.n += 1;
    }

    fn boolean(&mut self, b: bool) {
        // Safety: see above
        unsafe { ffi::lua_pushboolean(self.l, b as c_int) };
        self.n += 1;
    }

    fn native(&mut self, vtable: &'static NativeVTable, write: &mut dyn FnMut(*mut u8)) -> bool {
        // Safety: see above, the storage is initialised before its metatable, the only thing which can drop it, is set
        unsafe {
            let base = new_userdata(self.l, NativeStorage::size(vtable), vtable.drop.is_some());
            if !(base as usize).is_multiple_of(NativeStorage::ALIGN) {
                ffi::lua_pop(self.l, 1);
                return false;
            }
            NativeStorage::init(base, vtable, write);
            if !push_native_metatable(self.l, vtable) {
                // only without a world, the value is leaked: the userdata has no `__gc`
                ffi::lua_pop(self.l, 1);
                return false;
            }
            ffi::lua_setmetatable(self.l, -2);
        }
        self.n += 1;
        true
    }

    fn script_value(&mut self, value: ScriptValue) -> Result<(), InteropError> {
        // Safety: pushes onto the stack of the C function
        let n = unsafe {
            push_with_mlua(self.l, |lua, raw| {
                let values = LuaFunctionReturn(value).into_lua_multi(lua)?;
                let n = values.len() as c_int;
                for value in values {
                    value.push_into_stack(raw)?;
                }
                Ok(n)
            })
        }
        .map_err(IntoInteropError::to_bms_error)?;
        self.n += n;
        Ok(())
    }
}

/// A userdata of `size` bytes, pushed. With `gc`, Luau calls the `__gc` of its metatable through a destructor.
unsafe fn new_userdata(l: *mut ffi::lua_State, size: usize, gc: bool) -> *mut u8 {
    #[cfg(feature = "luau")]
    unsafe {
        if gc {
            return ffi::lua_newuserdatadtor(l, size, luau_gc) as *mut u8;
        }
    }
    let _ = gc;
    unsafe { ffi::lua_newuserdata(l, size) as *mut u8 }
}

#[cfg(feature = "luau")]
unsafe extern "C" fn luau_gc(_l: *mut ffi::lua_State, base: *mut c_void) {
    // Safety: only made for native values and holders, both initialised before their metatable was set
    unsafe { drop_userdata(base as *mut u8) }
}

/// Drops what a native value or function holder holds: holders are marked by a null first word.
unsafe fn drop_userdata(base: *mut u8) {
    // Safety: called once, by the collector, on initialised userdata
    unsafe {
        if (base as *const usize).read() == 0 {
            std::ptr::drop_in_place(holder_function(base));
        } else {
            NativeStorage::drop_value(base);
        }
    }
}

#[cfg(not(feature = "luau"))]
unsafe extern "C-unwind" fn gc(l: *mut ffi::lua_State) -> c_int {
    // Safety: only set as `__gc` of native values and holders
    unsafe {
        let base = ffi::lua_touserdata(l, 1) as *mut u8;
        if !base.is_null() {
            drop_userdata(base);
        }
    }
    0
}

// ---------------------------------------------------------------- calls

/// Function holders: `[0usize][DynamicScriptFunction]`, the zero tells them apart from native values in `drop_userdata`.
#[repr(C)]
struct Holder {
    marker: usize,
    function: DynamicScriptFunction,
}

unsafe fn holder_function(base: *mut u8) -> *mut DynamicScriptFunction {
    // Safety: holders are `Holder`s
    unsafe { &raw mut (*(base as *mut Holder)).function }
}

/// Pushes a holder of the function.
unsafe fn push_holder(l: *mut ffi::lua_State, function: DynamicScriptFunction) {
    unsafe {
        let base = new_userdata(l, size_of::<Holder>(), true) as *mut Holder;
        base.write(Holder {
            marker: 0,
            function,
        });
        if raw_get_p(l, ffi::LUA_REGISTRYINDEX, key(&HOLDER_METATABLE)) == ffi::LUA_TNIL {
            ffi::lua_pop(l, 1);
            ffi::lua_createtable(l, 0, 1);
            #[cfg(not(feature = "luau"))]
            {
                ffi::lua_pushcfunction(l, gc);
                ffi::lua_setfield(l, -2, c"__gc".as_ptr());
            }
            ffi::lua_pushvalue(l, -1);
            raw_set_p(l, ffi::LUA_REGISTRYINDEX, key(&HOLDER_METATABLE));
        }
        ffi::lua_setmetatable(l, -2);
    }
}

/// A Lua function calling the typed entry of `function`, see the [module docs](self).
pub(crate) fn native_function(
    lua: &Lua,
    function: DynamicScriptFunction,
) -> mlua::Result<mlua::Function> {
    // Safety: pushes a holder and a closure over it, which the stack guard of `exec_raw` keeps balanced
    unsafe {
        lua.exec_raw((), |l| {
            push_holder(l, function);
            ffi::lua_pushcclosure(l, native_entry, 1);
        })
    }
}

/// Calls the function held by the upvalue with the stack as arguments.
unsafe extern "C-unwind" fn native_entry(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        let function =
            &*holder_function(ffi::lua_touserdata(l, ffi::lua_upvalueindex(1)) as *mut u8);
        let len = ffi::lua_gettop(l) as usize;
        call_holder(l, function, 1, len)
    }
}

/// Calls `function` with the `len` values from `first` as arguments, returns how many results it pushed, raises its errors.
unsafe fn call_holder(
    l: *mut ffi::lua_State,
    function: &DynamicScriptFunction,
    first: c_int,
    len: usize,
) -> c_int {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        // Safety: the arguments are on the stack
        unsafe { call(l, function, first, len) }
    }));
    let error = match result {
        Ok(Ok(n)) => return n,
        Ok(Err(error)) => error,
        Err(_) => InteropError::str("a native function panicked"),
    };
    // Safety: everything with a destructor is moved into the error value pushed here, `lua_error` skips nothing to drop
    unsafe { raise(l, error) }
}

unsafe fn call(
    l: *mut ffi::lua_State,
    function: &DynamicScriptFunction,
    first: c_int,
    len: usize,
) -> Result<c_int, InteropError> {
    let args = LuaArgs { l, first, len };
    let mut ret = LuaReturn { l, n: 0 };
    match function.call_native(&args, &mut ret, LUA_CALLER_CONTEXT) {
        Some(result) => result.map(|()| ret.n),
        // a function without a typed entry, i.e. a method of a native type which isn't typed
        None => {
            let values = (0..len)
                .map(|i| args.script_value(i))
                .collect::<Result<Vec<_>, _>>()?;
            let result = function.call(values, LUA_CALLER_CONTEXT)?;
            ret.script_value(result)?;
            Ok(ret.n)
        }
    }
}

unsafe fn raise(l: *mut ffi::lua_State, mut error: InteropError) -> ! {
    unsafe {
        let pushed = push_with_mlua(l, move |lua, raw| {
            add_call_location(lua, &mut error);
            error.to_lua_error().push_into_stack(raw)?;
            Ok(1)
        });
        if pushed.is_err() {
            let message = c"failed to raise an error of a native function";
            ffi::lua_pushstring(l, message.as_ptr());
        }
        drop(pushed);
        ffi::lua_error(l)
    }
}

// ---------------------------------------------------------------- native values

/// Pushes the metatable of native values of `vtable`'s type, made on first use, false if the type isn't registered as native.
unsafe fn push_native_metatable(l: *mut ffi::lua_State, vtable: &'static NativeVTable) -> bool {
    let vtable_key = vtable as *const NativeVTable as *mut c_void;
    unsafe {
        if raw_get_p(l, ffi::LUA_REGISTRYINDEX, vtable_key) != ffi::LUA_TNIL {
            return true;
        }
        ffi::lua_pop(l, 1);
    }
    let Ok(world) = ThreadWorldContainer::try_get_context().map(|c| c.world) else {
        return false;
    };
    let (fields, methods) = {
        let registry = world.script_function_registry();
        let registry = registry.read();
        let Some(native) = registry.native_type(vtable.type_id) else {
            return false;
        };
        let fields = native
            .fields
            .iter()
            .map(|(name, field)| (name.clone(), field.get.clone(), field.set.clone()))
            .collect::<Vec<_>>();
        let methods = registry
            .iter_type(vtable.type_id)
            .map(|(key, function)| (key.name.clone(), function.clone()))
            .collect::<Vec<_>>();
        (fields, methods)
    };
    unsafe {
        ffi::lua_createtable(l, 0, 16);
        ffi::lua_pushboolean(l, 1);
        raw_set_p(l, -2, key(&NATIVE_MARKER));
        #[cfg(not(feature = "luau"))]
        if vtable.drop.is_some() {
            ffi::lua_pushcfunction(l, gc);
            ffi::lua_setfield(l, -2, c"__gc".as_ptr());
        }

        // __index: getters, then methods, then the slow path
        ffi::lua_createtable(l, 0, fields.len() as c_int);
        for (name, get, _) in &fields {
            push_holder(l, get.clone());
            set_field(l, name);
        }
        // the typed functions of the operators, i.e. `add` and the reflected `mul` for `2 * v`
        let typed = |name: &str| {
            methods
                .iter()
                .find(|(n, f)| n == name && f.is_native())
                .map(|(_, f)| f.clone())
        };
        let operators = OPERATORS.map(|(meta, operator)| {
            (
                meta,
                operator,
                typed(operator.script_function_name()),
                operator.reflected_script_function_name().and_then(typed),
            )
        });
        ffi::lua_createtable(l, 0, methods.len() as c_int);
        for (name, function) in methods {
            push_function(l, function);
            set_field(l, &name);
        }
        ffi::lua_pushcclosure(l, native_index, 2);
        ffi::lua_setfield(l, -2, c"__index".as_ptr());

        // __newindex: setters
        ffi::lua_createtable(l, 0, fields.len() as c_int);
        for (name, _, set) in fields {
            if let Some(set) = set {
                push_holder(l, set);
                set_field(l, &name);
            }
        }
        ffi::lua_pushcclosure(l, native_newindex, 1);
        ffi::lua_setfield(l, -2, c"__newindex".as_ptr());

        // upvalues: the operator, its typed function and its typed reflected function, if any
        for (name, operator, forward, reflected) in operators {
            ffi::lua_pushinteger(l, operator as ffi::lua_Integer);
            for function in [forward, reflected] {
                match function {
                    Some(function) => push_holder(l, function),
                    None => ffi::lua_pushnil(l),
                }
            }
            ffi::lua_pushcclosure(l, native_operator, 3);
            ffi::lua_setfield(l, -2, name.as_ptr());
        }
        ffi::lua_pushstring(l, c"BMS native value".as_ptr());
        ffi::lua_setfield(l, -2, c"__name".as_ptr());

        ffi::lua_pushvalue(l, -1);
        raw_set_p(l, ffi::LUA_REGISTRYINDEX, vtable_key);
    }
    true
}

/// `t[name] = v` where `t` is below `v` on top, popped.
unsafe fn set_field(l: *mut ffi::lua_State, name: &str) {
    unsafe {
        ffi::lua_pushlstring(l, name.as_ptr() as *const c_char, name.len());
        ffi::lua_insert(l, -2);
        ffi::lua_rawset(l, -3);
    }
}

/// Pushes a registered function as Lua sees it: typed ones call their typed entry, others go through mlua.
unsafe fn push_function(l: *mut ffi::lua_State, function: DynamicScriptFunction) {
    unsafe {
        if function.is_native() {
            push_holder(l, function);
            ffi::lua_pushcclosure(l, native_entry, 1);
            return;
        }
        let pushed = push_with_mlua(l, |lua, raw| {
            super::script_value::registered_function_into_lua(lua, function)?
                .push_into_stack(raw)?;
            Ok(1)
        });
        if pushed.is_err() {
            ffi::lua_pushnil(l);
        }
    }
}

/// `v.name`: the getter in upvalue 1, else the method in upvalue 2, else the slow path.
unsafe extern "C-unwind" fn native_index(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        ffi::lua_pushvalue(l, 2);
        if ffi::lua_rawget(l, ffi::lua_upvalueindex(1)) == ffi::LUA_TUSERDATA {
            let getter = &*holder_function(ffi::lua_touserdata(l, -1) as *mut u8);
            ffi::lua_settop(l, 1);
            return call_holder(l, getter, 1, 1);
        }
        ffi::lua_pop(l, 1);
        ffi::lua_pushvalue(l, 2);
        if ffi::lua_rawget(l, ffi::lua_upvalueindex(2)) != ffi::LUA_TNIL {
            return 1;
        }
        ffi::lua_pop(l, 1);
        slow_path(l, |l| {
            let receiver = script_value_at(l, 1)?;
            let key = script_value_at(l, 2)?;
            let ScriptValue::Reference(receiver) = receiver else {
                return Err(InteropError::str(
                    "a native value is indexed with a reference to it",
                ));
            };
            let world = ThreadWorldContainer::try_get_context()?.world;
            let member = dispatch::index(world, receiver, key, LUA_CALLER_CONTEXT)?;
            let mut ret = LuaReturn { l, n: 0 };
            match member {
                dispatch::Member::Method(function) => {
                    // cached as a method, found in Lua the next time
                    let name = ffi::lua_type(l, 2) == ffi::LUA_TSTRING;
                    push_function(l, function);
                    if name {
                        ffi::lua_pushvalue(l, 2);
                        ffi::lua_pushvalue(l, -2);
                        ffi::lua_rawset(l, ffi::lua_upvalueindex(2));
                    }
                    Ok(1)
                }
                dispatch::Member::Static(value) => {
                    ret.script_value(value)?;
                    Ok(ret.n)
                }
            }
        })
    }
}

/// `v.name = x`: the setter in upvalue 1, called with `(v, x)`.
unsafe extern "C-unwind" fn native_newindex(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        ffi::lua_pushvalue(l, 2);
        if ffi::lua_rawget(l, ffi::lua_upvalueindex(1)) == ffi::LUA_TUSERDATA {
            let setter = &*holder_function(ffi::lua_touserdata(l, -1) as *mut u8);
            ffi::lua_pop(l, 1);
            ffi::lua_remove(l, 2);
            return call_holder(l, setter, 1, 2);
        }
        ffi::lua_pop(l, 1);
        slow_path(l, |l| {
            let name = match script_value_at(l, 2)? {
                ScriptValue::String(name) => name,
                key => Cow::Owned(format!("{key:?}")),
            };
            Err(InteropError::string(format!(
                "`{name}` is not a field which can be set on a native value"
            )))
        })
    }
}

/// Operators and other metamethods: the operator's function of the native operand, see [`dispatch::binary`].
///
/// Typed functions are in upvalues 2 (the operator's) and 3 (its reflected one, for a native value on the right).
unsafe extern "C-unwind" fn native_operator(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        let operator = ffi::lua_tointeger(l, ffi::lua_upvalueindex(1));
        let typed = |upvalue: c_int| {
            (ffi::lua_type(l, ffi::lua_upvalueindex(upvalue)) == ffi::LUA_TUSERDATA).then(|| {
                &*holder_function(ffi::lua_touserdata(l, ffi::lua_upvalueindex(upvalue)) as *mut u8)
            })
        };
        match operator_from_index(operator) {
            Ok(ScriptOperatorNames::DisplayPrint) | Err(_) => {}
            Ok(ScriptOperatorNames::Negation | ScriptOperatorNames::Length) => {
                if native_base(l, 1).is_some()
                    && let Some(function) = typed(2)
                {
                    ffi::lua_settop(l, 1);
                    return call_holder(l, function, 1, 1);
                }
            }
            Ok(_) => {
                if native_base(l, 1).is_some() {
                    if let Some(function) = typed(2) {
                        ffi::lua_settop(l, 2);
                        return call_holder(l, function, 1, 2);
                    }
                } else if native_base(l, 2).is_some()
                    && let Some(function) = typed(3)
                {
                    // the native value is the receiver of the reflected function
                    ffi::lua_settop(l, 2);
                    ffi::lua_insert(l, 1);
                    return call_holder(l, function, 1, 2);
                }
            }
        }
        slow_path(l, |l| {
            let operator = operator_from_index(operator)?;
            let world = ThreadWorldContainer::try_get_context()?.world;
            let lhs = script_value_at(l, 1)?;
            let mut ret = LuaReturn { l, n: 0 };
            let result = match operator {
                ScriptOperatorNames::Negation | ScriptOperatorNames::Length => {
                    let ScriptValue::Reference(value) = lhs else {
                        return Err(InteropError::str("expected a native value"));
                    };
                    dispatch::unary(world, operator, value, LUA_CALLER_CONTEXT)?
                }
                ScriptOperatorNames::DisplayPrint => {
                    let ScriptValue::Reference(value) = lhs else {
                        return Err(InteropError::str("expected a native value"));
                    };
                    ScriptValue::String(dispatch::display(world, value, LUA_CALLER_CONTEXT)?.into())
                }
                _ => {
                    let rhs = script_value_at(l, 2)?;
                    dispatch::binary(world, operator, lhs, rhs, LUA_CALLER_CONTEXT)?
                }
            };
            ret.script_value(result)?;
            Ok(ret.n)
        })
    }
}

/// The metamethods of native values and their operators.
const OPERATORS: [(&std::ffi::CStr, ScriptOperatorNames); 11] = [
    (c"__add", ScriptOperatorNames::Addition),
    (c"__sub", ScriptOperatorNames::Subtraction),
    (c"__mul", ScriptOperatorNames::Multiplication),
    (c"__div", ScriptOperatorNames::Division),
    (c"__mod", ScriptOperatorNames::Remainder),
    (c"__pow", ScriptOperatorNames::Exponentiation),
    (c"__eq", ScriptOperatorNames::Equality),
    (c"__lt", ScriptOperatorNames::LessThanComparison),
    (c"__unm", ScriptOperatorNames::Negation),
    (c"__len", ScriptOperatorNames::Length),
    (c"__tostring", ScriptOperatorNames::DisplayPrint),
];

fn operator_from_index(index: ffi::lua_Integer) -> Result<ScriptOperatorNames, InteropError> {
    [
        ScriptOperatorNames::Addition,
        ScriptOperatorNames::Subtraction,
        ScriptOperatorNames::Multiplication,
        ScriptOperatorNames::Division,
        ScriptOperatorNames::Remainder,
        ScriptOperatorNames::Exponentiation,
        ScriptOperatorNames::Equality,
        ScriptOperatorNames::LessThanComparison,
        ScriptOperatorNames::Negation,
        ScriptOperatorNames::Length,
        ScriptOperatorNames::DisplayPrint,
    ]
    .into_iter()
    .find(|op| *op as ffi::lua_Integer == index)
    .ok_or_else(|| InteropError::str("unknown operator"))
}

/// Runs a slow path, raising its errors.
unsafe fn slow_path(
    l: *mut ffi::lua_State,
    f: impl FnOnce(*mut ffi::lua_State) -> Result<c_int, InteropError>,
) -> c_int {
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| f(l)));
    let error = match result {
        Ok(Ok(n)) => return n,
        Ok(Err(error)) => error,
        Err(_) => InteropError::str("a native value's metamethod panicked"),
    };
    // Safety: see `call_holder`
    unsafe { raise(l, error) }
}

/// Whether a type has a native form, i.e. to convert values of it.
#[allow(dead_code)]
pub(crate) fn is_native(type_id: TypeId) -> bool {
    ThreadWorldContainer::try_get_context()
        .ok()
        .map(|c| {
            c.world
                .script_function_registry()
                .read()
                .native_type(type_id)
                .is_some()
        })
        .unwrap_or(false)
}

/// The base of a native value held by mlua, if `value` is one.
pub(crate) fn native_value_base(lua: &Lua, value: &mlua::AnyUserData) -> Option<*mut u8> {
    // Safety: the value is pushed and popped by `exec_raw`'s stack guard
    unsafe {
        lua.exec_raw::<Option<mlua::LightUserData>>(value, |l| match native_base(l, -1) {
            Some(base) => ffi::lua_pushlightuserdata(l, base as *mut c_void),
            None => ffi::lua_pushnil(l),
        })
    }
    .ok()
    .flatten()
    .map(|p| p.0 as *mut u8)
}
