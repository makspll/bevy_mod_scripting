# References

Most values scripts get from Rust are **references**: they point at a value living somewhere else, and reading or writing through them reads or writes that value. A reference is a base, saying where the value lives, plus a path to the field being accessed.

| Base | Points at | Scripts get one from | Valid until |
|---|---|---|---|
| Component | A component of an entity | `world.get_component`, queries | The component is removed or the entity despawned |
| Resource | A resource | `world.get_resource`, script system parameters | The resource is removed |
| Allocation | A value BMS owns for scripts | `Type.new`, `:clone()`, values returned by functions | Kept alive as long as any reference to it is |
| Asset | An asset in `Assets<T>`, by handle | `Type.get_asset`, `world.get_asset` | The asset is removed |

Accessing a field extends the path, so the result still points into the same value:

```lua
local bms = import_namespace("bms")
local world = bms.world
local Transform = import_namespace("transform").Transform

local transform = assert(world.get_component(assert(bms.current_entity()), Transform))
transform.translation.x = 5        -- writes the component

local copy = transform:clone()     -- a new allocation, independent of the component
copy.translation.x = 0             -- the component keeps 5
```

Access is checked when a reference is used, not when it is made: a reference to a removed component fails with an error, and so does writing a value something else is reading at the same time, i.e. inside a query iteration. Script systems can only access what they declared, and assets, whose access is checked for all assets at once, only from exclusive script systems.

## Assets

Handles are plain values (allocations holding a `Handle<T>`), and asset references point into the `Assets<T>` resource through one:

```lua
local world = import_namespace("bms").world
local Image = import_namespace("render").Image

local handle = Image.load("textures/icon.png")  -- a Handle<Image>, loading in the background
-- later
if world.is_loaded(handle) then
    local image = Image.get_asset(handle)       -- a reference into Assets<Image>
    print(image)                                -- reading or writing its fields reads or writes the asset
end
```

Every asset type has `load`, `add_asset` and `get_asset`, and `world` has the same operations taking the asset type, plus `is_loaded`, `load_state` and `asset_path` for any handle. They are listed with their types in [Core Bindings](../core_bindings/types/world.html). An asset type needs `app.register_asset_reflect::<T>()` to be usable from scripts.

## References in Rust

Binding signatures choose how they receive references: `R<T>` borrows, `M<T>` borrows mutably, `V<T>` copies the value out, see [Controlling Script Bindings](../Summary/controlling-script-bindings.md#dynamic-functions). Return a `ReflectReference` to give scripts a reference, i.e. `ReflectReference::new_allocated(value)`.

For references whose type matters only to the docs, use `ValueOf<T>`: a `ReflectReference` at runtime, documented as `T`. Combined with the `A` [marker](../Summary/controlling-script-bindings.md#generics) for a generic parameter, a function can take the handle of any asset type, and still show up as taking a `Handle<A>` in the docs and the Lua language server files:

```rust,ignore
use bevy_mod_scripting::bindings::function::generics::{A, ValueOf};

#[script_bindings(remote, namespace = "my_game")]
impl NamespaceMembers {
    /// Works with the handles `Image.load`, `world.load_asset` and others return.
    fn asset_id(ctxt: FunctionCallContext, handle: ValueOf<Handle<A>>) -> Result<String, InteropError> {
        let untyped: UntypedHandle = handle.try_untyped_asset_handle(ctxt.world()?)?;
        Ok(untyped.id().to_string())
    }

    /// Gives scripts a reference to an asset, as `Image.get_asset` does.
    fn icon(ctxt: FunctionCallContext) -> Result<ValueOf<Image>, InteropError> {
        let world = ctxt.world()?;
        // `Icons` is a resource of your game, holding `Handle<Image>`s
        let handle = world.with_resource(|icons: &Icons| icons.default.clone().untyped())?;
        ReflectReference::new_asset_ref(handle, TypeId::of::<Image>(), world).map(ValueOf::new)
    }
}
```

For a handle of one asset type, take `ValueOf<Handle<Image>>`. `V<Handle<Image>>` works too, but copies the handle out first.
