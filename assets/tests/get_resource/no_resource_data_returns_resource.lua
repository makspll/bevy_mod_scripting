local resource = world.get_type_by_name("ResourceWithDefault")

local retrieved = world.get_resource(resource)
assert(retrieved ~= nil, "Resource should exist")
assert(retrieved[1] == "Initial Value", "Resource should have default value but got: " .. retrieved[1])
