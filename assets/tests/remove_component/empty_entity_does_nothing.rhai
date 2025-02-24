let entity = world.spawn_.call();
let type = world.get_type_by_name.call("TestComponent");

world.remove_component.call(entity, type);
world.remove_component.call(entity, type);