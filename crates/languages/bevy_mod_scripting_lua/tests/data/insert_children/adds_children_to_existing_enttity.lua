local entity = world:spawn()
local child = world:spawn()
local child2 = world:spawn()

world:insert_children(entity, 1, {child, child2})

assert(#world:get_children(entity) == 2)