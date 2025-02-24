let entity = world.spawn_.call();
let child = world.spawn_.call();
world.push_children.call(entity, [child]);
world.despawn_recursive.call(entity);

assert(world.has_entity.call(entity) == false, "Parent should be despawned");
assert(world.has_entity.call(child) == false, "Child should be despawned");
