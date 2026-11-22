//! LuaJIT: native types with a C layout ([`FfiLayout`]) are cdata.
//!
//! - The FFI library is loaded privately (`luaopen_ffi`, never a global or `require`-able), the first time a value of
//!   such a type reaches Lua.
//! - Each type is a C struct (`ffi.cdef`) with a metatype: fields are plain FFI fields; functions with an `extern "C"`
//!   form (`ScriptFunctionRegistry::register_ffi`, see [`FfiFunction`]) are called through the FFI with C types, after checking
//!   their arguments are of the expected types, else (and for other functions) through BMS' typed functions, which find
//!   cdata on the stack ([`cdata_at`], [`super::LuaArgs`]); operators call their functions the same way.
//! - Values Rust returns are copied into new cdata (see `Ready::Cdata`); scripts can write every byte of a cdata at any
//!   time, so BMS' functions never borrow one: they get copies, written back for `M<T>` (see `CallScope::copy_value`).
//! - Types are told apart by LuaJIT's ctype id, which scripts can't change, see [`cdata_at`].

use std::{
    any::TypeId,
    ffi::c_void,
    fmt::Write as _,
    os::raw::c_int,
    sync::atomic::{AtomicUsize, Ordering},
};

use bevy_mod_scripting_bindings::function::namespace::FunctionScope;
use bevy_mod_scripting_bindings::{
    InteropError, ScriptValue, WorldExtensions, dispatch,
    native::{FfiFunction, FfiLayout, FfiType, NativeType, NativeVTable, take_ffi_error},
};
use bevy_mod_scripting_bindings_domain::ScriptOperatorNames;
use bevy_mod_scripting_world::ThreadWorldContainer;
use mlua::{IntoLua, LightUserData, Lua, Table, ffi};

use super::{LUA_CALLER_CONTEXT, Ready, key, raw_get_p, raw_set_p, run, with_raw};
use crate::{IntoInteropError, IntoMluaError};

/// LuaJIT's type tag of cdata, not in the Lua C API.
pub(crate) const LUA_TCDATA: c_int = 10;

/// The offset from a cdata's payload back to its ctype id (`GCcdata::ctypeid`): 6 on GC64 builds, 2 on others, found
/// when the first types are made, 0 before.
static CTYPEID_OFFSET: AtomicUsize = AtomicUsize::new(0);

/// Its address keys the table `{ [ctype id] = vtable (light userdata) }` in the registry, see [`cdata_at`].
static CTYPES: u8 = 0;

/// Its address keys the table `{ [vtable] = { ctype, from_ptr, statics } }` in the registry, see [`type_info`].
static TYPES: u8 = 0;

/// The named registry value keeping the private FFI library alive, see [`private_ffi`].
const FFI_MODULE: &str = "bms_ffi";

/// The native value at `idx` if it's a cdata of a native type with a C layout: its vtable, its payload, alive while
/// the value is on the stack, and whether it's a read-only copy (a `const` struct). `[-0, +0, -]`, but needs 2 free slots.
///
/// # Safety
/// `idx` is on the stack of `l`.
pub(crate) unsafe fn cdata_at(
    l: *mut ffi::lua_State,
    idx: c_int,
) -> Option<(&'static NativeVTable, *mut u8, bool)> {
    unsafe {
        if ffi::lua_type(l, idx) != LUA_TCDATA {
            return None;
        }
        // no cdata of BMS' types exist before any type is made
        let offset = CTYPEID_OFFSET.load(Ordering::Acquire);
        if offset == 0 {
            return None;
        }
        let payload = ffi::lua_topointer(l, idx) as *mut u8;
        // Safety: every cdata starts with its `GCcdata` header, which ends with the ctype id (`offset` found for this build)
        let id = (payload.sub(offset) as *const u16).read_unaligned();
        // [.., ctypes, ctypes[id]]
        if raw_get_p(l, ffi::LUA_REGISTRYINDEX, key(&CTYPES)) != ffi::LUA_TTABLE {
            ffi::lua_pop(l, 1);
            return None;
        }
        ffi::lua_rawgeti(l, -1, id as ffi::lua_Integer);
        // the vtable, with its lowest bit set for the type's `const` form (vtables are aligned)
        let tagged = ffi::lua_touserdata(l, -1) as usize;
        ffi::lua_pop(l, 2);
        let vtable = (tagged & !1) as *const NativeVTable;
        // Safety: only vtables are kept in the table
        (!vtable.is_null()).then(|| (&*vtable, payload, tagged & 1 == 1))
    }
}

/// The cdata at `idx` as a script value, a reference to a copy, like other native values (see
/// `NativeStorage::to_script_value`). `None` if it's no cdata of a native type.
///
/// # Safety
/// `idx` is on the stack of `l`, which has 2 free slots.
pub(crate) unsafe fn cdata_script_value(l: *mut ffi::lua_State, idx: c_int) -> Option<ScriptValue> {
    unsafe {
        let (vtable, payload, _) = cdata_at(l, idx)?;
        Some(ScriptValue::Reference(vtable.plain_reference(payload)))
    }
}

