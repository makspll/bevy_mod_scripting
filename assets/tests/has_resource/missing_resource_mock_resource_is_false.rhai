let type = world._get_mock_resource_type.call();
assert(world.has_resource.call(type) == false, "Resource should not exist");