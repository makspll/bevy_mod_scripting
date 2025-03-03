local Resource = world.get_type_by_name("TestResourceWithVariousFields")
local resource = world.get_resource(Resource)

assert(resource.string_map:map_get("foo") == "bar", "Expected bar, got " .. resource.string_map:map_get("foo"))