<p align="center">
    <img src="./logo_bevy_scripting.svg" width="250" onerror="this.onerror=null; this.src='https://raw.githubusercontent.com/makspll/bevy_mod_scripting/main/logo_bevy_scripting.svg'">
</p>

# Bevy Scripting

While for Bevy out of the box scripting is a non-goal, scripting support is being worked on.
This crate is an attempt to make scripting a possibility with the current state of Bevy. It is by no means optimized, perfect or even complete so far. The API will likely change a lot as Bevy grows.

## Why Use Scripts?

- Re-load your game logic without re-compiling the entire crate
- If your game logic is encapsulated with scripts it becomes easily moddable
- Allows writing game logic/UI in a smaller language, making development more accessible to non-coders on your team

## Features

- Script host interface
- Hot re-loading scripts (on script asset changes, scripts using those assets are re-started)
- Mlua integration
- Rhai integration
- Customisable script API's
- Event based hooks (i.e. on_update)
- Flexible event scheduling (i.e. allow handling events at handling stages based on the event)
- Multiple scripts per entity
- Multiple instances of the same script on one entity
- Extensive callback argument type support
- General Bevy API.
- Lua implementation of Bevy API (and support for more langauges incoming)
- Utilities for generating script native documentation
- Loading external lua libraries via `require` (enabled with `unsafe_lua_modules` cargo feature due to potential unsafety)

## Support

Support for languages is expressed in three levels:

1. `ScriptHost` implementation, scripts can be loaded, scheduled and run in this language with support for custom `APIProvider`
2. A `<language>BevyAPIProvider` is implemented which enables access to `entity`,`world` etc and provides support for at least basic operations such as `get_component`, `add_component`, `spawn` etc
3. Macros for generating proxy wrapper structures exist and can be used for custom types with the ability to add script-side functionality
4. Macros instantiations are automatically generated for native Bevy structures

The languages currently supported are as follows:
|Language| Support Level | Documentation Generation |
|----|----|----|
|Lua|4|Yes|
|Rhai|2|No|

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
        app.add_plugin(ScriptingPlugin)
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

Scripts are triggered by firing `ScriptEvents`. This crate uses custom priority event writers and readers, so events are sent along with a priority. Together with your event pipeline this priority affects when your events are handled. A priority of 0 is the highest.

You can use this to create game loops akin to Unity's or other game engine's.

There are no guarantees that force the script callbacks to be executed fully for all scripts, i.e. before processing the next callback event, so this order guarantee only holds on a per script basis.

Examples of systems which generate callbacks can be seen below:

#### Mlua

Use valid lua function names for hook names and any number of arguments which are to be passed on to the function.

Any types implementing the `mlua::ToLua` trait can be used.

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

#### Rhai

Rhai supports any rust types implementing FuncArgs as function arguments.

```rust
use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;

#[cfg(feature = "rhai")]
#[derive(Clone)]
pub struct MyRhaiArgStruct {
    // ...
}

#[cfg(feature = "rhai")]
impl FuncArgs for MyRhaiArgStruct {
    fn parse<ARGS: Extend<rhai::Dynamic>>(self, _args: &mut ARGS) {
        // ...
    }
}

// event callback generator for rhai
// rhai event arguments can be any rust type implementing FuncArgs
#[cfg(feature = "rhai")]
pub fn trigger_on_update_rhai(mut w: PriorityEventWriter<RhaiEvent<MyRhaiArgStruct>>) {
    let event = RhaiEvent {
        hook_name: "on_update".to_string(),
        args: MyRhaiArgStruct {},
        recipients: Recipients::All
    };

    w.send(event,0);
}
```

### Adding scripts

A script consists of:

- an asset handle to their code file
- a name which is usually their path relative to the assets folder

Scripts are attached to entities in the form of `bevy_mod_scripting::ScriptCollection` components as seen below:

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
    let handle = server.load::<LuaFile, &str>(&path);


    commands.spawn(()).insert(ScriptCollection::<LuaFile> {
        scripts: vec![Script::<LuaFile>::new(
            path, handle,
        )],
    });
}
```

### Defining an API

To expose an API to your scripts, implement the APIProvider trait. To register this API with your script host use the `add_api_provider` of `App`. APIProviders are a little bit like plugins, since they can also have access to the bevy App via one of the methods provided, and

```rust
use ::std::sync::Mutex;
use bevy_mod_scripting::prelude::*;

#[cfg(feature = "lua")]
#[derive(Default)]
pub struct LuaAPI;

/// the custom Lua api, world is provided via a global pointer,
/// and callbacks are defined only once at script creation
#[cfg(feature = "lua")]
impl APIProvider for LuaAPI {
    type APITarget = Mutex<Lua>;
    type DocTarget = LuaDocFragment;
    type ScriptContext = Mutex<Lua>;

