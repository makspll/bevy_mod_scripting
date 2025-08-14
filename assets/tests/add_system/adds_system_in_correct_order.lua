-- add two systems, one before and one after the existing `on_test_post_update` callback, then assert all systems have run
-- in the `on_test_last` callback

local runs = {}

-- runs on `Update`
function on_test()
    local post_update_schedule = world.get_schedule_by_name("PostUpdate")

    local test_system = post_update_schedule:get_system_by_name("on_test_post_update")
    local script_attachment = ScriptAttachment.new_entity_script(entity, script_asset)

    local system_after = world.add_system(
        post_update_schedule,
        system_builder("custom_system_after", script_attachment)
        :after(test_system)
    )

    local system_before = world.add_system(
        post_update_schedule,
        system_builder("custom_system_before", script_attachment)
        :before(test_system)
    )

    local script_system_between = world.add_system(
        post_update_schedule,
        system_builder("custom_system_between", script_attachment)
        :after(test_system)
        :before(system_after)
    )
end

function custom_system_before()
    print("custom_system_before")
    runs[#runs + 1] = "custom_system_before"
end

-- runs on post_update
function on_test_post_update()
    print("on_test_post_update")
    runs[#runs + 1] = "on_test_post_update"
end

function custom_system_after()
    print("custom_system_after")
    runs[#runs + 1] = "custom_system_after"
end

function custom_system_between()
    print("custom_system_between")
    runs[#runs + 1] = "custom_system_between"
end

-- runs in the `Last` bevy schedule
function on_test_last()
    local string_table = table.concat(runs, ", ")
    assert(#runs == 4, "Expected 4 runs, got: " .. tostring(string_table))
    assert(runs[1] == "custom_system_before", "Expected custom_system_before to run first, got: " .. runs[1])
    assert(runs[2] == "on_test_post_update", "Expected on_test_post_update to run second, got: " .. runs[2])
    assert(runs[3] == "custom_system_between", "Expected custom_system_between to run third, got: " .. runs[3])
    assert(runs[4] == "custom_system_after", "Expected custom_system_after to run second, got: " .. runs[4])
end
