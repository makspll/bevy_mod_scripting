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

Components, bundles and events registered by scripts are in script namespaces, see below.

BMS' own types (`ScriptTypeRegistration`, `ScriptAttachment`, `ReflectReference`, ...) are in the `bms` namespace, instead of the internal crates they happen to be defined in.

## Overriding the namespace of a type

For types with script bindings, the simplest way is the `namespace` argument of `script_bindings`:

```rust,ignore
#[script_bindings(namespace = "my_namespace")]
impl MyType { /* ... */ }
```

If the type is already in a different namespace when this runs, a warning is logged and the first namespace wins.

Otherwise:

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

## Importing a namespace

`import(namespace)` returns a table of the types in a namespace, keyed by short name:

```lua
local bevy_transform = import("bevy_transform")
local Transform = bevy_transform.Transform

local bms = import("bms") -- BMS' own types
```

The table is built from the same shared index as the `types` global, but only contains the one namespace, so it is much cheaper than the full `types` table when you only need a few types. If two types in a namespace share a short name, the first one registered wins.

`import("")` imports the root namespace.

Importing a namespace that does not exist is an error:

```
namespace "bevy_transfrom" does not exist. Native namespaces are the names of the crates defining the types (i.e. `bevy_transform`), and only contain types registered in the type registry: check that the crate's bindings plugin is added and the type is registered. Did you mean: bevy_transform?
```

Names starting with `script.` are reserved for namespaces defined by scripts and get a hint about loading the script first.

The `types` global is still a flat table of all types by short name.

## Namespaces defined by scripts

Scripts register components, bundles and events into namespaces of their own, which they always pass explicitly (there is no default, as a name derived from the script's path would be fragile):

```lua
local Enemy = world.register_new_component("script.game.ai", "Enemy")
local Group = world.register_new_bundle("script.game.ai", "Group", { Enemy })
local Hit = world.register_new_event("script.game.ai", "Hit")

-- later, anywhere (including other scripts)
local ai = import("script.game.ai")
world.add_default_component(entity, ai.Enemy)
```

* Namespaces defined by scripts must start with `script.`, followed by segments of letters, digits and underscores separated by dots. This keeps them apart from the namespaces of crates, which scripts cannot add to.
* Names are unique within their namespace, so `script.game.ai.Enemy` and `script.game.ui.Enemy` are different components. Registering the same path twice is an error, unless `allow_existing` is passed.
* `types.Enemy` finds a script registered type by its name alone, which is ambiguous if several namespaces have one of that name (the one with the alphabetically first path wins). Prefer `import`.
* A namespace exists as soon as something is registered in it. Importing one before that fails with a hint that the script defining it needs to have run.
