local entity = world.spawn()
local _type = world.get_type_by_name('CompWithDefaultAndComponentData')
world.add_default_component(entity, _type)

local added = world.has_component(entity, _type)
assert(added ~= nil, 'Component not added')

local component = world.get_component(entity, _type)
assert(component._1 == "Default", 'Component did not have default value, got: ' .. component._1)