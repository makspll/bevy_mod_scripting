
let type = world.get_type_by_name.call("test_utils::test_data::TestResource");
world.remove_resource.call(type);
assert(world.has_resource.call(type) == false, "Resource was not removed");
