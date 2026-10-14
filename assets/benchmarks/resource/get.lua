local resource_type = world.get_type_by_name("test_utils::test_data::TestResource")

function bench()
    world.get_resource(resource_type);
end