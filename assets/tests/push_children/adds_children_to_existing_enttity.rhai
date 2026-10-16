let world = import_namespace.call("bms").world;
let entity = world.spawn_.call();
let child = world.spawn_.call();
let child2 = world.spawn_.call();

world.push_children.call(entity, [child, child2]);

assert(world.get_children.call(entity).len == 2);