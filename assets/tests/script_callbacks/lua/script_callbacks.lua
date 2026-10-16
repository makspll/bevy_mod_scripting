local Recipients = import_namespace("bms").Recipients
local ScriptAttachment = import_namespace("bms").ScriptAttachment
local TestComponent = import_namespace("test_utils").TestComponent
local bms = import_namespace("bms")
local world = bms.world
local function attachment_of(e)
    return ScriptAttachment.new_entity_script(e, bms.current_script())
end

-- the other entity running this script
local function other_entity()
    for _, result in pairs(world.query():component(world.get_type_by_name("bevy_mod_scripting_core::script::ScriptComponent")):build()) do
        local e = result:entity()
        if e:index() ~= bms.current_entity():index() then
            return e
        end
    end
    error("could not find the other entity")
end

function double(x)
    return x * 2
end

function add(a, b)
    return a + b
end

function bounce(back_to)
    return bms.run_script_callback("double", attachment_of(back_to), 1)
end

function record(value)
    world.insert_component(bms.current_entity(), world.get_type_by_name("test_utils::test_data::TestComponent"), TestComponent.new({ strings = { value } }))
end

function on_test()
    local other = other_entity()

    -- immediate callbacks return what the callback returned
    local doubled = bms.run_script_callback("double", attachment_of(other), 21)
    assert(doubled == 42, "Expected 42, got: " .. tostring(doubled))
    local sum = bms.run_script_callback("add", attachment_of(other), 1, 2)
    assert(sum == 3, "Expected 3, got: " .. tostring(sum))

    -- recipients return a list with one value per script
    local results = bms.run_script_callback("double", Recipients.entity_script(other, bms.current_script()), 5)
    assert(#results == 1, "Expected 1 result, got: " .. #results)
    assert(results[1] == 10, "Expected 10, got: " .. tostring(results[1]))

    -- busy contexts error instead of deadlocking
    assert_throws(function()
        bms.run_script_callback("double", attachment_of(bms.current_entity()), 1)
    end, "it is the script currently running")
    assert_throws(function()
        bms.run_script_callback("bounce", attachment_of(other), bms.current_entity())
    end, "further up the current chain")

    -- queued callbacks can target this script, and only run once this callback finishes
    bms.queue_script_callback("record", attachment_of(bms.current_entity()), "queued!")
    -- flushing the world must not run them early
    world.spawn()
    assert(not world.has_component(bms.current_entity(), world.get_type_by_name("test_utils::test_data::TestComponent")), "Queued callback ran before the callback queueing it finished")
    return "ok"
end

function on_test_post_update()
    return world.get_component(bms.current_entity(), world.get_type_by_name("test_utils::test_data::TestComponent")).strings[1]
end
