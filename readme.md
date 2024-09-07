<p align="center">
    <img src="./logo_bevy_scripting.svg" width="250" onerror="this.onerror=null; this.src='https://raw.githubusercontent.com/makspll/bevy_mod_scripting/main/logo_bevy_scripting.svg'">
</p>

# Bevy Scripting

Although Bevy doesn't directly support scripting, efforts are underway to incorporate it. This crate represents an initial attempt to enable scripting within Bevy's existing framework. It's important to note that this is a work in progress and not yet optimized or complete. As Bevy evolves, significant changes to this API are anticipated.

For a detailed look at how this crate works see [architecture.md](architecture.md)

## Why Use Scripts?

- Refresh your game mechanics without the need for full crate recompilation
- Encapsulating game logic in scripts paves way for modders to create custom content easilly
- Scripting game logic/UI in a simpler language broadens development accessibility to non-programmers on your team

## Features

- Hot re-loading scripts
- Lua, Teal, Rhai and Rune integrations
- Automatically generated Bevy bindings for Lua
- CLI rustc extensions for generating your own Lua bindings
- Event based hooks (i.e. `on_update`)
- Flexible event scheduling (i.e. allow handling events at handling stages based on the event)
- Multiple scripts per entity
- Multiple instances of the same script on one entity
- Extensive callback argument type support
- Utilities for generating script native documentation
- Loading external lua libraries via `require` (enabled with `unsafe_lua_modules` cargo feature due to potential unsafety)

## Support

Support for languages is expressed in three levels:

1. `ScriptHost` implementation, scripts can be loaded, scheduled and run in this language with support for custom `APIProvider`
2. A Bevy API Providedr is implemented which enables access to `entity`,`world` etc and provides support for at least basic operations such as `get_component`, `add_component`, `spawn` etc
3. Macros for generating proxy wrapper structures exist and can be used for custom types with the ability to add script-side functionality
4. Macros instantiations are automatically generated for native Bevy structures

