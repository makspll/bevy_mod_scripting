//! LuaJIT: views, the components and resources `world.get_component` and `world.get_resource` return, read and written
//! in place (see [`bevy_mod_scripting_world::RowView`], [`bevy_mod_scripting_bindings::view`]).
//!
//! A view is a cdata of `const` fields: the address of the value, of its changed tick (null if read-only), both epochs it
//! was made in, and what it is (entity, component id) for the slow paths. Its metatype is Lua code made per type (see
//! [`view_source`]): every access first compares the epochs with the state's and the structural one, raising if either
//! changed, then reads or writes the fields with a C form through FFI loads and stores at their measured offsets; other
//! names go through a reference to the component (the slow path).
//!
//! Scripts can't make views or change their fields: the fields are `const`, the FFI library is private, and the metatype
//! is only reachable through views.

use std::{ffi::c_void, os::raw::c_int, sync::atomic::Ordering};

use bevy_ecs::{component::ComponentId, entity::Entity};
use bevy_mod_scripting_bindings::{
    CallScope, InteropError, ReflectBase, ReflectReference, ScriptValue, WorldExtensions,
    native::{FfiType, NativePlace, NativeStorage, PlaceBase},
    view::{ViewLayout, settled_view_layout, view_layout},
};
use bevy_mod_scripting_world::{ThreadWorldContainer, ViewEpoch};
use mlua::{IntoLua, LightUserData, Lua, Table, ffi};

use super::{
    Ready,
    cdata::{CTYPEID_OFFSET, LUA_TCDATA, cdata_index, cdata_tostring, private_ffi},
    key, raw_get_p, raw_set_p, run, with_raw,
};
use crate::{IntoInteropError, IntoMluaError};

/// Its address keys the epoch of the state in the registry (a light userdata, see [`ViewEpoch::as_raw`])
static VIEW_EPOCH: u8 = 0;
/// Its address keys the table `{ [ctype id] = layout (light userdata) }` in the registry, see [`view_at`]
static VIEW_CTYPES: u8 = 0;
/// Its address keys the table `{ [layout (light userdata)] = view ctype }` in the registry, see [`push_view_ctype`]
static VIEW_TYPES: u8 = 0;

/// The payload of a view, `bms_view_*` in C, see [`view_source`].
#[repr(C)]
#[derive(Clone, Copy, Debug)]
pub(crate) struct ViewData {
    data: usize,
    changed_tick: usize,
    epoch: f64,
    structural: f64,
    entity: u64,
    component: u64,
    resource: u8,
}

/// A view to push, see [`super::LuaReturn`].
pub(crate) struct PendingView {
    layout: &'static ViewLayout,
    data: ViewData,
}

/// The epoch of the state, made the first time.
pub(crate) fn epoch(lua: &Lua) -> ViewEpoch {
    if let Some(epoch) = lua.app_data_ref::<ViewEpoch>() {
        return epoch.clone();
    }
    let epoch = ViewEpoch::default();
    lua.set_app_data(epoch.clone());
    let raw = epoch.as_raw();
    // Safety: protected mode, a light userdata into the registry. Ignoring a failure only means no views are made
    let _ = unsafe {
        lua.exec_raw::<()>((), |l| {
            ffi::lua_pushlightuserdata(l, raw.cast_mut());
            raw_set_p(l, ffi::LUA_REGISTRYINDEX, key(&VIEW_EPOCH));
        })
    };
    epoch
}

/// The epoch of the state of `l`, if it has one. `[-0, +0, -]`, needs 1 free slot.
///
/// # Safety
/// `l` is a running state.
pub(crate) unsafe fn epoch_at(l: *mut ffi::lua_State) -> Option<ViewEpoch> {
    unsafe {
        raw_get_p(l, ffi::LUA_REGISTRYINDEX, key(&VIEW_EPOCH));
        let raw = ffi::lua_touserdata(l, -1);
        ffi::lua_pop(l, 1);
        // Safety: the state's app data keeps the epoch alive while the state lives
        (!raw.is_null()).then(|| ViewEpoch::from_raw(raw))
    }
}

