fn on_test() {
    assert!(type_of(world) != "()", "World was not found");
    assert!(type_of(world.get_type_by_name.call("TestComponent")) != "()", "Could not find TestComponent type");
    Entity.from_raw.call(1);
}