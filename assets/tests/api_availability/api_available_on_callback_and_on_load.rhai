let world = import_namespace.call("bms").world;
let test_utils = import_namespace.call("test_utils");
assert(type_of(world) != "()", "World was not found");
assert(type_of(world.get_type_by_name.call("test_utils::test_data::TestComponent")) != "()", "Could not find TestComponent type");
let global_invocation = test_utils._entity_from_index.call(1);

fn on_test(){
    assert(type_of(world) != "()", "World was not found");
    assert(type_of(world.get_type_by_name.call("test_utils::test_data::TestComponent")) != "()", "Could not find TestComponent type");
    test_utils._entity_from_index.call(1);

    // assert global_invocation happened
    assert(type_of(global_invocation) != "()", "Global invocation did not happen");

    return true;
}

fn on_test_post_update(){
    return true;
}

fn on_test_last(){
    return true;
}