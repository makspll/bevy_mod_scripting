let world = import_namespace.call("bms").world;
let component = world.get_type_by_name.call("test_utils::test_data::TestComponent");
let entity = world.spawn_.call();
let retrieved = world.get_component.call(entity, component);

assert(type_of(retrieved) == "()", "Component found");