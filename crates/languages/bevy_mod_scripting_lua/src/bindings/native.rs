//! Typed calls and native values in Lua, see [`bevy_mod_scripting_bindings::native`].
//!
//! - Functions with a typed entry are raw C closures ([`native_function`]) reading their arguments straight off the stack
//!   ([`LuaArgs`]). Arguments stay on the stack for the whole call, so the values borrowed from them can't be collected.
//! - Native values are raw userdata holding a [`NativeStorage`], with one metatable per type, made on first use: fields are read
//!   and written with the type's typed accessors, methods are the functions registered on the type, everything else goes
//!   through a reference to a copy of the value (the slow path). Natives are told apart by a marker in their metatable,
//!   which scripts can't reach (`__metatable` is `false`), see [`native_base`].
//!
//! Lua errors are `longjmp`s (C++ exceptions on Luau), which must not skip Rust destructors. So every Lua API call which can
//! raise runs while no Rust value with a destructor is alive in the frames it would skip: in protected mode (making
//! metatables and function holders, through mlua's `exec_raw`, which only borrow what they push), or after every Rust value
//! of the call is gone: results are kept as plain values ([`Ready`]) and pushed last, errors are raised last.
//!
//! Finalizers leave tombstones: a collected function holder is marked dead and a collected native value loses its metatable,
//! so a script finalizer running afterwards gets an error instead of a freed value.
//!
//! # Reading the stack code
//!
//! - **Bottom and top**: the bottom of a C function's stack is its first argument, index `1`. The top is the last value
//!   pushed. Diagrams list the stack bottom → top, left to right, `..` being values below which the code doesn't touch:
//!   ```text
//!   [v, key, mt]   v at 1 (the bottom), mt at 3 = -1 (the top)
//!   ```
//! - **Absolute indices** are positive, counted up from the bottom: they keep pointing at the same value when values are
//!   pushed or popped above it. **Relative indices** are negative, counted down from the top (`-1` = the top, `-2` = just
//!   below it): they move with every push and pop. Helpers taking an index and pushing before using it turn it absolute
//!   first (`lua_absindex`).
//! - **Pseudo-indices** aren't on the stack: `LUA_REGISTRYINDEX` (the registry table) and `lua_upvalueindex(n)` (upvalue
//!   `n` of the running C closure).
//! - **Effects** are written like the Lua manual's: `[-o, +p, x]` pops `o` values, pushes `p`, and `x` is `-` (never
//!   raises), `m` (raises on memory errors only) or `e` (raises any error). Every function which can raise must be called
//!   where raising skips no Rust destructor, see above.
//! - **Room**: a C function starts with `LUA_MINSTACK` (20) free slots. Code which may push more, or runs on a stack it
//!   didn't get called on, calls `lua_checkstack` first.
#![allow(unsafe_code)]

use std::{
    borrow::Cow,
    ffi::c_void,
    mem::{ManuallyDrop, MaybeUninit},
    os::raw::{c_char, c_int},
    panic::{AssertUnwindSafe, catch_unwind},
};

use bevy_mod_scripting_bindings::{
    DynamicScriptFunction, InteropError, ReflectReference, ScriptValue, WorldExtensions, dispatch,
    native::{
        ArgSource, DirectField, DirectKind, NativeStorage, NativeType, NativeVTable, ReturnSink,
    },
};
use bevy_mod_scripting_bindings_domain::ScriptOperatorNames;
use bevy_mod_scripting_world::ThreadWorldContainer;
use mlua::{FromLua, IntoLua, IntoLuaMulti, Lua, MultiValue, ffi, state::RawLua};

use super::script_value::{
    LUA_CALLER_CONTEXT, LuaFunctionReturn, LuaScriptValue, add_call_location,
};
use crate::{IntoInteropError, IntoMluaError};

#[cfg(any(feature = "luajit", feature = "luajit52"))]
pub(crate) mod cdata;

/// Its address keys the marker in the metatables of native values, see [`native_base`]
static NATIVE_MARKER: u8 = 0;
/// Its address keys the metatable of function holders in the registry
static HOLDER_METATABLE: u8 = 0;

fn key(p: &'static u8) -> *mut c_void {
    p as *const u8 as *mut c_void
}

/// Pushes `t[p]`, `t` being the table at `idx` (any index, pseudo-indices included), and returns its type.
/// `[-0, +1, -]`: raw gets don't raise.
///
/// ```text
/// [.., t, ..]  ->  [.., t, .., t[p]]
/// ```
unsafe fn raw_get_p(l: *mut ffi::lua_State, idx: c_int, p: *mut c_void) -> c_int {
    unsafe {
        let idx = ffi::lua_absindex(l, idx);
        ffi::lua_pushlightuserdata(l, p);
        ffi::lua_rawget(l, idx)
    }
}

/// `t[p] = v`, `t` being the table at `idx` and `v` the top, which is popped. `[-1, +0, m]`: protected mode only.
///
/// ```text
/// [.., t, .., v]  ->  [.., t, ..]
/// ```
unsafe fn raw_set_p(l: *mut ffi::lua_State, idx: c_int, p: *mut c_void) {
    unsafe {
        // absolute: the push below would shift a relative index
        let idx = ffi::lua_absindex(l, idx);
        ffi::lua_pushlightuserdata(l, p); // [.., t, .., v, p]
        ffi::lua_insert(l, -2); // [.., t, .., p, v]
        ffi::lua_rawset(l, idx); // [.., t, ..]
    }
}

/// The alignment of storage and holders, Lua only guarantees its own maximum alignment for userdata, so blocks are
/// allocated with room to round up to it.
const ALIGN: usize = NativeStorage::ALIGN;

fn align_up(block: *mut u8) -> *mut u8 {
    let padding = (ALIGN - (block as usize & (ALIGN - 1))) & (ALIGN - 1);
    block.wrapping_add(padding)
}

/// The base of the native value at `idx`, if it is one: a userdata whose metatable holds the native marker.
///
/// The marker's key is a light userdata, which Lua code can't make, and scripts can't reach the metatables of native values
/// (`__metatable` is `false`) to copy it, so only BMS' metatables hold it. (The `debug` library reaches everything anyway.)
///
/// `[-0, +0, -]`, but needs 2 free slots, see `lua_checkstack` at callers which aren't plain C function entries.
pub(crate) unsafe fn native_base(l: *mut ffi::lua_State, idx: c_int) -> Option<*mut u8> {
    unsafe {
        if ffi::lua_type(l, idx) != ffi::LUA_TUSERDATA {
            return None;
        }
        let block = ffi::lua_touserdata(l, idx) as *mut u8;
        // [.., v, ..] -> [.., v, .., mt], nothing pushed without a metatable
        if block.is_null() || ffi::lua_getmetatable(l, idx) == 0 {
            return None;
        }
        // [.., v, .., mt, mt[marker]]
        let native = raw_get_p(l, -1, key(&NATIVE_MARKER)) != ffi::LUA_TNIL;
        // [.., v, ..]
        ffi::lua_pop(l, 2);
        native.then(|| align_up(block))
    }
}

/// Runs `f` with mlua's view of the state and the stack it works on, which may be another thread than `l`'s.
///
/// mlua's stack (`raw.state()`) is NOT `l`'s when `l` is a coroutine: indices of `l` must not be used on it, values are
/// moved between the two with `lua_xmove`, see [`at_index`] and [`push_with_mlua`].
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
        with_raw(l, |_, raw| value_at(raw, l, idx))
    }
}

/// The value at `idx` of `l` as a script value through mlua, which looks for native values too (after references).
unsafe fn value_at(
    raw: &RawLua,
    l: *mut ffi::lua_State,
    idx: c_int,
) -> Result<ScriptValue, InteropError> {
    unsafe { at_index::<LuaScriptValue>(raw, l, idx) }
        .map(|v| v.0)
        .map_err(IntoInteropError::to_bms_error)
}

/// The value at `idx` of `l` converted by mlua, which works on the stack of `raw`'s state. `[-0, +0, m]` on `l`.
///
/// When mlua's stack isn't `l`'s, the value is copied over for the conversion:
/// ```text
/// l:     [.., v, ..]  -> pushvalue -> [.., v, .., v]  -> xmove -> [.., v, ..]
/// state: [..]                                         -> xmove -> [.., v]  -> converted, popped -> [..]
/// ```
unsafe fn at_index<T: FromLua>(
    raw: &RawLua,
    l: *mut ffi::lua_State,
    idx: c_int,
) -> mlua::Result<T> {
    unsafe {
        // absolute: `l`'s top changes below
        let idx = ffi::lua_absindex(l, idx);
        let state = raw.state();
        if state == l {
            T::from_stack(idx, raw)
        } else if ffi::lua_checkstack(state, 1) == 0 {
            Err(mlua::Error::StackError)
        } else {
            ffi::lua_pushvalue(l, idx);
            ffi::lua_xmove(l, state, 1);
            // the top of mlua's stack
            let value = T::from_stack(-1, raw);
            ffi::lua_pop(state, 1);
            value
        }
    }
}

/// Pushes the `n` values `push` pushes on mlua's stack onto `l`, returns `n`. `[-0, +n, m]` on `l`.
///
/// ```text
/// state: [..] -> push -> [.., r1 .. rn]  -> xmove (if another stack) -> [..]
/// l:     [..]                            -> xmove (if another stack) -> [.., r1 .. rn]
/// ```
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

/// The arguments of a typed call: `len` values of the C function's stack from the absolute index `first` up.
///
/// ```text
/// [.., arg 0 (first), arg 1 (first + 1), .., arg len-1 (first + len - 1)]
/// ```
/// Argument `i` (0-based, as [`ArgSource`] counts) is at absolute index `first + i`, see [`LuaArgs::idx`]. Nothing is pushed
/// or popped below them during the call, so the indices stay valid.
pub(crate) struct LuaArgs<'a> {
    l: *mut ffi::lua_State,
    /// absolute index of argument 0
    first: c_int,
    len: usize,
    /// mlua, locked once for the call when an argument goes the slow way (else locked if one does anyway)
    raw: Option<&'a RawLua>,
    /// whether the first arguments are native values, looked up once per call (`Union` and `R` both ask)
    natives: [std::cell::Cell<Option<Option<*mut u8>>>; 4],
}

impl LuaArgs<'_> {
    /// The absolute stack index of argument `i` (0-based).
    fn idx(&self, i: usize) -> c_int {
        self.first + i as c_int
    }
}