/// The view of `reference` (a whole component or resource) returned by a call on `l`, `None` when it can't have one: the
/// state isn't the one whose script code runs (see [`ViewEpoch::enter`]), the type has no field read in place, or the
/// value is gone.
///
/// # Safety
/// `l` is the state of the call.
pub(crate) unsafe fn make_view(
    l: *mut ffi::lua_State,
    reference: &ReflectReference,
    scope: &CallScope,
) -> Result<Option<PendingView>, InteropError> {
    let (entity, component_id) = match *reference.base.base_id() {
        ReflectBase::Component(entity, id) => (Some(entity), id),
        ReflectBase::Resource(id) => (None, id),
        _ => return Ok(None),
    };
    let type_id = reference.base.type_id();
    let settled = settled_view_layout(type_id);
    if let Some(None) = settled {
        return Ok(None);
    }
    ViewEpoch::with_current(|current| unsafe {
        make_view_in(l, current, entity, component_id, type_id, settled, scope)
    })
    .unwrap_or(Ok(None))
}

/// [`make_view`] in the context whose code runs.
///
/// # Safety
/// See [`make_view`].
unsafe fn make_view_in(
    l: *mut ffi::lua_State,
    current: &ViewEpoch,
    entity: Option<Entity>,
    component_id: ComponentId,
    type_id: std::any::TypeId,
    settled: Option<Option<&'static ViewLayout>>,
    scope: &CallScope,
) -> Result<Option<PendingView>, InteropError> {
    // the state whose code runs. Safety: the state of the call, which has room for the call's results
    unsafe {
        raw_get_p(l, ffi::LUA_REGISTRYINDEX, key(&VIEW_EPOCH));
        let mine = ffi::lua_touserdata(l, -1);
        ffi::lua_pop(l, 1);
        if mine.cast_const() != current.as_raw() {
            return Ok(None);
        }
    }
    let world = scope.world()?;
    // Safety: the address is only given to the view metatype, which checks the epochs before each access (see
    // `view_source`), reads it as the type the layout was measured for, writes only scalars (numbers, `0`/`1` to `bool`)
    // and natives with a C layout, only through write views, writing `this_run` to the changed tick after each write
    // no access: a reference, which raises when it's used, like before views
    let Ok(Some(row)) = (unsafe { world.row_view(current, entity, component_id, true) }) else {
        return Ok(None);
    };
    let layout = match settled {
        Some(Some(layout)) => layout,
        _ => match unsafe { measured_layout(&world, type_id, component_id, row.data) }? {
            Some(layout) => layout,
            None => return Ok(None),
        },
    };
    Ok(Some(PendingView {
        layout,
        data: ViewData {
            data: row.data as usize,
            changed_tick: row.changed_tick as usize,
            epoch: row.epoch,
            structural: row.structural,
            entity: entity.map_or(0, Entity::to_bits),
            component: component_id.index() as u64,
            resource: entity.is_none() as u8,
        },
    }))
}

/// The view layout of `type_id`, measured on the live value at `value` if needed, see [`view_layout`].
///
/// # Safety
/// `value` is the component `component_id` of that type, alive and not borrowed mutably during the call.
unsafe fn measured_layout(
    world: &bevy_mod_scripting_bindings::WorldGuard,
    type_id: std::any::TypeId,
    component_id: ComponentId,
    value: *mut u8,
) -> Result<Option<&'static ViewLayout>, InteropError> {
    let Some(layout) = world
        .as_unsafe_world_cell()?
        .components()
        .get_info(component_id)
        .map(|info| info.layout())
    else {
        return Ok(None);
    };
    let natives = world.script_function_registry();
    let natives = natives.read();
    // natives whose C struct has scalars only: the ones every state declares
    let native_layout = |type_id| {
        let ffi = natives.native_type(type_id)?.ffi.as_ref()?;
        ffi.fields
            .iter()
            .all(|f| matches!(f.ty, FfiType::Scalar(_)))
            .then_some((ffi.size, ffi.align))
    };
    let registry = world.type_registry();
    let registry = registry.read();
    // Safety: see the function's contract, only read
    Ok(unsafe { view_layout(&registry, &native_layout, type_id, value, layout) })
}

