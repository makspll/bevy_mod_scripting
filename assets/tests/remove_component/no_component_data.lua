local world = import_namespace("bms").world

local entity = assert(world._get_entity_with_test_component("CompWithDefault"))
-- without `ReflectComponent` the declarations don't know it is a component
local component = world.get_type_by_name("test_utils::test_data::CompWithDefault") --[[@as bms.ScriptComponentRegistration<test_utils.CompWithDefault>]]
world.remove_component(entity, component)
assert(world.has_component(entity, component) == false, "Component was not removed")
