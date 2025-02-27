let component = world.get_type_by_name.call("TestComponent");
let entity = world._get_entity_with_test_component.call("TestComponent");
let retrieved = world.get_component.call(entity, component);

assert(type_of(retrieved) != "()", "Component was not found");
assert(retrieved.strings[0] == "Initial", "Component data was not retrieved correctly, retrieved.strings[0] was: " + retrieved.strings[0]);