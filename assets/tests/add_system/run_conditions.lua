-- run conditions are evaluated in rust, the system only runs if all of them hold
local ScriptAttachment = import_namespace("bms").ScriptAttachment
local bms = import_namespace("bms")
local world = bms.world
local c = bms.conditions
runs = {}
local ResourceType = world.get_type_by_name("test_utils::test_data::TestResource")
local ComponentA = world.get_type_by_name("test_utils::test_data::CompWithFromWorldAndComponentData")

function on_test()
    local post_update_schedule = world.get_schedule_by_name("PostUpdate")
    local script_attachment = ScriptAttachment.new_entity_script(bms.current_entity(), bms.current_script())

    local add = function(name, condition)
        world.add_system(post_update_schedule, bms.system_builder(name, script_attachment):run_if(condition))
    end

    add("system_resource_exists", c.resource_exists(ResourceType))
    add("system_not_resource_exists", c.not_(c.resource_exists(ResourceType)))
    add("system_any_with_component", c.any_with_component(ComponentA))
    add("system_run_once", c.run_once())
    add("system_and", c.and_(c.run_once(), c.not_(c.run_once())))
    add("system_or", c.or_(c.not_(c.run_once()), c.run_once()))

    local ok, err = pcall(c.on_timer, -1)
    assert(not ok and tostring(err):find("invalid timer interval", 1, true), tostring(err))
    return true
end

local function record(name) return function() runs[name] = true end end
system_resource_exists = record("resource_exists")
system_not_resource_exists = record("not_resource_exists")
system_any_with_component = record("any_with_component")
system_run_once = record("run_once")
system_and = record("and")
system_or = record("or")

function on_test_post_update()
    return true
end

function on_test_last()
    assert(runs.resource_exists, "resource_exists should run")
    assert(not runs.not_resource_exists, "not(resource_exists) should not run")
    assert(runs.any_with_component, "any_with_component should run")
    assert(runs.run_once, "run_once should run")
    assert(not runs["and"], "run_once and not run_once should not run")
    assert(runs["or"], "not run_once or run_once should run")
    return true
end
