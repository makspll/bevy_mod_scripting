# Managing Scripts

Scripts live in the standard Bevy `assets` directory. Loading a script means obtaining its byte representation and associated language.

Evaluating a script means:
- parsing the script body,
- and creating or updating the resources which store script state (i.e. context)

## Script attachments

Script attachments represent both the context in which a script "lives" and is executed, as well as its "residency identifier".

Scripts can be attached either:
- to an entity via `ScriptComponent`'s (i.e. `ScriptAttachment::EntityScript`)
- or by being statically activated against their script assset (i.e. `ScriptAttachment::StaticScript`)

When you insert/remove `ScriptComponent`'s, `ScriptAttachedEvent` and `ScriptDetachedEvent`'s are sent and processed by the [script pipeline](../ScriptPipeline/pipeline.md).

Other attachments, like static scripts have to be manually triggered via `AttachScript` and `DetachScript` commands.


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

To evaluate a script, add it to a `ScriptComponent` manually attach it.

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

<div class="warning">

Prefer using strong asset handles, internal references will only persist weak versions of the handle, leaving you in control of the asset handle via the container component.

</div>

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
<div class="warning">

Scripts added directly through assets, or via asset server without an asset path, will not contain path information, and logs will be slightly less useful.

</div>

## Static Scripts
You can use attach and detach commands, which will run the [script pipeline](../ScriptPipeline/pipeline.md), repeatedly until the requested script attachments are processed. If you don't want to incur big frametime slowdowns, you can instead send `ScriptAttachedEvent` and `ScriptDetachedEvent` manually, and let the pipeline pick these up as normal.

### Attach a static script
```rust
# extern crate bevy;
# extern crate bevy_mod_scripting;
# use bevy::{asset::Assets, prelude::*};
# use bevy_mod_scripting::prelude::*;

fn add_static_script(mut asset_server: ResMut<AssetServer>, mut commands: Commands) {
    let handle = asset_server.load(script_path);
    commands.queue(AttachScript::<LuaScriptingPlugin>::new(
        ScriptAttachment::StaticScript(handle)
    ))
}
```

### Detach a static script
```rust
# extern crate bevy;
# extern crate bevy_mod_scripting;
# use bevy::{asset::Assets, prelude::*};
# use bevy_mod_scripting::prelude::*;

fn remove_static_script(existing_handle: Local<Handle<ScriptAsset>>, mut commands: Commands) {
    commands.queue(DetachScript::<LuaScriptingPlugin>::new(
        ScriptAttachment::StaticScript(existing_handle)
    ))
}
```

## Unloading
When you no longer need a script asset you can freely unload it as soon as it has been loaded, but the script attachment will persist.
In order to trigger the `on_script_unloaded` etc. callbacks, you need to remove the script from the `ScriptComponent` or detach it manually via `DetachScript` if it's static.

Once the last script in a context is removed, the context itself will also be removed.

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