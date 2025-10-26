let component = world.get_type_by_name.call("CompWithDefault");
let entity = world._get_entity_with_test_component.call("CompWithDefault");
let retrieved = world.get_component.call(entity, component);

assert(type_of(retrieved) != "()", "Component was not found");
assert(retrieved[0] == "Initial Value", "Component data was not retrieved correctly, retrieved._0 was: " + retrieved[0]);