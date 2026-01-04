assert_throws(||{ 
    world.get_children.call(_make_invalid_entity.call());
}, "Missing or invalid entity");
