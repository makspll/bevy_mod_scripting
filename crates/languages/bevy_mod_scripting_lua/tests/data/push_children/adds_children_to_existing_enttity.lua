local entity = world:spawn()
local child = world:spawn()
local child2 = world:spawn()

world:push_children(entity, {child, child2})

assert(#world:get_children(entity) == 2)