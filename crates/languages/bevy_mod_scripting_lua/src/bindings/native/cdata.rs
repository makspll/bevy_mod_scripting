//! LuaJIT: native types with a C layout ([`FfiLayout`]) are cdata.
//!
//! - The FFI library is loaded privately (`luaopen_ffi`, never a global or `require`-able), the first time a value of
//!   such a type reaches Lua.
//! - Each type is a C struct (`ffi.cdef`) with a metatype: fields are plain FFI fields; functions with an `extern "C"`
//!   wrapper (`#[script_bindings(ffi)]`, see [`FfiFunction`]) are called through the FFI with C types, after checking
//!   their arguments are of the expected types, else (and for other functions) through BMS' typed functions, which read
//!   cdata in place ([`cdata_at`], [`super::LuaArgs`]); arithmetic of component-wise types is computed in Lua.
//! - Values Rust returns are copied into new cdata ([`push_from_ptr`]); scripts can write every byte of a cdata at any
//!   time, so BMS' functions never borrow one: they get copies, written back for `M<T>` (see `CallScope::copy_plain`).
//! - Types are told apart by LuaJIT's ctype id, which scripts can't change, see [`cdata_at`].

use std::{
    ffi::c_void,
    fmt::Write as _,
    os::raw::c_int,
    sync::atomic::{AtomicUsize, Ordering},
};

use bevy_mod_scripting_bindings::{
    InteropError, ScriptValue, WorldExtensions, dispatch,
    native::{FfiFunction, FfiType, NativeType, NativeVTable},
};
use bevy_mod_scripting_bindings_domain::ScriptOperatorNames;
use bevy_mod_scripting_world::ThreadWorldContainer;
use mlua::{IntoLua, LightUserData, Lua, Table, ffi};

use super::{LUA_CALLER_CONTEXT, LuaScriptValue, Ready, key, raw_get_p, raw_set_p, run, with_raw};
use crate::{IntoInteropError, IntoMluaError};

/// LuaJIT's type tag of cdata, not in the Lua C API.
pub(crate) const LUA_TCDATA: c_int = 10;

/// The offset from a cdata's payload back to its ctype id (`GCcdata::ctypeid`): 6 on GC64 builds, 2 on others, found
/// when the first types are made, 0 before.
static CTYPEID_OFFSET: AtomicUsize = AtomicUsize::new(0);

/// Its address keys the table `{ [ctype id] = vtable (light userdata) }` in the registry, see [`cdata_at`].
static CTYPES: u8 = 0;

/// Its address keys the table `{ [vtable] = from_ptr }` in the registry, see [`push_from_ptr`].
static FROM_PTRS: u8 = 0;

/// The named registry value of the table `{ [vtable] = { ctype, from_ptr, statics } }`.
const TYPES: &str = "bms_ffi_types";

/// The native value at `idx` if it's a cdata of a native type with a C layout: its vtable and its payload, alive while
/// the value is on the stack. `[-0, +0, -]`, but needs 2 free slots.
///
/// # Safety
/// `idx` is on the stack of `l`.
pub(crate) unsafe fn cdata_at(
    l: *mut ffi::lua_State,
    idx: c_int,
) -> Option<(&'static NativeVTable, *mut u8)> {
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
        let vtable = ffi::lua_touserdata(l, -1) as *const NativeVTable;
        ffi::lua_pop(l, 2);
        // Safety: only vtables are kept in the table
        (!vtable.is_null()).then(|| (&*vtable, payload))
    }
}

