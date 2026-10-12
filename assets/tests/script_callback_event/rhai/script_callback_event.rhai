fn on_test() {
    let label = CallbackLabel.new_.call(["callback_a"]);
    let recipients = Recipients.AllScripts.new_.call(#{});
    let event_type = types.ScriptCallbackMessage;
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
