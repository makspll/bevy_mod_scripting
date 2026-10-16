let world = import_namespace.call("bms").world;
fn bench(){
    world.get_resource.call(world.get_type_by_name.call("test_utils::test_data::TestResource"));
}