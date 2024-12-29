local entity = world:spawn()
local child = world:spawn()

world:push_children(entity, {child})

local children = world:get_children(entity)

assert(#children == 1, "Expected 1 child")
assert(children[1]:index() == child:index(), "Child is the wrong entity")