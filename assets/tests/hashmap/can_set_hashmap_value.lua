local Resource = world.get_type_by_name("TestResourceWithVariousFields")
local resource = world.get_resource(Resource)

resource.string_map["foo"] = "updated"
resource.string_map["new_key"] = "new_value"

local resource_changed = world.get_resource(Resource)
assert(resource_changed.string_map["foo"] == "updated", "Expected updated, got " .. resource_changed.string_map["foo"])
assert(resource_changed.string_map["new_key"] == "new_value",
    "Expected new_value, got " .. resource_changed.string_map["new_key"])
