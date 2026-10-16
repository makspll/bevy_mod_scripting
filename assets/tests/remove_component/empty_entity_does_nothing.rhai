let world = import_namespace.call("bms").world;
let entity = world.spawn_.call();
let type = world.get_type_by_name.call("test_utils::test_data::TestComponent");

world.remove_component.call(entity, type);
world.remove_component.call(entity, type);