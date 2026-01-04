assert(type_of(world) != "()", "World was not found");
assert(type_of(world.get_type_by_name.call("TestComponent")) != "()", "Could not find TestComponent type");
let global_invocation = _entity_from_index.call(1);

fn on_test(){
    assert(type_of(world) != "()", "World was not found");
    assert(type_of(world.get_type_by_name.call("TestComponent")) != "()", "Could not find TestComponent type");
    _entity_from_index.call(1);

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