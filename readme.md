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
- [ ] Rhai integration
- [x] Customisable Lua API
- [x] Event based hooks (i.e. on_update)
- [ ] Flexible event scheduling (i.e. allow handling events at different stages rather than a single stage based on the event) 
- [x] Multiple scripts per entity
- [x] Multiple instances of the same script on one entity (unlike Unity)
- [ ] Improved Ergonomics (some types are cumbersome right now)
- [ ] General Bevy API for all script hosts (i.e. Add component, remove component etc.). Blocked by https://github.com/bevyengine/bevy/issues/4474
- [ ] More extensive callback argument type support 
- [ ] Tests

## Usage

### Installation

``` rust
fn main() -> std::io::Result<()> {
    let mut app = App::new();
    app.add_plugins(DefaultPlugins)
        .add_plugin(ScriptingPlugin)

    // pick and register host
    RLuaScriptHost::<LuaAPIProvider>::register_with_app(
        &mut app,   
        CoreStage::PostUpdate // use any stage AFTER you main game systems
                              // in order for your script updates to propagate in a  
                              // single frame
    );

    app.run();

    Ok(())
}
```

### Firing Script Callbacks

Scripts are triggered by firing events, the order of events matters so trigger them in the order you'd like your scripts to process them.

As it stands currently there are no guarantees that force the script callbacks to be executed fully for all scripts, before processing the next callback event (i.e. this order guarantee only holds on a per script basis).

#### RLua 

Use valid lua function names for hook names and any number of arguments which are passed on to the function. 

Currently only integer arguments are supported.

``` rust 
pub fn trigger_on_update_script_callback(mut w: EventWriter<LuaEvent>) {
    let event = LuaEvent {
        hook_name: "on_update".to_string(), 
        args: Vec::default(),
    };

    w.send(event);
}
```


## Examples 

Examples are available in the examples directory, 

have a look at how you can use this crate in conjunction with bevy_debug_console and bevy_asset_loader: 
[link](bevy_scripting/examples/console_integration.rs)

to run this example use:

`cargo run --console_integration`

then in-game use `~` to bring up the console, then use:

`run_script "console_integration.lua"`
