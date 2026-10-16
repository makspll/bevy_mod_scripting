let world = import_namespace.call("bms").world;
let entity = world.spawn_.call();

world.insert_children.call(entity,0 ,[]);

assert(world.get_children.call(entity).len == 0);