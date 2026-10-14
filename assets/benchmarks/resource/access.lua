local resource_type = world.get_type_by_name("test_utils::test_data::TestResource")

function bench()
    local bytes = world.get_resource(resource_type).bytes;
end