    fn attach_api(&mut self, ctx: &mut Self::APITarget) -> Result<(),ScriptError> {
        // generate API here
        // world is provided via ctx see examples
        // ...
        Ok(())
    }
}

#[cfg(feature = "rhai")]
#[derive(Default)]
pub struct RhaiAPI {}

#[cfg(feature = "rhai")]
impl APIProvider for RhaiAPI {
    type APITarget = Engine;
    type DocTarget = RhaiDocFragment;
    type ScriptContext = RhaiContext;

    fn attach_api(&mut self, ctx: &mut Self::APITarget) -> Result<(),ScriptError> {
        // ...
        Ok(())
    }
}

```

Register the API providers like so:

```rust, ignore
    app.add_plugins(DefaultPlugins)
        .add_plugin(ScriptingPlugin)
        .add_script_host::<LuaScriptHost<MyLuaArg>>(PostUpdate)
        .add_script_host::<RhaiScriptHost<MyRhaiArg>>(PostUpdate)
        .add_api_provider::<LuaScriptHost<MyLuaArg>>(Box::new(LuaAPIProvider))
        .add_api_provider::<LuaScriptHost<MyRhaiArg>>(Box::new(RhaiAPIProvider))
        //...
```

Note that the `APIProvider` interface also contains `setup_script` and `get_doc_fragment` methods which are by default no-ops. These can be used to provide documentation (see examples) and guaranteed one-time-per-script setup (such as lua package path setup).

### Documentation Generation

Documentation features are exposed at runtime via the `update_documentation` builder trait method for `App`:

```rust, no_run
use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;

#[cfg(feature = "lua")]
fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(ScriptingPlugin)
        .add_script_host::<LuaScriptHost<()>>(PostUpdate)
        // Note: This is a noop in optimized builds unless the `doc_always` feature is enabled!
        // this will pickup any API providers added *BEFOREHAND* like this one
        // .add_api_provider::<LuaScriptHost<()>>(Box::new(LuaAPIProvider))
        .update_documentation::<LuaScriptHost<()>>()
        .add_script_handler::<LuaScriptHost<()>, 0, 0>(PostUpdate);

    Ok(())
}

```

Currently we generate documentation at runtime due to the way `tealr` works but this might change in the future as ideally this would be done statically.

It is probably a wise idea to set up a separate executable whose only purpose is to generate documentation, and run it every time before a release. But keeping this step in your main app will make sure your script environment is always set up correctly.

#### Lua

Lua documentation is provided by `tealr`, a wrapper around the `mlua` lua API which decorates their standard types. On top of providing documentation generation, it's also capable of generating `d.tl` files which can be used to introduce static typing to lua via the `teal` project (you do not need to use teal to generate documentation).

This can all be seen at work in [this example](bevy_mod_scripting/examples/lua/documentation_gen.rs). 

The docs for the bevy API provided in this crate are generated automatically each release onto this repo [here](https://github.com/makspll/bevy_mod_scripting_lua) and deployed [here](https://makspll.github.io/bevy_mod_scripting_lua/v0.3.0/). You might need to set the `page_root` to the path to something like: `assets/doc/YourAPI` in the automatically generated config file over at: `assets/doc/tealr_doc_gen_config.json`

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

#### Rhai

Rhai currently does not have any existing utilities for generating documentation (for the rust provided API), once something comes out we'll include it.

## Configuration

- `SCRIPT_DOC_DIR` - documentation is generated in `assets/scripts/docs` or to the path in this ENV variable if it's set.

## Scenes

The `Script` components will persist a scene load, but their script contexts won't, after a scene load you must manually reload the scripts using `Script::reload_script`

## Examples

To see more complex applications of this library have a look at the examples:

- [lua - complex game loop](examples/lua/complex_game_loop.rs)
- [lua - event recipients](examples/lua/event_recipients.rs)
- [lua - bevy API](examples/lua/bevy_api.rs)
- [rhai - bevy API](examples/rhai/bevy_api.rs)
- [generating statically typed lua wrappers + ScriptRef system](examples/wrappers.rs)
- [lua - documentation generation + lua static typing](examples/lua/documentation_gen.rs)
- [lua - bevy console integration](examples/lua/console_integration.rs)
- [rhai - bevy console integration](examples/rhai/console_integration.rs)
- [lua - game of life with teal](examples/lua/game_of_life.rs)
- [rhai - game of life](examples/rhai/game_of_life.rs)

Below is a video showcasing the game_of_life example:
[![Watch the video](https://img.youtube.com/vi/Mo9gh2g3ZHw/maxresdefault.jpg)](https://www.youtube.com/watch?v=Mo9gh2g3ZHw)
