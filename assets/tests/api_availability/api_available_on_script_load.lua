assert(world ~= nil, "World was not found")
assert(world.get_type_by_name("TestComponent") ~= nil, "Could not find TestComponent type")
Entity.from_raw(1)