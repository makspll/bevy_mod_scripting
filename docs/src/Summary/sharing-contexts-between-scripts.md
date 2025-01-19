# Shared Contexts

By default BMS will create an individual script context, or sandbox, for each script that is run. This means that each script will have its own set of global variables and functions that are isolated from other scripts. However, sometimes this might not be desirable, if you aren't worried about scripts interfering with each other, or if you want to easilly share data between scripts. In these cases, you can use shared contexts.

## Enabling Shared Contexts

You can enable shared contexts by configuring the relevant scripting plugin like so:
```rust,ignore
let mut plugin = LuaScriptingPlugin::default().enable_context_sharing();

app.add_plugins(plugin);
```

## Context Loading Settings

All context loading settings are stored in a separate resource per scripting plugin namely: `ContextLoadingSettings<Plugin>`. 

The settings are as follows:
- `loader` - the load and unload strategy for contexts. Each scripting plugin will have a load and unload function which is hooked up through here
- `assigner` - the strategy for assigning/unassigning contexts to scripts. This is used to determine how to assign a context to a script when it is run, and what to do with the context when the script is finished.
- `context_initializers` - stores all context initializers for the plugin
- `context_pre_handling_initializers` - stores all context pre-handling initializers for the plugin

More advanced applications might want to customize these settings to suit their needs.