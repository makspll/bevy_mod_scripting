# Necessary Features

In order for a language to be called "implemented" in BMS, it needs to support the following features.

## Values

Every `ScriptValue` variant needs a counterpart in the language, in both directions. Three of them are handles rather than data:
- `ScriptValue::Reference`: a reference to a reflected value, usually a userdata/custom type wrapping `ReflectReference`.
- `ScriptValue::Type`: a type itself, i.e. what scripts use to reach a type's static members. It carries only a `TypeId` and never allocates.
- `ScriptValue::Value`: a small value owned by the script (`InlineValue`, i.e. an `Entity` or a `Vec3`), stored inline instead of allocated. Scripts must not be able to tell it from a reference:
  - its methods are found with `dispatch::index_value`, `ValueMember::Method` functions are called with the value as the first argument
  - anything else (`ValueMember::NeedsPlace`: fields, assignments, functions which mutate it, iteration) needs a place: promote the value in place with `InlineValue::into_reference` and continue as for a reference, so variables sharing the value see the change
  - operators (`dispatch::binary`, `unary`, `len`) take it directly, printing can use a copy

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
- Rune has method syntax, but resolves `v.name` by a table fixed when its context is built, so every identifier in the script is registered when it is compiled (and `v["name"]` always works), see `bevy_mod_scripting_rune::types`. The receiver is passed to the function as the first argument.

### Static members

`Type.name` resolves, in order, to:
1. functions registered on the type itself
2. functions shared by all types (registered on `StaticTypeRef`, i.e. the `new` constructor), with the type bound as their first argument by `DynamicScriptFunction::bind_to_type`
3. the type's enum variants

Types reach scripts in two forms, which resolve static members the same way:
- `ScriptValue::Type`, i.e. `world` (`import_namespace("bms").world`), indexed with `dispatch::static_index`
- type registrations, i.e. the entries of `import_namespace`, which are references and are indexed with `dispatch::index` like any other value

Binding the type is done by BMS, so languages never handle `StaticTypeRef` arguments themselves.