// Safety: native bases are only handed out for userdata with a native metatable, which were initialised before getting it
// (see `push_ready`) and stay on the stack for the call
unsafe impl ArgSource for LuaArgs<'_> {
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
        let look_up = || unsafe { native_base(self.l, self.idx(i)) };
        let Some(cached) = self.natives.get(i) else {
            return look_up();
        };
        if let Some(native) = cached.get() {
            return native;
        }
        let native = look_up();
        cached.set(Some(native));
        native
    }

    #[cfg(any(feature = "luajit", feature = "luajit52"))]
    fn plain(&self, i: usize, type_id: std::any::TypeId) -> Option<(*mut u8, bool)> {
        // Safety: the index is on the stack, the cdata stays there for the call
        unsafe { cdata::cdata_at(self.l, self.idx(i)) }
            .filter(|(vtable, _, _)| vtable.type_id == type_id)
            .map(|(_, payload, read_only)| (payload, read_only))
    }

    fn string(&self, i: usize) -> Option<&str> {
        // Safety: the index is on the stack, the string stays there (or in its table) for the call
        unsafe {
            if ffi::lua_type(self.l, self.idx(i)) != ffi::LUA_TSTRING {
                return None;
            }
            let mut len = 0;
            let s = ffi::lua_tolstring(self.l, self.idx(i), &mut len);
            std::str::from_utf8(std::slice::from_raw_parts(s.cast(), len)).ok()
        }
    }

    fn list(&self, i: usize, items: &mut dyn FnMut(&dyn ArgSource)) -> bool {
        let l = self.l;
        let idx = self.idx(i);
        // Safety: the index is on the stack; the items are pushed above every argument and stay there until the call returns,
        // so native items stay alive for borrows; raw gets and `next` over an unchanged table don't raise
        unsafe {
            if ffi::lua_type(l, idx) != ffi::LUA_TTABLE || ffi::lua_checkstack(l, 2) == 0 {
                return false;
            }
            // a sequence, exactly the keys `1..=len`, as tables become lists (see `TableBuilder`)
            let len = ffi::lua_rawlen(l, idx);
            let mut count = 0;
            ffi::lua_pushnil(l);
            while ffi::lua_next(l, idx) != 0 {
                // [.., key, value] -> [.., key]
                ffi::lua_pop(l, 1);
                let key =
                    (ffi::lua_type(l, -1) == ffi::LUA_TNUMBER).then(|| ffi::lua_tonumber(l, -1));
                if !key.is_some_and(|k| k.fract() == 0.0 && k >= 1.0 && k <= len as ffi::lua_Number)
                {
                    ffi::lua_pop(l, 1);
                    return false;
                }
                count += 1;
            }
            if count != len || ffi::lua_checkstack(l, len as c_int) == 0 {
                return false;
            }
            let first = ffi::lua_gettop(l) + 1;
            for j in 1..=len {
                ffi::lua_rawgeti(l, idx, j as ffi::lua_Integer);
            }
            items(&LuaArgs {
                l,
                first,
                len,
                raw: self.raw,
                natives: Default::default(),
            });
        }
        true
    }

    fn map(
        &self,
        i: usize,
        entry: &mut dyn FnMut(&dyn ArgSource) -> Result<(), InteropError>,
    ) -> Option<Result<(), InteropError>> {
        let l = self.l;
        let idx = self.idx(i);
        // Safety: the index is on the stack, entries are pushed above every argument; `next` over an unchanged table doesn't
        // raise; values are popped after their entry, so natives aren't handed out (see `NoNatives`)
        unsafe {
            if ffi::lua_type(l, idx) != ffi::LUA_TTABLE || ffi::lua_checkstack(l, 3) == 0 {
                return None;
            }
            ffi::lua_pushnil(l);
            while ffi::lua_next(l, idx) != 0 {
                // [.., key, value]: string keys only, like the map a table becomes
                if ffi::lua_type(l, -2) != ffi::LUA_TSTRING {
                    ffi::lua_pop(l, 2);
                    return None;
                }
                let args = LuaArgs {
                    l,
                    first: ffi::lua_gettop(l) - 1,
                    len: 2,
                    raw: self.raw,
                    natives: Default::default(),
                };
                let read = entry(&NoNatives(&args));
                // [.., key]: the key for `next`
                ffi::lua_pop(l, 1);
                if let Err(error) = read {
                    ffi::lua_pop(l, 1);
                    return Some(Err(error));
                }
            }
        }
        Some(Ok(()))
    }

    fn script_value(&self, i: usize) -> Result<ScriptValue, InteropError> {
        // a native value found by a typed argument, else mlua, which finds natives after references (the common case)
        if let Some(Some(Some(base))) = self.natives.get(i).map(std::cell::Cell::get) {
            // Safety: a native base handed out above, on the stack for the call
            return unsafe { NativeStorage::to_script_value(base) };
        }
        // Safety: the index is on the stack
        unsafe {
            match self.raw {
                Some(raw) => value_at(raw, self.l, self.idx(i)),
                None => with_raw(self.l, |_, raw| value_at(raw, self.l, self.idx(i))),
            }
        }
    }
}

/// Arguments which don't stay on the stack for the whole call (the entries of a map), so their native values aren't handed
/// out: they're copied into references like other values (see [`ArgSource::script_value`]).
struct NoNatives<'a>(&'a LuaArgs<'a>);

// Safety: never hands out native values
unsafe impl ArgSource for NoNatives<'_> {
    fn len(&self) -> usize {
        self.0.len()
    }
    fn number(&self, i: usize) -> Option<f64> {
        self.0.number(i)
    }
    fn integer(&self, i: usize) -> Option<i64> {
        self.0.integer(i)
    }
    fn boolean(&self, i: usize) -> Option<bool> {
        self.0.boolean(i)
    }
    fn native(&self, _i: usize) -> Option<*mut u8> {
        None
    }
    fn script_value(&self, i: usize) -> Result<ScriptValue, InteropError> {
        self.0.script_value(i)
    }
    fn string(&self, i: usize) -> Option<&str> {
        self.0.string(i)
    }
    // nested lists and maps would hand out natives of values which don't stay on the stack: the slow path
}

// ---------------------------------------------------------------- results

/// Room for the native values most types need without a heap allocation.
#[repr(C, align(16))]
struct InlineBuffer([MaybeUninit<u8>; 64]);

// handles to components (`t.translation`) are made on every field access: they must fit without a heap allocation
const _: () = assert!(
    size_of::<bevy_mod_scripting_bindings::native::NativePlace>() <= size_of::<InlineBuffer>()
        && align_of::<bevy_mod_scripting_bindings::native::NativePlace>()
            <= align_of::<InlineBuffer>()
);

/// A native value made by a call, waiting to be pushed: plain bytes, so a Lua error raised while pushing it skips no
/// destructor (the value is leaked then).
struct PendingNative {
    vtable: &'static NativeVTable,
    /// null when the value is in `inline`
    heap: *mut u8,
    inline: InlineBuffer,
    /// pushed as a cdata (LuaJIT, types with a C layout), see `cdata`
    #[cfg_attr(not(any(feature = "luajit", feature = "luajit52")), allow(dead_code))]
    cdata: bool,
    /// a read-only copy, see [`ReturnSink::read_only`]
    read_only: bool,
}

impl PendingNative {
    fn ptr(&mut self) -> *mut u8 {
        match self.heap.is_null() {
            true => self.inline.0.as_mut_ptr() as *mut u8,
            false => self.heap,
        }
    }

    /// Frees the heap memory, the value having been moved out of it.
    unsafe fn free(&mut self) {
        if !self.heap.is_null() && self.vtable.layout.size() != 0 {
            // Safety: allocated in `LuaReturn::native` with this layout
            unsafe { std::alloc::dealloc(self.heap, self.vtable.layout) };
            self.heap = std::ptr::null_mut();
        }
    }

    /// Drops the value, which won't be pushed.
    unsafe fn discard(mut self) {
        // Safety: written by the call, not moved out
        unsafe {
            if let Some(drop) = self.vtable.drop {
                drop(self.ptr());
            }
            self.free();
        }
    }
}

enum Pending {
    Nothing,
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Native(PendingNative),
    Reference(ReflectReference),
    Value(ScriptValue),
}

/// The result of a typed call, kept until the call is over, see [`LuaReturn::prepare`].
pub(crate) struct LuaReturn {
    pending: Pending,
    /// Whether the native value returned next is a read-only copy, see [`ReturnSink::read_only`]
    read_only: bool,
}

impl LuaReturn {
    fn new() -> Self {
        Self {
            pending: Pending::Nothing,
            read_only: false,
        }
    }

    /// Makes what pushing the result needs (mlua values, the native metatable), while Rust values may still be around.
    ///
    /// `[-0, +(0|1), -]`: a native result leaves its metatable on the top for [`push_ready`] (`[..] -> [.., mt]`), others
    /// push nothing. Raising is avoided: the metatable is made in protected mode.
    unsafe fn prepare(self, l: *mut ffi::lua_State) -> Result<Ready, InteropError> {
        // Safety: `l` is the state of the call
        unsafe {
            match self.pending {
                // only script values need mlua
                Pending::Value(_) => with_raw(l, |lua, _| self.prepare_with(l, lua)),
                _ => self.prepare_plain(l),
            }
        }
    }

    /// [`Self::prepare`] of results which aren't script values: no mlua needed.
    unsafe fn prepare_plain(self, l: *mut ffi::lua_State) -> Result<Ready, InteropError> {
        Ok(match self.pending {
            Pending::Nothing | Pending::Value(_) => Ready::Nothing,
            Pending::Number(n) => Ready::Number(n),
            Pending::Integer(i) => Ready::Integer(i),
            Pending::Boolean(b) => Ready::Boolean(b),
            // the function making the cdata, left on the stack for `push_ready`; a userdata if there's none
            // Safety: `l` is the state of the call
            #[cfg(any(feature = "luajit", feature = "luajit52"))]
            Pending::Native(native)
                if native.cdata
                    && unsafe { cdata::push_from_ptr(l, native.vtable, native.read_only) }
                        .unwrap_or(false) =>
            {
                Ready::Cdata(native)
            }
            Pending::Native(native) => {
                // left on the stack for `push_ready`
                // Safety: `l` is the state of the call
                if let Err(e) = unsafe { push_native_metatable(l, native.vtable) } {
                    // Safety: never pushed
                    unsafe { native.discard() };
                    return Err(e);
                }
                Ready::Native(native)
            }
            Pending::Reference(reference) => {
                // left on the stack for `push_ready`
                // Safety: `l` is the state of the call
                unsafe { push_reference_metatable(l, reference.owns_heap()) }?;
                Ready::Reference(reference)
            }
        })
    }

    /// [`Self::prepare`] with mlua already at hand.
    unsafe fn prepare_with(self, l: *mut ffi::lua_State, lua: &Lua) -> Result<Ready, InteropError> {
        Ok(match self.pending {
            // a tuple of plain values (i.e. the results of a query step), pushed without mlua when allocating can't raise
            Pending::Value(ScriptValue::Tuple(values))
                if values.0.iter().all(Plain::is_plain) && unlikely_memory_error(lua) =>
            {
                for gc in [false, true] {
                    if values
                        .0
                        .iter()
                        .any(|v| matches!(v, ScriptValue::Reference(r) if r.owns_heap() == gc))
                    {
                        // Safety: `l` is the state of the call, the metatable is popped
                        unsafe {
                            push_reference_metatable(l, gc)?;
                            ffi::lua_pop(l, 1);
                        }
                    }
                }
                Ready::Plain(values.0.into_iter().map(Plain::from).collect())
            }
            // a tuple is several values, anything else one, without a `MultiValue`
            Pending::Value(value @ ScriptValue::Tuple(_)) => Ready::Values(
                LuaFunctionReturn(value)
                    .into_lua_multi(lua)
                    .map_err(IntoInteropError::to_bms_error)?,
            ),
            Pending::Value(value) => Ready::Value(
                LuaScriptValue(value)
                    .into_lua(lua)
                    .map_err(IntoInteropError::to_bms_error)?,
            ),
            // Safety: `l` is the state of the call
            _ => return unsafe { self.prepare_plain(l) },
        })
    }

    /// Drops a result which won't be pushed.
    fn discard(self) {
        if let Pending::Native(native) = self.pending {
            // Safety: never pushed
            unsafe { native.discard() }
        }
    }
}

// Safety: `native` calls `write` exactly once with memory for the vtable's layout, which it keeps and drops or pushes later
unsafe impl ReturnSink for LuaReturn {
    fn unit(&mut self) {}

    fn number(&mut self, n: f64) {
        self.pending = Pending::Number(n);
    }

    fn integer(&mut self, i: i64) {
        self.pending = Pending::Integer(i);
    }

    fn boolean(&mut self, b: bool) {
        self.pending = Pending::Boolean(b);
    }

    unsafe fn native(
        &mut self,
        native: &std::sync::Arc<NativeType>,
        vtable: &'static NativeVTable,
        write: &mut dyn FnMut(*mut u8),
    ) -> bool {
        let cdata = cfg!(any(feature = "luajit", feature = "luajit52"))
            && native.ffi.is_some()
            && !vtable.place;
        let layout = vtable.layout;
        let heap = if layout.size() <= size_of::<InlineBuffer>()
            && layout.align() <= align_of::<InlineBuffer>()
        {
            std::ptr::null_mut()
        } else if layout.size() == 0 {
            // zero sized and aligned more than the buffer: any aligned pointer, never freed (see `free`)
            std::ptr::without_provenance_mut(layout.align())
        } else {
            // Safety: not zero sized
            let heap = unsafe { std::alloc::alloc(layout) };
            if heap.is_null() {
                return false;
            }
            heap
        };
        self.pending = Pending::Native(PendingNative {
            vtable,
            heap,
            inline: InlineBuffer([MaybeUninit::uninit(); 64]),
            cdata,
            // a place is the value in the world, never a copy
            read_only: self.read_only && !vtable.place,
        });
        if let Pending::Native(native) = &mut self.pending {
            write(native.ptr());
        }
        true
    }

