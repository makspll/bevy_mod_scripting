
assert_throws(||{
    world.get_parent.call(_make_invalid_entity.call());
}, "Missing or invalid entity");
