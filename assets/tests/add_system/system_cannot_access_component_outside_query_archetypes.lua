-- bevy runs a system querying `A` with `B` in parallel with systems accessing `A` without `B`,
-- so the script system must not reach `A` on entities without `B`, even though it has access to `A`
local ScriptAttachment = import_namespace("bms").ScriptAttachment
local bms = import_namespace("bms")
local world = bms.world
runs = {}
local ComponentA = world.get_type_by_name("test_utils::test_data::CompWithFromWorldAndComponentData")
local ComponentB = world.get_type_by_name("test_utils::test_data::CompWithDefaultAndComponentData")

function on_test()
    local post_update_schedule = world.get_schedule_by_name("PostUpdate")
    local script_attachment = ScriptAttachment.new_entity_script(bms.current_entity(), bms.current_script())

    local matched = world.spawn()
    world.add_default_component(matched, ComponentA)
    world.add_default_component(matched, ComponentB)

    outsider = world.spawn()
    world.add_default_component(outsider, ComponentA)
    assert(world._archetype_of(matched) ~= world._archetype_of(outsider),
        "the entities should live in different archetypes")

    world.add_system(
        post_update_schedule,
        bms.system_builder("my_filtered_system", script_attachment)
        :query(world.query():component(ComponentA):with(ComponentB))
    )

    return true
end

function my_filtered_system(query)
    runs[#runs + 1] = "my_filtered_system"
    local ok, err = pcall(check_accesses, query)
    runs[#runs + 1] = ok and "finished" or tostring(err)
end

function check_accesses(query)
    assert(#query == 1, "Expected 1 result, got: " .. #query)
    local matched = query[1]
    assert(matched[2][1] == "Default", "the matched component should be accessible")
    assert(world.get_component(matched[1], ComponentA)[1] == "Default",
        "the matched component should be accessible through the world too")

    assert_throws(function()
        local component = world.get_component(outsider, ComponentA)
        local _ = component[1]
    end, ".*not matched by any query.*")

    -- `with` filters by `B` without accessing it
    assert_throws(function()
        local component = world.get_component(matched[1], ComponentB)
        local _ = component[1]
    end, ".*annot claim access.*")
end

function on_test_post_update()
    return true
end

function on_test_last()
    assert(#runs == 2 and runs[2] == "finished", "the system did not finish, runs: " .. table.concat(runs, ","))
    return true
end