/// The cdata held by mlua as a script value, see [`cdata_script_value`].
pub(crate) fn cdata_value(lua: &Lua, value: &mlua::Value) -> Option<ScriptValue> {
    // Safety: the value is pushed and popped, room is checked
    lua.exec_raw_lua(|raw| unsafe {
        let l = raw.state();
        if ffi::lua_checkstack(l, 3) == 0 || value.push_into_stack(raw).is_err() {
            return None;
        }
        let value = cdata_script_value(l, -1);
        ffi::lua_pop(l, 1);
        value
    })
}

/// The registry key of the C type of `vtable`, or of its `const` form: addresses inside the vtable, which nothing else
/// keys the registry with (the vtable's own address keys the metatable of the type's userdata form).
fn ctype_key(vtable: &'static NativeVTable, read_only: bool) -> *mut c_void {
    (vtable as *const NativeVTable as usize + if read_only { 3 } else { 2 }) as *mut c_void
}

/// Pushes the C type of `vtable` (its `const` form, whose values scripts can't write, if `read_only`), making the types
/// first if this state has none yet. Calling it makes a zeroed value, see [`super::push_ready`]. `[-0, +(0|1), m]`: nothing for types registered after the types were made, which stay userdata.
///
/// # Safety
/// `l` is the state of a call.
pub(super) unsafe fn push_ctype(
    l: *mut ffi::lua_State,
    vtable: &'static NativeVTable,
    read_only: bool,
) -> Result<bool, InteropError> {
    // where the info of a type keeps the types, see `types_source`
    let from_ptr = if read_only { 2 } else { 1 };
    unsafe {
        // made already: one raw get from the registry
        if ffi::lua_checkstack(l, 1) != 0 {
            // [.., ctype | nil]
            if raw_get_p(l, ffi::LUA_REGISTRYINDEX, ctype_key(vtable, read_only)) == LUA_TCDATA {
                return Ok(true);
            }
            ffi::lua_pop(l, 1);
            // without the type, the types were made before it was registered
            let made = raw_get_p(l, ffi::LUA_REGISTRYINDEX, key(&TYPES)) == ffi::LUA_TTABLE;
            ffi::lua_pop(l, 1);
            if made {
                return Ok(false);
            }
        }
        // the first value of such a type in this state: make the types
        super::push_with_mlua(l, |lua, raw| {
            let Some(info) = type_info(lua, vtable)? else {
                return Ok(0);
            };
            let ctype: mlua::Value = info.raw_get(from_ptr)?;
            ctype.push_into_stack(raw)?;
            Ok(1)
        })
    }
    .map(|n| n == 1)
    .map_err(IntoInteropError::to_bms_error)
}

/// The FFI form of the static function `name` of the type `type_id` (i.e. `Vec3.new`), if it has one.
pub(super) fn ffi_static(
    lua: &Lua,
    type_id: std::any::TypeId,
    name: &str,
) -> mlua::Result<Option<mlua::Value>> {
    let world = ThreadWorldContainer::try_get_context()
        .map_err(|e| InteropError::from(e).to_lua_error())?
        .world;
    let vtable = {
        let registry = world.script_function_registry();
        let registry = registry.read();
        match registry.native_type(type_id) {
            Some(native) if native.ffi.is_some() => native.vtable,
            _ => return Ok(None),
        }
    };
    let Some(info) = type_info(lua, vtable)? else {
        return Ok(None);
    };
    let statics: Table = info.raw_get(3)?;
    let value: mlua::Value = statics.raw_get(name)?;
    Ok((!value.is_nil()).then_some(value))
}

/// The table `{ ctype, from_ptr, statics }` of the type of `vtable`, making the types first if this state has none.
/// Types are made once per state: `None` for types registered later.
fn type_info(lua: &Lua, vtable: &'static NativeVTable) -> mlua::Result<Option<Table>> {
    let key = LightUserData(vtable as *const NativeVTable as *mut c_void);
    // Safety: protected mode, a raw get from the registry
    let types = || unsafe {
        lua.exec_raw::<Option<Table>>((), |l| {
            raw_get_p(l, ffi::LUA_REGISTRYINDEX, self::key(&TYPES));
        })
    };
    let types = match types()? {
        Some(types) => Some(types),
        None => {
            make_types(lua)?;
            types()?
        }
    };
    types.map_or(Ok(None), |types| types.raw_get(key))
}

