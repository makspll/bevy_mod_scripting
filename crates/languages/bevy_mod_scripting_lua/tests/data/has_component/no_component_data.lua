local entity = world._get_entity_with_test_component("CompWithDefault")
local component = world.get_type_by_name("CompWithDefault")
assert(world.has_component(entity, component) == true, "Component was not found")