    fn script_value(&mut self, value: ScriptValue) -> Result<(), InteropError> {
        self.pending = match value {
            // pushed without mlua, see `push_reference_with_metatable`
            ScriptValue::Reference(reference) => Pending::Reference(reference),
            value => Pending::Value(value),
        };
        Ok(())
    }

    fn read_only(&mut self) {
        self.read_only = true;
    }
}

/// A value pushed without mlua, see [`Ready::Plain`].
enum Plain {
    Nil,
    Boolean(bool),
    Integer(i64),
    Number(f64),
    Reference(ReflectReference),
}

impl Plain {
    fn is_plain(value: &ScriptValue) -> bool {
        matches!(
            value,
            ScriptValue::Unit
                | ScriptValue::Bool(_)
                | ScriptValue::Integer(_)
                | ScriptValue::Float(_)
                | ScriptValue::Reference(_)
        )
    }
}

impl From<ScriptValue> for Plain {
    fn from(value: ScriptValue) -> Self {
        match value {
            ScriptValue::Bool(b) => Plain::Boolean(b),
            ScriptValue::Integer(i) => Plain::Integer(i),
            ScriptValue::Float(n) => Plain::Number(n),
            ScriptValue::Reference(r) => Plain::Reference(r),
            // only plain values, see `is_plain`
            _ => Plain::Nil,
        }
    }
}

/// A result ready to be pushed, see [`push_ready`].
enum Ready {
    Nothing,
    Number(f64),
    Integer(i64),
    Boolean(bool),
    Native(PendingNative),
    /// A value of a type with a C layout, the function making its cdata on the top, see [`cdata::push_from_ptr`]
    #[cfg(any(feature = "luajit", feature = "luajit52"))]
    Cdata(PendingNative),
    /// A reference, its metatable on the top, see [`LuaReturn::prepare`]
    Reference(ReflectReference),
    /// Several values pushed without mlua, only made when allocating can't raise (see [`unlikely_memory_error`]): the
    /// values not pushed yet are alive while the others are
    Plain(Vec<Plain>),
    Value(mlua::Value),
    Values(MultiValue),
}

/// Pushes a result, after every other Rust value of the call is gone: allocating the userdata of a native value can raise.
///
/// `[-0, +n, m]`, returns `n`, the number of results. A native result expects its metatable on the top (see
/// [`LuaReturn::prepare`]), which it takes:
/// ```text
/// [.., mt] -> new_userdata -> [.., mt, v] -> insert(-2) -> [.., v, mt] -> setmetatable(-2) -> [.., v]
/// ```
unsafe fn push_ready(l: *mut ffi::lua_State, ready: Ready) -> Result<c_int, &'static str> {
    unsafe {
        match ready {
            Ready::Nothing => Ok(0),
            Ready::Number(n) => {
                ffi::lua_pushnumber(l, n);
                Ok(1)
            }
            Ready::Integer(i) => {
                ffi::lua_pushinteger(l, i as ffi::lua_Integer);
                Ok(1)
            }
            Ready::Boolean(b) => {
                ffi::lua_pushboolean(l, b as c_int);
                Ok(1)
            }
            Ready::Native(mut native) => {
                if ffi::lua_checkstack(l, 2) == 0 {
                    native.discard();
                    return Err("stack overflow");
                }
                let vtable = native.vtable;
                // only plain values are alive in this frame: if this raises, the value is leaked, nothing is skipped
                let block = new_userdata(
                    l,
                    NativeStorage::size(vtable) + ALIGN - 1,
                    vtable.drop.is_some().then_some(Dtor::Native),
                );
                let base = align_up(block);
                let source = native.ptr();
                let size = vtable.layout.size();
                // moved bitwise into the storage, before the metatable (the only thing which can drop it) is set
                NativeStorage::init(base, vtable, &mut |slot| {
                    std::ptr::copy_nonoverlapping(source, slot, size)
                });
                NativeStorage::header(base).read_only.set(native.read_only);
                native.free();
                // the metatable `LuaReturn::prepare` left below the value: [.., mt, v] -> [.., v, mt]
                ffi::lua_insert(l, -2);
                // pops mt into v's metatable: [.., v]
                ffi::lua_setmetatable(l, -2);
                Ok(1)
            }
            #[cfg(any(feature = "luajit", feature = "luajit52"))]
            Ready::Cdata(mut native) => {
                if ffi::lua_checkstack(l, 1) == 0 {
                    native.discard();
                    return Err("stack overflow");
                }
                // [.., from_ptr, p] -> [.., v]: the value is copied, plain bytes (no destructor), so raising leaks nothing
                // but the heap block of big values
                ffi::lua_pushlightuserdata(l, native.ptr() as *mut c_void);
                ffi::lua_call(l, 1, 1);
                native.free();
                Ok(1)
            }
            Ready::Reference(reference) => {
                if ffi::lua_checkstack(l, 1) == 0 {
                    return Err("stack overflow");
                }
                // the metatable `LuaReturn::prepare` left on the top
                push_reference_with_metatable(l, reference);
                Ok(1)
            }
            Ready::Plain(values) => {
                let n = values.len() as c_int;
                // a reference needs its metatable below it while it's made
                if ffi::lua_checkstack(l, n + 1) == 0 {
                    return Err("stack overflow");
                }
                for value in values {
                    match value {
                        Plain::Nil => ffi::lua_pushnil(l),
                        Plain::Boolean(b) => ffi::lua_pushboolean(l, b as c_int),
                        Plain::Integer(i) => ffi::lua_pushinteger(l, i as ffi::lua_Integer),
                        Plain::Number(n) => ffi::lua_pushnumber(l, n),
                        Plain::Reference(reference) => {
                            let gc = if reference.owns_heap() {
                                &REFERENCE_METATABLE_GC
                            } else {
                                &REFERENCE_METATABLE
                            };
                            // made when the result was prepared
                            raw_get_p(l, ffi::LUA_REGISTRYINDEX, key(gc));
                            push_reference_with_metatable(l, reference);
                        }
                    }
                }
                Ok(n)
            }
            Ready::Value(value) => {
                if ffi::lua_checkstack(l, 1) == 0 {
                    return Err("stack overflow");
                }
                push_with_mlua(l, move |_, raw| {
                    if ffi::lua_checkstack(raw.state(), 1) == 0 {
                        return Err(mlua::Error::StackError);
                    }
                    // an existing value: pushing it doesn't allocate
                    value.push_into_stack(raw)?;
                    Ok(1)
                })
                .map_err(|_| "pushing the result failed")
            }
            Ready::Values(values) => {
                let n = values.len() as c_int;
                if ffi::lua_checkstack(l, n) == 0 {
                    return Err("stack overflow");
                }
                push_with_mlua(l, move |_, raw| {
                    if ffi::lua_checkstack(raw.state(), n) == 0 {
                        return Err(mlua::Error::StackError);
                    }
                    // existing values: pushing them doesn't allocate
                    for value in values {
                        value.push_into_stack(raw)?;
                    }
                    Ok(n)
                })
                .map_err(|_| "pushing the results failed")
            }
        }
    }
}

/// Which destructor Luau runs for a userdata, other Lua versions use the `__gc` of its metatable.
#[derive(Clone, Copy)]
enum Dtor {
    Native,
    Holder,
    Reference,
}

/// Pushes a new userdata of `size` bytes and returns its block. `[-0, +1, m]`.
unsafe fn new_userdata(l: *mut ffi::lua_State, size: usize, dtor: Option<Dtor>) -> *mut u8 {
    #[cfg(feature = "luau")]
    unsafe {
        match dtor {
            Some(Dtor::Native) => {
                return ffi::lua_newuserdatadtor(l, size, luau_native_dtor) as *mut u8;
            }
            Some(Dtor::Holder) => {
                return ffi::lua_newuserdatadtor(l, size, luau_holder_dtor) as *mut u8;
            }
            Some(Dtor::Reference) => {
                return ffi::lua_newuserdatadtor(l, size, luau_reference_dtor) as *mut u8;
            }
            None => {}
        }
    }
    let _ = dtor;
    unsafe { ffi::lua_newuserdata(l, size) as *mut u8 }
}

#[cfg(feature = "luau")]
unsafe extern "C" fn luau_native_dtor(_l: *mut ffi::lua_State, block: *mut c_void) {
    // Safety: only for native values, initialised right after they're made
    let _ = catch_unwind(|| unsafe { NativeStorage::drop_value(align_up(block as *mut u8)) });
}

#[cfg(feature = "luau")]
unsafe extern "C" fn luau_holder_dtor(_l: *mut ffi::lua_State, block: *mut c_void) {
    // Safety: only for holders, initialised right after they're made
    unsafe { drop_holder(align_up(block as *mut u8)) }
}

/// `__gc` of native values: drops the value and takes its metatable, so it's no native value anymore.
///
/// Called with the collected value as the only argument: `[v]`.
#[cfg(not(feature = "luau"))]
unsafe extern "C-unwind" fn native_gc(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        if let Some(base) = native_base(l, 1) {
            // a panicking drop must not unwind through the collector
            let _ = catch_unwind(|| NativeStorage::drop_value(base));
            // [v, nil] -> setmetatable pops nil -> [v], without a metatable
            ffi::lua_pushnil(l);
            ffi::lua_setmetatable(l, 1);
        }
    }
    0
}

/// `__gc` of function holders: drops the function and marks the holder dead.
///
/// Called with the collected holder as the only argument: `[h]`.
#[cfg(not(feature = "luau"))]
unsafe extern "C-unwind" fn holder_gc(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        // [h] -> [h, mt]
        if ffi::lua_type(l, 1) != ffi::LUA_TUSERDATA || ffi::lua_getmetatable(l, 1) == 0 {
            return 0;
        }
        // [h, mt, holder_mt]: only BMS' holders have the registry's holder metatable
        raw_get_p(l, ffi::LUA_REGISTRYINDEX, key(&HOLDER_METATABLE));
        let holder = ffi::lua_rawequal(l, -1, -2) != 0;
        // [h]
        ffi::lua_pop(l, 2);
        if holder {
            drop_holder(align_up(ffi::lua_touserdata(l, 1) as *mut u8));
        }
    }
    0
}

// ---------------------------------------------------------------- calls

const HOLDER_ALIVE: usize = 0;
const HOLDER_DEAD: usize = 1;

/// Function holders: a function and whether it was collected already.
#[repr(C)]
struct Holder {
    state: usize,
    function: ManuallyDrop<DynamicScriptFunction>,
}

// holders are written at `align_up(block)`
const _: () = assert!(align_of::<Holder>() <= ALIGN);

/// The function of a holder, `None` once it was collected.
unsafe fn holder_function<'a>(block: *mut c_void) -> Option<&'a DynamicScriptFunction> {
    // Safety: holders are `Holder`s, initialised when they're made
    let holder = unsafe { &*(align_up(block as *mut u8) as *const Holder) };
    (holder.state == HOLDER_ALIVE).then(|| &*holder.function)
}

/// Drops the function of a holder, once.
unsafe fn drop_holder(base: *mut u8) {
    // Safety: holders are `Holder`s, initialised when they're made
    let holder = unsafe { &mut *(base as *mut Holder) };
    if holder.state == HOLDER_ALIVE {
        holder.state = HOLDER_DEAD;
        // Safety: alive until now, never used again
        let function = unsafe { ManuallyDrop::take(&mut holder.function) };
        let _ = catch_unwind(AssertUnwindSafe(|| drop(function)));
    }
}

/// Pushes a holder of a clone of the function. `[-0, +1, m]`: protected mode only.
///
/// Callers keep their Rust values outside of protected mode: a raise skips the destructors of this frame and its callers'.
unsafe fn push_holder(l: *mut ffi::lua_State, function: &DynamicScriptFunction) {
    unsafe {
        // [.., h]
        let block = new_userdata(l, size_of::<Holder>() + ALIGN - 1, Some(Dtor::Holder));
        // cloned once the userdata exists, nothing to drop if making it raises
        (align_up(block) as *mut Holder).write(Holder {
            state: HOLDER_ALIVE,
            function: ManuallyDrop::new(function.clone()),
        });
        // [.., h, holder_mt | nil]
        if raw_get_p(l, ffi::LUA_REGISTRYINDEX, key(&HOLDER_METATABLE)) == ffi::LUA_TNIL {
            // first holder of this state: [.., h] -> [.., h, holder_mt]
            ffi::lua_pop(l, 1);
            ffi::lua_createtable(l, 0, 2);
            #[cfg(not(feature = "luau"))]
            {
                ffi::lua_pushcfunction(l, holder_gc);
                ffi::lua_setfield(l, -2, c"__gc".as_ptr());
            }
            ffi::lua_pushboolean(l, 0);
            ffi::lua_setfield(l, -2, c"__metatable".as_ptr());
            // [.., h, holder_mt, holder_mt] -> the copy goes to the registry -> [.., h, holder_mt]
            ffi::lua_pushvalue(l, -1);
            raw_set_p(l, ffi::LUA_REGISTRYINDEX, key(&HOLDER_METATABLE));
        }
        // pops holder_mt into h's metatable: [.., h]
        ffi::lua_setmetatable(l, -2);
    }
}

