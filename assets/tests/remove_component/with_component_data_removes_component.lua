
local entity = world._get_entity_with_test_component("TestComponent")
local component = world.get_type_by_name("TestComponent")
world.remove_component(entity, component)
assert(world.has_component(entity, component) == false, "Component was not removed")
