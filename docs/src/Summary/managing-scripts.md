# Managing Scripts

Scripts live in the standard bevy `assets` directory. Loading a script means:
- Parsing the script body
- Creating or updating the resources which store script state
- Assigning a name/id to the script so it can be referred to by the rest of the application.

## Loading 
BMS listens to `ScriptAsset` events and reacts accordingly. In order to load a script, all you need to do is request a handle to it via the asset server and store it somewhere. 

Below is an example system which loads a script called `assets/my_script.lua` and stores the handle in a local system parameter:

```rust,ignore
fn load_script(server: Res<AssetServer>, mut handle: Local<Handle<ScriptAsset>>) {
    let handle_ = server.load::<ScriptAsset>("my_script.lua");
    *handle = handle_;
}
```

In practice you will likely store this handle in a resource or component, when your load all the scripts necessary for your application.

## Unloading 
Scripts are automatically unloaded when the asset is dropped. This means that if you have a handle to a script and it goes out of scope, the script will be unloaded.


This will delete references to the script and remove any internal handles to the asset. You will also need to clean up any handles to the asset you hold in your application in order for the asset to be unloaded.

## Hot-loading scripts
To enable hot-loading of assets, you need to enable the necessary bevy features as normal [see the bevy cheatbook for instructions](https://bevy-cheatbook.github.io/assets/hot-reload.html).

Assuming that hot-reloading is enabled for your app, any changes to script assets will automatically be picked up and the scripts re-loaded.

## File Extensions
Normally the set of supported extensions is pre-decided by each language plugin.

I.e. Lua supports ".lua" extensions and Rhai supports ".rhai" extensions.

Scripts are mapped to the corresponding language plugin based on these and so it's important to use them correctly.

If you would like to add more extensions you need to populate them via `app.add_supported_script_extensions`.

## Advanced
Normally not necessary, but knowing these exist could be useful for more advanced use cases.

### Manually (re)loading scripts
In order to manually re-load or load a script you can issue the `CreateOrUpdateScript` command:

```rust,ignore
CreateOrUpdateScript::<LuaScriptingPlugin>::new("my_script.lua".into(), "print(\"hello world from new script body\")".into(), asset_handle)
```

replace `LuaScriptingPlugin` with the scripting plugin you are using.

### Manually Deleting scripts
In order to delete a previously loaded script, you will need to issue a `DeleteScript` command like so:

```rust,ignore
DeleteScript::<LuaScriptingPlugin>::new("my_script.lua".into())
```

replace `LuaScriptingPlugin` with the scripting plugin you are using.

### Loading/Unloading timeframe
Scripts asset events are processed within the same frame they are issued. This means the moment an asset is loaded, it should be loaded and ready to use in the `Update` schedule. Similarly, the moment an asset is deleted, it should be unloaded and no longer usable in the `Update` schedule.