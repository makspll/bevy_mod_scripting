local entity = world:spawn()
local parent = world:get_parent(entity)

assert(parent == nil, "Expected no parents")
