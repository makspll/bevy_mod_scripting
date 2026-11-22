# Lua

How the Lua glue implements the [shared model](./introduction.md). The code is in `bevy_mod_scripting_lua/src/bindings/native.rs`. On LuaJIT, some native values are cdata instead, see [LuaJIT](./luajit.md).

mlua owns the state, loads scripts and calls into Lua. The hot paths below go around it, with the raw C API.

## What each value is in Lua

```mermaid
flowchart TD
    subgraph lua ["Lua"]
        NV["native value<br/>raw userdata"]
        RF["reference<br/>raw userdata"]
        TO["type (Vec3)<br/>table"]
        FN["function<br/>C closure"]
    end
    NV -- "metatable, one per type" --> NMT["{ __index, __newindex, operators,<br/>marker, __metatable = false }"]
    RF -- "one of two metatables" --> RMT["{ __index, __newindex, operators,<br/>marker, __metatable = false, (__gc) }"]
    TO -- "metatable, one per table" --> TMT["{ inner value, __index = cache,<br/>__newindex raises }"]
    FN -- "upvalue 1" --> HD["holder userdata<br/>DynamicScriptFunction"]
```

Values are told apart by a **marker** in their metatable, keyed by a light userdata. Lua code can't make light userdata and can't reach these metatables (`__metatable` is `false`), so only BMS' values have it.

### Native values

```mermaid
flowchart LR
    subgraph ud ["userdata block (lua_newuserdata)"]
        direction LR
        P["align padding"] --- H["NativeHeader"] --- V["value"]
    end
```

Lua only aligns userdata to its own maximum, so the block has room to round up (`align_up`).

### References

A `ReflectReference` written into a raw userdata, no box. There are two metatables: references to whole components and resources own no heap memory, so theirs has no `__gc`, which makes collecting them much cheaper.

### Types

`Vec3` is an empty table. The functions found through it are kept in the table which is its `__index`, so `Vec3.new` is a plain field after the first lookup.

```mermaid
flowchart LR
    T["t = Vec3<br/>(always empty)"] -- "metatable" --> MT["mt<br/>marker = inner value<br/>__newindex = raise<br/>__metatable = false"]
    MT -- "__index" --> C["c: function cache<br/>new, zero, .."]
    C -- "metatable" --> M["m<br/>__index = type_object_index"]
    M -. "miss: look up, store in c" .-> C
```

`t` stays empty so every write reaches `__newindex`.

## A function call

A registered function is a C closure (`native_entry`) reading its arguments off the stack where they are.

```mermaid
flowchart TD
    S["v:dot(w)"] --> E["native_entry<br/>stack: [v, w]"]
    E --> Q{"any argument which isn't a<br/>number, bool, nil or native?"}
    Q -- "no" --> NOLOCK["mlua is never touched"]
    Q -- "yes" --> LOCK["mlua locked once<br/>for the call"]
    NOLOCK --> CW["function.call_with(LuaArgs, LuaReturn)"]
    LOCK --> CW
    CW --> PEND["result kept as plain data<br/>(Pending)"]
    PEND --> PREP["prepare: the metatable pushed<br/>(made in protected mode if new)"]
    PREP --> DROP["every Rust value of the call is dropped"]
    DROP --> PUSH["push_ready: userdata allocated,<br/>result copied in"]
```

Arguments stay on the stack for the whole call, so nothing they point to can be collected.

### Errors are `longjmp`s

A Lua error jumps over the C frames between the raise and the `pcall`. A Rust destructor in between would be skipped. So everything which can raise runs in one of two places:

```mermaid
flowchart LR
    subgraph cfn ["a C function of BMS"]
        direction LR
        A["Rust work<br/>values with destructors alive<br/>NOTHING here may raise"] --> B["all dropped"] --> C["push the result / raise the error<br/>only plain data left"]
    end
    PM["protected mode (mlua exec_raw)<br/>making metatables and holders"] -. "called from" .-> A
```

