local component = world.get_type_by_name("test_utils::test_data::ResourceWithDefault")
assert(world.has_resource(component) == true, "Resource was not found")