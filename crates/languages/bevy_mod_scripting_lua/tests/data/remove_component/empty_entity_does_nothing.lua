local entity = world:spawn()
local type = world:get_type_by_name('TestComponent')

world:remove_component(entity, type)
world:remove_component(entity, type)