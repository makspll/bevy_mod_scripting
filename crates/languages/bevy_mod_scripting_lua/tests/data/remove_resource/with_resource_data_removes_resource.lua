
local type = world:get_type_by_name("TestResource")
world:remove_resource(type)
assert(world:has_resource(type) == false, "Resource was not removed")
