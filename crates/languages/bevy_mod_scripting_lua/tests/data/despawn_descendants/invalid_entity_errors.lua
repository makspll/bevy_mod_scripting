assert_throws(function()
    world:despawn_recursive(Entity.from_raw(9999))
end, "Missing or invalid entity")