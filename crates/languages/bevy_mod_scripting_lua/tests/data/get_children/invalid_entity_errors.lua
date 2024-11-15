
assert_throws(function() 
    world:get_children(Entity.from_raw(9999))
end, "Entity does not exist")
