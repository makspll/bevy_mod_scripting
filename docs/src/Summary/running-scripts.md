# Running Scripts

Once [attached](./managing-scripts.md), a script runs its top level code, then waits for **callbacks**: functions the script defines, which Rust (or [other scripts](../ScriptingReference/calling-other-scripts.md)) invoke by name.

```lua
print("hello from load time")

function on_event(arg1)
    print("hello from event time", arg1)
end
```

This prints "hello from load time" when the script is loaded, and "hello from event time" whenever an `on_event` callback targets this script. Besides your own callbacks, BMS calls the [core callbacks](../ScriptingReference/core-callbacks.md) (`on_script_loaded`, ...) on its own.

## Sending callback messages

Define a label for the callback, then write a `ScriptCallbackMessage` with it:

```rust,ignore
#[derive(Reflect)]
pub struct MyReflectType;

// define as many labels as you like here
callback_labels!(OnEvent => "on_event");

fn send_event(mut writer: MessageWriter<ScriptCallbackMessage>) {
    // allocated values live as long as references to them
    let payload = ReflectReference::new_allocated(MyReflectType);

    writer.write(ScriptCallbackMessage::new_for_all_scripts(
        OnEvent,
        vec![payload.into()],
    ));
}
```

The second argument is the list of arguments the callback receives. Any `Reflect` type can be sent as a `ReflectReference`, and the other `ScriptValue` variants carry plain data, i.e. `ScriptValue::Integer(5)`.

Besides `new_for_all_scripts`, messages can target one script (`new_for_static_script`, `new_for_script_entity`) or each context once (`new_for_all_contexts`).

## Event handlers

Messages are only delivered by an `event_handler` system for their label and language:

```rust,ignore
app.add_systems(
    Update,
    (
        send_event,
        event_handler::<OnEvent, LuaScriptingPlugin>.after(send_event),
    ),
);
```

The handler runs the `on_event` callback on every targeted script which defines it. Order the systems writing the messages *before* the handler, otherwise messages are only handled in the next frame.

## Commands

To run a callback right away, queue a `RunScriptCallback` command for a single script:

```rust,ignore
commands.queue(RunScriptCallback::<LuaScriptingPlugin>::new(
    ScriptAttachment::StaticScript(handle),
    OnEvent.into_callback_label(),
    vec![ScriptValue::Integer(5)],
    false, // don't send a `ScriptCallbackResponseMessage`
));
```

To keep a callback's logic across reloads, or share contexts between scripts, see [Callbacks](./callbacks.md).
