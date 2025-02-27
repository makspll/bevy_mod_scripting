local component = world.get_type_by_name("TestComponent")
local entity = world.spawn()
local retrieved = world.get_component(entity, component)

assert(retrieved == nil, "Component found")