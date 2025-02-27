
let entity = world._get_entity_with_test_component.call("TestComponent");
let component = world.get_type_by_name.call("TestComponent");
world.remove_component.call(entity, component);
assert(world.has_component.call(entity, component) == false, "Component was not removed");
