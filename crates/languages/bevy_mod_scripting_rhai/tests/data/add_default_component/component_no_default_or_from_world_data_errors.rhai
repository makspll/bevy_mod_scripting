let entity = world.spawn_.call();
let type = world.get_type_by_name.call("TestComponent");

assert_throws(||{
    world.add_default_component.call(entity, type);
},"Missing type data ReflectDefault or ReflectFromWorld for type: .*TestComponent.*");