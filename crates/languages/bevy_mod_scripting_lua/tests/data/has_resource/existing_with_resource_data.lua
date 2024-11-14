local component = world:get_type_by_name("TestResource")
assert(world:has_resource(component) == true, "Resource was not found")