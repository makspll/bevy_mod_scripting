local type = world._get_mock_type()
assert(world.has_resource(type) == false, "Resource should not exist")