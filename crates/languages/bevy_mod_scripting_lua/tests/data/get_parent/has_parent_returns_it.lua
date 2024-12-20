local entity = world:spawn()
local child = world:spawn()

world:push_children(entity, {child})

local parent = world:get_parent(child)

assert(parent ~= nil, "Expected a parent")
assert(parent:index() == entity:index(), "Parent is the wrong entity")