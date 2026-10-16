local world = import_namespace("bms").world
local entity = world.spawn()
local type = world._get_mock_component_type()

assert(world.has_component(entity, type) == false, "Entity should not have component")