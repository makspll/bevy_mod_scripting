# Necessary Features

In order for a language to be called "implemented" in BMS, it needs to support the following features.

## Values

Every `ScriptValue` variant needs a counterpart in the language, in both directions. Two of them are handles rather than data:
- `ScriptValue::Reference`: a reference to a reflected value, usually a userdata/custom type wrapping `ReflectReference`.
- `ScriptValue::Type`: a type itself, i.e. what scripts use to reach a type's static members. It carries only a `TypeId` and never allocates.

Every context gets the builtins as globals, nothing else: `bevy_mod_scripting_bindings::builtins::builtins()` (`import_namespace`, `export_namespace`) plus the language's own `register_callback`:

```rust,ignore
for (name, function) in builtins() {
    context.globals().set(name, LuaScriptValue::from(ScriptValue::Function(function)))?;
}
```

Script handlers, loaders etc. must be implemented such that the `ThreadWorldContainer` is set for every interaction with script contexts, or anywhere else it might be needed.

## Dispatching to Rust

All lookup rules live in `bevy_mod_scripting_bindings::dispatch`; a language maps its operators and metamethods onto it, and converts the values:

| Script operation | `dispatch` function |
|---|---|
| `v.name`, `v[key]` | `index`, returns `Member::Method` or `Member::Static` |
| `v.name = x` | `set_index` |
| `Type.name` on a `ScriptValue::Type` | `static_index` |
| `v + x`, `-`, `*`, `/`, `%`, `^`, `==`, `<` | `binary` with the matching `ScriptOperatorNames` (dispatches to `add`, `sub`, `mul`, `div`, `rem`, `pow`, `eq`, `lt`) |
| `2 * v` (value on the right) | `binary` too, which calls the operator's reflected function (`mul`, `rsub`, ...) |
| `-v` | `unary` with `Negation` (`neg`) |
| `#v` | `len` (lists directly, references through `len`) |
| iteration | `iter`, returns a function to call until it returns `ScriptValue::Unit` |
| printing | `display` / `debug` |

### Receivers

`index` returns functions which are called on the indexed value as `Member::Method`, without the value bound to them. Delivering the receiver is the language's job:
- Lua passes it with its colon syntax, `v:length()`, so the function is returned as is.
- Rhai has no method call syntax for function values, so it binds the receiver with `DynamicScriptFunction::curry`, `v.length.call()`.

### Static members

`Type.name` resolves, in order, to:
1. functions registered on the type itself
2. functions shared by all types (registered on `StaticTypeRef`, i.e. the `new` constructor), with the type bound as their first argument by `DynamicScriptFunction::bind_to_type`
3. the type's enum variants

Types reach scripts in two forms, which resolve static members the same way:
- `ScriptValue::Type`, i.e. `world` (`import_namespace("bms").world`), indexed with `dispatch::static_index`
- type registrations, i.e. the entries of `import_namespace`, which are references and are indexed with `dispatch::index` like any other value

Binding the type is done by BMS, so languages never handle `StaticTypeRef` arguments themselves.
