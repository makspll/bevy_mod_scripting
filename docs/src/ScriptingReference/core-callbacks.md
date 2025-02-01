# Core Callbacks

On top of callbacks which are registered by your application, BMS provides a set of core callbacks which are always available.

The two core callbacks are:
- `on_script_loaded`
- `on_script_unloaded`

## `on_script_loaded`

This will be called right after a script has been loaded or reloaded. This is a good place to initialize your script. You should avoid placing a lot of logic into the global body of your script, and instead put it into this callback. Otherwise errors in the initialization will fail the loading of the script.

```lua
print("you can also use this space, but it's not recommended")
function on_script_loaded()
    print("Hello world")
end
```

## `on_script_unloaded`

This will be called right before a script is unloaded. This is a good place to clean up any resources that your script has allocated. Note this is not called when a script is reloaded, only when it is being removed from the system.

```lua
function on_script_unloaded()
    print("Goodbye world")
end
```