# Contexts

Each script runs in a context. By default BMS will create an individual context, or sandbox, for each script-entity pair that is run. This means that each script-entity pair will have its own set of global variables and functions that are isolated from other scripts. However, sometimes this might not be desirable. If you are not worried about scripts interfering with each other, or if you want to easily share data between scripts, you can consider using a different context policy.

## Context Policies

### Shared Context
A shared context means that all scripts run in the same context; there is in fact only one context to run in. If the scripts interact heavily with each other, this may be what you want.

To enable a shared context, set the corresponding context policy on the scripting plugin:
```rust,ignore
app.add_plugins(LuaScriptingPlugin::default().set_context_policy(
    ContextPolicy::shared(),
));```

### Per Script Context
A per script context provides each script with their own context. However, scripts may be attached to multiple entities, in which case a single script context is shared by multiple entities. 

To enable per script contexts, insert the `ContextPolicy::per_script()` resource.
```rust,ignore
app.add_plugins(LuaScriptingPlugin::default().set_context_policy(
    ContextPolicy::per_script(),
));```

### Per Entity Context
A per entity context provides each entity with their own context. The scripts attached to an entity via `ScriptComponent` all run in the same context.

To enable per entity contexts, insert the `ContextPolicy::per_entity()` resource.
```rust,ignore
app.add_plugins(LuaScriptingPlugin::default().set_context_policy(
    ContextPolicy::per_entity(),
));```

### Per Entity and Script Context
A per entity-and-script context provides each entity-script pair with their own context. This is a maximally isolated way to run scripts.

To enable per entity-and-script contexts, insert the `ContextPolicy::per_entity_and_script()` resource.
```rust,ignore
app.add_plugins(LuaScriptingPlugin::default().set_context_policy(
    ContextPolicy::per_entity_and_script(),
));
```

## Custom Policies

Here is another way to write the `per_script()` policy.
```rust,ignore
let policy_a = ContextPolicy::per_script();
let policy_b = ContextPolicy { priorities: vec![ContextRule::Script, ContextRule::Shared] };
assert_eq!(policy_a, policy_b);
```
Reminding ourselves how `ContextKey` is defined,
```rust,ignore
pub struct ContextKey {
    pub entity: Option<Entity>,
    pub script: Option<Handle<ScriptAsset>>,
}
```
we read `policy_b` like this: if `ContextKey` has a script, return a `ContextKey` with only a script. Failing that `ContextRule::Shared` always returns an empty `ContextKey`.

One may also provide an entirely custom rule by implementing the `ContextKeySelector` trait.

## Context Loading Settings

All context loading settings are stored in a separate resource per scripting plugin namely: `ContextLoadingSettings<Plugin>`. 

The settings are as follows:
- `loader` - the load and unload strategy for contexts. Each scripting plugin will have a load and unload function which is hooked up through here
- `context_initializers` - stores all context initializers for the plugin
- `context_pre_handling_initializers` - stores all context pre-handling initializers for the plugin

More advanced applications might want to customize these settings to suit their needs.
