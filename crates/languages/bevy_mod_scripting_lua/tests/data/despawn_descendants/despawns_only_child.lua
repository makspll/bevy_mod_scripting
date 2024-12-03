local entity = world:spawn()
local child = world:spawn()
world:push_children(entity, {child})
world:despawn_descendants(entity)

assert(world:has_entity(entity) == true, "Parent should not be despawned")
assert(world:has_entity(child) == false, "Child should be despawned")