/// Makes the FFI forms of every native type with a C layout in this state, see the module docs.
fn make_types(lua: &Lua) -> mlua::Result<()> {
    let world = ThreadWorldContainer::try_get_context()
        .map_err(|e| InteropError::from(e).to_lua_error())?
        .world;
    let (types, functions) = {
        let registry = world.script_function_registry();
        let registry = registry.read();
        let with_layout = registry
            .native_types()
            .filter(|n| n.ffi.is_some())
            .collect::<Vec<_>>();
        // and whose fields of native types (if any) have layouts of scalars
        let layout_of = layout_lookup(&with_layout);
        let types = with_layout
            .iter()
            .filter(|n| {
                n.ffi
                    .as_ref()
                    .is_some_and(|l| l.c_declaration(&layout_of).is_some())
            })
            .map(|n| (*n).clone())
            .collect::<Vec<_>>();
        // every `#[ffi]` function's types need a C layout: it can't be a bound (see `FfiArg`), so it's checked here
        let has_layout = |type_id| types.iter().any(|n| n.vtable.type_id == type_id);
        let name = |type_id| {
            registry
                .native_types()
                .find(|n| n.vtable.type_id == type_id)
                .map_or_else(|| format!("{type_id:?}"), |n| n.vtable.type_name.to_owned())
        };
        for (owner, function) in registry.all_ffi_functions() {
            let used = function
                .params
                .iter()
                .chain(std::iter::once(&function.result));
            let missing = std::iter::once(owner)
                .chain(used.filter_map(|t| match t {
                    FfiType::Value(type_id) => Some(*type_id),
                    FfiType::Scalar(_) => None,
                }))
                .find(|type_id| !has_layout(*type_id));
            if let Some(missing) = missing {
                return Err(mlua::Error::RuntimeError(format!(
                    "`{}::{}` has an FFI form, but `{}` has no C layout: declare one with `NativeBuilder::ffi`, or don't register the FFI form",
                    name(owner),
                    function.name,
                    name(missing),
                )));
            }
        }
        let functions = types
            .iter()
            .map(|n| registry.ffi_functions(n.vtable.type_id).to_vec())
            .collect::<Vec<_>>();
        (types, functions)
    };
    let ffi_module = private_ffi(lua)?;
    let (source, pointers) = types_source(&types, &functions);
    // the BMS functions of each type: the slow paths
    let slows = lua.create_table()?;
    for native in &types {
        let slow = lua.create_table()?;
        for (name, function) in &native.methods {
            slow.raw_set(
                name.as_ref(),
                super::super::script_value::registered_function_into_lua(lua, function.clone())?,
            )?;
        }
        slows.raw_set(layout_name(native), slow)?;
    }
    // the vtable of each type, which `raise_ffi_error` is given
    let owners = lua.create_table()?;
    for native in &types {
        let vtable = LightUserData(native.vtable as *const NativeVTable as *mut c_void);
        owners.raw_set(layout_name(native), vtable)?;
    }
    let pointers = pointers
        .into_iter()
        .map(|p| LightUserData(p as *mut c_void))
        .collect::<Vec<_>>();
    // Safety: the functions follow the Lua C API
    let (index, tostring, raise) = unsafe {
        (
            lua.create_c_function(cdata_index)?,
            lua.create_c_function(cdata_tostring)?,
            lua.create_c_function(raise_ffi_error)?,
        )
    };
    let made: Table = lua
        .load(source)
        .set_name("=bms ffi types")
        .call((ffi_module, pointers, slows, index, tostring, raise, owners))?;
    // by vtable for Rust, by ctype id for C
    let by_vtable = lua.create_table()?;
    let by_id = lua.create_table()?;
    for native in &types {
        let info: Table = made.raw_get(layout_name(native))?;
        let id: i64 = info.raw_get(4)?;
        let vtable = LightUserData(native.vtable as *const NativeVTable as *mut c_void);
        by_vtable.raw_set(vtable, info.clone())?;
        by_id.raw_set(id, vtable)?;
        // the `const` form, which read-only copies are: the vtable with its lowest bit set, see `cdata_at`
        let const_id: i64 = info.raw_get(6)?;
        by_id.raw_set(
            const_id,
            LightUserData((native.vtable as *const NativeVTable as usize | 1) as *mut c_void),
        )?;
        // the same for every type and state of this build
        if CTYPEID_OFFSET.load(Ordering::Acquire) == 0 {
            calibrate(&info.raw_get::<mlua::Value>(5)?, id)?;
        }
        // the types themselves, found with one lookup when a value is returned, see `push_ctype`
        for (index, read_only) in [(1, false), (2, true)] {
            let ctype: mlua::Value = info.raw_get(index)?;
            let slot = ctype_key(native.vtable, read_only);
            // Safety: protected mode, a raw set of the value on the top into the registry
            unsafe {
                lua.exec_raw::<()>(ctype, |l| raw_set_p(l, ffi::LUA_REGISTRYINDEX, slot))?;
            }
        }
    }
    // Safety: protected mode, raw sets of the tables on the top into the registry
    unsafe {
        lua.exec_raw::<()>(by_id, |l| {
            raw_set_p(l, ffi::LUA_REGISTRYINDEX, key(&CTYPES))
        })?;
        lua.exec_raw::<()>(by_vtable, |l| {
            raw_set_p(l, ffi::LUA_REGISTRYINDEX, key(&TYPES))
        })?;
    }
    Ok(())
}

