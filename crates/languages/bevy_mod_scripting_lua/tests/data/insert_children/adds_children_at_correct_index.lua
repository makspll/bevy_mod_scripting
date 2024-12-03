local entity = world:spawn()
local child = world:spawn()
local child2 = world:spawn()

world:insert_children(entity, 1, {child})
world:insert_children(entity, 1, {child2})

assert(world:get_children(entity)[1]:index() == child2:index())