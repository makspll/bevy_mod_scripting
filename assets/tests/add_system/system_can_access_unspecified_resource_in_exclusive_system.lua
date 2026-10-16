local ScriptAttachment = import_namespace("bms").ScriptAttachment
local bms = import_namespace("bms")
runs = {}

function on_test()
    local post_update_schedule = world.get_schedule_by_name("PostUpdate")
    local script_attachment = ScriptAttachment.new_entity_script(bms.current_entity(), bms.current_script())
    world.add_system(
        post_update_schedule,
        bms.system_builder("my_exclusive_system", script_attachment):exclusive()
    )

    return true
end

function my_exclusive_system()
    print("my_exclusive_system")
    runs[#runs + 1] = "my_non_exclusive_system"

    local ResourceType = world.get_type_by_name("test_utils::test_data::TestResource")
    local res = world.get_resource(ResourceType)
    assert(res ~= nil, "Expected to get resource but got nil")
end

function on_test_post_update()
    return true
end

function on_test_last()
    assert(#runs == 1, "Expected 1 runs, got: " .. #runs)
    return true
end
