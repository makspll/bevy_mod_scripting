local world = import_namespace("bms").world
local script_asset = world.get_type_by_name("bevy_mod_scripting_asset::script_asset::ScriptAsset")
assert(script_asset ~= nil, "ScriptAsset is registered")

-- loading is asynchronous, so the handle is only checked for its path and state
local handle = world.load_asset("tests/bindings/asset/asset.lua", script_asset)
assert(world.asset_path(handle) == "tests/bindings/asset/asset.lua", "asset_path")

local state = world.load_state(handle)
assert(state == "NotLoaded" or state == "Loading" or state == "Loaded", "load_state: " .. state)

local loaded = world.is_loaded(handle)
assert(loaded == true or loaded == false, "is_loaded returns a boolean")
