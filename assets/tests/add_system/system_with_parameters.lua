runs = {}
local ResourceTypeA = world.get_type_by_name("TestResource")
local ResourceTypeB = world.get_type_by_name("TestResourceWithVariousFields")
local ComponentA = world.get_type_by_name("CompWithFromWorldAndComponentData")
local ComponentB = world.get_type_by_name("CompWithDefaultAndComponentData")

function on_test()
    local post_update_schedule = world.get_schedule_by_name("PostUpdate")


    local script_attachment = ScriptAttachment.new_entity_script(entity, script_asset)
    local entity = world.spawn()
    local entity2 = world.spawn()


    world.add_default_component(entity, ComponentA)
    world.add_default_component(entity, ComponentB)
    world.add_default_component(entity2, ComponentA)
    world.add_default_component(entity2, ComponentB)


    world.add_system(
        post_update_schedule,
        system_builder("my_parameterised_system", script_attachment)
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
        components = result:components()
        assert(#components == 2, "Expected 2 components, got " .. #components)
        local componentA = components[1]
        local componentB = components[2]
        assert(componentA._1 == "Default", "Expected 'Default', got: " .. componentA._1)
        assert(componentB._1 == "Default", "Expected 'Default', got: " .. componentA._1)
    end
end

function on_test_post_update()
    return true
end

function on_test_last()
    assert(#runs == 1, "Expected 1 runs, got: " .. #runs)
    return true
end
