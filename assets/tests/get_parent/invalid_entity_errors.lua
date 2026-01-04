assert_throws(function()
    world.get_parent(_make_invalid_entity())
end, "Missing or invalid entity")
