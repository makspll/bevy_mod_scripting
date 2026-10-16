let world = import_namespace.call("bms").world;
let test_utils = import_namespace.call("test_utils");
let fake_entity = test_utils._entity_from_index.call(0);
let fake_entity_valid = test_utils._make_invalid_entity.call();

assert_throws(||{
    world.insert_children.call(fake_entity_valid, 0, [fake_entity_valid]);
}, "Missing or invalid entity");

let entity = world.spawn_.call();
assert_throws(||{
    world.insert_children.call(entity, 0, [fake_entity]);
}, "Are you trying to use an entity in a callback in which it's unavailable?");