/// Pushes the view type of `view`, making it in this state the first time. `[-0, +1, m]` on success, `[-0, +0, -]` on
/// error.
///
/// # Safety
/// `l` is the state of a call.
pub(crate) unsafe fn push_view_ctype(
    l: *mut ffi::lua_State,
    view: &PendingView,
) -> Result<(), InteropError> {
    let layout = view.layout;
    let layout_key = layout as *const ViewLayout as *mut c_void;
    unsafe {
        if ffi::lua_checkstack(l, 2) == 0 {
            return Err(InteropError::str("stack overflow"));
        }
        // [.., types]
        if raw_get_p(l, ffi::LUA_REGISTRYINDEX, key(&VIEW_TYPES)) == ffi::LUA_TTABLE {
            // [.., types, ctype]
            ffi::lua_pushlightuserdata(l, layout_key);
            if ffi::lua_rawget(l, -2) == LUA_TCDATA {
                ffi::lua_remove(l, -2);
                return Ok(());
            }
            ffi::lua_pop(l, 1);
        }
        ffi::lua_pop(l, 1);
        super::push_with_mlua(l, |lua, raw| {
            let ctype = make_view_type(lua, layout)?;
            ctype.push_into_stack(raw)?;
            Ok(1)
        })
        .map(|_| ())
        .map_err(IntoInteropError::to_bms_error)
    }
}

/// Makes the view type of `layout` in this state, see [`view_source`], and registers it.
fn make_view_type(lua: &Lua, layout: &'static ViewLayout) -> mlua::Result<mlua::Value> {
    let world = ThreadWorldContainer::try_get_context()
        .map_err(|e| InteropError::from(e).to_lua_error())?
        .world;
    // the native types of fields: their C structs are declared with the types
    // and the methods of the type, if it's a native type: looked up once, not on each call
    let (natives, own_methods) = {
        let registry = world.script_function_registry();
        let registry = registry.read();
        let natives = layout
            .fields
            .iter()
            .filter_map(|f| match f.ty {
                FfiType::Value(type_id) => registry.native_type(type_id).cloned(),
                FfiType::Scalar(_) => None,
            })
            .collect::<Vec<_>>();
        let own_methods = registry
            .native_type(layout.type_id)
            .map(|n| n.methods.clone())
            .unwrap_or_default();
        (natives, own_methods)
    };
    let methods = lua.create_table()?;
    for (name, function) in own_methods {
        methods.raw_set(
            name.as_ref(),
            super::super::script_value::registered_function_into_lua(lua, function)?,
        )?;
    }
    let native_types = lua.create_table()?;
    for native in &natives {
        let Some(info) = super::cdata::type_info(lua, native.vtable)? else {
            return Err(mlua::Error::RuntimeError(format!(
                "{} has no C type in this state",
                native.vtable.type_name
            )));
        };
        // 1 the type, 2 its `const` form
        native_types.raw_set(
            super::cdata::layout_name(native).to_owned(),
            lua.create_sequence_from([info.raw_get::<mlua::Value>(1)?, info.raw_get(2)?])?,
        )?;
    }
    let type_name = |type_id| {
        natives
            .iter()
            .find(|n| n.vtable.type_id == type_id)
            .map(|n| super::cdata::layout_name(n).to_owned())
    };
    let source = view_source(layout, &type_name);
    let epoch = epoch(lua);
    let structural = bevy_mod_scripting_world::structural_epoch();
    let this_run = epoch.this_run_ptr();
    // Safety: the functions follow the Lua C API
    let (index, tostring, to_reference) = unsafe {
        (
            lua.create_c_function(cdata_index)?,
            lua.create_c_function(cdata_tostring)?,
            lua.create_c_function(view_reference)?,
        )
    };
    let check = view_check as *const () as usize;
    // the type, its id, and a probe value
    let (ctype, id, probe): (mlua::Value, i64, mlua::Value) =
        lua.load(source).set_name("=bms ffi types").call((
            private_ffi(lua)?,
            LightUserData(epoch.as_ptr().cast_mut().cast()),
            LightUserData(structural.cast_mut().cast()),
            LightUserData(this_run.cast_mut().cast()),
            index,
            tostring,
            to_reference,
            LightUserData(check as *mut c_void),
            native_types,
            methods,
            LightUserData(view_refresh as *const () as *mut c_void),
        ))?;
    let layout_key = LightUserData(layout as *const ViewLayout as *mut c_void);
    // the C types of views: `{ [id] = layout }`, `{ [layout] = ctype }`
    for (slot, k, v) in [
        (
            &VIEW_CTYPES,
            mlua::Value::Integer(id as _),
            layout_key.into_lua(lua)?,
        ),
        (&VIEW_TYPES, layout_key.into_lua(lua)?, ctype.clone()),
    ] {
        // Safety: protected mode, raw gets and sets in the registry
        let table = unsafe {
            lua.exec_raw::<Option<Table>>((), |l| {
                raw_get_p(l, ffi::LUA_REGISTRYINDEX, key(slot));
            })
        }?;
        let table = match table {
            Some(table) => table,
            None => {
                let table = lua.create_table()?;
                // Safety: see above
                unsafe {
                    lua.exec_raw::<()>(table.clone(), |l| {
                        raw_set_p(l, ffi::LUA_REGISTRYINDEX, key(slot))
                    })
                }?;
                table
            }
        };
        table.raw_set(k, v)?;
    }
    // cdata are told apart by their ctype id: no native type made, so the offset of the id isn't found yet
    if CTYPEID_OFFSET.load(Ordering::Acquire) == 0 {
        super::cdata::calibrate(&probe, id)?;
    }
    Ok(ctype)
}

