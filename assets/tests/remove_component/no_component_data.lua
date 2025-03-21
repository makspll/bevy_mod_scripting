
local entity = world._get_entity_with_test_component("CompWithDefault")
local component = world.get_type_by_name("CompWithDefault")
world.remove_component(entity, component)
assert(world.has_component(entity, component) == false, "Component was not removed")
