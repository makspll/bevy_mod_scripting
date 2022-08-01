<p align="center">
    <img src="./logo_bevy_scripting.svg" width="250">
</p>

# Bevy Scripting

While for Bevy out of the box scripting is a non-goal, scripting support is being worked on. 
This crate is an attempt to make scripting a possibility with the current state of Bevy.

The API will likely change in the future as more scripting support is rolled out.


## Why Use Scripts?

- Re-load your game logic without re-compiling the entire crate
- If your game logic is encapsulated with scripts it becomes easilly moddable
- Allows writing game logic/UI in a smaller language, making development more accessible to non-coders on your team

## State of this crate

- [x] Script host interface
- [x] Hot re-loading scripts (on script asset changes, scripts using those assets are re-started)
- [x] Rlua integration
- [x] Rhai integration
- [x] Customisable script API's
- [x] Event based hooks (i.e. on_update)
- [x] Flexible event scheduling (i.e. allow handling events at different stages rather than a single stage based on the event) 
- [x] Multiple scripts per entity
- [x] Multiple instances of the same script on one entity
- [x] Extensive callback argument type support 
- [ ] General Bevy API for all script hosts (i.e. Add component, remove component etc.). On track for Bevy 0.8
- [x] Utilities for generating script native documentation 
- [ ] Tests
- [x] Optionally loading external lua libraries via `require` (enabled with `unsafe_lua_modules` cargo feature due to potential unsafety)

## Usage

### Installation

To install:
- Add this crate to your Cargo.toml file dependencies
    - The crate is still in development so I recommended pinning to a git commit    
- Add ScriptingPlugin to your app
- Add the ScriptHosts you plan on using (`add_script_host`)
    - Make sure to attach it to a stage running AFTER any systems which may generate modify/create/remove script components
- Add script handler stages to capture events in the priority range you're expecting (`add_script_handler_stage`)   
- Add systems which generate ScriptEvents corresponding to your script host
- Add systems which add ScriptCollection components to your entities and fill them with scripts

An example can be seen below

```rust,ignore

fn main() -> std::io::Result<()> {
    let mut app = App::new();
        app.add_plugin(ScriptingPlugin)
        .add_plugins(DefaultPlugins) 
        // pick and register only the hosts you want to use
        // use any stage AFTER any systems which add/remove/modify script components 
        // in order for your script updates to propagate in a single frame
        .add_script_host::<RhaiScriptHost<MyRhaiArgStruct>,_>(CoreStage::PostUpdate)    
        .add_script_host::<RLuaScriptHost<MyLuaArgStruct>,_>(CoreStage::PostUpdate)
        
        // the handlers should be placed after any stages which produce script events
        // PostUpdate is okay only if your API doesn't require the core Bevy systems' commands
        // to run beforehand.
        // Note, this setup assumes a single script handler stage with all events having identical
        // priority of zero (see examples for more complex scenarios)
        .add_script_handler_stage::<RLuaScriptHost<MyLuaArg>, _, 0, 0>(
            CoreStage::PostUpdate,
        )
        .add_script_handler_stage::<RhaiScriptHost<RhaiEventArgs>, _, 0, 0>(
            CoreStage::PostUpdate,
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

``` rust
use bevy::prelude::*;
use bevy_mod_scripting::{*,events::*,langs::mlu::{mlua,mlua::prelude::*}};


#[derive(Clone)]
pub struct MyLuaArg;

impl<'lua> ToLua<'lua> for MyLuaArg {
    fn to_lua(self, _lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        Ok(mlua::Value::Nil)
    }
}

// event callback generator for lua
pub fn trigger_on_update_lua(mut w: PriorityEventWriter<LuaEvent<MyLuaArg>>) {
    let event = LuaEvent::<MyLuaArg> {
        hook_name: "on_update".to_string(), 
        args: Vec::default(),
        recipients: Recipients::All
    };

    w.send(event,0);
}
```

#### Rhai

Rhai supports any rust types implementing FuncArgs as function arguments.

``` rust
use bevy::prelude::*;
use bevy_mod_scripting::{*,events::*,langs::rhai::FuncArgs};

#[derive(Clone)]
pub struct MyRhaiArgStruct {
    // ...
}

impl FuncArgs for MyRhaiArgStruct {
    fn parse<ARGS: Extend<rhai::Dynamic>>(self, _args: &mut ARGS) {
        // ... 
    }
}

// event callback generator for rhai
// rhai event arguments can be any rust type implementing FuncArgs
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

A script consist of:
- an asset handle to their code file
- a name which is usually their path relative to the assets folder

Scripts are attached to entities in the form of `bevy_mod_scripting::ScriptCollection` components as seen below:

``` rust
use std::sync::Mutex;
use bevy::prelude::*;
use bevy_mod_scripting::{*,langs::mlu::mlua};

#[derive(Clone)]
pub struct MyLuaArg;

impl<'lua> mlua::ToLua<'lua> for MyLuaArg {
    fn to_lua(self, _lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {
        // ...
        Ok(mlua::Value::Nil)
    }
}

// An example of a startup system which loads the lua script "console_integration.lua" 
// placed in "assets/scripts/" and attaches it to a new entity
pub fn load_a_script(
    server: Res<AssetServer>,
    mut commands: Commands,
) {
    // this handle is kept by the script so it will not be unloaded
    let path = "scripts/console_integration.lua".to_string();
    let handle = server.load::<LuaFile, &str>(&path);


    commands.spawn().insert(ScriptCollection::<LuaFile> {
        scripts: vec![Script::<LuaFile>::new::<RLuaScriptHost<MyLuaArg>>(
            path, handle,
        )],
    });
}
```


### Defining an API
To expose an API to your scripts, implement the APIProvider trait. To register this API with your script host use the `add_api_provider` of `App`:

