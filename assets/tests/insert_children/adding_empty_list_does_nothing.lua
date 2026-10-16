local world = import_namespace("bms").world
local entity = world.spawn()

world.insert_children(entity,1 , {})

assert(#world.get_children(entity) == 0)