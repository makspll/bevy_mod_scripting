-- optional components are nil on entities without them, queries of a system may access the same component
-- on disjoint entities, but not on the same ones
local ScriptAttachment = import_namespace("bms").ScriptAttachment
local bms = import_namespace("bms")
local world = bms.world
runs = {}
local ComponentA = world.get_type_by_name("test_utils::test_data::CompWithFromWorldAndComponentData")
local ComponentB = world.get_type_by_name("test_utils::test_data::CompWithDefaultAndComponentData")

function on_test()
    local post_update_schedule = world.get_schedule_by_name("PostUpdate")
    local script_attachment = ScriptAttachment.new_entity_script(bms.current_entity(), bms.current_script())

    local both = world.spawn()
    world.add_default_component(both, ComponentA)
    world.add_default_component(both, ComponentB)
    local only_a = world.spawn()
    world.add_default_component(only_a, ComponentA)

    world.add_system(
        post_update_schedule,
        bms.system_builder("optional_system", script_attachment)
        :query(world.query():component_read(ComponentA):optional_read(ComponentB))
    )
    world.add_system(
        post_update_schedule,
        bms.system_builder("disjoint_system", script_attachment)
        :query(world.query():component(ComponentA):with(ComponentB))
        :query(world.query():component_read(ComponentA):without(ComponentB))
    )
    -- both queries write `A` on entities with `B`
    world.add_system(
        post_update_schedule,
        bms.system_builder("conflicting_system", script_attachment)
        :query(world.query():component(ComponentA))
        :query(world.query():component(ComponentA):with(ComponentB))
    )
    return true
end

function optional_system(query)
    local with_b, without_b = 0, 0
    for _, result in pairs(query) do
        local components = result:components()
        assert(components[1] ~= nil, "the required component should be present")
        if components[2] == nil then
            without_b = without_b + 1
        else
            assert(type(components[2][1]) == "string", "the optional component should be readable")
            with_b = with_b + 1
        end
    end
    assert(with_b >= 1 and without_b >= 1, "expected entities with and without B, got " .. with_b .. " and " .. without_b)
    runs[#runs + 1] = "optional_system"
end

function disjoint_system(with_b, without_b)
    assert(#with_b >= 1 and #without_b >= 1, "expected results for both queries")
    -- `A` is written on entities with `B`, but only read on the others
    with_b[1]:components()[1][1] = "written"
    local ok, err = pcall(function() without_b[1]:components()[1][1] = "written" end)
    assert(not ok and tostring(err):find("writing the component", 1, true), tostring(err))
    runs[#runs + 1] = "disjoint_system"
end

function conflicting_system()
    runs[#runs + 1] = "conflicting_system"
end

function on_test_post_update()
    return true
end

function on_test_last()
    table.sort(runs)
    local order = table.concat(runs, ",")
    assert(order == "disjoint_system,optional_system", "unexpected runs: " .. order)
    return true
end
