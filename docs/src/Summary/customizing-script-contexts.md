# Customizing Contexts

Most of the time, [registering script functions](./controlling-script-bindings.md) is all you need: they work in every language, and show up in the generated docs. For anything else, scripting plugins take initializers which run on each [context](./contexts.md) or runtime:

| Initializer | Runs |
|---|---|
| `add_context_initializer` | Once a context is created, and when its script is reloaded, before any callbacks |
| `add_context_pre_handling_initializer` | Before every callback, and on load and reload |
| `add_runtime_initializer` | Once, on the runtime shared by all contexts |

Initializers are plain functions (`fn` pointers), so they can't capture variables.

## Context initializers

i.e. setting some globals for every Lua script:

```rust,ignore
let plugin = LuaScriptingPlugin::default().add_context_initializer(|_attachment, context| {
    let globals = context.globals();
    for i in 0..10 {
        globals.set(i, i).map_err(IntoInteropError::to_bms_error)?;
    }
    Ok(())
});

app.add_plugins(plugin);
```

## Pre handling initializers

These receive the attachment of the script about to run, i.e. to expose its name:

```rust,ignore
app.add_plugins(LuaScriptingPlugin::default().add_context_pre_handling_initializer(
    |attachment, context| {
        let name = attachment.script().path().map(|p| p.to_string());
        context
            .globals()
            .set("script_name", name)
            .map_err(IntoInteropError::to_bms_error)?;
        Ok(())
    },
));
```

## Runtime initializers

Some languages have a `runtime`, shared by all contexts, which is customized with `add_runtime_initializer(|runtime| { ... })`. Lua's runtime type is `()`, as `mlua` has no separate runtime.

## Accessing the world

During initializers, the world is available through the `ThreadWorldContainer` thread local:

```rust,ignore
LuaScriptingPlugin::default().add_context_initializer(|_attachment, _context| {
    let world = ThreadWorldContainer::try_get_context()?.world;
    world.with_resource(|res: &MyResource| println!("My resource: {res:?}"))?;
    Ok(())
});
```
