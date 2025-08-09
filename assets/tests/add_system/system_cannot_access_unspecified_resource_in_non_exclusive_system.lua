runs = {}

function on_test()
    local post_update_schedule = world.get_schedule_by_name("PostUpdate")
    local script_attachment = ScriptAttachment.new_entity_script(entity, script_asset)

    world.add_system(
        post_update_schedule,
        system_builder("my_non_exclusive_system", script_attachment)
    )

    return true
end

function my_non_exclusive_system()
    print("my_non_exclusive_system")
    runs[#runs + 1] = "my_non_exclusive_system"

    local ResourceType = world.get_type_by_name("TestResource")
    assert_throws(function()
        local res = world.get_resource(ResourceType)
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
