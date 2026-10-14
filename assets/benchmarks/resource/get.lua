local resource_type = world.get_type_by_name("TestResource")

function bench()
    world.get_resource(resource_type);
end