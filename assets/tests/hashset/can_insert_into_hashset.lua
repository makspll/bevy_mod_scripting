local Resource = world.get_type_by_name("TestResourceWithVariousFields")
local resource = world.get_resource(Resource)

-- Insert new values into the set
resource.string_set["orange"] = "orange"
resource.string_set["grape"] = "grape"

local resource_changed = world.get_resource(Resource)

-- Verify the new values were added
local orange = resource_changed.string_set["orange"]
assert(orange ~= nil, "Expected to find 'orange' in set after insertion")

local grape = resource_changed.string_set["grape"]
assert(grape ~= nil, "Expected to find 'grape' in set after insertion")

-- Verify original values are still there
local apple = resource_changed.string_set["apple"]
assert(apple ~= nil, "Expected 'apple' to still be in set")
