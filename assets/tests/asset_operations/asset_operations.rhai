fn on_test() {
    let test_handle = create_test_asset(42, "TestAssetName");

    assert(test_handle != (), "Test asset handle should not be nil");
    assert(world.has_asset.call(test_handle) == true, "has_asset should return true");

    let retrieved_asset = world.get_asset.call(test_handle, types.TestAsset);
    assert(retrieved_asset.value == 42, "Asset value should be 42");
    assert(retrieved_asset.name == "TestAssetName", "Asset name should be 'TestAssetName'");
}