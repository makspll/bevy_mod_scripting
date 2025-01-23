let entity = world._get_entity_with_test_component.call("TestComponent");
let component = world.get_type_by_name.call("TestComponent");
assert(world.has_component.call(entity, component) == true, "Component was not found");