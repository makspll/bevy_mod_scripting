# Events and Messages

Scripts can send Bevy [events](https://docs.rs/bevy/latest/bevy/ecs/event/trait.Event.html) (observer-style, run immediately) and [messages](https://docs.rs/bevy/latest/bevy/ecs/message/trait.Message.html) (buffered, read later by `MessageReader`s) using the reflection data Bevy generates for them.

| Function | Rust type data required | Behaviour |
|---|---|---|
| `world.trigger_event(type, value)` | `ReflectEvent` | Triggers the event. Observers run **before the call returns**. |
| `world.write_message(type, value)` | `ReflectMessage` | Writes the message to its `Messages<T>` buffer, readers see it later in the frame. |

```lua
local MyEvent = types.MyEvent
world.trigger_event(MyEvent, construct(MyEvent, { amount = 3 }))

local MyMessage = types.MyMessage
world.write_message(MyMessage, construct(MyMessage, { amount = 3 }))
```

The value is copied when sent, so the same value can be sent again, and changing it afterwards does not affect events already sent.

## Registration requirements

Only types which are registered *with the matching reflect attribute* can be sent, anything else errors with a missing type data error:

```rust,ignore
#[derive(Event, Reflect)]
#[reflect(Event)]
struct MyEvent { amount: usize }

#[derive(Message, Reflect)]
#[reflect(Message)]
struct MyMessage { amount: usize }

app.register_type::<MyEvent>()
   .register_type::<MyMessage>()
   // messages must also be added as usual, otherwise there is nothing to write them to
   .add_message::<MyMessage>();
```

* The type must be registered with `register_type`, otherwise it will not appear in the `types` table.
* `#[reflect(Event)]` is only generated for events using the default (global) trigger. Events with custom triggers, such as entity events, need a hand-written `ReflectEvent`.
* Types which do not implement `FromReflect` (all fields must be reflectable) cannot be sent.
* Bevy's own events and messages are not usable unless they have the reflect attributes.

## Things to be aware of

* `trigger_event` needs exclusive access to the world, so it fails with an access error if the script is currently holding a borrowed reference elsewhere in the same call (e.g. inside a query iteration or `with_*` access). Trigger events from regular callbacks.
* Observers triggered by `trigger_event` run synchronously, but any commands they queue are deferred until the next time the world is flushed. That is not necessarily after your callback: other world operations later in the same callback, such as `world.spawn`, can flush them. An observer, or a command it queues, which runs a callback in the script context that is currently executing fails with an error. To run callbacks on other scripts, use `run_script_callback` or `queue_script_callback` (see [Calling Other Scripts](./calling-other-scripts.md)).

## Built-in messages

The following BMS messages are reflected and can be written from scripts:

* `ScriptCallbackEvent`: runs a callback on other scripts, processed later in the frame by the handler systems set up for its callback label. Labels without a handler system are never processed, prefer `queue_script_callback` to run callbacks from scripts.
* `ScriptAttachedEvent`, `ScriptDetachedEvent`, `ScriptAssetModifiedEvent`: drive the script loading pipeline

The internal loading pipeline state events (`LoadingCompleted`, `ContextAssigned`, ...) are intentionally not reflected.
