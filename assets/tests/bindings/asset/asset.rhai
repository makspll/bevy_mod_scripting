let world = import_namespace.call("bms").world;
let script_asset = world.get_type_by_name.call("bevy_mod_scripting_asset::script_asset::ScriptAsset");
assert(type_of(script_asset) != "()", "ScriptAsset is registered");

// loading is asynchronous, so the handle is only checked for its path and state
let handle = world.load_asset.call("tests/bindings/asset/asset.rhai", script_asset);
// this script is already loaded, so on windows the path is the one it was loaded with, with `\` separators
let path = world.asset_path.call(handle);
path.replace("\\", "/");
assert(path == "tests/bindings/asset/asset.rhai", "asset_path: " + path);

let state = world.load_state.call(handle);
assert(state == "NotLoaded" || state == "Loading" || state == "Loaded", "load_state");

assert(type_of(world.is_loaded.call(handle)) == "bool", "is_loaded returns a bool");
