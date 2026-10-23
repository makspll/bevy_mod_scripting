-- adding a system with the same callback label for the same script replaces the old one
local ScriptAttachment = import_namespace("bms").ScriptAttachment
local bms = import_namespace("bms")
local world = bms.world
runs = {}
local ResourceType = world.get_type_by_name("test_utils::test_data::TestResource")

function on_test()
    local post_update_schedule = world.get_schedule_by_name("PostUpdate")
    local script_attachment = ScriptAttachment.new_entity_script(bms.current_entity(), bms.current_script())

    world.add_system(
        post_update_schedule,
        bms.system_builder("my_replaced_system", script_attachment)
    )
    world.add_system(
        post_update_schedule,
        bms.system_builder("my_replaced_system", script_attachment)
        :resource(ResourceType)
    )

    return true
end

function my_replaced_system(resource)
    runs[#runs + 1] = resource
end

function on_test_post_update()
    return true
end

function on_test_last()
    assert(#runs == 1, "Expected 1 run, got: " .. #runs)
    assert(runs[1] ~= nil, "Expected the replacement with the resource parameter to run")
    return true
end
