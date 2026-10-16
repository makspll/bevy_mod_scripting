local world = import_namespace("bms").world
local entity = world._get_entity_with_test_component("CompWithDefault")
local component = world.get_type_by_name("test_utils::test_data::CompWithDefault")
assert(world.has_component(entity, component) == true, "Component was not found")