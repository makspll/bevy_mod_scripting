let entity = world.spawn_.call();
let child = world.spawn_.call();

world.push_children.call(entity, [child]);

let children = world.get_children.call(entity);

assert(children.len == 1, "Expected 1 child");
assert(children[0].index.call() == child.index.call(), "Child is the wrong entity");