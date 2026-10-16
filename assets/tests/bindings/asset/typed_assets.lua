local ScriptAsset = import_namespace("bms").ScriptAsset
local TestAsset = import_namespace("test_utils").TestAsset

-- this script is loaded before it runs, so loading its own path gives back the loaded asset
local handle = ScriptAsset.load("tests/bindings/asset/typed_assets.lua")
assert(world.is_loaded(handle), "the script asset of this test is loaded")

local script = ScriptAsset.get_asset(handle)
assert(script ~= nil, "get_asset returns the loaded script asset")

-- assets which aren't loaded yet are nil
local missing = ScriptAsset.load("tests/bindings/asset/does_not_exist.lua")
assert(ScriptAsset.get_asset(missing) == nil, "get_asset returns nil for an asset which isn't loaded")

-- added assets can be read and changed through their handle
local added = TestAsset.add_asset(TestAsset.new({ value = 7, name = "added" }))
local asset = TestAsset.get_asset(added)
assert(asset.value == 7, "the added asset has its value, got: " .. tostring(asset.value))
assert(asset.name == "added", "the added asset has its name")

asset.value = 8
assert(TestAsset.get_asset(added).value == 8, "changes to the returned asset change the asset")

-- values must be of the asset type
local ok, err = pcall(function() return TestAsset.add_asset(ScriptAsset.get_asset(handle)) end)
assert(not ok and string.find(tostring(err), "not of the asset type", 1, true), "expected a type error, got: " .. tostring(err))
