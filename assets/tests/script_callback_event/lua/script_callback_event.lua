function on_test()
    local label = construct(types.CallbackLabel, { ["1"] = "callback_a" })
    local recipients = construct(types.Recipients, { variant = "AllScripts" })
    local event_type = types.ScriptCallbackEvent
    local event = construct(event_type, {
        label = label,
        recipients = recipients,
        args = {},
        trigger_response = true,
    })

    world.write_message(event_type, event)
    return "sent"
end

function callback_a()
    return "callback_a was triggered from a script"
end
