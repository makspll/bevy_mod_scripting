# Asset Operations

BMS provides built-in support for working with Bevy assets from scripts. You can load assets, add new ones, and read or change asset data through the reflection system.

## Prerequisites

To use asset operations in your scripts, you need to ensure that:

1. Your asset type implements `Asset` and `Reflect`
2. The asset type is registered with `app.register_asset_reflect::<YourAsset>()`
3. The asset handle type has `ReflectHandle` type data registered (`register_asset_reflect` does this)

## Functions on asset types

Every asset type has these functions, which are documented with the type's own handle, i.e. `Handle<Image>` for `Image`:

### `Type.load(path)`

Starts loading an asset of this type from a path relative to the assets folder, and returns a handle to it right away. The asset is loaded in the background, see `world.is_loaded(handle)`.

### `Type.add_asset(value)`

Adds a value of this type as a new asset, and returns a strong handle to it.

### `Type.get_asset(handle)`

Returns the asset a handle points to, or `nil` if it isn't loaded or added yet. Changes to the returned value change the asset.

```lua
local ScriptAsset = import_namespace("bms").ScriptAsset
local handle = ScriptAsset.load("scripts/my_script.lua")
-- later, once `world.is_loaded(handle)` is true
local script = ScriptAsset.get_asset(handle)
```

## Functions on `world`

The same operations are available on `world`, taking the asset type as an argument, along with functions for any handle:

- `world.load_asset(path, asset_type)`: like `Type.load`
- `world.add_asset(value)`: like `Type.add_asset`
- `world.get_asset(handle, asset_type)`: a reference to the asset
- `world.has_asset(handle)`: whether the handle is a valid asset handle
- `world.is_loaded(handle)`: whether the asset and all of its dependencies are loaded
- `world.load_state(handle)`: `"NotLoaded"`, `"Loading"`, `"Loaded"` or `"Failed"`
- `world.asset_path(handle)`: the path the asset was loaded from, or `nil`

Examples:
See usage examples in the `assets/tests/asset_operations` and `assets/tests/bindings/asset` directories.