/// The FFI library, which scripts don't get: opening it doesn't make a global, but registers it in `package.loaded`, where
/// it's taken out of again.
///
/// If the host loaded it already it's used as it is: opening it again would reset the C types of the state.
fn private_ffi(lua: &Lua) -> mlua::Result<Table> {
    let loaded = lua.named_registry_value::<Option<Table>>("_LOADED")?;
    if let Some(loaded) = &loaded
        && let Some(module) = loaded.raw_get::<Option<Table>>("ffi")?
    {
        return Ok(module);
    }
    // Safety: protected mode, `luaopen_ffi` returns the module table
    let module: Table = unsafe {
        lua.exec_raw((), |l| {
            ffi::lua_pushcfunction(l, ffi::luaopen_ffi);
            ffi::lua_call(l, 0, 1);
        })?
    };
    // LuaJIT keeps the metatables and finalizers of C types alive through the module table only: it must stay
    // reachable, in the registry, which scripts can't read
    lua.set_named_registry_value(FFI_MODULE, &module)?;
    if let Some(loaded) = &loaded {
        loaded.raw_set("ffi", mlua::Nil)?;
    }
    Ok(module)
}

/// Finds where this LuaJIT build keeps the ctype id of a cdata, from a `probe` of the type `id`.
fn calibrate(probe: &mlua::Value, id: i64) -> mlua::Result<()> {
    let payload = probe.to_pointer() as *const u8;
    let offset = [6, 2].into_iter().find(|&offset| {
        // Safety: inside the header of the probe, which is alive
        let found = unsafe { (payload.sub(offset) as *const u16).read_unaligned() };
        found as i64 == id
    });
    match offset {
        Some(offset) => {
            CTYPEID_OFFSET.store(offset, Ordering::Release);
            Ok(())
        }
        None => Err(mlua::Error::external(
            "can't find the ctype ids of cdata in this LuaJIT build",
        )),
    }
}

/// The name of the C struct of a native type.
fn layout_name(native: &NativeType) -> &str {
    native
        .ffi
        .as_ref()
        .map(|l| l.name.as_ref())
        .unwrap_or_default()
}

/// The layouts of `types` by type id, for [`FfiLayout::c_fields`].
fn layout_lookup<'a, N: AsRef<NativeType>>(
    types: &'a [N],
) -> impl Fn(TypeId) -> Option<&'a FfiLayout> {
    move |type_id| {
        types
            .iter()
            .map(AsRef::as_ref)
            .find(|n| n.vtable.type_id == type_id)
            .and_then(|n| n.ffi.as_ref())
    }
}

/// The Lua source of the accessors of the fields of native types of `layout` (i.e. a matrix' columns), which are
/// flattened in the C struct (see [`FfiLayout::c_fields`]): `getters.x_axis` reads a `const` copy, `setters.x_axis` writes
/// a whole value of the field's type. Empty for types without such fields.
///
/// ```lua
/// do local K, N = ffi.typeof("const bms_Vec3"), C2
/// getters.x_axis = function(v) return K(v._x_axis_x, v._x_axis_y, v._x_axis_z) end
/// setters.x_axis = function(v, x) if not istype(N, x) then error("expected a glam::Vec3", 2) end
/// v._x_axis_x, v._x_axis_y, v._x_axis_z = x.x, x.y, x.z end end
/// ```
fn nested_fields_source<'a>(
    layout: &'a FfiLayout,
    layout_of: &dyn Fn(TypeId) -> Option<&'a FfiLayout>,
    ctype: &impl Fn(TypeId) -> Option<(String, String)>,
    type_name: &impl Fn(TypeId) -> &'a str,
) -> String {
    let mut out = String::new();
    for field in &layout.fields {
        let FfiType::Value(type_id) = field.ty else {
            continue;
        };
        let (Some(inner), Some((lua_ctype, c_name))) = (layout_of(type_id), ctype(type_id)) else {
            continue;
        };
        let name = field.name;
        let flat = inner
            .fields
            .iter()
            .map(|f| format!("v._{name}_{}", f.name))
            .collect::<Vec<_>>()
            .join(", ");
        let from = inner
            .fields
            .iter()
            .map(|f| format!("x[{:?}]", f.name))
            .collect::<Vec<_>>()
            .join(", ");
        let _ = writeln!(
            out,
            // names are keys, not identifiers: a field may be named like a keyword of Lua (`end`)
            "do local K, N = ffi.typeof(\"const {c_name}\"), {lua_ctype}\ngetters[{name:?}] = function(v) return K({flat}) end\nsetters[{name:?}] = function(v, x) if not istype(N, x) then error(\"expected a {}\", 2) end {flat} = {from} end end",
            type_name(type_id)
        );
    }
    out
}

