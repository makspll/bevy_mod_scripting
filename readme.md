<p align="center">
    <img src="./logo_bevy_scripting.svg" width="250">
</p>
# Bevy Scripting

While for Bevy out of the box scripting is a non-goal, scripting support is being worked on. 
This crate is an attempt to make scripting a possibility with the current state of Bevy.

The API will likely change in the future as more scripting support is rolled out.


## Why Use Scripts?

- Re-load your game logic without re-compiling the entire crate
- If your game logic is encapsualted with scripts it becomes moddable
- Scripting allows for writing game logic/UI in a simpler language, making development more accessible to non-coders on your team

## State of this crate

- [x] Script host interface
- [x] Hot re-loading scripts (on script asset changes, scripts using those assets are re-started)
- [x] Rlua integration
- [x] Rhai integration
- [x] Customisable Lua API
- [x] Event based hooks (i.e. on_update)
- [x] Flexible event scheduling (i.e. allow handling events at different stages rather than a single stage based on the event) 
- [x] Multiple scripts per entity
- [x] Multiple instances of the same script on one entity
- [x] Extensive callback argument type support 
- [ ] General Bevy API for all script hosts (i.e. Add component, remove component etc.). Blocked by <https://github.com/bevyengine/bevy/issues/4474>
- [ ] Tests

## Usage

### Installation

To install:
- Add ScriptingPlugin to your app
- Add the ScriptHosts you plan on using
    - Make sure to attach them to a stage running AFTER any systems which may generate either script events or modify/create/remove script components  
- Add systems which generate ScriptEvents corresponding to your script host
- Add systems which add ScriptCollection components to your entities and fill them with scripts

An example can be seen below

```rust,ignore

fn main() -> std::io::Result<()> {
    let mut app = App::new();
        app.add_plugin(ScriptingPlugin)
        .add_plugins(DefaultPlugins) 

        // pick and register only the hosts you want to use
        // use any stage AFTER you main game systems
        // in order for your script updates to propagate in a  
        // single frame
        .add_script_host::<RhaiScriptHost<MyRhaiArgStruct, RhaiAPI>,CoreStage>(CoreStage::PostUpdate)    
        .add_script_host::<RLuaScriptHost<MyLuaArgStruct,LuaAPI>,CoreStage>(CoreStage::PostUpdate)

        // generate events for scripts to pickup
        .add_system(trigger_on_update_script_callback)
        .add_system(trigger_on_update_rhai)

        // attach script components to entities
        .add_startup_system(load_a_script);

    // app.run();

    Ok(())
}
```

### Firing Script Callbacks

Scripts are triggered by firing 'ScriptEvents', the order of events matters so trigger them in the order you'd like your scripts to process them.

As it stands currently there are no guarantees that force the script callbacks to be executed fully for all scripts, before processing the next callback event (i.e. this order guarantee only holds on a per script basis).

#### RLua 

Use valid lua function names for hook names and any number of arguments which are passed on to the function. 

Any types implementing the `rlua::ToLua` trait can be used.

``` rust
use bevy::prelude::*;
use bevy_scripting::*;
use rlua::ToLua;

#[derive(Clone)]
pub struct MyLuaArg;

impl<'lua> ToLua<'lua> for MyLuaArg {
    fn to_lua(self, _lua: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        // ...
        Ok(rlua::Value::Nil)
    }
}

// event callback generator for lua
// right now only integer arguments are supported
pub fn trigger_on_update_script_callback(mut w: EventWriter<LuaEvent<MyLuaArg>>) {
    let event = LuaEvent::<MyLuaArg> {
        hook_name: "on_update".to_string(), 
        args: Vec::default(),
    };

    w.send(event);
}
```

#### Rhai

Rhai supports the use of any rust types implementing FuncArgs as function arguments (apart from references),

``` rust
use bevy::prelude::*;
use rhai::FuncArgs;
use bevy_scripting::*;

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
pub fn trigger_on_update_rhai(mut w: EventWriter<RhaiEvent<MyRhaiArgStruct>>) {
    let event = RhaiEvent {
        hook_name: "on_update".to_string(),
        args: MyRhaiArgStruct {},
    };

    w.send(event);
}
```

### Adding scripts

A script consist of:
- an asset handle to their code file
- a name which is usually their path relative to the assets folder

``` rust
use std::sync::Mutex;
use rlua::prelude::*;
use bevy::prelude::*;
use bevy_scripting::*;


#[derive(Default)]
pub struct LuaAPI {}

impl APIProvider for LuaAPI {
    type Ctx = Mutex<Lua>;
    fn attach_api(ctx: &mut Self::Ctx) {}
}

#[derive(Clone)]
pub struct MyLuaArg;

impl<'lua> ToLua<'lua> for MyLuaArg {
    fn to_lua(self, _lua: rlua::Context<'lua>) -> rlua::Result<rlua::Value<'lua>> {
        // ...
        Ok(rlua::Value::Nil)
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
        scripts: vec![Script::<LuaFile>::new::<RLuaScriptHost<MyLuaArg,LuaAPI>>(
            path, handle,
        )],
    });
}
```


### Defining an API
Simply implement the APIProvider trait + any others required by the specific ScriptHost your're using, and use your struct in the ScriptHost type

``` rust
use std::sync::Mutex;
use rhai::*;
use rlua::prelude::*;
use bevy_scripting::*;

#[derive(Default)]
pub struct LuaAPI {}

/// the custom Lua api, world is provided via a global pointer,
/// and callbacks are defined only once at script creation
impl APIProvider for LuaAPI {
    type Ctx = Mutex<Lua>;
    fn attach_api(ctx: &mut Self::Ctx) {
        // generate API here
        // world is provided via ctx see examples
        // ...
    }
}

#[derive(Default)]
pub struct RhaiAPI {}

impl APIProvider for RhaiAPI {
    type Ctx = RhaiContext;

    fn attach_api(ctx: &mut Self::Ctx) {
        // ...
    }
}

impl RhaiAPIProvider for RhaiAPI{
    fn setup_engine(engine : &mut Engine){
        // ...
    }
}
```

## Examples 

Examples are available in the examples directory:

- [lua - bevy console integration](bevy_scripting/examples/console_integration_lua.rs)
- [rhai - bevy console integration](bevy_scripting/examples/console_integration_rhai.rs)
- [lua - complex game loop](bevy_scripting/examples/complex_game_loop.rs)
