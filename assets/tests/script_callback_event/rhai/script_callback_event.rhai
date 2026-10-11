fn on_test() {
    let label = construct.call(types.CallbackLabel, #{ "0": "callback_a" });
    let recipients = construct.call(types.Recipients, #{ variant: "AllScripts" });
    let event_type = types.ScriptCallbackEvent;
    let event = construct.call(event_type, #{
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
