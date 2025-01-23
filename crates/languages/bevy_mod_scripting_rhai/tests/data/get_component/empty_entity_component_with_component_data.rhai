let component = world.get_type_by_name.call("TestComponent");
let entity = world.spawn_.call();
let retrieved = world.get_component.call(entity, component);

assert(type_of(retrieved) == "()", "Component found");