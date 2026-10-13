# Namespaces

Every type known to scripts lives in a namespace. By default the namespace of a type is the name of the crate that defines it, exactly as it appears in the type's path, so there are no renaming rules to remember:

| Type | Namespace |
|---|---|
| `Transform` | `bevy_transform` |
| `Entity` | `bevy_ecs` |
| `Vec3` | `glam` |
| primitives like `i32` | the root namespace (empty string) |

Scripts can ask a type for its namespace:

```lua
print(types.Transform:namespace()) -- bevy_transform
```

Components and bundles registered by scripts are in the root namespace for now.

## Overriding the namespace of a type

Rust types can pick a different namespace than their crate, for example when a type is defined in a crate other than the one users think of it as belonging to:

```rust,ignore
use bevy_mod_scripting::bindings::ScriptNamespaceAppExt;

app.set_script_namespace::<MyType>("my_namespace");
```

or by registering the `ScriptNamespace` type data yourself:

```rust,ignore
registry.register_type_data_with::<MyType, ScriptNamespace, _>("my_namespace");
```

Types from the standard library are split across `core`, `alloc` and `std` by their crate of origin (`Option` is in `core`, `Vec` is in `alloc`); use the override if you want them elsewhere.

The `types` global is still a flat table of all types by short name. Namespaces are used by the upcoming `import` support.
