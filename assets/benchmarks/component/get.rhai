let entity_with_component = world._get_entity_with_test_component.call("TestComponent");

fn bench(){
    world.get_component.call(entity_with_component, world.get_type_by_name.call("test_utils::test_data::TestComponent"));
}