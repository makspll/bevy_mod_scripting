local entity = world:spawn()
local type = world:get_type_by_name('TestComponent')

assert(world:has_component(entity, type) == false, "Entity should not have component")