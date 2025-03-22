local entity_with_component = world._get_entity_with_test_component("TestComponent")

function bench()
    world.get_component(entity_with_component, types.TestComponent)
end