/// The Lua source making the FFI forms of `types`, and the pointers of their FFI functions it's given (as `ptrs[i]`).
///
/// ```lua
/// local ffi, ptrs, slows, index, tostring_ = ...
/// ffi.cdef[[ typedef struct __attribute__((aligned(4))) { float x; float y; float z; } bms_Vec3; ]]
/// local C1 = ffi.typeof("bms_Vec3")
/// do
///     local T, slow, methods, statics = C1, slows.bms_Vec3, {}, {}
///     local f1 = ffi.cast("float (*)(const bms_Vec3*)", ptrs[1])
///     methods.length = function(a1) if istype(C1, a1) then return f1(a1) end return slow.length(a1) end
///     ..
/// end
/// ```
fn types_source(
    types: &[std::sync::Arc<NativeType>],
    functions: &[Vec<FfiFunction>],
) -> (String, Vec<*const ()>) {
    let ctype = |type_id| {
        types
            .iter()
            .position(|n| n.vtable.type_id == type_id)
            .map(|i| (format!("C{}", i + 1), layout_name(&types[i]).to_owned()))
    };
    let mut pointers = Vec::new();
    let mut out = String::from(
        // `E`: the flag fallible FFI functions write when they fail, see `function_source`
        "local ffi, ptrs, slows, index, tostring_, raise, owners = ...\nlocal istype, cast = ffi.istype, ffi.cast\nlocal E = ffi.new(\"uint8_t[1]\")\nffi.cdef[[\n",
    );
    let layout_of = layout_lookup(types);
    let type_name = |type_id| {
        types
            .iter()
            .find(|n| n.vtable.type_id == type_id)
            .map(|n| n.vtable.type_name)
            .unwrap_or_default()
    };
    // only types whose declaration can be made are given, see `make_types`
    for layout in types.iter().filter_map(|n| n.ffi.as_ref()) {
        if let Some(declaration) = layout.c_declaration(&layout_of) {
            let _ = writeln!(out, "{declaration}");
        }
    }
    out.push_str("]]\n");
    for (i, native) in types.iter().enumerate() {
        // the type, and its scratch value: where FFI functions write their result
        let _ = writeln!(
            out,
            "local C{n} = ffi.typeof(\"{}\") local S{n} = C{n}() assert(ffi.sizeof(C{n}) == {} and ffi.alignof(C{n}) == {}, \"{} has another size or alignment in C\")",
            layout_name(native),
            native.ffi.as_ref().map_or(0, |l| l.size),
            native.ffi.as_ref().map_or(0, |l| l.align),
            native.vtable.type_name,
            n = i + 1,
        );
    }
    out.push_str("local made = {}\n");
    for (i, (native, functions)) in types.iter().zip(functions).enumerate() {
        // only types with a C layout are given
        if native.ffi.is_none() {
            continue;
        }
        let t = format!("C{}", i + 1);
        let name = layout_name(native);
        let _ = writeln!(
            out,
            // `O`: the type, for the errors of its FFI functions
            "do\nlocal T, slow, methods, statics, O = {t}, slows.{name}, {{}}, {{}}, owners.{name}"
        );
        // functions with a C form
        for function in functions {
            let Some(code) = function_source(function, &ctype, pointers.len() + 1) else {
                continue;
            };
            pointers.push(function.function);
            out.push_str(&code);
        }
        // every other function through BMS
        out.push_str("for name, f in pairs(slow) do if methods[name] == nil then methods[name] = f end end\n");
        let nested = native
            .ffi
            .as_ref()
            .map(|layout| nested_fields_source(layout, &layout_of, &ctype, &type_name))
            .unwrap_or_default();
        if nested.is_empty() {
            out.push_str(
                "local mt = { __index = function(v, k) local m = methods[k] if m ~= nil then return m end return index(v, k) end,\n__tostring = tostring_ }\n",
            );
        } else {
            // fields of native types are read as `const` copies and written whole
            out.push_str("local getters, setters = {}, {}\n");
            out.push_str(&nested);
            let _ = writeln!(
                out,
                "local mt = {{ __index = function(v, k) local g = getters[k] if g ~= nil then return g(v) end local m = methods[k] if m ~= nil then return m end return index(v, k) end,\n__newindex = function(v, k, x) local s = setters[k] if s ~= nil then return s(v, x) end error(\"{} has no field `\" .. tostring(k) .. \"`\", 2) end,\n__tostring = tostring_ }}",
                native.vtable.type_name
            );
        }
        operators_source(&mut out, native);
        let _ = writeln!(
            out,
            // 1 the type, 2 its `const` form (a read-only copy), 3 the statics, 4 the type's id, 5 a probe, 6 the id of the
            // `const` form
            "T = ffi.metatype(T, mt)\nlocal K = ffi.typeof(\"const {name}\")\nmade.{name} = {{ T, K, statics, tonumber(T), T(), tonumber(K) }}\nend"
        );
    }
    out.push_str("return made\n");
    (out, pointers)
}

