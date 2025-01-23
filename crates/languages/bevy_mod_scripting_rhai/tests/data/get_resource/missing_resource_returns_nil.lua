local type = world._get_mock_resource_type()
assert(world.get_resource(type) == nil, "Resource should not exist")