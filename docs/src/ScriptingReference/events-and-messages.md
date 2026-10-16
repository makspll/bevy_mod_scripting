# Events and Messages

Scripts can send Bevy [events](https://docs.rs/bevy/latest/bevy/ecs/event/trait.Event.html) (observer-style, run immediately) and [messages](https://docs.rs/bevy/latest/bevy/ecs/message/trait.Message.html) (buffered, read later by `MessageReader`s) using the reflection data Bevy generates for them.

| Function | Rust type data required | Behaviour |
|---|---|---|
| `world.trigger_event(type, value)` | `ReflectEvent` | Triggers the event. Observers run, and all pending commands are applied, **before the call returns**. Also triggers [script events](#script-defined-events). |
| `world.write_message(type, value)` | `ReflectMessage` | Writes the message to its `Messages<T>` buffer, readers see it later in the frame. |

```lua
local my_game = import_namespace("my_game") -- the crate defining the types
world.trigger_event(my_game.MyEvent, my_game.MyEvent.new({ amount = 3 }))
world.write_message(my_game.MyMessage, my_game.MyMessage.new({ amount = 3 }))
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

* The type must be registered with `register_type`, otherwise scripts cannot import it.
* `#[reflect(Event)]` is only generated for events using the default (global) trigger. Events with custom triggers, such as entity events, need a hand-written `ReflectEvent`.
* Types which do not implement `FromReflect` (all fields must be reflectable) cannot be sent.
* Bevy's own events and messages are not usable unless they have the reflect attributes.

## Things to be aware of

* `trigger_event` needs exclusive access to the world, so it fails with an access error if the script is currently holding a borrowed reference elsewhere in the same call (e.g. inside a query iteration or `with_*` access). Trigger events from regular callbacks.
* Observers run before `trigger_event` returns, and so do the commands they queue (as for other operations such as spawning). A Rust observer or command which runs a callback in the currently executing script context fails with an error. Use `run_script_callback` or `queue_script_callback` instead (see [Calling Other Scripts](./calling-other-scripts.md)).

## Observing events from scripts

`world.add_observer(event, callback, components?, entities?)` runs the calling script's callback `callback` (registered with `register_callback`, or a top level function of that name) with a copy of the event, whenever the event is triggered. It returns the observer entity: despawn it to remove the observer.

```lua
local ecs = import_namespace("ecs")
local Mana = world.register_new_component("script.my_game", "Mana")

register_callback("on_mana_added", function(event)
    print("Mana was added to", event.entity)
end)

local observer = world.add_observer(ecs.AddEvent, "on_mana_added", { Mana })
world.despawn(observer) -- later
```

* The lifecycle events `AddEvent`, `InsertEvent`, `DiscardEvent`, `RemoveEvent` and `DespawnEvent` (in `ecs`) work for any native or script component. Other events need `#[reflect(Event)]`.
* As in Bevy, `components` is an `OR` filter, and `entities` restricts the observer to those entities.
* Callbacks run right after the triggering operation, in unspecified order. If the script's context is busy (e.g. it triggered the event), the callback runs as soon as the busy callback finishes, so observers never re-enter a running script.
* The world is already past the change: removed component data can't be read in `RemoveEvent` callbacks.
* Errors are reported like those of other callbacks (`ScriptErrorMessage`), and don't fail the triggering operation.
* `add_observer` only works inside a script callback.

## Script defined events

Scripts can define events carrying any value:

```lua
local Damaged = world.register_new_event("script.my_game", "Damaged")

register_callback("on_damaged", function(payload)
    print("took damage:", payload.amount)
end)

world.add_observer(Damaged, "on_damaged")
world.trigger_event(Damaged, { amount = 5 })
```

* `register_new_event` fails if the name is taken, unless `allow_existing` (third argument) is passed.
* Script events are global: `components` and `entities` are ignored.
* Other scripts get them with `import_namespace`, i.e. `import_namespace("script.my_game").Damaged`.

## Built-in messages

The following BMS messages are reflected and can be written from scripts:

* `ScriptCallbackMessage`: runs a callback on other scripts, processed later in the frame by the handler systems set up for its callback label. Labels without a handler system are never processed, prefer `queue_script_callback` to run callbacks from scripts.
* `ScriptAttachedMessage`, `ScriptDetachedMessage`, `ScriptAssetModifiedMessage`: drive the script loading pipeline

The internal loading pipeline state events (`LoadingCompleted`, `ContextAssigned`, ...) are intentionally not reflected.
