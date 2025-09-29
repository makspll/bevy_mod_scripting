# Asset Operations

BMS provides built-in support for working with Bevy assets from scripts. You can check for asset existence, retrieve assets by handle, and manipulate asset data through the reflection system.

## Prerequisites

To use asset operations in your scripts, you need to ensure that:

1. Your asset type implements `Asset` and `Reflect`
2. The asset type is registered with `app.register_asset_reflect::<YourAsset>()`
3. The asset handle type has `ReflectHandle` type data registered

## Available Functions

### `world.has_asset(handle)`

Checks if an asset exists and is loaded for the given handle.

**Parameters:**

- `handle`: A ReflectReference to an existing asset handle

**Returns:**

- `boolean`: `true` if the asset exists and is loaded, `false` otherwise

### `world.get_asset(handle, asset_type)`

Retrieves a loaded asset by its handle and returns a reflected reference to it.

**Parameters:**

- `handle`: A reflected reference to an asset handle  
- `asset_type`: The type registration of the asset (e.g., `types.Image`, `types.Mesh`)

**Returns:**

- `ReflectReference`: A reference to the asset data, or `nil`/`()` if not loaded

Examples:
See usage examples in the `assets\tests\asset_operations` directory.