/// A Lua function calling the typed entry of `function`, see the [module docs](self): a C closure of [`native_entry`] whose
/// upvalue 1 is the holder of `function`.
pub(crate) fn native_function(
    lua: &Lua,
    function: DynamicScriptFunction,
) -> mlua::Result<mlua::Function> {
    // Safety: protected mode, the stack guard of `exec_raw` keeps the stack balanced, `function` is dropped outside of it
    unsafe {
        lua.exec_raw((), |l| {
            // [h]
            push_holder(l, &function);
            // pops h into upvalue 1: [f]
            ffi::lua_pushcclosure(l, native_entry, 1);
        })
    }
}

/// Calls the function held by upvalue 1 with the whole stack as arguments: `[a1 (1), .., an (n = top)]`.
unsafe extern "C-unwind" fn native_entry(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        let Some(function) = holder_function(ffi::lua_touserdata(l, ffi::lua_upvalueindex(1)))
        else {
            raise_str(l, c"the function was collected")
        };
        // the index of the top is the number of values
        let len = ffi::lua_gettop(l) as usize;
        call_holder(l, function, 1, len)
    }
}

/// Calls `function` with the `len` values from the absolute index `first` up as arguments, pushes its results and returns
/// how many, raises its errors. `[-0, +n, e]`, for C functions to return directly: Lua takes the top `n` values as results.
///
/// ```text
/// [.., a1 (first), .., alen]  ->  [.., a1, .., alen, r1 .. rn]
/// ```
unsafe fn call_holder(
    l: *mut ffi::lua_State,
    function: &DynamicScriptFunction,
    first: c_int,
    len: usize,
) -> c_int {
    // Safety: the arguments are on the stack
    unsafe { run(l, || call(l, function, first, len)) }
}

/// Runs `f`, then pushes its result or raises its error, once nothing of it is left to drop. `[-0, +n, e]`, returns `n`.
unsafe fn run(l: *mut ffi::lua_State, f: impl FnOnce() -> Result<Ready, InteropError>) -> c_int {
    let ready = match catch_unwind(AssertUnwindSafe(f)) {
        Ok(Ok(ready)) => ready,
        // Safety: the error is moved into the raised value
        Ok(Err(error)) => unsafe { raise(l, error) },
        // Safety: the payload is moved into the raised value
        Err(payload) => unsafe { resume_panic(l, payload) },
    };
    // Safety: only the result is left, it's plain values
    match unsafe { push_ready(l, ready) } {
        Ok(n) => n,
        // Safety: nothing left to drop
        Err(_) => unsafe { raise_str(l, c"failed to push the results of a native function") },
    }
}

/// Raises a panic the way mlua does for its callbacks: by panicking in one, so it's resumed once it reaches Rust (a
/// script's `pcall` catches it, as it does mlua's). `[-0, +1, e]`, never returns: `[..] -> [.., callback]`, called.
unsafe fn resume_panic(l: *mut ffi::lua_State, payload: Box<dyn std::any::Any + Send>) -> ! {
    unsafe {
        if ffi::lua_checkstack(l, 1) == 0 {
            // nothing may be left to drop when raising
            drop(payload);
            raise_str(l, c"a native function panicked")
        }
        let payload = std::cell::Cell::new(Some(payload));
        // the closure takes the payload, it's dropped by mlua either way
        let failed = push_with_mlua(l, move |lua, raw| {
            lua.create_function(move |_, ()| match payload.take() {
                Some(payload) => std::panic::resume_unwind(payload),
                None => Ok(()),
            })?
            .push_into_stack(raw)?;
            Ok(1)
        })
        .is_err();
        if !failed {
            // raises the panic, only plain values are alive in this frame
            ffi::lua_call(l, 0, 0);
        }
        raise_str(l, c"a native function panicked")
    }
}

/// Calls `function` with the `len` values from the absolute index `first` up as arguments (see [`LuaArgs`]).
///
/// `[-0, +(0|1), -]`: returns the result to push; a native result's metatable is left on the top, see
/// [`LuaReturn::prepare`]. The arguments stay where they are.
unsafe fn call(
    l: *mut ffi::lua_State,
    function: &DynamicScriptFunction,
    first: c_int,
    len: usize,
) -> Result<Ready, InteropError> {
    let natives: [std::cell::Cell<Option<Option<*mut u8>>>; 4] = Default::default();
    // whether an argument goes the slow way (anything but numbers, booleans, nil and natives): mlua is then locked once
    // for the whole call instead of once per argument (the lock is reentrant, callbacks into Lua lock it again)
    let needs_mlua = (0..len).any(|i| {
        let idx = first + i as c_int;
        // Safety: the index is on the stack
        match unsafe { ffi::lua_type(l, idx) } {
            ffi::LUA_TNUMBER | ffi::LUA_TBOOLEAN | ffi::LUA_TNIL => false,
            ffi::LUA_TUSERDATA => {
                // Safety: see above
                let native = unsafe { native_base(l, idx) };
                if let Some(cached) = natives.get(i) {
                    cached.set(Some(native));
                }
                native.is_none()
            }
            _ => true,
        }
    });
    let run = |lua: Option<&Lua>, raw: Option<&RawLua>| {
        let args = LuaArgs {
            l,
            first,
            len,
            raw,
            natives,
        };
        let mut ret = LuaReturn::new();
        let result = function.call_with(&args, &mut ret, LUA_CALLER_CONTEXT);
        match result {
            // Safety: `l` is the state of the call
            Ok(()) => unsafe {
                match lua {
                    Some(lua) => ret.prepare_with(l, lua),
                    None => ret.prepare(l),
                }
            },
            Err(error) => {
                ret.discard();
                Err(error)
            }
        }
    };
    match needs_mlua {
        // Safety: `l` is the state of the call
        true => unsafe { with_raw(l, |lua, raw| run(Some(lua), Some(raw))) },
        false => run(None, None),
    }
}

/// Raises `error` as a Lua error. `[-0, +1, e]`, never returns: `[..] -> [.., error]`, raised.
unsafe fn raise(l: *mut ffi::lua_State, mut error: InteropError) -> ! {
    unsafe {
        let line = caller_line(l);
        let failed = push_with_mlua(l, move |lua, raw| {
            add_call_location(lua, &mut error, || line);
            error.to_lua_error().push_into_stack(raw)?;
            Ok(1)
        })
        .is_err();
        if failed {
            raise_str(l, c"failed to raise an error of a native function")
        }
        ffi::lua_error(l)
    }
}

/// Raises `message` as a Lua error. `[-0, +1, e]`, never returns: `[..] -> [.., message]`, raised.
unsafe fn raise_str(l: *mut ffi::lua_State, message: &'static std::ffi::CStr) -> ! {
    unsafe {
        ffi::lua_pushstring(l, message.as_ptr());
        ffi::lua_error(l)
    }
}

/// The line of the script calling the C function running on `l`, which may be a coroutine mlua isn't on.
unsafe fn caller_line(l: *mut ffi::lua_State) -> Option<u32> {
    unsafe {
        let mut ar: ffi::lua_Debug = std::mem::zeroed();
        #[cfg(feature = "luau")]
        let found = ffi::lua_getinfo(l, 1, c"l".as_ptr(), &mut ar) != 0;
        #[cfg(not(feature = "luau"))]
        let found = ffi::lua_getstack(l, 1, &mut ar) != 0
            && ffi::lua_getinfo(l, c"l".as_ptr(), &mut ar) != 0;
        found.then_some(ar.currentline.max(0) as u32)
    }
}

// ---------------------------------------------------------------- native values

/// Pushes the metatable of native values of `vtable`'s type, made in protected mode if this Lua state has none yet.
///
/// `[-0, +1, -]` on success (`[..] -> [.., mt]`), `[-0, +0, -]` on error. Metatables live in the registry, keyed by the
/// vtable's address.
unsafe fn push_native_metatable(
    l: *mut ffi::lua_State,
    vtable: &'static NativeVTable,
) -> Result<(), InteropError> {
    let vtable_key = vtable as *const NativeVTable as *mut c_void;
    unsafe {
        // [.., mt | nil]
        if raw_get_p(l, ffi::LUA_REGISTRYINDEX, vtable_key) != ffi::LUA_TNIL {
            return Ok(());
        }
        // [..]
        ffi::lua_pop(l, 1);
    }
    let needs_drop = vtable.drop.is_some();
    let world = ThreadWorldContainer::try_get_context()?.world;
    let (fields, methods) = {
        let registry = world.script_function_registry();
        let registry = registry.read();
        let native = registry.native_type(vtable.type_id).ok_or_else(|| {
            InteropError::string(format!("{} is not a native type", vtable.type_name))
        })?;
        let fields = native
            .fields
            .iter()
            .map(|(name, field)| {
                (
                    name.clone(),
                    field.get.clone(),
                    field.set.clone(),
                    // places are in the world: their fields are read and written through the accessors
                    field.direct.filter(|_| !vtable.place),
                )
            })
            .collect::<Vec<_>>();
        let methods = native
            .methods
            .iter()
            .map(|(name, function)| (name.clone(), function.clone()))
            .collect::<Vec<_>>();
        (fields, methods)
    };
    // Safety: protected mode, the stack guard of `exec_raw` keeps the stack balanced, the functions are dropped outside of it
    unsafe {
        with_raw(l, |lua, _| {
            lua.exec_raw::<()>((), |state| {
                build_native_metatable(state, vtable_key, needs_drop, &fields, &methods)
            })
        })
    }
    .map_err(IntoInteropError::to_bms_error)?;
    // [.., mt]
    // Safety: a raw get, made above
    unsafe { raw_get_p(l, ffi::LUA_REGISTRYINDEX, vtable_key) };
    Ok(())
}

type Fields = Vec<(
    Cow<'static, str>,
    DynamicScriptFunction,
    Option<DynamicScriptFunction>,
    Option<DirectField>,
)>;

/// A [`DirectField`] as a Lua integer, kept in the getters and setters of a metatable: `offset << 3 | kind`.
fn direct_code(field: DirectField) -> ffi::lua_Integer {
    let kind = match field.kind {
        DirectKind::F32 => 0,
        DirectKind::F64 => 1,
        DirectKind::I32 => 2,
        DirectKind::U32 => 3,
        DirectKind::Bool => 4,
    };
    ((field.offset as ffi::lua_Integer) << 3) | kind
}

/// The field of the native value at 1 a [`direct_code`] stands for: whether the value is read-only, the field's address
/// and kind.
///
/// # Safety
/// The value at 1 is a native value of the type the code was made for.
unsafe fn direct_field(l: *mut ffi::lua_State, code: ffi::lua_Integer) -> (bool, *mut u8, u8) {
    unsafe {
        let base = align_up(ffi::lua_touserdata(l, 1) as *mut u8);
        // the userdata is on the stack for the call
        let read_only = NativeStorage::header(base).read_only.get();
        let field = NativeStorage::value(base).add((code >> 3) as usize);
        (read_only, field, (code & 7) as u8)
    }
}

/// Pushes the direct field `code` of the native value at 1, see [`DirectField`]. `[-0, +1, -]`.
///
/// # Safety
/// The value at 1 is a native value of the type the code was made for.
unsafe fn direct_get(l: *mut ffi::lua_State, code: ffi::lua_Integer) {
    unsafe {
        let (_, field, kind) = direct_field(l, code);
        // the same values the getter returns: floats as numbers, integers as integers
        match kind {
            0 => ffi::lua_pushnumber(l, *(field as *const f32) as ffi::lua_Number),
            1 => ffi::lua_pushnumber(l, *(field as *const f64) as ffi::lua_Number),
            2 => ffi::lua_pushinteger(l, *(field as *const i32) as ffi::lua_Integer),
            3 => ffi::lua_pushinteger(l, *(field as *const u32) as ffi::lua_Integer),
            _ => ffi::lua_pushboolean(l, *(field as *const bool) as c_int),
        }
    }
}

