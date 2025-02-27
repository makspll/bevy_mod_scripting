let type = world._get_mock_resource_type.call();
assert(type_of(world.get_resource.call(type)) == "()", "Resource should not exist");