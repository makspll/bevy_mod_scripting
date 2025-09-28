function on_test()
    local asset_data = create_test_asset(42, "TestAssetName")
    local test_handle = asset_data[1]
    local asset_type_reg = asset_data[2]

    assert(test_handle ~= nil, "Test asset handle should not be nil")
    assert(asset_type_reg ~= nil, "TestAsset type registration should exist")

    -- Check asset exists and retrieve it
    assert(world.has_asset(test_handle) == true, "has_asset should return true")

    local retrieved_asset = world.get_asset(test_handle, asset_type_reg)
    assert(retrieved_asset ~= nil, "Should be able to retrieve the test asset")
    assert(retrieved_asset.value == 42, "Asset value should be 42")
    assert(retrieved_asset.name == "TestAssetName", "Asset name should be 'TestAssetName'")
end
