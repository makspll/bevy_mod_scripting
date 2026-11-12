# Managing Scripts

Scripts are Bevy assets (`ScriptAsset`), loaded from the `assets` directory like any other asset. A loaded script does nothing until it is **attached**, which evaluates it:

- parsing the script body,
- creating or updating the script's [context](./contexts.md), the environment holding its globals.

## Script attachments

A `ScriptAttachment` says where a script "lives", and is the identity scripts are addressed by:

- `ScriptAttachment::EntityScript(entity, handle)`: attached to an entity through its `ScriptComponent`,
- `ScriptAttachment::StaticScript(handle)`: attached to nothing, activated through the `AttachScript` command.

Inserting or removing a `ScriptComponent` sends `ScriptLifecycleMessage`s (`Attached` / `Detached`), which the [script pipeline](../ScriptPipeline/pipeline.md) processes. Static scripts are attached and detached with the `AttachScript` and `DetachScript` commands.

## Loading

Load scripts with the `AssetServer`, or create them in memory:

```rust,ignore
let handle = asset_server.load::<ScriptAsset>("my_script.lua");

let mut script = ScriptAsset::from("x = 0".to_string());
script.language = Language::Lua;
let handle = script_assets.add(script);
```

Neither runs any code yet.

> [!NOTE]
> Scripts created in memory, or loaded without an asset path, have no path information, so their logs are less useful.

## Attaching scripts to entities

Add a `ScriptComponent` to an entity:

```rust,ignore
fn load_script(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle = asset_server.load::<ScriptAsset>("my_script.lua");
    commands.spawn(ScriptComponent::new(vec![handle]));
}
```

In the script, `import_namespace("bms").current_entity()` returns the entity the script is attached to.

> [!TIP]
> Keep strong handles in the `ScriptComponent`. BMS only keeps weak handles internally, so the component decides how long the asset lives.

## Static scripts

Scripts which don't belong to an entity are attached as static scripts:

```rust,ignore
fn add_static_script(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle = asset_server.load("scripts/my_script.lua");
    commands.queue(AttachScript::<LuaScriptingPlugin>::new(
        ScriptAttachment::StaticScript(handle),
    ));
}

fn remove_static_script(handle: Local<Handle<ScriptAsset>>, mut commands: Commands) {
    commands.queue(DetachScript::<LuaScriptingPlugin>::new(
        ScriptAttachment::StaticScript(handle.clone()),
    ));
}
```

The commands run the [script pipeline](../ScriptPipeline/pipeline.md) until the attachment is processed. To avoid frame time spikes, write `ScriptLifecycleMessage`s instead, and let the pipeline pick them up as normal.

## Unloading

Script assets can be unloaded as soon as they are loaded, the attachment keeps running. To unload a script, and run its `on_script_unloaded` [callback](../ScriptingReference/core-callbacks.md), remove it from its `ScriptComponent` or detach it with `DetachScript`.

A context is removed together with the last script in it.

## Hot reloading

Enable Bevy's `file_watcher` feature, and changes to script assets are picked up and the scripts reloaded. Use [`on_script_unloaded` and `on_script_reloaded`](../ScriptingReference/core-callbacks.md) to keep state across reloads.

## File extensions

Each language plugin maps file extensions to its language, i.e. `lua` and `luau` for Lua. To add more:

```rust,ignore
app.add_plugins(LuaScriptingPlugin::default().add_supported_extension("pua"));
```

> [!WARNING]
> Script asset paths are platform dependent. Build them with `std::path::PathBuf` if you target multiple platforms.

Next, [run your scripts](./running-scripts.md).
