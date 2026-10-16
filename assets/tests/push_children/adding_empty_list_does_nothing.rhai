let world = import_namespace.call("bms").world;
let entity = world.spawn_.call();

world.push_children.call(entity, []);

assert(world.get_children.call(entity).len == 0);