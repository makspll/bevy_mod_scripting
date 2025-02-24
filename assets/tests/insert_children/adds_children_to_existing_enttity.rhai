let entity = world.spawn_.call();
let child = world.spawn_.call();
let child2 = world.spawn_.call();
world.insert_children.call(entity, 0, [child, child2]);

assert(world.get_children.call(entity).len == 2);