local ScriptAttachment = import_namespace("bms").ScriptAttachment
local bms = import_namespace("bms")
local world = bms.world
runs = {}

function on_test()
    local post_update_schedule = assert(world.get_schedule_by_name("PostUpdate"))
    local script_attachment = ScriptAttachment.new_entity_script(assert(bms.current_entity()), assert(bms.current_script()))

    world.add_system(
        post_update_schedule,
        bms.system_builder("my_non_exclusive_system", script_attachment)
    )

    return true
end

function my_non_exclusive_system()
    print("my_non_exclusive_system")
    runs[#runs + 1] = "my_non_exclusive_system"

    local ResourceType = world.get_type_by_name("test_utils::test_data::TestResource")
    assert_throws(function()
        local res = assert(world.get_resource(ResourceType))
        ---@diagnostic disable-next-line: undefined-field
        local blah = res.blahblah
    end, ".*annot claim access to.*")
end

function on_test_post_update()
    return true
end

function on_test_last()
    assert(#runs == 1, "Expected 1 runs, got: " .. #runs)
    return true
end
