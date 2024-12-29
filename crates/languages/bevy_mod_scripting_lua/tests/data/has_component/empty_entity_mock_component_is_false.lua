local entity = world.spawn()
local type = _get_mock_type()

assert(world.has_component(entity, type) == false, "Entity should not have component")