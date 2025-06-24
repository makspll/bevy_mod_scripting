# Core Callbacks

On top of callbacks which are registered by your application, BMS provides a set of core callbacks which are always available.

The three core callbacks are:
- `on_script_loaded`
- `on_script_unloaded`
- `on_script_reloaded`

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

This will be called right before a script is unloaded. This is a good place to clean up any resources that your script has allocated. Note this is not called when a script is reloaded, only when it is being removed from the system.

This callback will not have access to the `entity` variable, as when the script is being unloaded it might not be attached to an entity.

```lua
function on_script_unloaded()
    print("Goodbye world")
end
```

## `on_script_reloaded`

This will be called twice: right before and after a script is reloaded.

The first parameter `save` informs whether it is time to save a value or restore it.

Before the script reload, `on_script_reloaded` is called with two arguments:
`true`, `nil`. The value returned is kept. After the script reload,
`on_script_reloaded` is called with two arguments: `false` and the value
returned from the preceding call. 

```lua
mode = 1
function on_script_reloaded(save, value)
    if save then
        print("Before I go, take this.")
        return mode
    else
        print("I'm back. Where was I?")
        mode = value
    end
end
```

Using `on_script_reloaded` one can make a script reload event not disrupt the current script state.
