let world = import_namespace.call("bms").world;
let entity = world.spawn_.call();
let children = world.get_children.call(entity);

assert(children.len == 0);