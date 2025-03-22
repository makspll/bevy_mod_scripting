let entity_with_component = world._get_entity_with_test_component.call("TestComponent");

fn bench(){
    let strings = world.get_component.call(entity_with_component, types.TestComponent).strings;
}