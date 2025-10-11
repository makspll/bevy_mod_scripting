let Resource = world.get_type_by_name.call("TestResourceWithVariousFields");
let resource = world.get_resource.call(Resource);

// Insert new values into the set
resource.string_set["orange"] = "orange";
resource.string_set["grape"] = "grape";

let resource_changed = world.get_resource.call(Resource);

// Verify the new values were added
let orange = resource_changed.string_set["orange"];
assert(orange != (), "Expected to find 'orange' in set after insertion");

let grape = resource_changed.string_set["grape"];
assert(grape != (), "Expected to find 'grape' in set after insertion");

// Verify original values are still there
let apple = resource_changed.string_set["apple"];
assert(apple != (), "Expected 'apple' to still be in set");
