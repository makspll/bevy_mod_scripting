# Callbacks

Callbacks generally refer to hooks called either manually or via rust event handlers, on scripts which can choose to subscribe to them.

Callbacks come in two variants:
- Freestanding, top level functions
- Registered, or "frozen" callbacks

## Freestanding callbacks
An example of a freestanding callback:
```lua
function on_script_loaded()
    print("doing things")
end
```

this callback is refered to by "name", and on the rust side can be invoked either via `RunScriptCallback` command, or by setting up an event handler system which passes on `ScriptCallbackEvent`'s to scripts.

The key thing to note about this type of callback, is that if the script is ever reloaded, and the contents of this callback change, the logic inside it will also be hot-reloaded.

## Registerd Callbacks
You can also register a callback like so:
```lua
register_callback("on_script_loaded", my_registered_callback)

function my_registered_callback()
    print("doing things")
end 
```

Registered callbacks, take priority over freestanding ones, and contrary to freestanding callbacks, they are "frozen". I.e. once a callback is registereed in this manner,
hot reloads won't affect the logic inside them.

This works well when using shared contexts, where scripts will overwrite top level functions when being loaded. You can use the `on_script_loaded` callback to register all your scripts callbacks while they are loaded as top level functions, and when future loads happen, every callback will be issued correctly.

This functionality is implemented at script plugin level, so some languages might not support this. All core languages do however.