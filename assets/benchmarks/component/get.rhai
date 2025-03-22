let entity_with_component = world._get_entity_with_test_component.call("TestComponent");

fn bench(){
    world.get_component.call(entity_with_component, types.TestComponent);
}