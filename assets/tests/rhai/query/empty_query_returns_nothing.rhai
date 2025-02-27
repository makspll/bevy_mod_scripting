let component_a = world.get_type_by_name.call("TestComponent");

for (result, i) in world.query.call().component.call(component_a).without.call(component_a).build.call() {
    assert(false, "This should not be reached");
}