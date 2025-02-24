let entity = world.spawn_.call();
let children = world.get_children.call(entity);

assert(children.len == 0);