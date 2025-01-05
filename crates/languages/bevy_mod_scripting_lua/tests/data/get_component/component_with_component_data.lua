local component = world.get_type_by_name("TestComponent")
local entity = _get_entity_with_test_component("TestComponent")
local retrieved = world.get_component(entity, component)

assert(retrieved ~= nil, "Component was not found")
assert(retrieved.strings[1] == "Initial", "Component data was not retrieved correctly, retrieved.strings[1] was: " .. retrieved.strings[1])