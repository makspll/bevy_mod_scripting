assert_throws(function()
    world.get_children(_make_invalid_entity())
end, "Missing or invalid entity")
