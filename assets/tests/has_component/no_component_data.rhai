let world = import_namespace.call("bms").world;
let entity = world._get_entity_with_test_component.call("CompWithDefault");
let component = world.get_type_by_name.call("test_utils::test_data::CompWithDefault");
assert(world.has_component.call(entity, component) == true, "Component was not found");