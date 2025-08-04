local expected_asset_path = "assets/tests/handle/handle_asset_path.lua"
assert(script_id:asset_path() == expected_asset_path, "Expected script asset path to match")

function on_test()
    assert(script_id:asset_path() == expected_asset_path, "Expected script asset path to match")
end