/// The Lua source making the view type of `layout`, called with
/// `(ffi, epoch, structural, this_run, index, tostring, to_reference, check, natives)`:
///
/// ```lua
/// ffi.cdef[[
/// typedef struct __attribute__((aligned(8))) { uint8_t _bms_pad0[24]; float t; .. } bms_vd_55d0..;
/// typedef struct { const uintptr_t _bms_p; const uintptr_t _bms_t; const double _bms_e; const double _bms_g;
///     const uint64_t _bms_ent; const uint64_t _bms_cid; const uint8_t _bms_res; } bms_view_55d0..;
/// ]]
/// getters["t"] = function(p) return p.t end
/// setters["t"] = function(p, x) if type(x) ~= "number" then bad("t", x, "a number") end p.t = x end
/// __index = function(h, k)
///     if h._bms_e ~= L[0] or h._bms_g ~= G[0] then stale() end
///     local g = getters[k] if g ~= nil then return g(cast(P, h._bms_p)) end
///     return index(h, k)
/// end
/// ```
fn view_source(
    layout: &ViewLayout,
    native_name: &dyn Fn(std::any::TypeId) -> Option<String>,
) -> String {
    use std::fmt::Write as _;
    let tag = format!("{:x}", layout as *const ViewLayout as usize);
    let name = layout.type_name;
    let mut out = String::from(
        "local ffi, L, G, RUN, index, tostring_, to_reference, check, natives, methods, refresh = ...\n\
         local istype, cast, type, error, tostring = ffi.istype, ffi.cast, type, error, tostring\n\
         L, G, RUN = cast(\"const double*\", L), cast(\"const double*\", G), cast(\"const uint32_t*\", RUN)\n",
    );
    // the value: the fields at their offsets, padding elsewhere
    let mut fields = String::new();
    let mut end = 0;
    for field in &layout.fields {
        if field.offset > end {
            let _ = write!(fields, "uint8_t _bms_pad{end}[{}]; ", field.offset - end);
        }
        let c_type = match field.ty {
            FfiType::Scalar(c) => c.to_owned(),
            FfiType::Value(type_id) => native_name(type_id).unwrap_or_default(),
        };
        // prefixed: Rust names may be C keywords (`float`)
        let _ = write!(fields, "{c_type} f_{}; ", field.name);
        end = field.offset + field.size;
    }
    if layout.size > end {
        let _ = write!(fields, "uint8_t _bms_pad{end}[{}]; ", layout.size - end);
    }
    let _ = writeln!(
        out,
        "ffi.cdef[[\ntypedef struct __attribute__((aligned({align}))) {{ {fields}}} bms_vd_{tag};\n\
         typedef struct {{ const uintptr_t _bms_p; const uintptr_t _bms_t; const double _bms_e; const double _bms_g; \
         const uint64_t _bms_ent; const uint64_t _bms_cid; const uint8_t _bms_res; }} bms_view_{tag};\n\
         typedef struct {{ uintptr_t p; uintptr_t t; double e; double g; uint64_t ent; uint64_t cid; uint8_t res; }} bms_vs_{tag};\n]]\n\
         local D, P, T = ffi.typeof(\"bms_vd_{tag}\"), ffi.typeof(\"bms_vd_{tag}*\"), ffi.typeof(\"uint32_t*\")\n\
         local S = ffi.typeof(\"bms_vs_{tag}*\")\n\
         local REFRESH = cast(\"bool (*)(bms_vs_{tag}*, const double*)\", refresh)",
        align = layout.align,
    );
    // L1: C agrees with the measured layout
    let _ = write!(
        out,
        "assert(ffi.sizeof(D) == {} and ffi.alignof(D) == {}",
        layout.size, layout.align
    );
    for field in &layout.fields {
        let _ = write!(
            out,
            " and ffi.offsetof(D, \"f_{}\") == {}",
            field.name, field.offset
        );
    }
    let _ = writeln!(out, ", \"the view of {name} has another layout in C\")");
    let _ = writeln!(
        out,
        "local function gone() error(\"the view of {name} can't be refreshed: its entity or component is gone, or this system has no access to it\", 3) end\n\
         local function bad(k, x, expected) error(\"{name}.\" .. k .. \" expects \" .. expected .. \", got \" .. type(x), 3) end\n\
         local getters, setters = {{}}, {{}}"
    );
    for field in &layout.fields {
        let k = field.name;
        let (get, set) = match field.ty {
            FfiType::Scalar("float" | "double") => (
                format!("return p.f_{k}"),
                format!(
                    "if type(x) ~= \"number\" then bad(\"{k}\", x, \"a number\") end p.f_{k} = x"
                ),
            ),
            FfiType::Scalar("bool") => (
                format!("return p.f_{k}"),
                format!(
                    "if type(x) ~= \"boolean\" then bad(\"{k}\", x, \"a boolean\") end p.f_{k} = x"
                ),
            ),
            FfiType::Scalar(c) => {
                let (min, max) = int_range(c);
                // 64 bits are boxed by LuaJIT: numbers, like elsewhere
                let get = if field.size == 8 {
                    format!("return tonumber(p.f_{k})")
                } else {
                    format!("return p.f_{k}")
                };
                (
                    get,
                    format!(
                        "if type(x) ~= \"number\" or x % 1 ~= 0 or x < {min} or x > {max} then bad(\"{k}\", x, \"an integer of {c}\") end p.f_{k} = x"
                    ),
                )
            }
            FfiType::Value(type_id) => {
                let native = native_name(type_id).unwrap_or_default();
                (
                    // a read-only copy, written whole, like fields of native values
                    format!("return natives.{native}[2](p.f_{k})"),
                    format!(
                        "if not istype(natives.{native}[1], x) then bad(\"{k}\", x, \"a {native}\") end p.f_{k} = x"
                    ),
                )
            }
        };
        let _ = writeln!(
            out,
            "getters[\"{k}\"] = function(p) {get} end\nsetters[\"{k}\"] = function(p, x) {set} end"
        );
    }
    // P1: every access re-resolves the view through Bevy in debug builds, catching a missing epoch bump
    let check = if cfg!(debug_assertions) {
        "if not CHECK(s.ent, s.cid, s.res, s.p) then error(\"view check failed: the view of NAME points elsewhere\", 2) end"
            .replace("NAME", name)
    } else {
        String::new()
    };
    let _ = writeln!(
        out,
        "local CHECK = cast(\"bool (*)(uint64_t, uint64_t, uint8_t, uintptr_t)\", check)\n\
         local mt = {{\n\
         __index = function(h, k)\n\
           local s = cast(S, h)\n\
           if s.e ~= L[0] or s.g ~= G[0] then if not REFRESH(s, L) then gone() end end\n\
           {check}\n\
           local g = getters[k] if g ~= nil then return g(cast(P, s.p)) end\n\
           local m = methods[k] if m ~= nil then return m end\n\
           return index(h, k)\n\
         end,\n\
         __newindex = function(h, k, x)\n\
           local s = cast(S, h)\n\
           if s.e ~= L[0] or s.g ~= G[0] then if not REFRESH(s, L) then gone() end end\n\
           {check}\n\
           local set = setters[k]\n\
           if set == nil then to_reference(h)[k] = x return end\n\
           if s.t == 0 then error(\"cannot claim access to write {name}: the view is read-only\", 2) end\n\
           set(cast(P, s.p), x)\n\
           cast(T, s.t)[0] = RUN[0]\n\
         end,\n\
         __tostring = tostring_,\n\
         }}\n\
         local V = ffi.metatype(\"bms_view_{tag}\", mt)\n\
         return V, tonumber(V), V()"
    );
    out
}

