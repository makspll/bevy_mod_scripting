assert_throws(||{
    world.despawn_recursive.call(_make_invalid_entity.call())
}, "Missing or invalid entity");