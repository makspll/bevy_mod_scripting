let world = import_namespace.call("bms").world;
let entity = world.spawn_.call();
let parent = world.get_parent.call(entity);

assert(type_of(parent) == "()", "Expected no parents");
