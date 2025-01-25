
let type = world.get_type_by_name.call("TestResource");
world.remove_resource.call(type);
assert(world.has_resource.call(type) == false, "Resource was not removed");
