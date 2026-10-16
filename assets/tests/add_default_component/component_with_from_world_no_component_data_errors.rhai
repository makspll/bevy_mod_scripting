let world = import_namespace.call("bms").world;
let entity = world.spawn_.call();
let _type = world.get_type_by_name.call("test_utils::test_data::CompWithFromWorld");

assert_throws(||{
    world.add_default_component.call(entity, _type);
}, "Missing type data ReflectComponent for type: .*CompWithFromWorld.*")