``` rust
use std::sync::Mutex;
use bevy_mod_scripting::{*,langs::{mlu::mlua::prelude::*,rhai::*}};

#[derive(Default)]
pub struct LuaAPI {}

/// the custom Lua api, world is provided via a global pointer,
/// and callbacks are defined only once at script creation
impl APIProvider for LuaAPI {
    type Target = Mutex<Lua>;
    type DocTarget = LuaDocFragment;

    fn attach_api(&mut self, ctx: &mut Self::Target) -> Result<(),ScriptError> {
        // generate API here
        // world is provided via ctx see examples
        // ...
        Ok(())
    }
}

#[derive(Default)]
pub struct RhaiAPI {}

impl APIProvider for RhaiAPI {
    type Target = Engine;
    type DocTarget = RhaiDocFragment;

    fn attach_api(&mut self, ctx: &mut Self::Target) -> Result<(),ScriptError> {
        // ...
        Ok(())
    }
}

```

Register the API providers like so:

```rust,ignore
    app.add_plugins(DefaultPlugins)
        .add_plugin(ScriptingPlugin)
        .add_script_host::<RLuaScriptHost<MyLuaArg>, _>(CoreStage::PostUpdate)
        .add_script_host::<RhaiScriptHost<MyRhaiArg>, _>(CoreStage::PostUpdate)
        .add_api_provider::<RLuaScriptHost<MyLuaArg>>(Box::new(LuaAPIProvider))
        .add_api_provider::<RLuaScriptHost<MyRhaiArg>>(Box::new(RhaiAPIProvider))
        //...
```


### Documentation Generation
Documentation features are exposed at runtime via the `update_documentation` builder trait method for `App`:

```rust
use bevy::prelude::*;
use bevy_mod_scripting::{*,langs::mlu::mlua};

#[derive(Clone)]
pub struct MyLuaArg;
impl<'lua> mlua::ToLua<'lua> for MyLuaArg {
    fn to_lua(self, lua: &'lua mlua::Lua) -> mlua::Result<mlua::Value<'lua>> {Ok(mlua::Value::Nil)}
}

fn main() -> std::io::Result<()> {
    let mut app = App::new();

    app.add_plugins(DefaultPlugins)
        .add_plugin(ScriptingPlugin)
        .add_script_host::<RLuaScriptHost<MyLuaArg>, _>(CoreStage::PostUpdate)
        // Note: This is a noop in optimized builds unless the `doc_always` feature is enabled!
        // this will pickup any API providers added *BEFOREHAND* like this one
        // .add_api_provider::<RLuaScriptHost<MyLuaArg>>(Box::new(LuaAPIProvider))
        .update_documentation::<RLuaScriptHost<MyLuaArg>>()
        .add_script_handler_stage::<RLuaScriptHost<MyLuaArg>, _, 0, 0>(
            CoreStage::PostUpdate,
        );

    Ok(())
}

```
Currently we generate documentation at runtime due to the way `tealr` works but this might change in the future as ideally this would be done statically.

It is probably a wise idea to setup a separate executable whose purpose is to only generate documentation, and run it every time before a release. But keeping this step in your main app will make sure your script environment is always setup correctly.

#### Lua

Lua documentation is provided by `tealr`, a wrapper around the `mlua` lua api which decorates their standard types. On top of providing documentation generation it's also capable of generating `d.tl` files which can be used to introduce static typing to lua via the `teal` project (you do not need to use teal to generate documentation). 

This can all be seen at work in the [this example](bevy_mod_scripting/examples/documentation_gen_lua.rs).

##### Teal - Lua static typing

Teal is the reccomended way of introducing lua to your bevy game. This functionality is locked behind the `teal` cargo feature however, since it's quite opinionanted when it comes to your asset structure, and also requires `lua` + `teal` to be installed (see https://github.com/teal-language/tl).

Once enabled, `.tl` files can be loaded as lua scripts in addition to `.lua` files and compiled on the fly. With full hot-reloading support. When you're ready to release your game, you just need to run `tl build` from the `assets/scripts` directory to compile your teal files.

If `teal` is enabled and you've added the `update_documentation` step to your app, every time you run/build your app in development the following will be generated/synced:
    - a `scripts/doc` directory containing documentation for your lua exposed API
    - a `scripts/types` directory containing `.d.tl` files for your lua IDE
    - a `scripts/tlconfig.lua` file will be generated *once* if it does not yet exist
    - any scripts with a `.tl` extension will be compiled to lua code and type checked
On optimized release builds none of this happens (no debug_asserts).

The reccomended workflow is to use vscode and the official teal extension, with the script directory as the root of your workspace (as a second window to your main project), this will ensure your environment is properly configured out of the box.

#### Rhai

Rhai currently does not have any utilities existing for generating documentation (for the rust provided API), once something comes out we'll include it.

## Configuration

- `SCRIPT_DOC_DIR` - documentation is generated in `assets/scripts/docs` or to the path in this ENV variable if it's set.

## Examples 

To see more complex applications of this library have a look at the examples:

- [lua - complex game loop](bevy_mod_scripting/examples/complex_game_loop.rs)
- [lua - event recipients](bevy_mod_scripting/examples/event_recipients.rs)
- [lua - bevy API](bevy_mod_scripting/examples/bevy_api_lua.rs)
- [generating statically typed wrappers + ScriptRef system](bevy_mod_scripting/examples/wrappers.rs)
- [lua - documentation generation + lua static typing](bevy_mod_scripting/examples/documentation_gen_lua.rs)
- [lua - bevy console integration](bevy_mod_scripting/examples/console_integration_lua.rs)
- [rhai - bevy console integration](bevy_mod_scripting/examples/console_integration_rhai.rs)



