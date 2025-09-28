fn on_test() {
    let asset_data = create_test_asset(42, "TestAssetName");
    let test_handle = asset_data[0];
    let asset_type_reg = asset_data[1];

    assert(test_handle != (), "Test asset handle should not be nil");
    assert(asset_type_reg != (), "TestAsset type registration should exist");

    // Check asset exists and retrieve it
    assert(world.has_asset.call(test_handle) == true, "has_asset should return true");

    let retrieved_asset = world.get_asset.call(test_handle, asset_type_reg);
    assert(retrieved_asset != (), "Should be able to retrieve the test asset");
    assert(retrieved_asset.value == 42, "Asset value should be 42");
    assert(retrieved_asset.name == "TestAssetName", "Asset name should be 'TestAssetName'");
}