local world = import_namespace("bms").world
-- registered without `ReflectComponent`, so the declarations don't know it's a component
local component = world.get_type_by_name("test_utils::test_data::CompWithDefault") --[[@as bms.ScriptComponentRegistration<test_utils.CompWithDefault>]]
local entity = assert(world._get_entity_with_test_component("CompWithDefault"))
local retrieved = world.get_component(entity, component)

assert(retrieved ~= nil, "Component was not found")
assert(retrieved[1] == "Initial Value", "Component data was not retrieved correctly, retrieved._1 was: " .. retrieved[1])
