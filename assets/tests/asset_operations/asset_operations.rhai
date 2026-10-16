let world = import_namespace.call("bms").world;
let test_utils = import_namespace.call("test_utils");

fn on_test() {
    let test_handle = test_utils.create_test_asset.call(42, "TestAssetName");

    assert(test_handle != (), "Test asset handle should not be nil");
    assert(world.has_asset.call(test_handle) == true, "has_asset should return true");

    let retrieved_asset = world.get_asset.call(test_handle, world.get_type_by_name.call("test_utils::test_data::TestAsset"));
    assert(retrieved_asset.value == 42, "Asset value should be 42");
    assert(retrieved_asset.name == "TestAssetName", "Asset name should be 'TestAssetName'");
}