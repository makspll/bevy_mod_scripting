let component = world.get_type_by_name.call("ResourceWithDefault");
assert(world.has_resource.call(component) == true, "Resource was not found");