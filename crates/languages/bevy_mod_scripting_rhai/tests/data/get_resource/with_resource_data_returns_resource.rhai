let resource = world.get_type_by_name.call("TestResource");

let retrieved = world.get_resource.call(resource);
assert(type_of(retrieved) != "()", "Resource should exist");
assert(retrieved.bytes[1] == 1, "Resource should have default value but got resource with #retrieved.bytes[1]: " + retrieved.bytes[1]);
