local world = import_namespace("bms").world

local entity = world._get_entity_with_test_component("TestComponent")
local component = world.get_type_by_name("test_utils::test_data::TestComponent")
world.remove_component(entity, component)
assert(world.has_component(entity, component) == false, "Component was not removed")
