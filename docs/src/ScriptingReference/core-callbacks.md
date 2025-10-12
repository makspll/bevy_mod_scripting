# Core Callbacks

On top of callbacks which are registered by your application, BMS provides a set of core callbacks which are always available (unless disabled via plugin settings).

The three core callbacks are:
- `on_script_loaded`
- `on_script_unloaded`
- `on_script_reloaded`

For more information on how callbacks generally work see [the callbacks section](../Summary/callbacks.md).

## `on_script_loaded`

This will be called right after a script has been loaded or reloaded. This is a good place to initialize your script. You should avoid placing a lot of logic into the global body of your script, and instead put it into this callback. Otherwise errors in the initialization will fail the loading of the script.

This callback will not have access to the `entity` variable, as when the script is being loaded it's not attached to an entity yet.

```lua
print("you can also use this space, but it's not recommended")
function on_script_loaded()
    print("Hello world")
end
```

## `on_script_unloaded`

This will be called right before a script is unloaded. This is a good place to clean up any resources that your script has allocated. This is called both before a script is removed as well as before a script is reloaded. If you want to preserve the state of your script across reloads, you can return a value from this callback, which will be passed to `on_script_reloaded` when the script is reloaded.

This callback will not have access to the `entity` variable, as when the script is being unloaded it might not be attached to an entity.

```lua
function on_script_unloaded()
    print("Goodbye world")
    return "house key"
end
```

## `on_script_reloaded`

Called right after `on_script_loaded` but only if the script was reloaded. 
The callback is passed a state argument, this state is exactly what is returned by the script through `on_script_unloaded` before a reload happens.

This callback does not have access to the `entity` variable.

```lua
mode = 1
function on_script_reloaded(value)
    if value then
        print("I'm back. Thanks for the keys!")
    else
        print('I have not saved any state before unloading')
    end
end
```

Using `on_script_reloaded` one can make a script reload preserve its current state.