/// The range of an integer C type, as Lua numbers.
fn int_range(c: &str) -> (String, String) {
    let (min, max): (f64, f64) = match c {
        "int8_t" => (i8::MIN as f64, i8::MAX as f64),
        "int16_t" => (i16::MIN as f64, i16::MAX as f64),
        "int32_t" => (i32::MIN as f64, i32::MAX as f64),
        "uint8_t" => (0.0, u8::MAX as f64),
        "uint16_t" => (0.0, u16::MAX as f64),
        "uint32_t" => (0.0, u32::MAX as f64),
        "uint64_t" | "uintptr_t" => (0.0, u64::MAX as f64),
        _ => (i64::MIN as f64, i64::MAX as f64),
    };
    (format!("{min:?}"), format!("{max:?}"))
}

/// The view at `idx`, if it's one: its layout and payload. `[-0, +0, -]`, needs 2 free slots.
///
/// # Safety
/// `idx` is on the stack of `l`.
pub(crate) unsafe fn view_at(
    l: *mut ffi::lua_State,
    idx: c_int,
) -> Option<(&'static ViewLayout, ViewData)> {
    unsafe {
        if ffi::lua_type(l, idx) != LUA_TCDATA {
            return None;
        }
        let offset = CTYPEID_OFFSET.load(Ordering::Acquire);
        if offset == 0 {
            return None;
        }
        let payload = ffi::lua_topointer(l, idx) as *const u8;
        // Safety: every cdata starts with its header, which ends with the ctype id, see `cdata_at`
        let id = (payload.sub(offset) as *const u16).read_unaligned();
        if raw_get_p(l, ffi::LUA_REGISTRYINDEX, key(&VIEW_CTYPES)) != ffi::LUA_TTABLE {
            ffi::lua_pop(l, 1);
            return None;
        }
        ffi::lua_rawgeti(l, -1, id as ffi::lua_Integer);
        let layout = ffi::lua_touserdata(l, -1) as *const ViewLayout;
        ffi::lua_pop(l, 2);
        // Safety: only layouts are kept in the table, they live for the program; views of the type have its payload
        (!layout.is_null()).then(|| (&*layout, (payload as *const ViewData).read_unaligned()))
    }
}

