let fake_entity = Entity.from_raw.call(9999);

assert_throws(||{
    world.push_children.call(fake_entity, [fake_entity]);
}, "Missing or invalid entity");

let entity = world.spawn_.call();
assert_throws(||{
    world.push_children.call(entity, [fake_entity]);
}, "Missing or invalid entity");