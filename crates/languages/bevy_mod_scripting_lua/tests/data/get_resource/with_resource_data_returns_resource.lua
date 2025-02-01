local resource = world.get_type_by_name("TestResource")

local retrieved = world.get_resource(resource)
assert(retrieved ~= nil, "Resource should exist")
assert(retrieved.bytes[2] == 1, "Resource should have default value but got resource with #retrieved.bytes[1]: " .. tostring(retrieved.bytes[2]))
