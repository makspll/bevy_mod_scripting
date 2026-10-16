local world = import_namespace("bms").world
local entity = world.spawn()

world.push_children(entity, {})

assert(#world.get_children(entity) == 0)
