# Modifying Script Contexts

You should be able to achieve what you need by registering script functions in most cases. However sometimes you might want to override the way contexts are loaded, or how the runtime is initialized.

This is possible using `Context Initializers` and `Context Pre Handling Initializers` as well as `Runtime Initializers`.


## Context Initializers

For example, let's say you want to set a dynamic amount of globals in your script, depending on some setting in your app.

You could do this by customizing the scripting plugin:
```rust,ignore
let plugin = LuaScriptingPlugin::default();
plugin.add_context_initializer(|script_id: &str, context: &mut Lua| {
    let globals = context.globals();
    for i in 0..10 {
        globals.set(i, i);
    }
});

app.add_plugins(plugin)
```

The above will run every time the script is loaded or re-loaded.

## Context Pre Handling Initializers

If you want to customize your context before every time it's about to handle events, you can use `Context Pre Handling Initializers`:
```rust,ignore
let plugin = LuaScriptingPlugin::default();
plugin.add_context_pre_handling_initializer(|script_id: &str, entity: Entity, context: &mut Lua| {
    let globals = context.globals();
    globals.set("script_name", script_id.to_owned());
});
```
## Runtime Initializers

Some scripting languages, have the concept of a `runtime`. This is a global object which is shared between all contexts. You can customize this object using `Runtime Initializers`:
```rust,ignore
let plugin = SomeScriptingPlugin::default();
plugin.add_runtime_initializer(|runtime: &mut Runtime| {
    runtime.set_max_stack_size(1000);
});
```