local expected_asset_path = "tests/handle/handle_asset_path.lua"
assert(script_id:asset_path() == expected_asset_path,
    "Expected script asset path to match, got :" .. tostring(script_id:asset_path()))

function on_test()
    assert(script_id:asset_path() == expected_asset_path,
        "Expected script asset path to match, got: " .. tostring(script_id:asset_path()))
end
