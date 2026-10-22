local ScriptAttachment = import_namespace("bms").ScriptAttachment
local bms = import_namespace("bms")
local world = bms.world
runs = {}
local ResourceTypeA = world.get_type_by_name("test_utils::test_data::TestResource")
local ResourceTypeB = world.get_type_by_name("test_utils::test_data::TestResourceWithVariousFields")
local ComponentA = world.get_type_by_name("test_utils::test_data::CompWithFromWorldAndComponentData")
local ComponentB = world.get_type_by_name("test_utils::test_data::CompWithDefaultAndComponentData")

function on_test()
    local post_update_schedule = world.get_schedule_by_name("PostUpdate")


    local script_attachment = ScriptAttachment.new_entity_script(bms.current_entity(), bms.current_script())
    local entity = world.spawn()
    local entity2 = world.spawn()


    world.add_default_component(entity, ComponentA)
    world.add_default_component(entity, ComponentB)
    world.add_default_component(entity2, ComponentA)
    world.add_default_component(entity2, ComponentB)


    world.add_system(
        post_update_schedule,
        bms.system_builder("my_parameterised_system", script_attachment)
        :resource(ResourceTypeA)
        :query(world.query():component(ComponentA):component(ComponentB))
        :resource(ResourceTypeB)
    )

    return true
end

function my_parameterised_system(resourceA, query, resourceB)
    print("my_parameterised_system")
    runs[#runs + 1] = "my_non_exclusive_system"

    assert(resourceA ~= nil, "Expected to get resource but got nil")
    assert(query ~= nil, "Expected to get query but got nil")
    assert(resourceB ~= nil, "Expected to get resource but got nil")

    assert(#resourceA.bytes == 6, "Expected 6 bytes, got: " .. #resourceA.bytes)
    assert(resourceB.string == "Initial Value", "Expected 'Initial Value', got: " .. resourceB.string)
    assert(#query == 2, "Expected 3 results, got: " .. #query)
    for i, result in pairs(query) do
        -- the entity, then the components
        assert(#result == 3, "Expected the entity and 2 components, got " .. #result)
        local componentA = result[2]
        local componentB = result[3]
        assert(componentA[1] == "Default", "Expected 'Default', got: " .. componentA[1])
        assert(componentB[1] == "Default", "Expected 'Default', got: " .. componentB[1])
    end
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