/// Writes the value at 3 to the direct field `code` of the native value at 1, see [`DirectField`]. `[-0, +0, -]`: false
/// (nothing written) if the value is read-only or the new value isn't a number (a boolean for booleans), which the setter
/// handles.
///
/// # Safety
/// The value at 1 is a native value of the type the code was made for.
unsafe fn direct_set(l: *mut ffi::lua_State, code: ffi::lua_Integer) -> bool {
    unsafe {
        let (read_only, field, kind) = direct_field(l, code);
        if read_only {
            return false;
        }
        if kind == 4 {
            if ffi::lua_type(l, 3) != ffi::LUA_TBOOLEAN {
                return false;
            }
            *(field as *mut bool) = ffi::lua_toboolean(l, 3) != 0;
            return true;
        }
        if ffi::lua_type(l, 3) != ffi::LUA_TNUMBER {
            return false;
        }
        // like the setter: integers first, then numbers, truncated with `as`
        #[cfg(any(feature = "lua54", feature = "lua53", feature = "luau"))]
        if ffi::lua_isinteger(l, 3) != 0 {
            let i = ffi::lua_tointeger(l, 3) as i64;
            match kind {
                0 => *(field as *mut f32) = i as f32,
                1 => *(field as *mut f64) = i as f64,
                2 => *(field as *mut i32) = i as i32,
                _ => *(field as *mut u32) = i as u32,
            }
            return true;
        }
        let n = ffi::lua_tonumber(l, 3) as f64;
        match kind {
            0 => *(field as *mut f32) = n as f32,
            1 => *(field as *mut f64) = n,
            2 => *(field as *mut i32) = n as i32,
            _ => *(field as *mut u32) = n as u32,
        }
        true
    }
}