/// The view at `idx` as a script value: a reference to its component or resource. Stale views too: references check what
/// they point to.
///
/// # Safety
/// See [`view_at`].
pub(crate) unsafe fn view_script_value(l: *mut ffi::lua_State, idx: c_int) -> Option<ScriptValue> {
    let (layout, data) = unsafe { view_at(l, idx) }?;
    let component = ComponentId::new(data.component as usize);
    // Safety: views are made of components and resources of their layout's type
    let reference = unsafe {
        if data.resource != 0 {
            ReflectReference::new_resource_ref_by_id(component, layout.type_id)
        } else {
            ReflectReference::new_component_ref_by_id(
                Entity::try_from_bits(data.entity)?,
                component,
                layout.type_id,
            )
        }
    };
    Some(ScriptValue::Reference(reference))
}

/// The view at `idx` as a place of its native type (see [`NativePlace`]), stored in `room`, if it's a view of a native
/// type: typed functions then reach the component like through `get_component`'s places on other Lua versions (claiming
/// access for the call). Stale views too: places check what they point to. `[-0, +0, -]`, needs 2 free slots.
///
/// # Safety
/// `idx` is on the stack of `l`, `room` points to `size` writable bytes aligned to [`NativeStorage::ALIGN`], alive for
/// the call.
pub(crate) unsafe fn view_place(
    l: *mut ffi::lua_State,
    idx: c_int,
    room: *mut u8,
    size: usize,
) -> Option<*mut u8> {
    let (layout, data) = unsafe { view_at(l, idx) }?;
    let world = ThreadWorldContainer::try_get_context().ok()?.world;
    let registry = world.script_function_registry();
    let registry = registry.read();
    let vtable = registry.native_type(layout.type_id)?.vtable.place_vtable();
    if NativeStorage::size(vtable) > size {
        return None;
    }
    let id = ComponentId::new(data.component as usize);
    let base = if data.resource != 0 {
        PlaceBase::Resource(id)
    } else {
        PlaceBase::Component(Entity::try_from_bits(data.entity)?, id)
    };
    let mut place = Some(NativePlace::new(base, layout.type_id));
    // Safety: room for the storage, aligned, see the function's contract; the place vtable stores a `NativePlace`
    unsafe {
        NativeStorage::init(room, vtable, &mut |slot| {
            if let Some(place) = place.take() {
                (slot as *mut NativePlace).write(place)
            }
        })
    };
    Some(room)
}