/// The Lua source of the FFI form of a function, `fN` calling `ptrs[N]`, `None` if a value parameter or the result is of a
/// type without a C layout.
fn function_source(
    function: &FfiFunction,
    ctype: &impl Fn(std::any::TypeId) -> Option<(String, String)>,
    pointer: usize,
) -> Option<String> {
    let mut c_params = Vec::new();
    let mut guards = Vec::new();
    let params = (1..=function.params.len())
        .map(|i| format!("a{i}"))
        .collect::<Vec<_>>();
    for (param, kind) in params.iter().zip(&function.params) {
        match kind {
            FfiType::Scalar(c) => {
                c_params.push((*c).to_owned());
                let lua_type = if *c == "bool" { "boolean" } else { "number" };
                guards.push(format!("type({param}) == \"{lua_type}\""));
            }
            FfiType::Value(type_id) => {
                let (lua_ctype, c_name) = ctype(*type_id)?;
                c_params.push(format!("const {c_name}*"));
                guards.push(format!("istype({lua_ctype}, {param})"));
            }
        }
    }
    let args = params.join(", ");
    // every wrapper takes the out-pointer and the `failed` pointer after the parameters
    c_params.extend(["void*".to_owned(), "uint8_t*".to_owned()]);
    let name = function.name;
    let (failed, check) = match function.fallible {
        // a failure writes the flag: the error kept by the wrapper is raised (not a tail call: it's reported at the line
        // of the script below this chunk, see `caller_line`), `None` is nil
        true => (
            "E",
            format!(
                "local e = E[0] if e ~= 0 then E[0] = 0 if e == 2 then return nil end return (raise({name:?}, O)) end "
            ),
        ),
        false => ("nil", String::new()),
    };
    let with_out = |out: &str| match args.is_empty() {
        true => format!("{out}, {failed}"),
        false => format!("{args}, {out}, {failed}"),
    };
    let (c_result, call) = match &function.result {
        FfiType::Scalar(c) => (
            c.to_string(),
            match function.fallible {
                true => format!("local r = f{pointer}({}) {check}return r", with_out("nil")),
                false => format!("return f{pointer}({})", with_out("nil")),
            },
        ),
        FfiType::Value(type_id) => {
            let (lua_ctype, _) = ctype(*type_id)?;
            // written to the type's scratch value and copied from it: a result made here and passed to C would always
            // be allocated, a copy made by Lua can be optimised away. Nothing else points to the scratch value.
            let scratch = lua_ctype.replacen('C', "S", 1);
            (
                "void".to_owned(),
                format!(
                    "f{pointer}({}) {check}return {lua_ctype}({scratch})",
                    with_out(&scratch)
                ),
            )
        }
    };
    let guard = match guards.is_empty() {
        true => "true".to_owned(),
        false => guards.join(" and "),
    };
    Some(format!(
        // in a block: a chunk has room for 200 locals only
        // other arguments go to the function's FFI form made before this one (a function taking a `Union` has one for
        // each alternative), else through BMS
        "do local f{pointer}, other = cast(\"{c_result} (*)({})\", ptrs[{pointer}]), methods[{name:?}] or slow[{name:?}]\nlocal function w({args}) if {guard} then {call} end return other({args}) end\nmethods[{name:?}] = w statics[{name:?}] = w end\n",
        c_params.join(", ")
    ))
}

/// The Lua source of the operators of a type's metatable `mt`: they call the operator's function in `methods`, its FFI
/// form if it has one, like native values' call BMS' (see `OPERATORS`).
fn operators_source(out: &mut String, native: &NativeType) {
    let has = |name: &str| native.methods.contains_key(name);
    // calls the function `name`, else raises like BMS does for a missing operator function
    let call = |name: Option<&str>, args: &str| match name {
        Some(name) if has(name) => format!("return methods.{name}({args})"),
        name => format!(
            "error(\"{} has no function `{}`\", 2)",
            native.vtable.type_name,
            name.unwrap_or("operator")
        ),
    };
    for (meta, operator) in super::OPERATORS {
        let meta = meta.to_str().unwrap_or_default();
        let forward = operator.script_function_name();
        let reflected = operator.reflected_script_function_name();
        match operator {
            ScriptOperatorNames::DisplayPrint => continue,
            ScriptOperatorNames::Equality => {
                let other = match has(forward) {
                    true => format!(
                        "local ta, tb = type(a), type(b)\nif (ta == \"cdata\" or ta == \"userdata\") and (tb == \"cdata\" or tb == \"userdata\") then return methods.{forward}(a, b) end\n"
                    ),
                    false => String::new(),
                };
                let _ = writeln!(out, "mt.{meta} = function(a, b)\n{other}return false\nend");
            }
            ScriptOperatorNames::Negation | ScriptOperatorNames::Length => {
                if has(forward) {
                    let _ = writeln!(
                        out,
                        "mt.{meta} = function(a) return methods.{forward}(a) end"
                    );
                }
            }
            ScriptOperatorNames::LessThanComparison => {
                if has(forward) {
                    let _ = writeln!(
                        out,
                        "mt.{meta} = function(a, b) return methods.{forward}(a, b) end"
                    );
                    let _ = writeln!(
                        out,
                        "mt.__le = function(a, b) return not methods.{forward}(b, a) end"
                    );
                }
            }
            _ => {
                let forward = call(Some(forward), "a, b");
                let reflected = call(reflected, "b, a");
                let _ = writeln!(
                    out,
                    "mt.{meta} = function(a, b) if istype(T, a) then {forward} end {reflected} end"
                );
            }
        }
    }
}

