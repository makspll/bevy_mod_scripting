local ScriptAttachment = import_namespace("bms").ScriptAttachment
local bms = import_namespace("bms")
runs = {}

function on_test()
    local post_update_schedule = world.get_schedule_by_name("PostUpdate")
    local script_attachment = ScriptAttachment.new_entity_script(bms.current_entity(), bms.current_script())

    world.add_system(
        post_update_schedule,
        bms.system_builder("my_importing_system", script_attachment)
    )

    return true
end

-- a non-exclusive system can only access what it declared, `import_namespace` must not need anything else
function my_importing_system()
    runs[#runs + 1] = "my_importing_system"

    local bevy_transform = import_namespace("transform")
    assert(bevy_transform.Transform ~= nil, "Transform should be importable from a system")
    assert(import_namespace("bms").ScriptAttachment ~= nil)

    local ok, err = pcall(import_namespace, "not_a_namespace")
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
