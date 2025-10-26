# Attaching Scripts


Once you have scripts discovered and loaded, you'll want to run them.

At the moment BMS supports three methods of making scripts runnable: 
- Attaching them to entities via `ScriptComponent`'s
- Adding static scripts
- Creating dynamic systems ⚗️ (see [the script systems section](../ScriptSystems/introduction.md))

And then sending script event's which trigger callbacks on the scripts.

## Attaching scripts to entities

In order to attach a script and make it runnable simply add a `ScriptComponent` to an entity
```rust,ignore
    commands.entity(my_entity).insert(ScriptComponent::new(vec![my_script_handle, another_script_handle]));
```

When this script is run the `entity` global will represent the entity the script is attached to. This allows you to interact with the entity in your script easilly.

<div class="warning">
Be wary of path separators, by default script ID's are derived from asset paths, which are platform dependent. Make sure to use `std::path::PathBuf` if you are targetting multiple platforms.
</div>

## Making static scripts runnable

Some scripts do not require attaching to an entity. You can run these scripts by loading them first as you would with any other script, then either adding them by issuing a `AttachScript` command like so:

```rust,ignore
    commands.queue(AttachScript::new(script_attachment));
```

The script will then be run as any other script but without being attached to any entity.

# Running Scripts

Scripts can run logic either when loaded or when triggered by an event. For example the script:

```lua
print("hello from load time")
function on_event(arg1)
    print("hello from event time")
    print(arg1)
end
```

Will print "hello from load time" when the script is loaded, and "hello from event time" when the script receives an event targeting the `on_event` callback with a receiver list including this script or entity.

In order to trigger `on_event` you need to first define a label, then send an event containing the label:
```rust,ignore

#[derive(Reflect)]
pub struct MyReflectType;

// define the label, you can define as many as you like here
callback_labels!(OnEvent => "on_event");

// trigger the event
fn send_event(mut writer: EventWriter<ScriptCallbackEvent>, mut allocator: ResMut<AppReflectAllocator>) {

    let allocator = allocator.write();
    let my_reflect_payload = ReflectReference::new_allocated(MyReflectType, &mut allocator);

    writer.send(ScriptCallbackEvent::new_for_all_scripts(
        OnEvent,
        vec![my_reflect_payload.into()],
    ));
}
```

Note the second argument is the payload we are sending with the event, in this case we are sending an arbitrary reflect type `MyReflectType`. This can be any type you like, as long as it implements `Reflect`.

Other variants of the `ScriptValue` enum are available for sending different types of data, such as `ScriptValue::Integer` for primtive, types.


# Event Handlers

In order for the events you send to actually be picked up, you need to inject special systems into your application. These systems will listen for the events and trigger the appropriate callbacks on the scripts:

```rust,ignore
app.add_systems(Update, event_handler::<OnEvent, LuaScriptingPlugin>);
```

Note the system is parameterized by the label we defined earlier, and the scripting plugin we are using. You can add as many of these systems as you like.

The event handler will catch all events with the label `OnEvent` and trigger the `on_event` callback on all targeted scripts which have that callback defined.

In order to handle events in the same frame and not accidentally have events "spill over" into the next frame, you should make sure to order any systems which produce these events *before* the event handler systems.

# Commands

You can also use manually issued `RunScriptCallback` commands to trigger script callbacks as well. These must be run from a exclusive system, or via a any other system but with limited access to the world (See the `WithWorldGuard` system param, which will allow you to create a `WorldGuard` and use it to run the commands)