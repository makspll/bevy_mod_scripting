let entity = world.spawn_.call();
let type = world.get_type_by_name.call("test_utils::test_data::TestComponent");

assert_throws(||{
    world.add_default_component.call(entity, type);
},"Missing type data ReflectDefault or ReflectFromWorld for type: .*TestComponent.*");