let resource = world.get_type_by_name.call("test_utils::test_data::ResourceWithDefault");

let retrieved = world.get_resource.call(resource);
assert(type_of(retrieved) != "()", "Resource should exist");
assert(retrieved[0] == "Initial Value", "Resource should have default value but got: " + retrieved[0]);
