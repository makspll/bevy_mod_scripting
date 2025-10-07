let Resource = world.get_type_by_name.call("TestResourceWithVariousFields");
let resource = world.get_resource.call(Resource);


resource.string_map["foo"] = "updated";
resource.string_map["new_key"] = "new_value";

let resource_changed = world.get_resource.call(Resource);
assert(resource_changed.string_map["foo"] == "updated", "Expected updated, got " + resource_changed.string_map["foo"]);
assert(resource_changed.string_map["new_key"] == "new_value", "Expected new_value, got " + resource_changed.string_map["new_key"]);
