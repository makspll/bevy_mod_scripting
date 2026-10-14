local resource_type = world.get_type_by_name("TestResource")

function bench()
    local bytes = world.get_resource(resource_type).bytes;
end