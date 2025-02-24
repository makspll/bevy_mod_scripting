let entity = world.spawn_.call();
let type = world.get_type_by_name.call("TestComponent");
let entity_with_component = world._get_entity_with_test_component.call("TestComponent");
let existing_component = world.get_component.call(entity_with_component, type);

assert(world.has_component.call(entity, type) == false, "Expected entity to not have component before adding, test invalid");
world.insert_component.call(entity, type, existing_component);
assert(world.has_component.call(entity, type) == true, "Expected entity to have component after adding");
