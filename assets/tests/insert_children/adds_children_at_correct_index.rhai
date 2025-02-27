let entity = world.spawn_.call();
let child = world.spawn_.call();
let child2 = world.spawn_.call();

world.insert_children.call(entity, 0, [child]);
world.insert_children.call(entity, 0, [child2]);

assert(world.get_children.call(entity)[0].index.call() == child2.index.call());