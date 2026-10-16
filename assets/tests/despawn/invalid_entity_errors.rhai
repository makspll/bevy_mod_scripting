let world = import_namespace.call("bms").world;
let test_utils = import_namespace.call("test_utils");
assert_throws(||{
    world.despawn_recursive.call(test_utils._make_invalid_entity.call())
}, "Missing or invalid entity");