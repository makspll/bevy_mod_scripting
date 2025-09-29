function on_test()
    local test_handle = create_test_asset(42, "TestAssetName")

    assert(test_handle ~= nil, "Test asset handle should not be nil")
    assert(world.has_asset(test_handle) == true, "has_asset should return true")

    local retrieved_asset = world.get_asset(test_handle, types.TestAsset)
    assert(retrieved_asset.value == 42, "Asset value should be 42")
    assert(retrieved_asset.name == "TestAssetName", "Asset name should be 'TestAssetName'")
end
