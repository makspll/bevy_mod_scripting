let world = import_namespace.call("bms").world;
let test_utils = import_namespace.call("test_utils");
let fake_entity = test_utils._make_invalid_entity.call();

assert_throws(||{
    world.push_children.call(fake_entity, [fake_entity]);
}, "Missing or invalid entity");

let entity = world.spawn_.call();
assert_throws(||{
    world.push_children.call(entity, [fake_entity]);
}, "Missing or invalid entity");