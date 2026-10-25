-- script systems order against other script systems by label, and against script and Rust sets
local ScriptAttachment = import_namespace("bms").ScriptAttachment
local bms = import_namespace("bms")
local world = bms.world
runs = {}

function on_test()
    local post_update_schedule = world.get_schedule_by_name("PostUpdate")
    local script_attachment = ScriptAttachment.new_entity_script(bms.current_entity(), bms.current_script())

    local group = world.register_new_system_set("script.test", "group")
    -- registering again returns the same set, other namespaces are not for scripts
    assert(world.register_new_system_set("script.test", "group", true):name() == group:name())
    assert(not pcall(world.register_new_system_set, "script.test", "group"))
    assert(not pcall(world.register_new_system_set, "transform", "group"))

    -- Rust sets are members of their namespace, and found by their full name, which never finds script sets
    local propagate = import_namespace("transform.TransformSystems").Propagate
    assert(propagate:name() == "Propagate", propagate:name())
    assert(world.get_system_set_by_name("transform.TransformSystems.Propagate"):name() == "Propagate")
    assert(world.get_system_set_by_name("script.test.missing") == nil)

    world.add_system(
        post_update_schedule,
        bms.system_builder("custom_system_first", script_attachment)
        :in_set(group)
    )
    world.add_system(
        post_update_schedule,
        bms.system_builder("custom_system_last", script_attachment)
        :after_set(group)
        :after_set(propagate)
    )
    world.add_system(
        post_update_schedule,
        bms.system_builder("custom_system_second", script_attachment)
        :after_script_system("custom_system_first", script_attachment)
        :before_script_system("custom_system_last", script_attachment)
    )

    world._assert_system_runs_before(post_update_schedule, "custom_system_first", "custom_system_last")
    world._assert_system_runs_before(post_update_schedule, "custom_system_first", "custom_system_second")
    world._assert_system_runs_before(post_update_schedule, "custom_system_second", "custom_system_last")

    -- ordering against a system which doesn't exist fails
    local ok, err = pcall(world.add_system, post_update_schedule,
        bms.system_builder("custom_system_never", script_attachment)
        :after_script_system("does_not_exist", script_attachment))
    assert(not ok and tostring(err):find("does not exist", 1, true), tostring(err))
    return true
end

function custom_system_first() runs[#runs + 1] = "first" end

function custom_system_second() runs[#runs + 1] = "second" end

function custom_system_last() runs[#runs + 1] = "last" end

function on_test_post_update()
    return true
end

function on_test_last()
    local order = table.concat(runs, ",")
    assert(order == "first,second,last", "unexpected order: " .. order)
    return true
end
