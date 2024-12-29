local entity = world.spawn()
local child = world.spawn()
world.push_children(entity, {child})
world.despawn(entity)

assert(world.has_entity(entity) == false, "Parent should be despawned")
assert(world.has_entity(child) == true, "Child should not be despawned")
