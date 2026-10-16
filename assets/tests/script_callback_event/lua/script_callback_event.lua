local world = import_namespace("bms").world
local CallbackLabel = import_namespace("bms").CallbackLabel
local Recipients = import_namespace("bms").Recipients
local ScriptCallbackMessage = import_namespace("bms").ScriptCallbackMessage
function on_test()
    local label = CallbackLabel.new({ "callback_a" })
    local recipients = Recipients.AllScripts:new({})
    local event_type = world.get_type_by_name("bevy_mod_scripting_core::event::ScriptCallbackMessage")
    local event = ScriptCallbackMessage.new({
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
