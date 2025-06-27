# Contexts

Each script runs in a context. By default BMS will create an individual context, or sandbox, for each script-entity pair that is run. This means that each script-entity pair will have its own set of global variables and functions that are isolated from other scripts. However, sometimes this might not be desirable. If you are not worried about scripts interfering with each other, or if you want to easily share data between scripts, you can consider using a different context assignment strategy.

## Shared Context
A shared context means that all scripts run in the same context; there is in fact only one context to run in. If the scripts interact heavily with each other, this may be what you want.

To enable a shared context, insert the `ScriptContext::shared()` resource.
```rust,ignore
app.insert_resource(ScriptContext::<LuaScriptingPlugin>::shared());
```

## Per Script Context
A per script context provides each script with their own context. However, scripts may be attached to multiple entities, in which case a single script context is shared by multiple entities. 

To enable per script contexts, insert the `ScriptContext::per_script()` resource.
```rust,ignore
app.insert_resource(ScriptContext::<LuaScriptingPlugin>::per_script());
```

## Per Entity Context
A per entity context provides each entity with their own context. The scripts attached to an entity via `ScriptComponent` all run in the same context.

To enable per entity contexts, insert the `ScriptContext::per_entity()` resource.
```rust,ignore
app.insert_resource(ScriptContext::<LuaScriptingPlugin>::per_entity());
```

## Per Entity and Script Context
A per entity-and-script context provides each entity-script pair with their own context. This is a maximally isolated way to run scripts.

To enable per entity-and-script contexts, insert the `ScriptContext::per_entity_and_script()` resource.
```rust,ignore
app.insert_resource(ScriptContext::<LuaScriptingPlugin>::per_entity_and_script());
```

# Domains
The above context assignment strategies cut along regular boundaries inherent in Bevy's entity and asset architecture. However, sometimes one needs to group the execution of scripts in a more ad-hoc manner. Domains provide an escape hatch that allow one to group their scripts into contexts by whatever criterion they choose. 

Suppose one had a set of scripts they wish to run within an "player" context.
```rust,ignore
commands.spawn((
    ScriptComponent(vec![asset_server.load("player.lua")]),
    ScriptDomain(Domain::new("player")),
)).with_children(|parent| {
    parent.spawn((
        ScriptComponent(vec![asset_server.load("sword.lua")]),
        ScriptDomain(Domain::new("player")),
    ));
}
```
And another set of scripts they wish to run within an "environment" context.
```rust,ignore
commands.spawn((
    ScriptComponent(vec![asset_server.load("monster.lua")]),
    ScriptDomain(Domain::new("environment")),
    ));
```
## Enable Domains

To enable domains-based contexts, insert the `ScriptContext::domains()` resource.
```rust,ignore
app.insert_resource(ScriptContext::<LuaScriptingPlugin>::domains());
```

But in addition, domains can be enabled on any of the above strategies. This means that when no `ScriptDomain` is given the other strategy will be used.
```rust,ignore
ScriptContext::<LuaScriptingPlugin>::shared().with_domains()
ScriptContext::<LuaScriptingPlugin>::per_script().with_domains()
ScriptContext::<LuaScriptingPlugin>::per_entity().with_domains()
ScriptContext::<LuaScriptingPlugin>::per_entity_and_script().with_domains()
```
The last context, the per entity-script pair with domains is the same as `ScriptContext::default()` and is the default context assignment strategy for BMS. 

## Context Loading Settings

All context loading settings are stored in a separate resource per scripting plugin namely: `ContextLoadingSettings<Plugin>`. 

The settings are as follows:
- `loader` - the load and unload strategy for contexts. Each scripting plugin will have a load and unload function which is hooked up through here
- `context_initializers` - stores all context initializers for the plugin
- `context_pre_handling_initializers` - stores all context pre-handling initializers for the plugin

More advanced applications might want to customize these settings to suit their needs.
