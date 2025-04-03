# Installation

## Cargo

First you need to install the crate by adding this entry to your `Cargo.toml` dependencies list:

```toml
bevy_mod_scripting = { version = "0.9.0", features = ["lua54"]}
```

Choose the language features you wish enabled and add them to the features block.


## Bevy Plugin

The next step is to add the BMS plugin to your application, on top of any other extras you want included in your app:

```rust,ignore
app.add_plugins((
    LuaScriptingPlugin::default(),
    ScriptFunctionsPlugin
));
```

The above is how you'd setup BMS for Lua, if you want to use another language, simply use a corresponding plugin from the integration crate.


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
| bevy_core_bindings | Enables bindings for the `bevy_core` module |
| bevy_ecs_bindings | Enables bindings for the `bevy_ecs` module |
| bevy_hierarchy_bindings | Enables bindings for the `bevy_hierarchy` module |
| bevy_input_bindings | Enables bindings for the `bevy_input` module |
| bevy_math_bindings | Enables bindings for the `bevy_math` module |
| bevy_reflect_bindings | Enables bindings for the `bevy_reflect` module |
| bevy_time_bindings | Enables bindings for the `bevy_time` module |
| bevy_transform_bindings | Enables bindings for the `bevy_transform` module |
| mlua_async | Enables `mlua/async`|
| mlua_serialize | Enables `mlua/serialize` |
| mlua_macros | Enables `mlua/macros` |
| unsafe_lua_modules | Allows loading unsafe modules via `require` in lua |
 


