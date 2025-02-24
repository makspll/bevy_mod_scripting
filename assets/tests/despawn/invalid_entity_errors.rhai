assert_throws(||{
    world.despawn_recursive.call(Entity.from_raw.call(9999))
}, "Missing or invalid entity");