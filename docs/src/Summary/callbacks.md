# Callbacks

Callbacks are script functions which BMS calls by name, through an [event handler or a `RunScriptCallback` command](./running-scripts.md), from [other scripts](../ScriptingReference/calling-other-scripts.md), or as one of the [core callbacks](../ScriptingReference/core-callbacks.md).

They come in two variants:
- freestanding: top level functions,
- registered: functions passed to `register_callback`, which are "frozen".

## Freestanding callbacks

```lua
function on_script_loaded()
    print("doing things")
end
```

The callback is looked up by name each time it runs, so when the script is reloaded, the new version of the function runs.

## Registered callbacks

```lua
local function my_registered_callback()
    print("doing things")
end

register_callback("on_script_loaded", my_registered_callback)
```

Registered callbacks take priority over freestanding ones, and are "frozen": once registered, reloading other scripts won't change them.

This matters with [shared contexts](./contexts.md#context-policies), where scripts overwrite each other's top level functions when loaded. Register each script's callbacks while it loads, and every callback will reach the right script.

Registered callbacks are implemented by each language plugin, Lua and Rune both support them.
