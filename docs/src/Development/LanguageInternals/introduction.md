# Language Internals

How a script value reaches a Rust function and back, and why that is both fast and safe. This page is what every language shares, the language pages are what each does with it:

- [Lua](./lua.md): raw userdata and C closures on the Lua stack
- [LuaJIT](./luajit.md): cdata and FFI calls the JIT compiles
- [Rune](./rune.md): `Any` values and protocols

## What a script holds

Every value a script has is one of four things. Which one decides how fast it is and what a write does.

```mermaid
flowchart TD
    V["a value in a script"] --> P["primitive<br/>number, bool, string"]
    V --> N["native value<br/>Vec3, Quat, Entity"]
    V --> H["native handle (place)<br/>a Transform component"]
    V --> R["reference<br/>anything reflected"]
    P --> PL["the language's own value"]
    N --> NL["stored BY the language<br/>a copy the script owns"]
    H --> HL["entity + component id<br/>reads and writes the world"]
    R --> RL["base + reflection path<br/>reads and writes through reflection"]
```

| | Where the data is | A read | A write |
|---|---|---|---|
| Primitive | the language | free | free |
| Native value | the language (`script_native!` types) | a typed accessor, or in place | changes the script's copy |
| Native handle | the world | claims access, reads the component | claims access, writes the component |
| Reference | the world or an allocation | reflection | reflection |

Native types are **values**: there is no write-through. A native field (`t.translation`) is read as a **read-only copy**, so `t.translation.x = 5` is an error instead of a lost write, and `t.translation = v` writes the whole field.

## A native value in memory

A language stores a native value as a header followed by the value. The header is how a typed function knows what it was given.

```mermaid
flowchart LR
    subgraph storage ["NativeStorage, allocated by the language"]
        direction LR
        A["NativeHeader<br/>vtable: NativeVTable<br/>read_only: Cell of bool"] --- B["padding"] --- C["the value<br/>i.e. Vec3 { x, y, z }"]
    end
    A -. "type id, layout,<br/>clone, drop" .-> VT["NativeVTable<br/>one per type, never freed"]
```

A handle uses the same storage with the type's *place* vtable, and a `NativePlace` where the value would be:

```mermaid
flowchart LR
    subgraph storage ["NativeStorage of a handle"]
        direction LR
        A["NativeHeader<br/>vtable.place = true"] --- C["NativePlace<br/>entity + component id, or resource id"]
    end
    C -. "resolved for each call" .-> W["the component in the world"]
```

## How a call works

Functions never see the language. They read arguments from an `ArgSource` and write the result to a `ReturnSink`, which each language implements over its own values.

```mermaid
flowchart TD
    S["script: v:dot(w)"] --> L["language glue<br/>makes an ArgSource over its values"]
    L --> F["DynamicScriptFunction::call_with(args, sink)"]
    F --> A{"each parameter:<br/>T::from_arg(args, i, scope)"}
    A -- "number / integer / boolean" --> FAST["read directly"]
    A -- "native value of T" --> COPY["copied into the CallScope"]
    A -- "handle of T" --> CLAIM["access claimed,<br/>pointer into the world"]
    A -- "anything else" --> SLOW["args.script_value(i)<br/>then T::from_script"]
    FAST --> BODY["the Rust function runs"]
    COPY --> BODY
    CLAIM --> BODY
    SLOW --> BODY
    BODY --> RET{"result.into_ret(sink)"}
    RET -- "number / bool" --> RN["sink.number(..)"]
    RET -- "native type" --> RV["sink.native(vtable, write)<br/>written into the language's storage"]
    RET -- "anything else" --> RS["sink.script_value(..)"]
```

The slow arm is always there: a function works with every language and every argument, the fast arms only skip building `ScriptValue`s.

```rust,ignore
pub unsafe trait ArgSource {
    fn number(&self, i: usize) -> Option<f64>;
    fn native(&self, i: usize) -> Option<*mut u8>;           // base of a NativeStorage
    fn plain(&self, i: usize, ty: TypeId) -> Option<(*mut u8, bool)>; // no header (LuaJIT cdata)
    fn script_value(&self, i: usize) -> Result<ScriptValue, InteropError>; // the slow path
    // integer, boolean, string, list, map ..
}
```

## Safety: the CallScope

Every call runs in a `CallScope`. It owns everything a call borrows, and gives it all back when the call ends, also when the function panics.

```mermaid
flowchart LR
    subgraph scope ["CallScope (lives for one call)"]
        direction TB
        CL["claims<br/>world accesses, released on drop"]
        CP["copies<br/>of native values, 4 inline slots of 64 bytes"]
        WG["world guard<br/>looked up only when needed"]
    end
    R["R of T / M of T arguments"] -- "borrow from, can't outlive" --> scope
```

### Script-owned values are copied

A script can write a value it owns at any time, also *while* a call holds it (a callback back into the script). So a call never points into the script's value: `R<T>` and `M<T>` point to the scope's copy, and `M<T>` is written back when the scope ends.

```mermaid
sequenceDiagram
    participant S as Script
    participant L as Language value (a)
    participant C as CallScope
    participant F as Rust function
    S->>F: call_back_mut(a, callback)
    F->>C: M of T::from_arg
    C->>L: copy a
    C-->>F: mutable borrow of the copy
    F->>S: callback()
    S->>L: a.x = 5 (allowed, nothing points here)
    S-->>F: returns
    F->>C: copy.x += 1
    F-->>C: call ends
    C->>L: write the copy back
    Note over L: the last write wins
```

A copy is a `memcpy` for plain types and the vtable's `clone` for the others. `V<T>` needs no scope copy at all: it's read once.

### World values are claimed

A handle or reference points into the world, where systems and other arguments may point too. The scope claims read or write access to the component for the call, and conflicting claims are errors.

```mermaid
sequenceDiagram
    participant F as Rust function
    participant C as CallScope
    participant W as World access map
    F->>C: M of Transform::from_arg (a handle)
    C->>W: claim write access to the component
    alt already read or written
        W-->>F: error: conflicting access
    else free
        W-->>C: claimed
        C-->>F: mutable borrow of the Transform in the world
        F-->>C: call ends
        C->>W: release
    end
```

### Read-only copies

Reading a native field returns a copy with `read_only` set in its header. Passing it as `M<T>`, or writing one of its fields, is an error.

```mermaid
flowchart LR
    T["t (handle to Transform)"] -- "t.translation" --> C["Vec3 copy<br/>read_only = true"]
    C -- "c.x" --> OK["reads fine"]
    C -- "c.x = 5" --> ERR["error: read-only copy of a field"]
    C -- "c:clone()" --> M["Vec3 copy<br/>read_only = false"]
    M -- "t.translation = m" --> T
```

### The slow path of a native value

Anything untyped (a `ScriptValue` parameter, reflection, `tostring`) gets a reference to a fresh copy of the value. Write access to such a copy is refused, so a change is never silently lost.

## What makes it fast

| Cost | Avoided by |
|---|---|
| Building a `ScriptValue` per argument | `ArgSource` reads the language's value directly |
| An allocation per returned vector | the language stores the value (`ReturnSink::native`) |
| Reflection for `v.x` | typed accessors, or the field read in place (`DirectField`) |
| Looking up the world per call | lazy scopes: `CallScope::with_lazy` only finds the world when an argument needs it |
| Registry locks per result | each function caches the native types it returns (`NativeReturnType`) |

Native types and their functions are registered before any script runs: languages look them up once and keep them.
