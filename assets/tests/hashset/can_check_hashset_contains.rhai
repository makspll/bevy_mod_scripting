let Resource = world.get_type_by_name.call("TestResourceWithVariousFields");
let resource = world.get_resource.call(Resource);

// Check if "apple" exists in the set (should return Some(apple))
let result = resource.string_set["apple"];
assert(result != (), "Expected to find 'apple' in set");

// Check if "banana" exists in the set (should return Some(banana))
let result2 = resource.string_set["banana"];
assert(result2 != (), "Expected to find 'banana' in set");

// Check if "nonexistent" doesn't exist in the set (should return None)
let result3 = resource.string_set["nonexistent"];
assert(result3 == (), "Expected not to find 'nonexistent' in set");
