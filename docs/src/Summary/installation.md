# Installation

## Cargo

First you need to install the crate by adding this entry to your `Cargo.toml` dependencies list:

```toml
bevy_mod_scripting = { version = "0.9.0", features = ["lua54"]}
```

Choose the language features you wish enabled and add them to the features block.


## Bevy Plugin

The next step is to add the BMS plugin to your application.

```rust,ignore
app.add_plugins(BMSPlugin);
```

You can modify each of the plugins contained within the plugin group using `set(MySubPlugin)`.

## Language Features

Each language supported by BMS can be switched-on via feature flag as below:

| Language | Feature Flag |
| ---- | ---- |
| Lua51 | lua51 | 
| Lua52 | lua54 |
| Lua53 | lua53 |
| Lua54 | lua54 |
| Luajit | luajit |
| Luajit52 | luajit52 |
| Luau | luau |
| Rhai | rhai |
| Rune | rune |

## Extra Features

In order to fit as many use cases as possible, BMS allows you to disable a lot of its functionality. 

By default all of the useful features are enabled, but you may disable them if you wish if you are only needing BMS for script lifecycle management, and want to populate the bindings yourself.

| Feature | Description |
| ---- | ---- | 
| core_functions | If enabled, will enable all core functions, i.e. bevy integrations which let you interact with Bevy via reflection |
| bevy_<bevy_crate>_bindings | enables bindings for the specified bevy crate, e.g. `bevy_ecs`, `bevy_transform`, `bevy_pbr` etc. for a full list of available modules check out [The Cargo manifest](https://github.com/makspll/bevy_mod_scripting/blob/432b91b1022a133b34d1b4a7eb382268f34b76ea/crates/bevy_mod_scripting_functions/Cargo.toml#L14) | 
| mlua_async | Enables `mlua/async`|
| mlua_serialize | Enables `mlua/serialize` |
| mlua_macros | Enables `mlua/macros` |
| unsafe_lua_modules | Allows loading unsafe modules via `require` in lua |
 


