let entity = world.spawn_.call();
let _type = world.get_type_by_name.call("CompWithDefault");
let entity_with_component = world._get_entity_with_test_component.call("CompWithDefault");
let existing_component = world.get_component.call(entity_with_component, _type);

assert_throws(||{
    world.insert_component.call(entity, _type, existing_component);
}, "Missing type data ReflectComponent for type: .*CompWithDefault.*");
