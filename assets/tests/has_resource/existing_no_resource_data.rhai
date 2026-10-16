let world = import_namespace.call("bms").world;
let component = world.get_type_by_name.call("test_utils::test_data::ResourceWithDefault");
assert(world.has_resource.call(component) == true, "Resource was not found");