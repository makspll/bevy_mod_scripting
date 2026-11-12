# Scripting Reference

This part of the book covers the user-facing API of the scripting languages supported by BMS. This will be where you will want to forward your script users to get started with scripting in BMS.

If you are a modder, welcome! 👋 Apologies for the Rust-centricity of this guide, we are working on it!

## Globals

Scripts have only a few builtin globals:
- `import_namespace(namespace)`: returns the types and functions of a [namespace](./namespaces.md)
- `export_namespace(namespace, name, callback)`: see [Exporting functions](./namespaces.md#exporting-functions)
- `register_callback(label, function)`: registers a function as a [callback](../Summary/callbacks.md#registered-callbacks) handler

Everything else is imported. `world` is a static reference to the world, with all sorts of functions available: `import_namespace("bms").world`. BMS' own functions are in `bms`, i.e. `bms.current_entity()` (the entity the script is attached to, or `nil`) and `bms.current_script()` (the script's handle):

```lua
local bms = import_namespace("bms")
print(bms.current_entity())
```

## In this section

| Page | Covers |
|---|---|
| [Namespaces](./namespaces.md) | Importing types, script defined components and events, exporting functions |
| [References](./references.md) | What script values point at, assets and handles |
| [Core Callbacks](./core-callbacks.md) | `on_script_loaded`, `on_script_unloaded`, `on_script_reloaded` |
| [Constructing Arbitrary Types](./constructing-arbitrary-types.md) | `Type.new`, enums and `bms.construct` |
| [Calling Other Scripts](./calling-other-scripts.md) | `bms.run_script_callback` and `bms.queue_script_callback` |
| [Events and Messages](./events-and-messages.md) | Sending and observing Bevy events and messages |
| [Bundles](./bundles.md) | Inserting and spawning bundles |
| [Core Bindings](../ladfiles/bindings.lad.json) | Every type and function available to scripts |