/// Makes the metatable of a native type and keeps it in the registry (`registry[vtable_key] = mt`).
/// `[-0, +0, m]`: protected mode only, so it owns no Rust value.
///
/// Every step below leaves the stack as `[.., mt]`, `mt` on the top, so `-2` is `mt` after pushing a value to set in it.
unsafe fn build_native_metatable(
    l: *mut ffi::lua_State,
    vtable_key: *mut c_void,
    needs_drop: bool,
    fields: &Fields,
    methods: &[(Cow<'static, str>, DynamicScriptFunction)],
) {
    unsafe {
        // [.., mt]
        ffi::lua_createtable(l, 0, 16);
        // Luau drops through the userdata's destructor instead
        #[cfg(feature = "luau")]
        let _ = needs_drop;
        // values which need no drop need no finalizer either: their memory is only freed once nothing, a finalizer
        // included, can reach them, and finalizers make collecting slower
        #[cfg(not(feature = "luau"))]
        if needs_drop {
            ffi::lua_pushcfunction(l, native_gc);
            ffi::lua_setfield(l, -2, c"__gc".as_ptr());
        }
        // scripts can't get or change it
        ffi::lua_pushboolean(l, 0);
        ffi::lua_setfield(l, -2, c"__metatable".as_ptr());

        // the functions of the operators, i.e. `add` and the reflected `mul` for `2 * v`
        let typed = |name: &str| methods.iter().find(|(n, _)| n == name).map(|(_, f)| f);
        let operators = OPERATORS.map(|(meta, operator)| {
            (
                meta,
                operator,
                typed(operator.script_function_name()),
                operator.reflected_script_function_name().and_then(typed),
            )
        });

        // __index: getters, then methods, then the slow path
        // [.., mt, getters], each `getters[name] = holder`, or the field's `direct_code`
        ffi::lua_createtable(l, 0, fields.len() as c_int);
        for (name, get, _, direct) in fields {
            match direct {
                Some(direct) => ffi::lua_pushinteger(l, direct_code(*direct)),
                None => push_holder(l, get),
            }
            set_field(l, name);
        }
        // [.., mt, getters, methods], each `methods[name] = function`
        ffi::lua_createtable(l, 0, methods.len() as c_int);
        for (name, function) in methods {
            push_function(l, function);
            set_field(l, name);
        }
        // pops both into upvalues 1 and 2: [.., mt, __index]
        ffi::lua_pushcclosure(l, native_index, 2);
        // [.., mt]
        ffi::lua_setfield(l, -2, c"__index".as_ptr());

        // __newindex: setters, each `setters[name] = holder` or the field's `direct_code`, and the setters of direct fields
        // [.., mt, setters, accessors] -> pushcclosure -> [.., mt, __newindex] -> setfield -> [.., mt]
        ffi::lua_createtable(l, 0, fields.len() as c_int);
        for (name, _, set, direct) in fields {
            match (set, direct) {
                (Some(_), Some(direct)) => ffi::lua_pushinteger(l, direct_code(*direct)),
                (Some(set), None) => push_holder(l, set),
                (None, _) => continue,
            }
            set_field(l, name);
        }
        ffi::lua_createtable(l, 0, 0);
        for (name, _, set, direct) in fields {
            if let (Some(set), Some(_)) = (set, direct) {
                push_holder(l, set);
                set_field(l, name);
            }
        }
        ffi::lua_pushcclosure(l, native_newindex, 2);
        ffi::lua_setfield(l, -2, c"__newindex".as_ptr());

        // upvalues: the operator, its typed function and its typed reflected function, if any
        for (name, operator, forward, reflected) in operators {
            // [.., mt, operator, forward | nil, reflected | nil]
            ffi::lua_pushinteger(l, operator as ffi::lua_Integer);
            for function in [forward, reflected] {
                match function {
                    Some(function) => push_holder(l, function),
                    None => ffi::lua_pushnil(l),
                }
            }
            // pops the three into upvalues 1, 2, 3: [.., mt, metamethod] -> setfield -> [.., mt]
            ffi::lua_pushcclosure(l, native_operator, 3);
            // comparisons are shared by every metatable, which call the type's own, see `push_comparison`
            let name = match operator {
                ScriptOperatorNames::Equality => OWN_EQ,
                ScriptOperatorNames::LessThanComparison => OWN_LT,
                _ => name,
            };
            ffi::lua_setfield(l, -2, name.as_ptr());
        }
        for (name, op) in [(c"__eq", EQ), (c"__lt", LT), (c"__le", LE)] {
            push_comparison(l, op);
            ffi::lua_setfield(l, -2, name.as_ptr());
        }
        ffi::lua_pushstring(l, c"BMS native value".as_ptr());
        ffi::lua_setfield(l, -2, c"__name".as_ptr());

        // tells native values apart, see `native_base`: [.., mt, true] -> [.., mt]
        ffi::lua_pushboolean(l, 1);
        raw_set_p(l, -2, key(&NATIVE_MARKER));

        // pops mt into the registry: [..]
        raw_set_p(l, ffi::LUA_REGISTRYINDEX, vtable_key);
    }
}

/// `t[name] = v`, `v` being the top and `t` just below it; `v` is popped. `[-1, +0, m]`: protected mode only.
///
/// ```text
/// [.., t, v]  ->  [.., t]
/// ```
unsafe fn set_field(l: *mut ffi::lua_State, name: &str) {
    unsafe {
        // [.., t, v, name]
        ffi::lua_pushlstring(l, name.as_ptr() as *const c_char, name.len());
        // [.., t, name, v]
        ffi::lua_insert(l, -2);
        // [.., t]
        ffi::lua_rawset(l, -3);
    }
}

/// Pushes a registered function as Lua sees it: a [`native_entry`] closure over its holder. `[-0, +1, m]`: protected mode
/// only.
unsafe fn push_function(l: *mut ffi::lua_State, function: &DynamicScriptFunction) {
    unsafe {
        push_holder(l, function);
        ffi::lua_pushcclosure(l, native_entry, 1);
    }
}

/// A registered function as a value to return.
unsafe fn function_value(
    l: *mut ffi::lua_State,
    function: DynamicScriptFunction,
) -> Result<Ready, InteropError> {
    // Safety: `l` is the state of the call
    unsafe {
        with_raw(l, |lua, _| {
            let value = super::script_value::registered_function_into_lua(lua, function)?;
            Ok::<_, mlua::Error>(MultiValue::from_vec(vec![value]))
        })
    }
    .map(Ready::Values)
    .map_err(IntoInteropError::to_bms_error)
}

/// Pushes `cache[v][name]` if it's of the type `kind`: what was kept for the name on the value `v` by
/// [`cache_for_value`]. `cache` (a table with weak keys) is at an index which stays valid when values are pushed.
/// `[-0, +(0|1), -]`, needs 2 free slots.
///
/// Called with `[v (1), name (2), ..]`.
unsafe fn cached_for_value(l: *mut ffi::lua_State, cache: c_int, kind: c_int) -> bool {
    unsafe {
        let top = ffi::lua_gettop(l);
        // [.., cache[v]]
        ffi::lua_pushvalue(l, 1);
        if ffi::lua_rawget(l, cache) == ffi::LUA_TTABLE {
            // [.., cache[v], cache[v][name]]
            ffi::lua_pushvalue(l, 2);
            if ffi::lua_rawget(l, -2) == kind {
                ffi::lua_remove(l, -2);
                return true;
            }
        }
        ffi::lua_settop(l, top);
        false
    }
}

/// `cache[v][name] = result`, see [`cached_for_value`]. `[-0, +0, m]`, needs 3 free slots: called once no Rust value of
/// the call is left.
///
/// Called with `[v (1), name (2), result (3), ..]`.
unsafe fn cache_for_value(l: *mut ffi::lua_State, cache: c_int) {
    unsafe {
        let top = ffi::lua_gettop(l);
        // [.., cache[v]]
        ffi::lua_pushvalue(l, 1);
        if ffi::lua_rawget(l, cache) != ffi::LUA_TTABLE {
            ffi::lua_pop(l, 1);
            ffi::lua_createtable(l, 0, 1);
            // cache[v] = {}
            ffi::lua_pushvalue(l, 1);
            ffi::lua_pushvalue(l, -2);
            ffi::lua_rawset(l, cache);
        }
        // cache[v][name] = result
        ffi::lua_pushvalue(l, 2);
        ffi::lua_pushvalue(l, 3);
        ffi::lua_rawset(l, -3);
        ffi::lua_settop(l, top);
    }
}

/// `__index`, `v.name`: the getter in upvalue 1 (`getters`, read in place for direct fields), else the method in upvalue 2
/// (`methods`), else the slow path.
///
/// Called with `[v (1), name (2)]`.
unsafe extern "C-unwind" fn native_index(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        // [v, name, name] -> rawget pops the key -> [v, name, getters[name]]
        ffi::lua_pushvalue(l, 2);
        let getter = ffi::lua_rawget(l, ffi::lua_upvalueindex(1));
        if getter == ffi::LUA_TNUMBER {
            // a field read in place
            let code = ffi::lua_tointeger(l, -1);
            direct_get(l, code);
            return 1;
        }
        if getter == ffi::LUA_TUSERDATA {
            // kept alive by the table in the upvalue
            let getter = holder_function(ffi::lua_touserdata(l, -1));
            // [v, name]
            ffi::lua_settop(l, 2);
            let Some(getter) = getter else {
                raise_str(l, c"the getter was collected")
            };
            // the getter's only argument is v: [v, name, result]
            return call_holder(l, getter, 1, 1);
        }
        // [v, name] -> [v, name, methods[name]]
        ffi::lua_pop(l, 1);
        ffi::lua_pushvalue(l, 2);
        if ffi::lua_rawget(l, ffi::lua_upvalueindex(2)) != ffi::LUA_TNIL {
            // the method on the top is the result
            return 1;
        }
        // [v, name]
        ffi::lua_pop(l, 1);
        run(l, || {
            let receiver = script_value_at(l, 1)?;
            let key = script_value_at(l, 2)?;
            let ScriptValue::Reference(receiver) = receiver else {
                return Err(InteropError::str(
                    "a native value is indexed with a reference to it",
                ));
            };
            let world = ThreadWorldContainer::try_get_context()?.world;
            match dispatch::index(world, receiver, key, LUA_CALLER_CONTEXT)? {
                dispatch::Member::Method(function) => function_value(l, function),
                dispatch::Member::Static(value) => {
                    let mut ret = LuaReturn::new();
                    ret.script_value(value)?;
                    ret.prepare(l)
                }
            }
        })
    }
}

/// `__newindex`, `v.name = x`: the setter in upvalue 1 (`setters`), called with `(v, x)`.
///
/// Called with `[v (1), name (2), x (3)]`.
unsafe extern "C-unwind" fn native_newindex(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        // [v, name, x, name] -> rawget pops the key -> [v, name, x, setters[name]]
        ffi::lua_pushvalue(l, 2);
        let mut setter = ffi::lua_rawget(l, ffi::lua_upvalueindex(1));
        if setter == ffi::LUA_TNUMBER {
            // a field written in place, else by its setter in upvalue 2 (`accessors`)
            let code = ffi::lua_tointeger(l, -1);
            if direct_set(l, code) {
                return 0;
            }
            // [v, name, x, accessors[name]]
            ffi::lua_pop(l, 1);
            ffi::lua_pushvalue(l, 2);
            setter = ffi::lua_rawget(l, ffi::lua_upvalueindex(2));
        }
        if setter == ffi::LUA_TUSERDATA {
            // kept alive by the table in the upvalue
            let setter = holder_function(ffi::lua_touserdata(l, -1));
            // [v, name, x]
            ffi::lua_pop(l, 1);
            let Some(setter) = setter else {
                raise_str(l, c"the setter was collected")
            };
            // [v, x]: the setter's arguments
            ffi::lua_remove(l, 2);
            return call_holder(l, setter, 1, 2);
        }
        // [v, name, x]
        ffi::lua_pop(l, 1);
        run(l, || {
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
/// Upvalue 1 is the operator (its index in [`OPERATORS`]), typed functions are in upvalues 2 (the operator's) and 3 (its
/// reflected one, for a native value on the right), each a holder or nil.
///
/// Called with `[a (1), b (2)]` for binary operators, `[a (1), ..]` for unary ones (some Lua versions pass `a` twice).
unsafe extern "C-unwind" fn native_operator(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        let operator = ffi::lua_tointeger(l, ffi::lua_upvalueindex(1));
        let typed = |upvalue: c_int| {
            (ffi::lua_type(l, ffi::lua_upvalueindex(upvalue)) == ffi::LUA_TUSERDATA)
                .then(|| holder_function(ffi::lua_touserdata(l, ffi::lua_upvalueindex(upvalue))))
                .flatten()
        };
        match operator_from_index(operator) {
            Ok(ScriptOperatorNames::DisplayPrint) | Err(_) => {}
            Ok(ScriptOperatorNames::Negation | ScriptOperatorNames::Length) => {
                if native_base(l, 1).is_some()
                    && let Some(function) = typed(2)
                {
                    // [a]
                    ffi::lua_settop(l, 1);
                    return call_holder(l, function, 1, 1);
                }
            }
            Ok(_) => {
                if native_base(l, 1).is_some() {
                    if let Some(function) = typed(2) {
                        // [a, b] -> f(a, b)
                        ffi::lua_settop(l, 2);
                        return call_holder(l, function, 1, 2);
                    }
                } else if native_base(l, 2).is_some()
                    && let Some(function) = typed(3)
                {
                    // the native value is the receiver of the reflected function:
                    // [a, b] -> insert(1) moves the top to the bottom -> [b, a] -> reflected(b, a)
                    ffi::lua_settop(l, 2);
                    ffi::lua_insert(l, 1);
                    return call_holder(l, function, 1, 2);
                }
            }
        }
        run(l, || {
            let operator = operator_from_index(operator)?;
            let world = ThreadWorldContainer::try_get_context()?.world;
            let lhs = script_value_at(l, 1)?;
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
            let mut ret = LuaReturn::new();
            ret.script_value(result)?;
            ret.prepare(l)
        })
    }
}

/// The metamethod of an operator of references, see [`reference_operator`].
pub(crate) fn reference_operator_function(operator: ScriptOperatorNames) -> ffi::lua_CFunction {
    use ScriptOperatorNames::*;
    macro_rules! of {
        ($($op:ident),*) => {
            match operator {
                $($op => reference_operator::<{ $op as ffi::lua_Integer }>,)*
                // raises "unknown operator"
                _ => reference_operator::<-1>,
            }
        };
    }
    of!(
        Addition,
        Subtraction,
        Multiplication,
        Division,
        Remainder,
        Exponentiation,
        Equality,
        LessThanComparison,
        Negation,
        Length
    )
}

// References: `ReflectReference`s in raw userdata with BMS metatables (no mlua userdata: no box, no metatable lookup by
// type), told apart like native values by a marker in their metatable. References owning nothing to free (whole
// components and resources) get a metatable without a finalizer, which makes collecting them cheaper.

/// Its address keys the marker in the metatables of references, see [`reference_at`].
static REFERENCE_MARKER: u8 = 0;
/// Its address keys the metatable of references which own nothing to free in the registry.
static REFERENCE_METATABLE: u8 = 0;
/// Its address keys the metatable of references with a finalizer in the registry.
static REFERENCE_METATABLE_GC: u8 = 0;

/// The bytes of the userdata of a reference: the reference, aligned by arithmetic like native values.
const REFERENCE_SIZE: usize = size_of::<ReflectReference>() + ALIGN - 1;
const _: () = assert!(align_of::<ReflectReference>() <= ALIGN);

/// The reference at `idx`, if it's one: a userdata whose metatable holds the reference marker. Valid while the value is
/// on the stack. `[-0, +0, -]`, but needs 2 free slots.
pub(crate) unsafe fn reference_at<'a>(
    l: *mut ffi::lua_State,
    idx: c_int,
) -> Option<&'a ReflectReference> {
    unsafe {
        if ffi::lua_type(l, idx) != ffi::LUA_TUSERDATA {
            return None;
        }
        let block = ffi::lua_touserdata(l, idx) as *mut u8;
        // [.., r, .., mt], nothing pushed without a metatable (i.e. once collected)
        if block.is_null() || ffi::lua_getmetatable(l, idx) == 0 {
            return None;
        }
        let reference = raw_get_p(l, -1, key(&REFERENCE_MARKER)) != ffi::LUA_TNIL;
        ffi::lua_pop(l, 2);
        // Safety: only BMS' reference metatables hold the marker, set on userdata initialised with a reference
        reference.then(|| &*(align_up(block) as *const ReflectReference))
    }
}

pub(crate) fn reference_value(lua: &Lua, value: &mlua::AnyUserData) -> Option<ReflectReference> {
    // pushing an existing value and the lookup don't raise: no protected call
    // Safety: room is checked, the value is popped right after the reference is cloned
    lua.exec_raw_lua(|raw| unsafe {
        let l = raw.state();
        if ffi::lua_checkstack(l, 3) == 0 || value.push_into_stack(raw).is_err() {
            return None;
        }
        let reference = reference_at(l, -1).cloned();
        ffi::lua_pop(l, 1);
        reference
    })
}

/// Pushes the metatable of references (with a finalizer if `gc`), made in protected mode if this Lua state has none
/// yet. `[-0, +1, -]` on success, `[-0, +0, -]` on error.
unsafe fn push_reference_metatable(l: *mut ffi::lua_State, gc: bool) -> Result<(), InteropError> {
    let mt_key = key(if gc {
        &REFERENCE_METATABLE_GC
    } else {
        &REFERENCE_METATABLE
    });
    unsafe {
        if raw_get_p(l, ffi::LUA_REGISTRYINDEX, mt_key) != ffi::LUA_TNIL {
            return Ok(());
        }
        ffi::lua_pop(l, 1);
        with_raw(l, |lua, _| {
            lua.exec_raw::<()>((), |state| build_reference_metatable(state, mt_key, gc))
        })
        .map_err(IntoInteropError::to_bms_error)?;
        raw_get_p(l, ffi::LUA_REGISTRYINDEX, mt_key);
    }
    Ok(())
}

/// Makes a metatable of references and keeps it in the registry. `[-0, +0, m]`: protected mode only.
unsafe fn build_reference_metatable(l: *mut ffi::lua_State, mt_key: *mut c_void, gc: bool) {
    unsafe {
        // [.., mt]
        ffi::lua_createtable(l, 0, 24);
        #[cfg(feature = "luau")]
        let _ = gc;
        #[cfg(not(feature = "luau"))]
        if gc {
            ffi::lua_pushcfunction(l, reference_gc);
            ffi::lua_setfield(l, -2, c"__gc".as_ptr());
        }
        // scripts can't get or change it
        ffi::lua_pushboolean(l, 0);
        ffi::lua_setfield(l, -2, c"__metatable".as_ptr());
        let functions: [(&std::ffi::CStr, ffi::lua_CFunction); 3] = [
            (c"__index", reference_index),
            (c"__newindex", reference_newindex),
            (c"__tostring", reference_tostring),
        ];
        for (name, function) in functions {
            ffi::lua_pushcfunction(l, function);
            ffi::lua_setfield(l, -2, name.as_ptr());
        }
        #[cfg(any(
            feature = "lua54",
            feature = "lua53",
            feature = "lua52",
            feature = "luajit52",
        ))]
        {
            ffi::lua_pushcfunction(l, reference_pairs);
            ffi::lua_setfield(l, -2, c"__pairs".as_ptr());
        }
        // operators call the operator's function with the operands as they are on the stack, see `reference_operator`
        for (name, operator) in [
            (c"__add", ScriptOperatorNames::Addition),
            (c"__sub", ScriptOperatorNames::Subtraction),
            (c"__mul", ScriptOperatorNames::Multiplication),
            (c"__div", ScriptOperatorNames::Division),
            (c"__mod", ScriptOperatorNames::Remainder),
            (c"__pow", ScriptOperatorNames::Exponentiation),
            (c"__unm", ScriptOperatorNames::Negation),
            (c"__len", ScriptOperatorNames::Length),
        ] {
            ffi::lua_pushcfunction(l, reference_operator_function(operator));
            ffi::lua_setfield(l, -2, name.as_ptr());
        }
        // comparisons are the functions native values share, which also compare references
        for (name, op) in [(c"__eq", EQ), (c"__lt", LT), (c"__le", LE)] {
            push_comparison(l, op);
            ffi::lua_setfield(l, -2, name.as_ptr());
        }
        ffi::lua_pushstring(l, c"ReflectReference".as_ptr());
        ffi::lua_setfield(l, -2, c"__name".as_ptr());
        // tells references apart, see `reference_at`
        ffi::lua_pushboolean(l, 1);
        raw_set_p(l, -2, key(&REFERENCE_MARKER));
        // pops mt into the registry: [..]
        raw_set_p(l, ffi::LUA_REGISTRYINDEX, mt_key);
    }
}

/// Pushes `reference` as a new userdata, its metatable being on the top (see [`push_reference_metatable`]), which it
/// takes: `[.., mt] -> [.., r]`, `[-1, +1, m]`. Only the reference is alive in this frame and it isn't dropped by it: if
/// allocating raises, it's leaked, nothing is skipped.
unsafe fn push_reference_with_metatable(l: *mut ffi::lua_State, reference: ReflectReference) {
    let gc = reference.owns_heap();
    let reference = ManuallyDrop::new(reference);
    unsafe {
        let block = new_userdata(l, REFERENCE_SIZE, gc.then_some(Dtor::Reference));
        (align_up(block) as *mut ReflectReference).write(ManuallyDrop::into_inner(reference));
        // [.., mt, r] -> [.., r, mt] -> setmetatable pops mt -> [.., r]
        ffi::lua_insert(l, -2);
        ffi::lua_setmetatable(l, -2);
    }
}

/// Whether allocating can't raise in this state, as mlua decides for its own userdata: mlua's allocator aborts on out of
/// memory instead of failing, unless a memory limit is set. LuaJIT allocates by itself, so it always can.
fn unlikely_memory_error(lua: &Lua) -> bool {
    #[cfg(any(feature = "luajit", feature = "luajit52"))]
    {
        let _ = lua;
        false
    }
    #[cfg(not(any(feature = "luajit", feature = "luajit52")))]
    // mlua has no getter for the limit: setting one returns the previous one, which is put back
    match lua.set_memory_limit(0) {
        Ok(0) => true,
        Ok(limit) => {
            let _ = lua.set_memory_limit(limit);
            false
        }
        Err(_) => false,
    }
}

