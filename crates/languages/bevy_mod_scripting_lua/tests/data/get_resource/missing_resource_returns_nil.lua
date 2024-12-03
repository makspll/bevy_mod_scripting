local type = _get_mock_type()
assert(world:get_resource(type) == nil, "Resource should not exist")