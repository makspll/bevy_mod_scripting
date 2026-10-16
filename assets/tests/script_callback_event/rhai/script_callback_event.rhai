let world = import_namespace.call("bms").world;

fn on_test() {
    let CallbackLabel = import_namespace.call("bms").CallbackLabel;
    let Recipients = import_namespace.call("bms").Recipients;
    let ScriptCallbackMessage = import_namespace.call("bms").ScriptCallbackMessage;
    let label = CallbackLabel.new_.call(["callback_a"]);
    let recipients = Recipients.AllScripts.new_.call(#{});
    let event_type = world.get_type_by_name.call("bevy_mod_scripting_core::event::ScriptCallbackMessage");
    let event = ScriptCallbackMessage.new_.call(#{
        label: label,
        recipients: recipients,
        args: [],
        trigger_response: true,
    });

    world.write_message.call(event_type, event);
    return "sent";
}

fn callback_a() {
    return "callback_a was triggered from a script";
}
