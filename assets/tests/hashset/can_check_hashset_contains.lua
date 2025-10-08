local Resource = world.get_type_by_name("TestResourceWithVariousFields")
local resource = world.get_resource(Resource)

-- Check if "apple" exists in the set (should return Some(apple))
local result = resource.string_set["apple"]
assert(result ~= nil, "Expected to find 'apple' in set")

-- Check if "banana" exists in the set (should return Some(banana))
local result2 = resource.string_set["banana"]
assert(result2 ~= nil, "Expected to find 'banana' in set")

-- Check if "nonexistent" doesn't exist in the set (should return None)
local result3 = resource.string_set["nonexistent"]
assert(result3 == nil, "Expected not to find 'nonexistent' in set")
