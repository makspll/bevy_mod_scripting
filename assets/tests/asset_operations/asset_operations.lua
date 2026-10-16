local world = import_namespace("bms").world
local test_utils = import_namespace("test_utils")

function on_test()
    local test_handle = test_utils.create_test_asset(42, "TestAssetName")

    assert(test_handle ~= nil, "Test asset handle should not be nil")
    assert(world.has_asset(test_handle) == true, "has_asset should return true")

    local retrieved_asset = world.get_asset(test_handle, world.get_type_by_name("test_utils::test_data::TestAsset"))
    assert(retrieved_asset.value == 42, "Asset value should be 42")
    assert(retrieved_asset.name == "TestAssetName", "Asset name should be 'TestAssetName'")
end
