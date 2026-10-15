let ScriptAsset = import_namespace.call("bms").ScriptAsset;
let TestAsset = import_namespace.call("test_utils").TestAsset;

// this script is loaded before it runs, so loading its own path gives back the loaded asset
let handle = ScriptAsset.load.call("tests/bindings/asset/typed_assets.rhai");
assert(world.is_loaded.call(handle), "the script asset of this test is loaded");

let script = ScriptAsset.get_asset.call(handle);
assert(type_of(script) != "()", "get_asset returns the loaded script asset");

// assets which aren't loaded yet are nil
let missing = ScriptAsset.load.call("tests/bindings/asset/does_not_exist.rhai");
assert(type_of(ScriptAsset.get_asset.call(missing)) == "()", "get_asset returns nil for an asset which isn't loaded");

// added assets can be read and changed through their handle
let added = TestAsset.add_asset.call(TestAsset.new_.call(#{ value: 7, name: "added" }));
let asset = TestAsset.get_asset.call(added);
assert(asset.value == 7, "the added asset has its value");
assert(asset.name == "added", "the added asset has its name");

asset.value = 8;
assert(TestAsset.get_asset.call(added).value == 8, "changes to the returned asset change the asset");