/// The reference to the component or resource of a view, for the slow path of `__newindex`. Called with `[view (1)]`.
unsafe extern "C-unwind" fn view_reference(l: *mut ffi::lua_State) -> c_int {
    unsafe {
        ffi::lua_settop(l, 1);
        run(l, || {
            let Some(value) = view_script_value(l, 1) else {
                return Err(InteropError::str("expected a view"));
            };
            with_raw(l, |lua, _| {
                super::super::script_value::LuaScriptValue(value).into_lua(lua)
            })
            .map(Ready::Value)
            .map_err(IntoInteropError::to_bms_error)
        })
    }
}

/// Refreshes a view whose epochs changed: finds its component again (by entity and component id, so a moved row is
/// found), claiming access under the running callback's world, and writes the new address, tick and epochs into it.
/// False if it's gone or can't be accessed, or the view isn't of the state whose code runs (`epoch`). Called through the
/// FFI: it doesn't touch the Lua state.
///
/// The view is read and written through a non-`const` pointer: LuaJIT may reuse loads of `const` fields across calls.
extern "C" fn view_refresh(view: *mut ViewData, epoch: *const f64) -> bool {
    std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        ViewEpoch::with_current(|current| refresh_in(current, view, epoch)).unwrap_or(false)
    }))
    .unwrap_or(false)
}

/// [`view_refresh`] in the context whose code runs.
fn refresh_in(current: &ViewEpoch, view: *mut ViewData, epoch: *const f64) -> bool {
    if current.as_ptr() != epoch {
        return false;
    }
    let Ok(context) = ThreadWorldContainer::try_get_context() else {
        return false;
    };
    // Safety: the view's payload, alive for the call (Lua passed it)
    let mut data = unsafe { view.read_unaligned() };
    let id = ComponentId::new(data.component as usize);
    let entity = match data.resource {
        0 => match Entity::try_from_bits(data.entity) {
            Some(entity) => Some(entity),
            None => return false,
        },
        _ => None,
    };
    // Safety: as in `make_view`: the same component, so the same type and layout
    let Ok(Some(row)) = (unsafe { context.world.row_view(current, entity, id, true) }) else {
        return false;
    };
    data.data = row.data as usize;
    data.changed_tick = row.changed_tick as usize;
    data.epoch = row.epoch;
    data.structural = row.structural;
    // Safety: see above
    unsafe { view.write_unaligned(data) };
    true
}

/// P1 (debug builds): whether a view's address is still where Bevy has its component or resource. Called through the FFI
/// before every access, so a structural change which didn't bump the epoch is caught instead of corrupting the world.
extern "C" fn view_check(entity: u64, component: u64, resource: u8, address: usize) -> bool {
    std::panic::catch_unwind(|| {
        let Ok(context) = ThreadWorldContainer::try_get_context() else {
            return false;
        };
        let Ok(cell) = context.world.as_unsafe_world_cell() else {
            return false;
        };
        let id = ComponentId::new(component as usize);
        let entity = if resource != 0 {
            // Safety: only the resource's entity is read
            unsafe { cell.resource_entities() }.get(id)
        } else {
            Entity::try_from_bits(entity)
        };
        let Some(Ok(entity)) = entity.map(|e| cell.get_entity(e)) else {
            return false;
        };
        // Safety: only the address is compared, nothing is read
        unsafe { entity.get_by_id(id) }.is_some_and(|ptr| ptr.as_ptr() as usize == address)
    })
    .unwrap_or(false)
}

/// Pushes a view, its type on the top (see [`push_view_ctype`]): `[.., ctype] -> [.., view]`. `[-1, +1, m]`.
///
/// # Safety
/// `l` is the state of a call.
pub(crate) unsafe fn push_view(l: *mut ffi::lua_State, view: PendingView) {
    unsafe {
        // calling the type makes a zeroed value (a C function of LuaJIT, no Lua code runs), its `const` fields are
        // written here
        ffi::lua_call(l, 0, 1);
        let payload = ffi::lua_topointer(l, -1) as *mut ViewData;
        payload.write_unaligned(view.data);
    }
}
