
assert_throws(||{
    world.get_parent.call(Entity.from_raw.call(9999));
}, "Missing or invalid entity");
