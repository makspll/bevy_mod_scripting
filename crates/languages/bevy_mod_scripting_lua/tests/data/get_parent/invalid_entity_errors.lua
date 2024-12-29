
assert_throws(function() 
    world:get_parent(Entity.from_raw(9999))
end, "Missing or invalid entity")
