let entity = world.spawn_.call();
let child = world.spawn_.call();

world.push_children.call(entity, [child]);

let parent = world.get_parent.call(child);

assert(type_of(parent) != "()", "Expected a parent");
assert(parent.index.call() == entity.index.call(), "Parent is the wrong entity");