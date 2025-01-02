# Attaching Scripts

Once you have scripts discovered and loaded, you'll want to run them. At the moment BMS supports one method of triggering scripts, and that is by attaching them to entities via `ScriptComponent`'s and then sending script event's which trigger callbacks on the scripts.

In order to attach a script and make it runnable simply add a `ScriptComponent` to an entity
```rust,ignore
    commands.entity(my_entity).insert(ScriptComponent::new(vec!["my_script.lua", "my_other_script.lua"]));
```

# Running Scripts

Scripts can run logic either when loaded or when triggered by an event. For example the script:

```lua
print("hello from load time")
function on_event()
    print("hello from event time")
end
```

Will print "hello from load time" when the script is loaded, and "hello from event time" when the script receives an event targeting the `on_event` callback with a receiver list including this script or entity.

In order to trigger `on_event` you need to first define a label, then send an event containing the label:
```rust,ignore
// define the label
struct OnEventCallback;
impl IntoCallbackLabel for OnEventCallback {
    fn into_callback_label() -> CallbackLabel {
        "on_event".into()
    }
}

// trigger the event
fn send_event(mut writer: EventWriter<ScriptCallbackEvent<()>>) {
    writer.send(ScriptCallbackEvent::new_for_all(
        OnEventCallback::into_callback_label(),
        (),
    ));
}
```

Note the `()` corresponds to the payload for the event, i.e. in this case we are not sending any arguments.

TODO: this should be replaced with `ScriptValue` before release  
```
assert!(false, "TODO: replace with ScriptValue");
```