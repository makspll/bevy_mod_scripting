runs = {}

function on_test()
    local post_update_schedule = world.get_schedule_by_name("PostUpdate")
    local script_attachment = ScriptAttachment.new_entity_script(entity, script_asset)

    world.add_system(
        post_update_schedule,
        system_builder("my_importing_system", script_attachment)
    )

    return true
end

-- a non-exclusive system can only access what it declared, `import` must not need anything else
function my_importing_system()
    runs[#runs + 1] = "my_importing_system"

    local bevy_transform = import("bevy_transform")
    assert(bevy_transform.Transform ~= nil, "Transform should be importable from a system")
    assert(import("bms").ScriptAttachment ~= nil)

    local ok, err = pcall(import, "not_a_namespace")
    assert(not ok and tostring(err):find("does not exist", 1, true), tostring(err))

    -- errors inside systems are only logged, so record that we got to the end
    runs[#runs + 1] = "finished"
end

function on_test_post_update()
    return true
end

function on_test_last()
    assert(#runs == 2 and runs[2] == "finished", "the system did not finish, runs: " .. table.concat(runs, ","))
    return true
end