pub(crate) fn reference_into_lua(
    lua: &Lua,
    reference: ReflectReference,
) -> mlua::Result<mlua::Value> {
    let gc = reference.owns_heap();
    let mt_key = key(if gc {
        &REFERENCE_METATABLE_GC
    } else {
        &REFERENCE_METATABLE
    });
    let unlikely_memory_error = unlikely_memory_error(lua);
    // the common case under one lock: the metatable exists (a raw get, which doesn't raise) and allocating can't raise
    // (like mlua's own userdata, no protected call then)
    // Safety: room is checked, the stack is left as it was
    let reference = lua.exec_raw_lua(|raw| unsafe {
        let l = raw.state();
        if ffi::lua_checkstack(l, 2) == 0 {
            return Err(Err(mlua::Error::StackError));
        }
        if raw_get_p(l, ffi::LUA_REGISTRYINDEX, mt_key) != ffi::LUA_TTABLE || !unlikely_memory_error
        {
            ffi::lua_pop(l, 1);
            return Ok(reference);
        }
        push_reference_with_metatable(l, reference);
        let value = mlua::Value::from_stack(-1, raw);
        ffi::lua_pop(l, 1);
        Err(value)
    });
    let reference = match reference {
        Ok(reference) => reference,
        Err(value) => return value,
    };
    // the first reference of this state, or allocating can raise: protected mode
    let mut reference = Some(reference);
    // Safety: the reference is moved into the userdata (`take` leaves nothing to drop in this frame), the stack is left
    // with the value only
    unsafe {
        lua.exec_raw::<mlua::Value>((), |l| {
            if ffi::lua_checkstack(l, 2) == 0 {
                raise_str(l, c"stack overflow");
            }
            if push_reference_metatable(l, gc).is_err() {
                raise_str(l, c"failed to make the metatable of references");
            }
            if let Some(reference) = reference.take() {
                push_reference_with_metatable(l, reference);
            }
        })
    }
}

/// `__gc` of references which own memory: drops the reference and takes the metatable, so it's no reference anymore.
///
/// Called with the collected value as the only argument: `[r]`.
#[cfg(not(feature = "luau"))]
unsafe extern "C-unwind" fn reference_gc(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        if let Some(reference) = reference_at(l, 1) {
            let reference = reference as *const ReflectReference as *mut ReflectReference;
            // a panicking drop must not unwind through the collector
            let _ = catch_unwind(AssertUnwindSafe(|| std::ptr::drop_in_place(reference)));
            ffi::lua_pushnil(l);
            ffi::lua_setmetatable(l, 1);
        }
    }
    0
}

#[cfg(feature = "luau")]
unsafe extern "C" fn luau_reference_dtor(_l: *mut ffi::lua_State, block: *mut c_void) {
    // Safety: only for references, initialised right after they're made
    let _ = catch_unwind(AssertUnwindSafe(|| unsafe {
        std::ptr::drop_in_place(align_up(block as *mut u8) as *mut ReflectReference)
    }));
}

/// The reference at 1 of a metamethod, or an error.
unsafe fn receiver(l: *mut ffi::lua_State) -> Result<ReflectReference, InteropError> {
    unsafe { reference_at(l, 1) }
        .cloned()
        .ok_or_else(|| InteropError::str("expected a reference"))
}

/// `__newindex` of references, `r.key = value`, see [`dispatch::set_index`]. Called with `[r (1), key (2), value (3)]`.
unsafe extern "C-unwind" fn reference_newindex(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        ffi::lua_settop(l, 3);
        run(l, || {
            let world = ThreadWorldContainer::try_get_context()?.world;
            let receiver = receiver(l)?;
            let (key, value) = with_raw(l, |_, raw| {
                Ok::<_, InteropError>((value_at(raw, l, 2)?, value_at(raw, l, 3)?))
            })?;
            dispatch::set_index(world, receiver, key, value, LUA_CALLER_CONTEXT)?;
            Ok(Ready::Nothing)
        })
    }
}

/// `__tostring` of references, see [`dispatch::display`]. Called with `[r (1)]`.
unsafe extern "C-unwind" fn reference_tostring(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        ffi::lua_settop(l, 1);
        run(l, || {
            let world = ThreadWorldContainer::try_get_context()?.world;
            let text = dispatch::display(world, receiver(l)?, LUA_CALLER_CONTEXT)?;
            with_raw(l, |lua, _| text.into_lua(lua))
                .map(Ready::Value)
                .map_err(IntoInteropError::to_bms_error)
        })
    }
}

/// `__pairs` of references, see [`dispatch::iter`]. Called with `[r (1)]`.
#[cfg(any(
    feature = "lua54",
    feature = "lua53",
    feature = "lua52",
    feature = "luajit52",
))]
unsafe extern "C-unwind" fn reference_pairs(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        ffi::lua_settop(l, 1);
        run(l, || {
            let world = ThreadWorldContainer::try_get_context()?.world;
            let iter = dispatch::iter(world, receiver(l)?, LUA_CALLER_CONTEXT)?;
            with_raw(l, |lua, _| LuaScriptValue(iter).into_lua(lua))
                .map(Ready::Value)
                .map_err(IntoInteropError::to_bms_error)
        })
    }
}

/// Its address keys the cache of the functions of registrations in the registry, see [`cached_registration_member`].
static REGISTRATION_MEMBERS: u8 = 0;

/// Pushes the cached function `name` of the registration `r`, i.e. `Vec3.new`, if there is one. `[-0, +(0|1), -]`.
///
/// Members of registrations depend on the registration (see [`dispatch::index_name`]), so they are cached per registration
/// value: `cache[r][name]`, the cache having weak keys. Kept for good: functions are registered before scripts run.
///
/// Called with `[r (1), name (2)]`.
unsafe fn cached_registration_member(l: *mut ffi::lua_State) -> bool {
    unsafe {
        // [r, name, cache]
        if raw_get_p(l, ffi::LUA_REGISTRYINDEX, key(&REGISTRATION_MEMBERS)) == ffi::LUA_TTABLE
            && cached_for_value(l, 3, ffi::LUA_TFUNCTION)
        {
            // [r, name, cache, function] -> [r, name, function]
            ffi::lua_replace(l, 3);
            return true;
        }
        ffi::lua_settop(l, 2);
        false
    }
}

/// Caches the function on the top as `name` of the registration `r`, see [`cached_registration_member`]. `[-0, +0, m]`:
/// called once no Rust value of the call is left.
///
/// Called with `[r (1), name (2), function (3)]`.
unsafe fn cache_registration_member(l: *mut ffi::lua_State) {
    unsafe {
        if ffi::lua_checkstack(l, 4) == 0 {
            return;
        }
        // [r, name, function, cache]
        if raw_get_p(l, ffi::LUA_REGISTRYINDEX, key(&REGISTRATION_MEMBERS)) != ffi::LUA_TTABLE {
            // the first one of this state: a table with weak keys
            ffi::lua_settop(l, 3);
            ffi::lua_newtable(l);
            ffi::lua_newtable(l);
            ffi::lua_pushstring(l, c"__mode".as_ptr());
            ffi::lua_pushstring(l, c"k".as_ptr());
            ffi::lua_rawset(l, -3);
            ffi::lua_setmetatable(l, -2);
            ffi::lua_pushvalue(l, -1);
            raw_set_p(l, ffi::LUA_REGISTRYINDEX, key(&REGISTRATION_MEMBERS));
        }
        cache_for_value(l, 4);
        ffi::lua_settop(l, 3);
    }
}

/// The Lua function of a function of a type or registration (`Vec3.new`): these live in the registry, so it's made once.
/// On LuaJIT it's the function's FFI form if it has one, see `cdata::ffi_static`.
fn static_function_value(lua: &Lua, function: DynamicScriptFunction) -> mlua::Result<mlua::Value> {
    #[cfg(any(feature = "luajit", feature = "luajit52"))]
    if let bevy_mod_scripting_bindings::function::namespace::FunctionScope::OnType(type_id) =
        function.info.namespace
        && let Some(value) = cdata::ffi_static(lua, type_id, &function.info.name)?
    {
        return Ok(value);
    }
    super::script_value::registered_function_into_lua(lua, function)
}

/// `__index` of types (`LuaTypeRef`, i.e. `world.spawn`), see [`dispatch::static_index`]: functions are cached per type
/// value like the functions of registrations, see [`cached_registration_member`].
///
/// Called with `[t (1), key (2)]`.
pub(super) unsafe extern "C-unwind" fn type_ref_index(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        ffi::lua_settop(l, 2);
        let named = ffi::lua_type(l, 2) == ffi::LUA_TSTRING;
        if named && cached_registration_member(l) {
            return 1;
        }
        let mut cache = false;
        let n = run(l, || {
            let world = ThreadWorldContainer::try_get_context()?.world;
            with_raw(l, |lua, raw| {
                let type_id =
                    at_index::<mlua::UserDataRef<super::reference::LuaTypeRef>>(raw, l, 1)
                        .map_err(IntoInteropError::to_bms_error)?
                        .0;
                let key = value_at(raw, l, 2)?;
                let value = match dispatch::static_index(world, type_id, key, LUA_CALLER_CONTEXT)? {
                    // functions of types live in the registry (bound ones are cached there too)
                    ScriptValue::Function(function) => {
                        cache = named;
                        static_function_value(lua, function)
                    }
                    member => LuaScriptValue(member).into_lua(lua),
                };
                value
                    .map(Ready::Value)
                    .map_err(IntoInteropError::to_bms_error)
            })
        });
        // [t, name, function]: nothing Rust is left
        if cache && n == 1 {
            cache_registration_member(l);
        }
        n
    }
}

/// `__index` of references, `r.name` (see [`dispatch::index_name`]) or `r[key]` (see [`dispatch::index`]): methods come
/// back unbound, `r:method()` passes the reference itself.
///
/// Called with `[r (1), key (2)]`.
pub(super) unsafe extern "C-unwind" fn reference_index(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        ffi::lua_settop(l, 2);
        // functions of registrations (`Vec3.new`) are cached per registration: found before anything else is looked up
        let registration = ffi::lua_type(l, 2) == ffi::LUA_TSTRING
            && reference_at(l, 1).is_some_and(|receiver| {
                receiver.reflect_path.is_empty()
                    && dispatch::members_depend_on_value(receiver.base.type_id())
            });
        if registration && cached_registration_member(l) {
            return 1;
        }
        let mut cache = false;
        let n = run(l, || {
            let world = ThreadWorldContainer::try_get_context()?.world;
            // one mlua call for the lookup and the result
            // on the stack for the call
            let receiver =
                reference_at(l, 1).ok_or_else(|| InteropError::str("expected a reference"))?;
            with_raw(l, |lua, raw| {
                let member = if ffi::lua_type(l, 2) == ffi::LUA_TSTRING {
                    cache = registration;
                    let mut len = 0;
                    let name = ffi::lua_tolstring(l, 2, &mut len);
                    // the string is kept alive by the stack for the call
                    let name = std::str::from_utf8(std::slice::from_raw_parts(name.cast(), len))
                        .map_err(|_| InteropError::str("member names must be valid UTF-8"))?;
                    dispatch::index_name(world, Cow::Borrowed(receiver), name, LUA_CALLER_CONTEXT)?
                } else {
                    let key = at_index::<LuaScriptValue>(raw, l, 2)
                        .map_err(IntoInteropError::to_bms_error)?;
                    dispatch::index(world, receiver.clone(), key.into(), LUA_CALLER_CONTEXT)?
                };
                // functions of registrations have FFI forms on LuaJIT, methods of references are called with references
                let registration = cache;
                let function_value = |function: DynamicScriptFunction| match registration {
                    true => static_function_value(lua, function),
                    false => super::script_value::registered_function_into_lua(lua, function),
                };
                let value = match member {
                    dispatch::Member::Method(function) => function_value(function),
                    dispatch::Member::Static(ScriptValue::Function(function)) if cache => {
                        function_value(function)
                    }
                    dispatch::Member::Static(value) => {
                        cache = false;
                        match value {
                            ScriptValue::Float(n) => return Ok(Ready::Number(n)),
                            ScriptValue::Integer(i) => return Ok(Ready::Integer(i)),
                            ScriptValue::Bool(b) => return Ok(Ready::Boolean(b)),
                            ScriptValue::Reference(reference) => {
                                // left on the stack for `push_ready`
                                push_reference_metatable(l, reference.owns_heap())?;
                                return Ok(Ready::Reference(reference));
                            }
                            // one value, tuples too
                            value => LuaScriptValue(value).into_lua(lua),
                        }
                    }
                };
                value
                    .map(Ready::Value)
                    .map_err(IntoInteropError::to_bms_error)
            })
        });
        // [r, name, function]: nothing Rust is left
        if cache && n == 1 {
            cache_registration_member(l);
        }
        n
    }
}