/// `__index` of cdata for names which are no fields or functions: through a reference to a copy, like native values'
/// slow path, see [`super::member_at`]. Called with `[v (1), key (2)]`.
unsafe extern "C-unwind" fn cdata_index(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        ffi::lua_settop(l, 2);
        run(l, || {
            let Some(ScriptValue::Reference(receiver)) = cdata_script_value(l, 1) else {
                return Err(InteropError::str("expected a BMS cdata"));
            };
            super::member_at(
                l,
                super::Receiver::Reference(std::borrow::Cow::Owned(receiver)),
            )
        })
    }
}

/// Raises the error of the FFI function which just failed (see `FfiReturn`), like BMS raises the errors of functions: C
/// functions called through the FFI can't raise, the generated Lua calls this when one wrote `FFI_ERROR`. Called with
/// `[name of the function (1), vtable of its type (2)]`.
unsafe extern "C-unwind" fn raise_ffi_error(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        let vtable = ffi::lua_touserdata(l, 2) as *const NativeVTable;
        let name = match ffi::lua_tostring(l, 1) {
            name if name.is_null() => "an FFI function".into(),
            name => std::ffi::CStr::from_ptr(name).to_string_lossy(),
        };
        run(l, || {
            let error = take_ffi_error()
                .unwrap_or_else(|| InteropError::str("an FFI function failed without an error"));
            Err(match vtable.as_ref() {
                Some(vtable) => InteropError::function_interop_error(
                    name,
                    FunctionScope::OnType(vtable.type_id),
                    error,
                    Some(LUA_CALLER_CONTEXT),
                ),
                None => error,
            })
        })
    }
}

/// `__tostring` of cdata, see [`dispatch::display`]. Called with `[v (1)]`.
unsafe extern "C-unwind" fn cdata_tostring(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        ffi::lua_settop(l, 1);
        run(l, || {
            let Some(ScriptValue::Reference(value)) = cdata_script_value(l, 1) else {
                return Err(InteropError::str("expected a BMS cdata"));
            };
            let world = ThreadWorldContainer::try_get_context()?.world;
            let text = dispatch::display(world, value, LUA_CALLER_CONTEXT)?;
            with_raw(l, |lua, _| text.into_lua(lua))
                .map(Ready::Value)
                .map_err(IntoInteropError::to_bms_error)
        })
    }
}

#[cfg(test)]
mod test {
    use bevy_ecs::world::World;
    use bevy_mod_scripting_bindings::{
        CurrentScriptAttachment, DynamicScriptFunction, FunctionCallContext, M, R, ScriptFunction,
        Union, V,
        function::{namespace::NamespaceBuilder, script_function::AppScriptFunctionRegistry},
    };
    use bevy_mod_scripting_world::WorldAccessGuard;

    use super::*;

    #[derive(bevy_reflect::Reflect, Clone, Copy, Default)]
    struct C {
        x: f32,
    }

