local entity_with_component = world._get_entity_with_test_component("TestComponent")

local component_type = world.get_type_by_name("test_utils::test_data::TestComponent")

function bench()
    world.get_component(entity_with_component, component_type)
end