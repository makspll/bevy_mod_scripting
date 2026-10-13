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

## Observing events from scripts

`world.add_observer(event, components, callback, entities?)` spawns an observer which calls `callback` with a copy of the event whenever an event of the given type is triggered, and returns the observer entity. Despawn that entity to remove the observer.

```lua
local Mana = world.register_new_component("Mana")

local observer = world.add_observer(types.AddEvent, { Mana }, function(event)
    print("Mana was added to", event.entity)
end)

-- later
world.despawn(observer)
```

The component lifecycle events `AddEvent`, `InsertEvent`, `DiscardEvent`, `RemoveEvent` and `DespawnEvent` are observable out of the box, for any native or script registered component. As in Bevy, a list of components is an `OR` filter. Pass a list of entities as the last argument to only observe those entities. Custom events need `#[reflect(Event)]`, see above.

* The callback runs as soon as the commands of the triggering operation are applied, i.e. right after the operation, not in the middle of it. Data of removed components is therefore no longer readable inside `RemoveEvent` callbacks.
* Errors inside the callback are logged, they do not fail the operation that triggered the event.
* Function pointers from Rhai cannot be converted to callable functions yet, so `add_observer` is currently only usable from Lua.

## Script defined events

Scripts can define their own events, carrying any value as payload:

```lua
local Damaged = world.register_new_event("Damaged")

world.add_observer(Damaged, {}, function(payload)
    print("took damage:", payload.amount)
end)

world.trigger(Damaged, { amount = 5 })
```

`register_new_event` fails if the name is taken, unless `allow_existing` is passed as the second argument (this is useful if the script runs more than once). Script events are global: they are not targeted at entities, and ignore the `components` and `entities` arguments of `add_observer`. Observers run right after `trigger`, see the notes above. Events are looked up by name, so they are not part of the `types` global.

## Built-in messages

The following BMS messages are reflected and can be written from scripts:

* `ScriptCallbackMessage`: runs a callback on other scripts, processed later in the frame by the handler systems set up for its callback label. Labels without a handler system are never processed, prefer `queue_script_callback` to run callbacks from scripts.
* `ScriptAttachedMessage`, `ScriptDetachedMessage`, `ScriptAssetModifiedMessage`: drive the script loading pipeline

The internal loading pipeline state events (`LoadingCompleted`, `ContextAssigned`, ...) are intentionally not reflected.
