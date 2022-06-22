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
- [ ] General Bevy API for all script hosts (i.e. Add component, remove component etc.). Blocked by <https://github.com/bevyengine/bevy/issues/4474>
- [x] Utilities for generating script native documentation 
- [ ] Tests

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
To expose an API to your scripts, implement the APIProvider trait along with any others requested by the specific `ScriptHost` your're using. Use the implementing struct in yout `ScriptHost`s type signature

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
        // this needs to be placed after any `add_api_provider` and `add_script_host` calls
        // it will generate `doc` and `types` folders under `assets/scripts` containing the documentation and teal declaration files
        // respectively. See example asset folder to see how they look like. The `teal_file.tl` script in example assets shows the usage of one of those 
        // declaration files, use the teal vscode extension to explore the type hints!

        // Note: This is a noop in optimized builds unless the `doc_always` feature is enabled!
        .update_documentation::<RLuaScriptHost<MyLuaArg>>()
        .add_script_handler_stage::<RLuaScriptHost<MyLuaArg>, _, 0, 0>(
            CoreStage::PostUpdate,
        );

    Ok(())
}

```

#### Lua
Lua 


#### Rhai

Rhai currently does not have any utilities existing for generating documentation, once something comes out we'll include it.


## Examples 

To see more complex applications of this library have a look at the examples:

- [lua - bevy console integration](bevy_mod_scripting/examples/console_integration_lua.rs)
- [rhai - bevy console integration](bevy_mod_scripting/examples/console_integration_rhai.rs)
- [lua - complex game loop](bevy_mod_scripting/examples/complex_game_loop.rs)
