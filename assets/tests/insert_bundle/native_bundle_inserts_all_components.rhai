let bundle_type = world.get_type_by_name.call("TestBundle");
let comp_type = world.get_type_by_name.call("CompWithDefaultAndComponentData");
let simple_type = world.get_type_by_name.call("SimpleStruct");

let entity = world.spawn_.call();

// unspecified fields, including the nested bundle, are defaulted by the constructor
world.insert_bundle.call(entity, bundle_type, TestBundle.new_.call(#{ comp: CompWithDefaultAndComponentData.new_.call(["bundled"]) }));

assert(world.get_component.call(entity, comp_type)[0] == "bundled", "Expected the bundled component value to be inserted");
assert(world.has_component.call(entity, simple_type) == true, "Expected the nested bundle's component to be inserted");

let spawned = world.spawn_bundle.call(bundle_type, TestBundle.new_.call(#{}));
assert(world.has_component.call(spawned, comp_type) == true, "Expected spawn_bundle to spawn an entity with the bundle");
assert(world.has_component.call(spawned, simple_type) == true, "Expected spawn_bundle to insert the nested bundle");
