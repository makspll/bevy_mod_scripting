# Bevy Scripting

While for Bevy out of the box scripting is a non-goal, scripting support is being worked on. 

This crate is an attempt to make multi-language scripting a possibility with the current state of Bevy.

## State of this crate

- [x] Script host interface
- [x] Rlua integration
- [ ] Rhai integration
- [x] Customisable Lua API
- [x] Event based hooks (i.e. on_update)
- [ ] Flexible event scheduling (i.e. allow handling events at different stages rather than a single stage based on the event) 
- [ ] Improved Ergonomics 
- [ ] Tests


As of now script component removals do not work properly just yet

## Examples 

Examples are available in the examples directory, 

have a look at how you can use this crate in conjunction with bevy_debug_console and bevy_asset_loader: 
[link](bevy_scripting/examples/console_integration.rs)

to run this example use:

`cargo run --console_integration`

then in-game use `~` to bring up the console, then use:

`run_script "console_integration.lua"`