    /// Values of a type with a C layout are cdata scripts can write at any time: BMS' functions get copies, written back
    /// for `M<T>`, and scripts don't get the FFI library.
    #[test]
    fn cdata_arguments_are_copies() {
        let mut world = World::default();
        world
            .get_resource_or_init::<bevy_ecs::reflect::AppTypeRegistry>()
            .write()
            .register::<C>();
        bevy_mod_scripting_bindings::assert_ffi_layout!(C { x: f32 });
        // Safety: the field is at its offset, any bits are a `C`
        unsafe {
            NativeType::of::<C>()
                .field("x", std::mem::offset_of!(C, x), |c| &c.x)
                .ffi()
        }
        .register(&mut world);
        // a type with a field of the cdata type: reading it gives a read-only copy, a `const` cdata
        #[derive(bevy_reflect::Reflect, Clone, Default)]
        struct Holder {
            c: C,
        }
        world
            .get_resource_or_init::<bevy_ecs::reflect::AppTypeRegistry>()
            .write()
            .register::<Holder>();
        // Safety: the field is at its offset
        unsafe {
            NativeType::of::<Holder>().value_field("c", std::mem::offset_of!(Holder, c), |h| &h.c)
        }
        .register(&mut world);
        // a type with padding after its fields
        #[derive(bevy_reflect::Reflect, Clone, Copy, Default)]
        struct Padded {
            x: f64,
            y: f32,
        }
        world
            .get_resource_or_init::<bevy_ecs::reflect::AppTypeRegistry>()
            .write()
            .register::<Padded>();
        bevy_mod_scripting_bindings::assert_ffi_layout!(Padded { x: f64, y: f32 });
        // Safety: the fields are at their offsets, any bits are a `Padded`
        unsafe {
            NativeType::of::<Padded>()
                .field("x", std::mem::offset_of!(Padded, x), |p| &p.x)
                .field("y", std::mem::offset_of!(Padded, y), |p| &p.y)
                .ffi()
        }
        .register(&mut world);
        NamespaceBuilder::<C>::new(&mut world)
            .register("add", |a: R<C>, b: Union<R<C>, f32>| match b.into_left() {
                Ok(b) => V(C { x: a.x + b.x }),
                Err(b) => V(C { x: a.x + b }),
            })
            .register("not", |a: R<C>| V(C { x: -a.x }))
            .register("eq", |a: R<C>, b: R<C>| a.x == b.x);
        let value = FfiType::Value(std::any::TypeId::of::<C>());
        let registry = world
            .get_resource_or_init::<AppScriptFunctionRegistry>()
            .0
            .clone();
        // `add` through BMS adds, its FFI form (for a `C` on the right) also adds 100, to tell which one ran; `not` is
        // named like a keyword of Lua, its FFI form also subtracts 100
        {
            let mut registry = registry.clone();
            let mut registry = registry.write();
            let c = std::any::TypeId::of::<C>();
            registry.register_ffi(c, "add", |a: R<C>, b: R<C>| {
                V(C {
                    x: a.x + b.x + 100.0,
                })
            });
            registry.register_ffi(c, "not", |a: R<C>| V(C { x: -a.x - 100.0 }));
        }
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        WorldAccessGuard::with_static_guard(&mut world, cache, |_| {
            let lua = Lua::new();
            let globals = [
                (
                    "value",
                    (|x: f32| V(C { x })).into_dynamic_script_function(),
                ),
                (
                    "holder",
                    (|x: f32| V(Holder { c: C { x } })).into_dynamic_script_function(),
                ),
                (
                    "padded",
                    (|| V(Padded::default())).into_dynamic_script_function(),
                ),
                (
                    "call_back",
                    (|ctx: FunctionCallContext,
                      v: R<C>,
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
                      mut v: M<C>,
                      f: DynamicScriptFunction|
                     -> Result<(), InteropError> {
                        f.call([], ctx)?;
                        v.x += 1.0;
                        Ok(())
                    })
                    .into_dynamic_script_function(),
                ),
            ];
            for (name, function) in globals {
                let function = super::super::native_function(&lua, function).unwrap();
                lua.globals().set(name, function).unwrap();
            }
            lua.load(
                r#"
                local a = value(1)
                assert(type(a) == "cdata" and a.x == 1, "a cdata")
                assert(not pcall(require, "ffi") and package.loaded.ffi == nil, "no FFI library for scripts")
                -- the library stays alive without `package.loaded`: collections must not free what C types need
                collectgarbage() collectgarbage()
                for i = 1, 1000 do a = a + value(0) end
                collectgarbage()
                a = value(1)
                assert((a + a).x == 102 and a:add(a).x == 102, "an operator and a method with an FFI form")
                assert((a + 3).x == 4 and a == value(1), "other arguments and functions through BMS")
                assert(a["not"](a).x == -101, "an FFI form named like a keyword")
                -- padding is scalar bytes: an array would be a reference scripts could index out of bounds
                local p = padded()
                assert(type(p) == "cdata" and type(p._pad12) == "number", "padding is no array")
                local x = call_back(a, function() a.x = 5 end)
                assert(x == 1 and a.x == 5, "the call reads its copy, the script's write stays")
                call_back_mut(a, function() assert(a.x == 5) end)
                assert(a.x == 6, "written back when the call ends")
                -- native values have no write-through: a field is read as a read-only copy and written whole
                local h = holder(1)
                local c = h.c
                assert(type(c) == "cdata" and c.x == 1 and (c + c).x == 102, "a read-only copy is read like any value")
                assert(not pcall(function() c.x = 5 end) and c.x == 1, "but not written")
                assert(not pcall(call_back_mut, c, function() end), "nor changed by a call")
                local m = c:clone()
                m.x = 5
                h.c = m
                assert(h.c.x == 5 and c.x == 1, "a clone is written, and the field with it")
                "#,
            )
            .exec()
            .unwrap();
            // an FFI function of a type without a C layout (`Holder`): an error when a state makes its C types
            // Safety: never called, the C types can't be made
            unsafe {
                registry.clone().write().register_ffi_function(
                    std::any::TypeId::of::<C>(),
                    FfiFunction {
                        name: "holder_of",
                        function: std::ptr::null(),
                        params: vec![FfiType::Value(std::any::TypeId::of::<Holder>())],
                        result: value,
                        fallible: false,
                    },
                )
            };
            let lua = Lua::new();
            let function = super::super::native_function(
                &lua,
                (|x: f32| V(C { x })).into_dynamic_script_function(),
            )
            .unwrap();
            lua.globals().set("value", function).unwrap();
            let error = lua.load("return value(1)").exec().unwrap_err().to_string();
            assert!(error.contains("::Holder` has no C layout"), "{error}");
        });
    }
}
