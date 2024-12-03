
assert_throws(function() 
    world:get_parent(Entity.from_raw(9999))
end, "Entity does not exist")
