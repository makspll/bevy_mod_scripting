local component = world:get_type_by_name("CompWithDefault")
local entity = _get_entity_with_test_component("CompWithDefault")
local retrieved = world:get_component(entity, component)

assert(retrieved ~= nil, "Component was not found")
assert(retrieved._1 == "Initial Value", "Component data was not retrieved correctly, retrieved._1 was: " .. retrieved._1)