/// The cdata at `idx` as a script value, a reference to a copy, like other native values (see
/// `NativeStorage::to_script_value`). `None` if it's no cdata of a native type.
///
/// # Safety
/// `idx` is on the stack of `l`, which has 2 free slots.
pub(crate) unsafe fn cdata_script_value(l: *mut ffi::lua_State, idx: c_int) -> Option<ScriptValue> {
    unsafe {
        let (vtable, payload) = cdata_at(l, idx)?;
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

/// Pushes the function making a cdata of the type of `vtable` from a pointer to a value, making the types first if this
/// state has none yet. `[-0, +(0|1), m]`: nothing for types registered after the types were made, which stay userdata.
///
/// # Safety
/// `l` is the state of a call.
pub(super) unsafe fn push_from_ptr(
    l: *mut ffi::lua_State,
    vtable: &'static NativeVTable,
) -> Result<bool, InteropError> {
    unsafe {
        // made already: `from_ptrs[vtable]`, raw gets only
        if ffi::lua_checkstack(l, 2) != 0 {
            // [.., from_ptrs]
            if raw_get_p(l, ffi::LUA_REGISTRYINDEX, key(&FROM_PTRS)) == ffi::LUA_TTABLE {
                // [.., from_ptrs, from_ptr | nil]
                ffi::lua_pushlightuserdata(l, vtable as *const NativeVTable as *mut c_void);
                let made = ffi::lua_rawget(l, -2) == ffi::LUA_TFUNCTION;
                if made {
                    ffi::lua_remove(l, -2);
                } else {
                    ffi::lua_pop(l, 2);
                }
                // without the type, the types were made before it was registered
                return Ok(made);
            }
            ffi::lua_pop(l, 1);
        }
        // the first value of such a type in this state: make the types
        super::push_with_mlua(l, |lua, raw| {
            let Some(info) = type_info(lua, vtable)? else {
                return Ok(0);
            };
            let from_ptr: mlua::Function = info.raw_get(2)?;
            from_ptr.push_into_stack(raw)?;
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
    let types = match lua.named_registry_value::<Option<Table>>(TYPES)? {
        Some(types) => types,
        None => {
            make_types(lua)?;
            lua.named_registry_value::<Table>(TYPES)?
        }
    };
    types.raw_get::<Option<Table>>(key)
}

/// Makes the FFI forms of every native type with a C layout in this state, see the module docs.
fn make_types(lua: &Lua) -> mlua::Result<()> {
    let world = ThreadWorldContainer::try_get_context()
        .map_err(|e| InteropError::from(e).to_lua_error())?
        .world;
    let (types, functions) = {
        let registry = world.script_function_registry();
        let registry = registry.read();
        let types = registry
            .native_types()
            .filter(|n| n.ffi.is_some())
            .cloned()
            .collect::<Vec<_>>();
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
    let pointers = pointers
        .into_iter()
        .map(|p| LightUserData(p as *mut c_void))
        .collect::<Vec<_>>();
    // Safety: the functions follow the Lua C API
    let (index, tostring) = unsafe {
        (
            lua.create_c_function(cdata_index)?,
            lua.create_c_function(cdata_tostring)?,
        )
    };
    let made: Table = lua
        .load(source)
        .set_name("=bms ffi types")
        .call((ffi_module, pointers, slows, index, tostring))?;
    // by vtable for Rust, by ctype id for C
    let by_vtable = lua.create_table()?;
    let by_id = lua.create_table()?;
    let from_ptrs = lua.create_table()?;
    for native in &types {
        let info: Table = made.raw_get(layout_name(native))?;
        let id: i64 = info.raw_get(4)?;
        let vtable = LightUserData(native.vtable as *const NativeVTable as *mut c_void);
        by_vtable.raw_set(vtable, info.clone())?;
        by_id.raw_set(id, vtable)?;
        from_ptrs.raw_set(vtable, info.raw_get::<mlua::Function>(2)?)?;
        calibrate(&info.raw_get::<mlua::Value>(5)?, id)?;
    }
    lua.set_named_registry_value(TYPES, by_vtable)?;
    // Safety: protected mode, raw sets of the tables on the top into the registry
    unsafe {
        lua.exec_raw::<()>(by_id, |l| {
            raw_set_p(l, ffi::LUA_REGISTRYINDEX, key(&CTYPES))
        })?;
        lua.exec_raw::<()>(from_ptrs, |l| {
            raw_set_p(l, ffi::LUA_REGISTRYINDEX, key(&FROM_PTRS))
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
        "local ffi, ptrs, slows, index, tostring_ = ...\nlocal istype, cast = ffi.istype, ffi.cast\nffi.cdef[[\n",
    );
    for layout in types.iter().filter_map(|n| n.ffi.as_ref()) {
        let _ = writeln!(out, "{}", layout.c_declaration());
    }
    out.push_str("]]\n");
    for (i, native) in types.iter().enumerate() {
        let _ = writeln!(
            out,
            "local C{} = ffi.typeof(\"{}\")",
            i + 1,
            layout_name(native)
        );
    }
    out.push_str("local made = {}\n");
    for (i, (native, functions)) in types.iter().zip(functions).enumerate() {
        // only types with a C layout are given
        let Some(layout) = native.ffi.as_ref() else {
            continue;
        };
        let t = format!("C{}", i + 1);
        let name = layout_name(native);
        let _ = writeln!(
            out,
            "do\nlocal T, P, slow, methods, statics = {t}, ffi.typeof(\"const {name}*\"), slows.{name}, {{}}, {{}}"
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
        out.push_str(
            "local mt = { __index = function(v, k) local m = methods[k] if m ~= nil then return m end return index(v, k) end,\n__tostring = tostring_ }\n",
        );
        operators_source(&mut out, native, layout.component_wise);
        let _ = writeln!(
            out,
            "T = ffi.metatype(T, mt)\nmade.{name} = {{ T, function(p) return T(cast(P, p)[0]) end, statics, tonumber(T), T() }}\nend"
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
            FfiType::Mut(type_id) => {
                let (lua_ctype, c_name) = ctype(*type_id)?;
                c_params.push(format!("{c_name}*"));
                guards.push(format!("istype({lua_ctype}, {param})"));
            }
            FfiType::Unit => return None,
        }
    }
    // a value written through a pointer must not be read through another (`v:op_assign(v)`): through BMS then, which
    // checks borrows
    for (i, kind) in function.params.iter().enumerate() {
        if matches!(kind, FfiType::Mut(_)) {
            for (j, other) in function.params.iter().enumerate() {
                if i != j && !matches!(other, FfiType::Scalar(_)) {
                    guards.push(format!("not rawequal({}, {})", params[i], params[j]));
                }
            }
        }
    }
    let args = params.join(", ");
    let (c_result, call) = match function.result {
        FfiType::Unit => ("void".to_owned(), format!("f{pointer}({args}) return")),
        // 64 bit integers are cdata in LuaJIT, BMS gives numbers
        FfiType::Scalar(c @ ("int64_t" | "uint64_t" | "size_t")) => {
            (c.to_owned(), format!("return tonumber(f{pointer}({args}))"))
        }
        FfiType::Scalar(c) => (c.to_owned(), format!("return f{pointer}({args})")),
        FfiType::Value(type_id) => {
            let (lua_ctype, c_name) = ctype(type_id)?;
            c_params.push(format!("{c_name}*"));
            let out_args = match args.is_empty() {
                true => "r".to_owned(),
                false => format!("{args}, r"),
            };
            (
                "void".to_owned(),
                format!("local r = {lua_ctype}() f{pointer}({out_args}) return r"),
            )
        }
        FfiType::Mut(_) => return None,
    };
    let guard = match guards.is_empty() {
        true => "true".to_owned(),
        false => guards.join(" and "),
    };
    let name = function.name;
    Some(format!(
        "local f{pointer} = cast(\"{c_result} (*)({})\", ptrs[{pointer}])\nlocal function w{pointer}({args}) if {guard} then {call} end return slow.{name}({args}) end\nmethods.{name} = w{pointer} statics.{name} = w{pointer}\n",
        c_params.join(", ")
    ))
}

/// The Lua source of the operators of a type's metatable `mt`: component-wise ones computed in Lua (`a + b`, `v * 2`,
/// `-v`, `a == b`), the others through BMS' functions, like native values' (see `OPERATORS`).
fn operators_source(out: &mut String, native: &NativeType, component_wise: bool) {
    let fields = native
        .ffi
        .as_ref()
        .map(|l| l.fields.iter().map(|f| f.name).collect::<Vec<_>>())
        .unwrap_or_default();
    let each =
        |f: &dyn Fn(&str) -> String| fields.iter().map(|n| f(n)).collect::<Vec<_>>().join(", ");
    let has = |name: &str| native.methods.contains_key(name);
    // calls the BMS function `name`, else raises like BMS does for a missing operator function
    let call = |name: Option<&str>, args: &str| match name {
        Some(name) if has(name) => format!("return slow.{name}({args})"),
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
        let binary_lua = match operator {
            ScriptOperatorNames::Addition => Some("+"),
            ScriptOperatorNames::Subtraction => Some("-"),
            ScriptOperatorNames::Multiplication => Some("*"),
            ScriptOperatorNames::Division => Some("/"),
            _ => None,
        };
        match operator {
            ScriptOperatorNames::DisplayPrint => continue,
            _ if component_wise && binary_lua.is_some() => {
                let op = binary_lua.unwrap_or_default();
                let vv = each(&|n| format!("a.{n} {op} b.{n}"));
                let vs = each(&|n| format!("a.{n} {op} b"));
                let sv = each(&|n| format!("a {op} b.{n}"));
                let other = call(Some(forward), "a, b");
                let reflected = call(reflected, "b, a");
                let _ = writeln!(
                    out,
                    "mt.{meta} = function(a, b)\nif istype(T, a) then\nif istype(T, b) then return T({vv}) end\nif type(b) == \"number\" then return T({vs}) end\n{other}\nend\nif type(a) == \"number\" then return T({sv}) end\n{reflected}\nend"
                );
            }
            ScriptOperatorNames::Negation if component_wise => {
                let neg = each(&|n| format!("-a.{n}"));
                let _ = writeln!(out, "mt.{meta} = function(a) return T({neg}) end");
            }
            ScriptOperatorNames::Equality => {
                let fields_equal = fields
                    .iter()
                    .map(|n| format!("a.{n} == b.{n}"))
                    .collect::<Vec<_>>()
                    .join(" and ");
                let same = match component_wise {
                    true => {
                        format!("if istype(T, a) and istype(T, b) then return {fields_equal} end\n")
                    }
                    false => String::new(),
                };
                let other = match has(forward) {
                    true => format!(
                        "local ta, tb = type(a), type(b)\nif (ta == \"cdata\" or ta == \"userdata\") and (tb == \"cdata\" or tb == \"userdata\") then return slow.{forward}(a, b) end\n"
                    ),
                    false => String::new(),
                };
                let _ = writeln!(
                    out,
                    "mt.{meta} = function(a, b)\n{same}{other}return false\nend"
                );
            }
            ScriptOperatorNames::Negation | ScriptOperatorNames::Length => {
                if has(forward) {
                    let _ = writeln!(out, "mt.{meta} = function(a) return slow.{forward}(a) end");
                }
            }
            ScriptOperatorNames::LessThanComparison => {
                if has(forward) {
                    let _ = writeln!(
                        out,
                        "mt.{meta} = function(a, b) return slow.{forward}(a, b) end"
                    );
                    let _ = writeln!(
                        out,
                        "mt.__le = function(a, b) return not slow.{forward}(b, a) end"
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
/// slow path, see [`dispatch::index`]. Called with `[v (1), key (2)]`.
unsafe extern "C-unwind" fn cdata_index(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        ffi::lua_settop(l, 2);
        run(l, || {
            let receiver = cdata_script_value(l, 1)
                .ok_or_else(|| InteropError::str("expected a BMS cdata"))?;
            let ScriptValue::Reference(receiver) = receiver else {
                return Err(InteropError::str("expected a reference"));
            };
            let world = ThreadWorldContainer::try_get_context()?.world;
            with_raw(l, |lua, raw| {
                let key = super::value_at(raw, l, 2)?;
                let value = match dispatch::index(world, receiver, key, LUA_CALLER_CONTEXT)? {
                    dispatch::Member::Method(function) => {
                        super::super::script_value::registered_function_into_lua(lua, function)
                    }
                    dispatch::Member::Static(value) => LuaScriptValue(value).into_lua(lua),
                };
                value
                    .map(Ready::Value)
                    .map_err(IntoInteropError::to_bms_error)
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
        V,
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
        bevy_mod_scripting_bindings::script_native!(&mut world, ffi vector C { x: f32 });
        let cache = WorldAccessGuard::setup_cache(&world, CurrentScriptAttachment::empty());
        WorldAccessGuard::with_static_guard(&mut world, cache, |_| {
            let lua = Lua::new();
            let globals = [
                (
                    "value",
                    (|x: f32| V(C { x })).into_dynamic_script_function(),
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
                assert((a + a).x == 2 and (a * 3).x == 3 and a == value(1), "operators in Lua")
                local x = call_back(a, function() a.x = 5 end)
                assert(x == 1 and a.x == 5, "the call reads its copy, the script's write stays")
                call_back_mut(a, function() assert(a.x == 5) end)
                assert(a.x == 6, "written back when the call ends")
                "#,
            )
            .exec()
            .unwrap();
        });
    }
}
