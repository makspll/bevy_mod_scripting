assert_throws(function()
    world.despawn_recursive(_make_invalid_entity())
end, "Missing or invalid entity")
