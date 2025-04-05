let fake_entity = Entity.from_raw.call(0);

assert_throws(||{
    world.insert_children.call(fake_entity, 0, [fake_entity]);
}, "Missing or invalid entity");

let entity = world.spawn_.call();
assert_throws(||{
    world.insert_children.call(entity, 0, [fake_entity]);
}, "Missing or invalid entity");