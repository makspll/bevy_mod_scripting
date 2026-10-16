# Scripting Reference

This part of the book covers the user-facing API of the scripting languages supported by BMS. This will be where you will want to forward your script users to get started with scripting in BMS.

If you are a modder, welcome! 👋, apologies for the rust-centricity of this guide, we are working on it!

## Globals

Scripts have only a few builtin globals:
- `world`: a static reference to the world, with all sorts of functions available
- `import_namespace(namespace)`: returns the types and functions of a [namespace](./namespaces.md)
- `export_namespace` and `register_callback`: see [Exporting functions](./namespaces.md#exporting-functions)

Everything else is reached through `import_namespace`, i.e. the `bms` namespace holds BMS' own functions:
- `bms.current_entity()`: the entity the script is attached to, or `nil` if none, not available on load/unload callbacks, and in dynamic system callbacks.
- `bms.current_script()`: the handle of the current script

```lua
local bms = import_namespace("bms")
print(bms.current_entity())
```
