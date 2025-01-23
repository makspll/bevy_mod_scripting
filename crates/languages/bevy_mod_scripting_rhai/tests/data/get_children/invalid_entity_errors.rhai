assert_throws(||{ 
    world.get_children.call(Entity.from_raw.call(9999));
}, "Missing or invalid entity");