/// Operators of references, i.e. `t.translation + v` or `-t.translation`: the operator's function of the reference operand
/// (see [`dispatch::binary_function`]) is called with the operands as they are on the stack, the reference first.
///
/// Called with `[a (1), b (2)]` for binary operators, `[b, a]` when the reference is `b`; `[a (1), ..]` for unary ones.
/// Also called by [`comparison`] on its own stack.
unsafe extern "C-unwind" fn reference_operator<const OPERATOR: ffi::lua_Integer>(
    l: *mut ffi::lua_State,
) -> c_int {
    unsafe {
        let unary = matches!(
            operator_from_index(OPERATOR),
            Ok(ScriptOperatorNames::Negation | ScriptOperatorNames::Length)
        );
        // Lua passes the operand twice to unary metamethods: [a] for unary operators, [a, b] for binary ones
        let len = if unary { 1 } else { 2 };
        ffi::lua_settop(l, len);
        run(l, || {
            let operator = operator_from_index(OPERATOR)?;
            let world = ThreadWorldContainer::try_get_context()?.world;
            let (function, on_left) = {
                // on the stack for the call
                let reference = |idx| reference_at(l, idx);
                if unary {
                    let receiver =
                        reference(1).ok_or_else(|| InteropError::str("expected a reference"))?;
                    dispatch::unary_function(world, operator, receiver, LUA_CALLER_CONTEXT)
                        .map(|function| (function, true))
                } else {
                    let (receiver, on_left) = match reference(1) {
                        Some(receiver) => (receiver, true),
                        None => match reference(2) {
                            Some(receiver) => (receiver, false),
                            None => {
                                return Err(InteropError::str(
                                    "neither operand of this operator is a value",
                                ));
                            }
                        },
                    };
                    dispatch::binary_function(
                        world,
                        operator,
                        receiver,
                        on_left,
                        LUA_CALLER_CONTEXT,
                    )
                    .map(|function| (function, on_left))
                }
            }?;
            if !on_left {
                // the reference is the receiver, the first argument: [a, b] -> [b, a]
                ffi::lua_insert(l, 1);
            }
            call(l, &function, 1, len as usize)
        })
    }
}

/// Its addresses key the comparison functions in the registry, see [`push_comparison`].
static COMPARISON_KEYS: [u8; 3] = [0; 3];
/// `==`, see [`push_comparison`]
pub(crate) const EQ: usize = 0;
/// `<`
pub(crate) const LT: usize = 1;
/// `<=`
pub(crate) const LE: usize = 2;
/// Where a native type's own comparisons are in its metatable, `__eq` and `__lt` are the shared ones.
const OWN_EQ: &std::ffi::CStr = c"bms_eq";
const OWN_LT: &std::ffi::CStr = c"bms_lt";

/// Pushes the comparison `op` (`EQ`, `LT`, `LE`): one function per Lua state, the `__eq`, `__lt` and `__le` of every native
/// metatable and of references. Lua 5.1, 5.2, LuaJIT and Luau only call them when both operands have the same one, i.e.
/// a component's handle and a value of its type, which have different metatables.
/// `[-0, +1, m]`: protected mode only.
pub(crate) unsafe fn push_comparison(l: *mut ffi::lua_State, op: usize) {
    unsafe {
        let key = key(&COMPARISON_KEYS[op.min(LE)]);
        // [.., f | nil]
        if raw_get_p(l, ffi::LUA_REGISTRYINDEX, key) == ffi::LUA_TFUNCTION {
            return;
        }
        // first use in this state: [..] -> [.., f, f] -> the copy goes to the registry -> [.., f]
        ffi::lua_pop(l, 1);
        let function: ffi::lua_CFunction = match op {
            EQ => comparison::<EQ>,
            LT => comparison::<LT>,
            _ => comparison::<LE>,
        };
        ffi::lua_pushcfunction(l, function);
        ffi::lua_pushvalue(l, -1);
        raw_set_p(l, ffi::LUA_REGISTRYINDEX, key);
    }
}

/// Compares with the native operand's own comparison (the first operand's, as Lua 5.4 would), or as references when neither
/// operand is native. `a <= b` is `not (b < a)`.
///
/// Called with `[a (1), b (2)]`, returns the top: `[x, y, .., result]`.
unsafe extern "C-unwind" fn comparison<const OP: usize>(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        // [a, b]; below, `x` and `y` are the operands as compared: `a`, `b`, swapped for `<=`
        ffi::lua_settop(l, 2);
        if OP == LE {
            // `b < a`: [a, b] -> [b, a]
            ffi::lua_insert(l, 1);
        }
        match [1, 2].into_iter().find(|i| native_base(l, *i).is_some()) {
            Some(native) => {
                // its metatable has the type's comparisons: [x, y, mt]
                ffi::lua_getmetatable(l, native);
                // [x, y, mt, own_key] -> rawget -> [x, y, mt, own] -> remove(-2) -> [x, y, own]
                ffi::lua_pushstring(l, if OP == EQ { OWN_EQ } else { OWN_LT }.as_ptr());
                ffi::lua_rawget(l, -2);
                ffi::lua_remove(l, -2);
                // [x, y, own, x, y] -> call pops the function and arguments -> [x, y, result]
                ffi::lua_pushvalue(l, 1);
                ffi::lua_pushvalue(l, 2);
                ffi::lua_call(l, 2, 1);
            }
            None if OP == EQ => {
                reference_operator::<{ ScriptOperatorNames::Equality as ffi::lua_Integer }>(l);
            }
            None => {
                reference_operator::<{ ScriptOperatorNames::LessThanComparison as ffi::lua_Integer }>(
                    l,
                );
            }
        }
        if OP == LE {
            // [x, y, b < a, not (b < a)]
            let less = ffi::lua_toboolean(l, -1);
            ffi::lua_pushboolean(l, (less == 0) as c_int);
        }
        // the top is the result
        1
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
    OPERATORS
        .into_iter()
        .map(|(_, op)| op)
        .find(|op| *op as ffi::lua_Integer == index)
        .ok_or_else(|| InteropError::str("unknown operator"))
}

/// The base of a native value held by mlua, if `value` is one.
pub(crate) fn native_value_base(lua: &Lua, value: &mlua::AnyUserData) -> Option<*mut u8> {
    // Safety: the value is pushed and popped by `exec_raw`'s stack guard
    unsafe {
        // `exec_raw` pushes `value` and takes the top as the result: [value] -> [value, base | nil]
        lua.exec_raw::<Option<mlua::LightUserData>>(value, |l| {
            // the lookup pushes two values, the result one; protected mode, so failing raises
            if ffi::lua_checkstack(l, 2) == 0 {
                raise_str(l, c"stack overflow");
            }
            match native_base(l, -1) {
                Some(base) => ffi::lua_pushlightuserdata(l, base as *mut c_void),
                None => ffi::lua_pushnil(l),
            }
        })
    }
    .ok()
    .flatten()
    .map(|p| p.0 as *mut u8)
}

#[cfg(test)]
mod test {
    use bevy_ecs::world::World;
    use bevy_mod_scripting_bindings::{CurrentScriptAttachment, ScriptFunction};
    use bevy_mod_scripting_world::WorldAccessGuard;

    use super::*;

    /// A typed function's panic is resumed in Rust, a script's `pcall` catches it: like the panics of mlua's callbacks.
    #[test]
    fn panics_behave_like_mlua_callbacks() {
        let mut world = World::default();
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        WorldAccessGuard::with_static_guard(&mut world, cache, |_| {
            let lua = Lua::new();
            let function =
                (|| -> bool { std::panic::panic_any("boom") }).into_dynamic_script_function();
            let function = native_function(&lua, function).unwrap();
            lua.globals().set("f", function).unwrap();
            let payload = catch_unwind(AssertUnwindSafe(|| lua.load("f()").exec()))
                .expect_err("the panic is resumed");
            assert_eq!(payload.downcast_ref::<&str>(), Some(&"boom"));
            let caught: bool = lua.load("return pcall(f)").eval().unwrap();
            assert!(!caught, "pcall catches it");
        });
    }

    #[derive(bevy_reflect::Reflect, Clone, Default)]
    struct P {
        x: f32,
    }

    /// Comparisons across metatables (native values, references, both), which Lua 5.1, 5.2, LuaJIT and Luau only call when
    /// both operands share the metamethod, and a typed function with a native argument calling back into Lua from a
    /// coroutine, which may write the argument: the call has a copy.
    #[test]
    fn comparisons_and_callbacks() {
        use bevy_mod_scripting_bindings::{
            FunctionCallContext, M, R, ReflectReference, V, function::namespace::NamespaceBuilder,
        };
        let mut world = World::default();
        world
            .get_resource_or_init::<bevy_ecs::reflect::AppTypeRegistry>()
            .write()
            .register::<P>();
        bevy_mod_scripting_bindings::script_native!(&mut world, P { x: f32 });
        NamespaceBuilder::<P>::new(&mut world)
            .register("eq", |a: R<P>, b: R<P>| a.x == b.x)
            .register("lt", |a: R<P>, b: R<P>| a.x < b.x);
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        WorldAccessGuard::with_static_guard(&mut world, cache, |_| {
            let lua = Lua::new();
            let globals = [
                (
                    "value",
                    (|x: f32| V(P { x })).into_dynamic_script_function(),
                ),
                (
                    "reference",
                    (|x: f32| ReflectReference::new_allocated(P { x }))
                        .into_dynamic_script_function(),
                ),
                (
                    "call_back",
                    (|ctx: FunctionCallContext,
                      v: R<P>,
                      f: DynamicScriptFunction|
                     -> Result<f32, InteropError> {
                        f.call([], ctx)?;
                        Ok(v.x)
                    })
                    .into_dynamic_script_function(),
                ),
                (
                    "call_back_mut",
                    (|ctx: FunctionCallContext,
                      mut v: M<P>,
                      f: DynamicScriptFunction|
                     -> Result<(), InteropError> {
                        f.call([], ctx)?;
                        v.x = 0.0;
                        Ok(())
                    })
                    .into_dynamic_script_function(),
                ),
            ];
            for (name, function) in globals {
                let function = native_function(&lua, function).unwrap();
                lua.globals().set(name, function).unwrap();
            }
            lua.load(
                r#"
                local a, b, r = value(1), value(2), reference(1)
                assert(a == value(1) and a ~= b and a < b and a <= value(1) and not (b <= a), "values")
                assert(a == r and r == a and r <= b and a <= r and not (b < r), "a value and a reference")
                assert(r == reference(1) and r < reference(2), "references")
                local x = coroutine.wrap(function()
                    return call_back(a, function() a.x = 5 end)
                end)()
                assert(x == 1 and a.x == 5, "a callback into Lua from a coroutine: the call has its copy of the argument")
                call_back_mut(a, function() assert(a.x == 5) end)
                assert(a.x == 0, "a changed argument is written back when the call ends")
                a.x = 3
                b.x = 4.5
                assert(a.x == 3 and b.x == 4.5 and a < b, "fields written in place")
                assert(not pcall(function() a.x = "3" end) or a.x == 3, "other values through the setter")
                "#,
            )
            .exec()
            .unwrap();
            // closing a LuaJIT state holding mlua userdata (the reference) crashes in LuaJIT's GC in some build modes, a
            // known runtime issue unrelated to native values: https://github.com/mlua-rs/mlua/issues/757
            #[cfg(feature = "luajit")]
            std::mem::forget(lua);
        });
    }
}
