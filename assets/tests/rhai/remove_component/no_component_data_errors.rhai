
let entity = world._get_entity_with_test_component.call("CompWithDefault");
let component = world.get_type_by_name.call("CompWithDefault");

assert_throws(||{
    world.remove_component.call(entity, component);
}, "Missing type data ReflectComponent for type: .*CompWithDefault.*")
