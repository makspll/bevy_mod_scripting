local expected_asset_path = "tests/handle/handle_asset_path_when_script_loaded.lua"
function normalize_path(path)
    return string.gsub(tostring(path), "\\", "/")
end

local normalized_gotten_asset_path = normalize_path(script_asset:asset_path())
assert(normalized_gotten_asset_path == expected_asset_path,
    "Expected script asset path to match, got :" .. normalized_gotten_asset_path)

function on_test()
    local normalized_gotten_asset_path = normalize_path(script_asset:asset_path())
    assert(normalized_gotten_asset_path == expected_asset_path,
        "Expected script asset path to match, got: " .. normalized_gotten_asset_path)
end
