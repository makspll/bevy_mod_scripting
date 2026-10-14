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
* Observers triggered by `trigger_event` run synchronously, and the commands they queue are applied before `trigger_event` returns, like for other operations such as spawning or inserting components. This is also how callbacks of script observers (see below) are delivered: a script observer whose script context is the one currently executing does not fail, its callback instead runs as soon as the executing callback finishes. This only applies to script observers, a Rust observer or command which runs a callback in the executing context directly still fails with an error. To run callbacks on other scripts yourself, use `run_script_callback` or `queue_script_callback` (see [Calling Other Scripts](./calling-other-scripts.md)).

## Observing events from scripts

`world.add_observer(event, callback, components?, entities?)` spawns an observer which runs the callback named `callback` on the calling script, with a copy of the event as argument, whenever an event of the given type is triggered. It returns the observer entity, despawn it to remove the observer.

The callback is a normal script callback, i.e. a function registered with `register_callback`, or a top level function of that name:

```lua
local bevy_ecs = import_namespace("bevy_ecs")
local Mana = world.register_new_component("script.my_game", "Mana")

register_callback("on_mana_added", function(event)
    print("Mana was added to", event.entity)
end)

local observer = world.add_observer(bevy_ecs.AddEvent, "on_mana_added", { Mana })

-- later
world.despawn(observer)
```

The component lifecycle events `AddEvent`, `InsertEvent`, `DiscardEvent`, `RemoveEvent` and `DespawnEvent` are observable out of the box, for any native or script registered component. They live in the `bevy_ecs` namespace. As in Bevy, a list of components is an `OR` filter. Pass a list of entities as the last argument to only observe those entities. Custom events need `#[reflect(Event)]`, see above.

Observer callbacks run through the same machinery as every other callback (see [Calling Other Scripts](./calling-other-scripts.md)):

* They run right after the operation which triggered the event, once the other observers of that operation have been notified. The order in which observers of one event run is unspecified, as in Bevy.
* If the script's context is busy running another callback, e.g. the one that triggered the event (all scripts sharing a context count too), the callback instead runs as soon as that callback finishes. Observers can therefore never re-enter a running script, and whether a callback ran immediately or later can depend on the context policy, so do not rely on either.
* The world is already past the change: data of removed components is no longer readable inside `RemoveEvent` callbacks.
* They get the same setup as other callbacks, such as the script's globals and its current attachment.
* Errors are reported like those of any other callback (as `ScriptErrorMessage`s, which are logged by default), and do not fail the operation that triggered the event.
* `add_observer` can only be called from a script callback.

## Script defined events

Scripts can define their own events, carrying any value as payload:

```lua
local Damaged = world.register_new_event("script.my_game", "Damaged")

register_callback("on_damaged", function(payload)
    print("took damage:", payload.amount)
end)

world.add_observer(Damaged, "on_damaged")

world.trigger(Damaged, { amount = 5 })
```

`register_new_event` fails if the name is taken, unless `allow_existing` is passed as the third argument (this is useful if the script runs more than once). Script events are global: they are not targeted at entities, and ignore the `components` and `entities` arguments of `add_observer`. The observers' callbacks run as described above. Script events are not types, so `import_namespace` does not list them: keep the handle returned by `register_new_event` (or get it again with `allow_existing`).

## Built-in messages

The following BMS messages are reflected and can be written from scripts:

* `ScriptCallbackMessage`: runs a callback on other scripts, processed later in the frame by the handler systems set up for its callback label. Labels without a handler system are never processed, prefer `queue_script_callback` to run callbacks from scripts.
* `ScriptAttachedMessage`, `ScriptDetachedMessage`, `ScriptAssetModifiedMessage`: drive the script loading pipeline

The internal loading pipeline state events (`LoadingCompleted`, `ContextAssigned`, ...) are intentionally not reflected.
