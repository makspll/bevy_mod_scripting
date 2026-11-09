local world = import_namespace("bms").world
local entity = assert(world._get_entity_with_test_component("CompWithDefault"))
-- without `ReflectComponent` the declarations don't know it is a component
local component = world.get_type_by_name("test_utils::test_data::CompWithDefault") --[[@as bms.ScriptComponentRegistration]]
assert(world.has_component(entity, component) == true, "Component was not found")