# Managing Scripts

Scripts live in the standard Bevy `assets` directory. Loading a script means obtaining its byte representation and associated language.

Evaluating a script means:
- parsing the script body,
- and creating or updating the resources which store script state.

## Loading 
Scripts can be loaded into memory via the `AssetServer`.
```rust,ignore
let handle = asset_server.load::<ScriptAsset>("my_script.lua");
```
Or scripts can be created in memory. 
```rust,ignore
let mut script = ScriptAsset::from("x = 0".into());
script.language = Language::Lua;
let handle = script_assets.add(script);
```
This will not evaluate any code yet. 

## Evaluating
A script does not participate in any callbacks until it is evaluated, to evaluate a script you must first attach it to an entity, or to a static script entry.

To evaluate a script, add it to a `ScriptComponent` or to `StaticScripts`.
### Load File via `AssetServer`
```rust
# extern crate bevy;
# extern crate bevy_mod_scripting;
# use bevy::prelude::*;
# use bevy_mod_scripting::prelude::*;

fn load_script(asset_server: Res<AssetServer>, mut commands: Commands) {
    let handle = asset_server.load::<ScriptAsset>("my_script.lua");
    commands.spawn(ScriptComponent(vec![handle]));
}
```
### Create `ScriptAsset` and Add It
```rust
# extern crate bevy;
# extern crate bevy_mod_scripting;
# use bevy::{asset::Assets, prelude::*};
# use bevy_mod_scripting::prelude::*;

fn add_script(mut script_assets: ResMut<Assets<ScriptAsset>>, mut commands: Commands) {
    let content: String = "x = 0".into();
    let mut script = ScriptAsset::from(content);
    script.language = Language::Lua;
    let handle = script_assets.add(script);
    commands.spawn(ScriptComponent(vec![handle]));
}
```

## Unloading
When you no longer need a script asset you can freely unload it, but the script attachment will persist.
In order to trigger the `on_script_unloaded` etc. callbacks, you need to remove the script from the `ScriptComponent` or `StaticScripts`.

When that happens a corresponding `ScriptEvent::Detached` will be dispatched, and then handled by a `DeleteScript` command. Once the last script in a context is removed, the context itself will also be removed.

## Hot-loading scripts
To enable hot-loading of assets, you need to enable the necessary Bevy features as normal [see the bevy cheatbook for instructions](https://bevy-cheatbook.github.io/assets/hot-reload.html).

Assuming that hot-reloading is enabled for your app, any changes to script assets will automatically be picked up and the scripts re-loaded.

## File Extensions
Normally the set of supported extensions is pre-decided by each language plugin.

I.e. Lua supports ".lua" extensions and Rhai supports ".rhai" extensions.

Scripts are mapped to the corresponding language plugin based on these and so it is important to use them correctly.

If you would like to add more extensions, you need to populate them via `app.add_supported_script_extensions`.
```rust,ignore
    app.add_supported_script_extensions(&[".pua"], Language::Lua);
```

## Advanced
Normally not necessary but knowing these exist could be useful for more advanced use cases.

### Manually (re)loading scripts
In order to manually re-load or load a script you can issue the `CreateOrUpdateScript` command:

```rust,ignore
# use bevy::prelude::*;
# use bevy_mod_scripting::prelude::*;
let create_or_update = CreateOrUpdateScript::<LuaScriptingPlugin>::new(script_handle)
    .with_content("print(\"hello world from new script body\")");
commands.queue(create_or_update);
```

replace `LuaScriptingPlugin` with the scripting plugin you are using.

### Manually Deleting scripts
In order to delete a previously loaded script, you will need to issue a `DeleteScript` command like so:

```rust,ignore
commands.queue(DeleteScript::<LuaScriptingPlugin>::new(script_handle));
```

Replace `LuaScriptingPlugin` with the scripting plugin you are using.

### Loading/Unloading timeframe

Script asset changes are processed together with bevy asset systems, in the `Last` schedule.
These are converted to `ScriptEvent`'s which are handled right after via the `ScriptingSystemSet::ScriptingCommandDispatch` system set.
