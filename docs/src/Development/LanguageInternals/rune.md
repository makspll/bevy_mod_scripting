# Rune

How the Rune glue implements the [shared model](./introduction.md). The code is in `bevy_mod_scripting_rune/src/{types,native,names}.rs`.

Rune is a safe Rust VM: there is no stack to manage and no `longjmp`. Errors are `Result`s, values are reference counted and borrow checked at runtime. The glue is mostly about Rune's static member resolution.

## What each value is in Rune

```mermaid
flowchart TD
    subgraph rune ["Rune values (Any types)"]
        NA["RuneNative<br/>a native value or handle"]
        RR["RuneRef<br/>a reference"]
        RT["RuneType<br/>a type"]
        NS["RuneNamespace"]
        RFN["RuneFunction"]
    end
    NA --> NB["NativeBox: [NativeHeader][value] on the heap<br/>+ Arc of NativeType: its fields and methods"]
    RR --> RF["ReflectReference"]
    RFN --> DF["DynamicScriptFunction"]
```

A `RuneNative` keeps the `NativeType` it was made with, so reading a field or calling a method takes no registry lookup.

```mermaid
flowchart LR
    subgraph nb ["NativeBox (one allocation)"]
        direction LR
        H["NativeHeader<br/>vtable, read_only"] --- P["padding"] --- V["value"]
    end
    RN["RuneNative"] --> nb
    RN --> NT["Arc of NativeType<br/>fields: name to typed get/set<br/>methods: name to function"]
```

## Members are resolved by name hash

Rune looks `v.name` and `v.name()` up by the hash of the name, in a table fixed when the context is built, with no fallback for unknown names. So every identifier in the script's source is registered as a field and a method of each BMS type; whether the member exists is checked when it's used.

```mermaid
flowchart LR
    SRC["script source"] -- "names::source_names" --> NAMES["every identifier<br/>translation, x, dot, .."]
    NAMES -- "register_types" --> MOD["Rune module:<br/>field + method name on<br/>RuneRef, RuneType, RuneNamespace, RuneNative"]
    MOD --> VM["v.translation resolves<br/>to a closure knowing the name"]
    VM --> D["BMS looks the member up"]
```

`v["name"]` goes through the index protocol, which gets the name as a value and needs no registration.

## A function call

```mermaid
flowchart TD
    S["v.dot(w)"] --> RAW["raw function dot<br/>the VM's values as a slice"]
    RAW --> B["receiver borrowed: BorrowRef of RuneNative"]
    B --> M{"native.methods[dot]"}
    M -- "found" --> ARGS["RuneArgs: borrows every native argument<br/>for the call (guards)"]
    M -- "not found" --> SH{"shared reference functions"}
    SH -- "found" --> ARGS
    SH -- "not found" --> CP["through a reference to a copy"]
    ARGS --> CW["function.call_with(RuneArgs, RuneReturn)"]
    CW --> RET["RuneReturn: a number, a new RuneNative,<br/>or a converted ScriptValue"]
```

The arguments of short calls stay in a stack array, longer ones use a `Vec`.

```rust,ignore
pub(crate) enum RuneArg<'a> {
    Value(&'a Value),       // a value of the VM
    Native(&'a RuneNative), // the receiver of a protocol, already borrowed
    Ref(&'a RuneRef),
}
```

### Guards

`ArgSource::native` hands out pointers into `NativeBox`es. Rune's own borrow tracking keeps them valid: every native argument is borrowed (`BorrowRef`) for the whole call, so nothing the call runs can take or drop the value.

```mermaid
sequenceDiagram
    participant VM as Rune VM
    participant A as RuneArgs
    participant F as Rust function
    VM->>A: values of the call
    A->>VM: borrow_ref each RuneNative
    Note over A: guards held until the call returns
    A->>F: call_with
    F->>A: args.native(i)
    A-->>F: base of the NativeBox
    F->>F: R of T / M of T: a copy in the CallScope
    F-->>A: returns, M of T written back
    A->>VM: guards released
```

The call still works on copies ([CallScope](./introduction.md#safety-the-callscope)): the guards keep the storage alive, the copies keep a callback's writes from aliasing a `&mut`.

## Fields and operators

```mermaid
flowchart TD
    G["v.name"] --> F{"native.fields[name]"}
    F -- "found" --> TG["the typed getter<br/>(a native field: a read-only copy)"]
    F -- "not found" --> RC["a reference to a copy:<br/>methods as values, anything else"]
    O["v + w"] --> OM{"native.methods[add]"}
    OM -- "found" --> TC["typed call, both values as they are"]
    OM -- "not found" --> DS["dispatch::binary on a copy"]
```

`v.name += x` reads, applies the operator and writes: Rune has a protocol for each compound assignment. Equality with a value which is neither a native value nor a reference is `false` without a call.

## Compared to Lua

| | Lua | LuaJIT | Rune |
|---|---|---|---|
| Native storage | userdata | cdata (types with a C layout) | `NativeBox` in an `Any` |
| Field read | in place (`DirectField`) | compiled struct access | typed getter |
| Type's functions found | metatable, made once per state | generated Lua, made once per state | the value's `Arc<NativeType>` |
| Lists, maps, strings | read off the stack | read off the stack | converted (`ScriptValue`) |
| Errors | `longjmp`, raised last | same, FFI failures through a flag | `Result` |
| What keeps an argument alive | the stack | the stack | `BorrowRef` guards |

## Safety checklist

| Risk | Guard |
|---|---|
| A value dropped or taken while a call points to its storage | `BorrowRef` guards for the call; a failed borrow is an error |
| A script writing a value a call holds | calls get copies |
| Writing through a copy of a field | `read_only` in the `NativeHeader` |
| Unknown members | checked by BMS when used, every name resolves |

The only `unsafe` here is reading and writing `NativeStorage` through the vtable.
