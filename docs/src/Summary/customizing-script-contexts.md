# Modifying Script Contexts

You should be able to achieve what you need by registering script functions in most cases. However sometimes you might want to override the way contexts are loaded, or how the runtime is initialized.

This is possible using `Context Initializers` and `Context Pre Handling Initializers` as well as `Runtime Initializers`.

It is however always reccomened to use the dynamic script function registry whenever possible, as it is more flexible and easier to use. It also allows you to introspect available functions easier.

## Context Initializers

For example, let's say you want to set a dynamic amount of globals in your script, depending on some setting in your app.

You could do this by customizing the scripting plugin:
```rust,ignore
let plugin = LuaScriptingPlugin::default().add_context_initializer(|_context_key: &ContextKey, context: &mut Lua| {
    let globals = context.globals();
    for i in 0..10 {
        globals.set(i, i);
    }
    Ok(())
});

app.add_plugins(plugin)
```

The above will run every time the script is loaded or re-loaded and before it handles any callbacks.

## Context Pre Handling Initializers

If you want to customize your context before every time it's about to handle events (and when it's loaded + reloaded), you can use `Context Pre Handling Initializers`:
```rust
use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;
fn scripting_plugin(app: &mut App) {
    app.add_plugins(LuaScriptingPlugin::default()
                       .add_context_pre_handling_initializer(|context_key: &ContextKey, entity: Entity, context: &mut Lua| {
        let globals = context.globals();
        if let Some(script_id) = context_key.script_id.as_ref() {
            globals.set("script_name", script_id.to_owned());
        }
        Ok(())
    }));
}
```
## Runtime Initializers

Some scripting languages, have the concept of a `runtime`. This is a global object which is shared between all contexts. You can customize this object using `Runtime Initializers`:
```rust
use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;
fn scripting_plugin(app: &mut App) {
    app.add_plugin(SomeScriptingPlugin::default().add_runtime_initializer(|runtime: &mut Runtime| {
        runtime.set_max_stack_size(1000);
        Ok(())
    }));
    
}
```

In the case of Lua, the runtime type is `()` i.e. This is because `mlua` does not have a separate runtime concept.

## Accessing the World in Initializers

You can access the world in these initializers by using the thread local: `ThreadWorldContainer`:
```rust
use bevy::prelude::*;
use bevy_mod_scripting::prelude::*;
fn scripting_plugin(app: &mut App) {
    let plugin = LuaScriptingPlugin::default();
    plugin.add_context_initializer(|_context_key: &ContextKey, context: &mut Lua| {
        let world = ThreadWorldContainer.try_get_world().unwrap();
        world.with_resource::<MyResource>(|res| println!("My resource: {:?}", res));
        Ok(())
    });
    app.add_plugins(plugin);
}
```