The languages currently supported are as follows:
|Language| Support Level | Documentation Generation |
|----|----|----|
|Lua|4|[Yes](https://makspll.github.io/bevy_mod_scripting_lua/latest/)|
|Rhai|2|No|
|Rune|1|No|

## Usage

### Installation

To install:

- Add this crate to your Cargo.toml file dependencies
  - The crate is still in development so I recommended pinning to a git commit
- Add ScriptingPlugin to your app
- Add the ScriptHosts you plan on using (`add_script_host`, `add_script_host_to_set`)
  - Make sure to attach it to a system set running AFTER any systems which may generate modify/create/remove script components
- Add script handlers to capture events in the priority range you're expecting (`add_script_handler_to_set`,`add_script_handler`)
- Add systems which generate ScriptEvents corresponding to your script host
- Add systems which add ScriptCollection components to your entities and fill them with scripts

An example can be seen below

```rust, ignore

fn main() -> std::io::Result<()> {
    let mut app = App::new();
        app.add_plugins(ScriptingPlugin)
        .add_plugins(DefaultPlugins)
        // pick and register only the hosts you want to use
        // use any system set AFTER any systems which add/remove/modify script components
        // in order for your script updates to propagate in a single frame
        .add_script_host::<RhaiScriptHost<MyRhaiArgStruct>>(PostUpdate)
        .add_script_host::<LuaScriptHost<MyLuaArgStruct>>(PostUpdate)

        // the handlers should be ran after any systems which produce script events.
        // The PostUpdate set is okay only if your API doesn't require the core Bevy systems' commands
        // to run beforehand.
        // Note, this setup assumes a single script handler system set with all events having identical
        // priority of zero (see examples for more complex scenarios)
        .add_script_handler::<LuaScriptHost<MyLuaArg>, 0, 0>(
            CoreSet::PostUpdate,
        )
        .add_script_handler::<RhaiScriptHost<RhaiEventArgs>, 0, 0>(
            CoreSet::PostUpdate,
        )

        // generate events for scripts to pickup
        .add_system(trigger_on_update_lua)
        .add_system(trigger_on_update_rhai)

        // attach script components to entities
        .add_startup_system(load_a_script);
    app.run();

    Ok(())
}
```

### Firing Script Callbacks

Scripts are activated by dispatching `ScriptEvents`. This crate employs custom priority event writers and readers, which means events are transmitted with an associated priority. This priority, in conjunction with your event pipeline, influences the sequence in which your events are processed. A priority of 0 is considered the highest.

This mechanism can be utilized to construct game loops similar to those found in Unity or other game engines.

An example event dispatching system can be seen below:

```rust
use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;


// event callback generator for lua
#[cfg(feature = "lua")]
pub fn trigger_on_update_lua(mut w: PriorityEventWriter<LuaEvent<()>>) {
    let event = LuaEvent::<()> {
        hook_name: "on_update".to_string(),
        args: (),
        recipients: Recipients::All
    };

    w.send(event,0);
}
```

### Adding scripts

A script is composed of:

- A reference to its code file, represented as an asset handle
- A name, typically the path relative to the assets folder

Scripts are associated with entities through `bevy_mod_scripting::ScriptCollection` components, as illustrated below:

```rust
use std::sync::Mutex;
use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;

// An example of a startup system which loads the lua script "console_integration.lua"
// placed in "assets/scripts/" and attaches it to a new entity
#[cfg(feature = "lua")]
pub fn load_a_script(
    server: Res<AssetServer>,
    mut commands: Commands,
) {
    // this handle is kept by the script so it will not be unloaded
    let path = "scripts/console_integration.lua".to_string();
    let handle = server.load::<LuaFile>(&path);


    commands.spawn(()).insert(ScriptCollection::<LuaFile> {
        scripts: vec![Script::<LuaFile>::new(
            path, handle,
        )],
    });
}
```

### Defining an API

To make an API accessible to your scripts, you need to implement the `APIProvider` trait. This can be registered with your script host using the `add_api_provider` method of `App`. `APIProviders` function similarly to plugins:

```rust
use ::std::sync::Mutex;
use bevy_mod_scripting::prelude::*;

#[cfg(feature = "lua")]
#[derive(Default)]
pub struct LuaAPI;

#[cfg(feature = "lua")]
impl APIProvider for LuaAPI {
    type APITarget = Mutex<Lua>;
    type DocTarget = LuaDocFragment;
    type ScriptContext = Mutex<Lua>;

    fn attach_api(&mut self, ctx: &mut Self::APITarget) -> Result<(),ScriptError> {
        // ... access the lua context here when the script loads
        Ok(())
    }
}
```

Register your API providers like so:

```rust, ignore
    app.add_plugins(DefaultPlugins)
        .add_plugins(ScriptingPlugin)
        .add_script_host::<LuaScriptHost<MyLuaArg>>(PostUpdate)
        .add_api_provider::<LuaScriptHost<MyLuaArg>>(Box::new(LuaAPI))
        //...
```
The `APIProvider` interface also includes `setup_script` and `get_doc_fragment` methods. By default, these methods do not perform any operation. However, they can be utilized for specific purposes. For instance, `get_doc_fragment` can be used to generate documentation (refer to examples), and `setup_script` can ensure a one-time setup per script, like setting up a Lua package path.

### Documentation Generation

Documentation features are exposed at runtime via the `update_documentation` builder trait method for `App`:

```rust, no_run
use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugins(ScriptingPlugin);
    #[cfg(feature = "lua")]
    {
    app.add_script_host::<LuaScriptHost<()>>(PostUpdate)
        // Note: This is a noop in optimized builds unless the `doc_always` feature is enabled!
        // this will pickup any API providers added *BEFOREHAND* like this one
        .add_api_provider::<LuaScriptHost<()>>(Box::new(LuaBevyAPIProvider))
        .add_api_provider::<LuaScriptHost<()>>(Box::new(LuaCoreBevyAPIProvider))
        .update_documentation::<LuaScriptHost<()>>()
        .add_script_handler::<LuaScriptHost<()>, 0, 0>(PostUpdate);
    }

    Ok(())
}

```
#### Lua

`tealr`, a wrapper around the `mlua` crate, provides mechanisms for Lua documentation generation. It can generate `d.tl` files for static typing in Lua via the `teal` project, but using `teal` isn't necessary for documentation generation. 

See [this example](examples/lua/documentation_gen.rs) for a demonstration. 

The Bevy API documentation for this crate is auto-generated with each release and can be found [here](https://github.com/makspll/bevy_mod_scripting_lua) and [here](https://makspll.github.io/bevy_mod_scripting_lua/v0.3.0/). You may need to adjust the `page_root` in the auto-generated `assets/doc/tealr_doc_gen_config.json` file to a path like `assets/doc/YourAPI`.

##### Teal - Lua static typing

Teal is the recommended way of introducing lua to your bevy game. This functionality is locked behind the `teal` cargo feature however, since it's quite opinionanted when it comes to your asset structure (`script` and `scripts/build`, folders under `assets`), and also requires `lua` + `teal` + `tealr_doc_gen` (`cargo install --git https://github.com/lenscas/tealr_doc_gen --rev 91afd4a528e7f5b746ac3a6b299c422b42c05db6`) to be installed (see https://github.com/teal-language/tl and `tealr`).

Once enabled, `.tl` files can be loaded as lua scripts in addition to `.lua` files and compiled on the fly. With full hot-reloading support. When you're ready to release your game, you just need to run `tl build` from the `assets/scripts` directory to compile your teal files. This will generate `.lua` files under `assets/scripts/build`. You can manage loading scripts using the [`bevy_mod_scripting::lua_path`] macro.

If `teal` is enabled and you've added the `update_documentation` step to your app, every time you run/build your app in development the following will be generated/synced: - a `scripts/doc` directory containing documentation for your lua exposed API - a `scripts/types` directory containing `.d.tl` files for your lua IDE - a `scripts/tlconfig.lua` file will be generated _once_ if it does not yet exist - any scripts with a `.tl` extension will be compiled to lua code and type checked
On optimized release builds none of this happens (no debug_asserts).

The recommended workflow is to use vscode and the official teal extension with an additional `tlconfig.lua` file at the **root** of your workspace with the
following content:

```lua
return {
    include_dir = {
        "path_to_your_lib/",
    }
}
```

## Configuration

- `SCRIPT_DOC_DIR` - documentation is generated in `assets/scripts/docs` or to the path in this ENV variable if it's set.

## Examples

To see more complex applications of this library have a look at the examples:

- [lua - complex game loop](examples/lua/complex_game_loop.rs)
- [lua - event recipients](examples/lua/event_recipients.rs)
- [lua - bevy API](examples/lua/bevy_api.rs)
- [rhai - bevy API](examples/rhai/bevy_api.rs)
- [generating statically typed lua wrappers + ReflectReference system](examples/wrappers.rs)
- [lua - documentation generation + lua static typing](examples/lua/documentation_gen.rs)
- [lua - bevy console integration](examples/lua/console_integration.rs)
- [rhai - bevy console integration](examples/rhai/console_integration.rs)
- [lua - game of life with teal](examples/lua/game_of_life.rs)
- [rhai - game of life](examples/rhai/game_of_life.rs)

Below is a video showcasing the game_of_life example:
[![Watch the video](https://img.youtube.com/vi/Mo9gh2g3ZHw/maxresdefault.jpg)](https://www.youtube.com/watch?v=Mo9gh2g3ZHw)

# Compatibility

| bevy_mod_scripting  | bevy   |
|---------------------|--------|
| 0.7                 | 0.14   |
| 0.6                 | 0.13.1 |
