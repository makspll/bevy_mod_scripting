let fake_entity = Entity.from_raw.call(0);
let fake_entity_valid = Entity.from_raw.call(9999);

assert_throws(||{
    world.insert_children.call(fake_entity_valid, 0, [fake_entity_valid]);
}, "Missing or invalid entity");

let entity = world.spawn_.call();
assert_throws(||{
    world.insert_children.call(entity, 0, [fake_entity]);
}, "Are you trying to use an entity in a callback in which it's unavailable?");