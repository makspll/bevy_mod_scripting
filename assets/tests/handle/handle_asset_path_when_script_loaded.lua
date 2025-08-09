local expected_asset_path = "tests/handle/handle_asset_path_when_script_loaded.lua"
assert(script_asset:asset_path() == expected_asset_path,
    "Expected script asset path to match, got :" .. tostring(script_asset:asset_path()))

function on_test()
    assert(script_asset:asset_path() == expected_asset_path,
        "Expected script asset path to match, got: " .. tostring(script_asset:asset_path()))
end
