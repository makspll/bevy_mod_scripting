let Resource = world.get_type_by_name.call("TestResourceWithVariousFields");
let resource = world.get_resource.call(Resource);

assert(resource.string_map["foo"] == "bar", "Expected bar, got " + resource.string_map["foo"]);