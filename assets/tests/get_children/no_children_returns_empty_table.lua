local entity = world.spawn()
local children = world.get_children(entity)

assert(#children == 0)