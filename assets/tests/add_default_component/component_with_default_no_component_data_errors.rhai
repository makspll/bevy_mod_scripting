let entity = world.spawn_.call();
let _type = world.get_type_by_name.call("CompWithDefault");

assert_throws(||{
    world.add_default_component.call(entity, _type);
}, "Missing type data ReflectComponent for type: .*CompWithDefault.*")
