let bundle_type = world.get_type_by_name.call("TestBundle");
let comp_type = world.get_type_by_name.call("CompWithDefaultAndComponentData");
let simple_type = world.get_type_by_name.call("SimpleStruct");

let entity = world.spawn_.call();
assert(world.has_bundle.call(entity, bundle_type) == false, "Expected entity to not have the bundle before inserting, test invalid");

// unspecified fields, including the nested bundle, are defaulted by the constructor
world.insert_bundle.call(entity, bundle_type, TestBundle.new_.call(#{ comp: CompWithDefaultAndComponentData.new_.call(["bundled"]) }));

assert(world.has_bundle.call(entity, bundle_type) == true, "Expected entity to have the bundle after inserting");
assert(world.get_component.call(entity, comp_type)[0] == "bundled", "Expected the bundled component value to be inserted");
assert(world.has_component.call(entity, simple_type) == true, "Expected the nested bundle's component to be inserted");
let components = bundle_type.components.call(bundle_type);
assert(components.len() == 2, "Expected the nested bundle to be flattened into the bundle's components");

let taken = world.take_bundle.call(entity, bundle_type);
assert(taken.comp[0] == "bundled", "Expected the taken bundle to contain the component value");
assert(world.has_component.call(entity, comp_type) == false, "Expected take_bundle to remove the components");
assert(type_of(world.take_bundle.call(entity, bundle_type)) == "()", "Expected taking a missing bundle to return unit");

world.add_default_bundle.call(entity, bundle_type);
assert(world.get_component.call(entity, comp_type)[0] == "Default", "Expected the default bundle to be inserted");
world.remove_bundle.call(entity, bundle_type);
assert(world.has_bundle.call(entity, bundle_type) == false, "Expected remove_bundle to remove the components");

let spawned = world.spawn_bundle.call(bundle_type, TestBundle.new_.call(#{}));
assert(world.has_bundle.call(spawned, bundle_type) == true, "Expected spawn_bundle to spawn an entity with the bundle");