```rust,ignore
unsafe fn run(l: *mut lua_State, f: impl FnOnce() -> Result<Ready, InteropError>) -> c_int {
    let ready = match catch_unwind(AssertUnwindSafe(f)) {
        Ok(Ok(ready)) => ready,
        Ok(Err(error)) => unsafe { raise(l, error) },       // nothing left to drop
        Err(payload) => unsafe { resume_panic(l, payload) }, // like mlua's callbacks
    };
    unsafe { push_ready(l, ready) } // plain values only
}
```

## Reading a field

`__index` of a native value tries the cheapest thing first.

```mermaid
flowchart TD
    I["v.name"] --> G{"getters[name]"}
    G -- "an integer:<br/>offset and kind, packed" --> D["direct: read the f32/f64/i32/u32/bool<br/>at the offset in the userdata"]
    G -- "a holder" --> A["call the typed getter<br/>(native fields: returns a read-only copy)"]
    G -- "nil" --> M{"methods[name]"}
    M -- "a function" --> MF["returned as it is"]
    M -- "nil" --> SL["slow path: member_at<br/>on a reference to a copy"]
```

`__newindex` mirrors it: a direct field is written in place unless the value is read-only or the new value isn't a number, in which case the typed setter produces the error.

The direct path trusts that the value at index 1 is a userdata of this type. That holds because the function is only reachable as the `__index` of this metatable, which scripts can't get.

## Member lookup

Every `__index` slow path ends in one function, `member_at`:

```mermaid
flowchart LR
    R["reference_index"] --> MA["member_at"]
    T["type_ref_index"] --> MA
    N["native_index (slow path)"] --> MA
    CD["cdata_index (LuaJIT)"] --> MA
    MA --> D["dispatch::index_name / index / static_index"]
    D -- "function of a type" --> ST["static form, cached by the type object"]
    D -- "method" --> UB["unbound: v:method() passes v"]
    D -- "number / bool" --> PR["pushed directly"]
    D -- "anything else" --> LR2["like a function result"]
```

## Lists, maps and strings

Typed parameters read tables off the stack without building `ScriptValue`s:

| Parameter | Read as |
|---|---|
| `String` | borrowed from the Lua string, copied once |
| `Vec<T>` | a sequence (keys exactly `1..=n`): the items are pushed and stay on the stack for the call, each read with `T::from_arg` |
| `HashMap<String, V>` | string keys only: each value is read then popped, so native values in it aren't handed out as pointers (`NoNatives`) |

Anything else (holes, mixed keys) takes the slow path and converts as before.

## Garbage collection

```mermaid
sequenceDiagram
    participant GC as Lua GC
    participant U as userdata (native value or reference)
    participant F as script finalizer (resurrects it)
    GC->>U: __gc
    U->>U: drop the Rust value in place
    U->>U: remove the metatable (tombstone)
    GC->>F: __gc of a table holding it
    F->>U: use the resurrected value
    U-->>F: error: no marker, not a BMS value anymore
```

A panicking destructor is caught: nothing unwinds through the collector. On Luau, userdata destructors (`lua_newuserdatadtor`) do the same job.

## Safety checklist

| Risk | Guard |
|---|---|
| A foreign userdata read as a native value | marker in a metatable scripts can't reach |
| `longjmp` skipping a Rust destructor | raise only once everything is dropped, or in protected mode |
| A script writing a value a call holds | calls get copies ([CallScope](./introduction.md#safety-the-callscope)) |
| Use after `__gc` | tombstones |
| Stack overflow | `lua_checkstack` before every push which isn't covered by `LUA_MINSTACK` |
| Out of memory while pushing | allocation outside protected mode only when mlua says it can't fail (no memory limit set) |

The `debug` library reaches every metatable and upvalue, and bytecode (`load`) isn't verified by the VM: none of this holds for scripts which have either. mlua's safe state doesn't load `debug`, but does allow bytecode, see [Sandboxing](../../Summary/customizing-script-contexts.md#sandboxing).
