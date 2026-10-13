let script_comp_type = world.register_new_component.call("script.test", "ScriptBundleComp");
let comp_type = world.get_type_by_name.call("CompWithDefaultAndComponentData");
let simple_type = world.get_type_by_name.call("SimpleStruct");
let nested_type = world.get_type_by_name.call("TestNestedBundle");

// bundles passed as members are flattened into their components
let bundle_type = world.register_new_bundle.call("script.test", "ScriptBundle", [script_comp_type, comp_type, nested_type]);
let components = bundle_type.components.call(bundle_type);
assert(components.len() == 3, "Expected the nested bundle to be flattened");
assert(type_of(world.get_type_by_name.call("ScriptBundle")) != "()", "Expected the bundle to be retrievable by name");

// members without a value are default constructed
let entity = world.spawn_bundle.call(bundle_type, DynamicBundle.new_.call(#{
    data: #{
        ScriptBundleComp: "hello",
        CompWithDefaultAndComponentData: CompWithDefaultAndComponentData.new_.call(["given"])
    }
}));

assert(world.has_bundle.call(entity, bundle_type) == true, "Expected entity to have the bundle");
assert(world.get_component.call(entity, script_comp_type).data == "hello", "Expected the script component value to be inserted");
assert(world.get_component.call(entity, comp_type)[0] == "given", "Expected the native component value to be inserted");
assert(world.has_component.call(entity, simple_type) == true, "Expected the missing member to be default constructed");

let taken = world.take_bundle.call(entity, bundle_type);
assert(taken.data.ScriptBundleComp == "hello", "Expected the taken bundle to contain the script component value");
assert(world.has_bundle.call(entity, bundle_type) == false, "Expected take_bundle to remove the components");
assert(world.has_component.call(entity, simple_type) == false, "Expected take_bundle to remove all the components");

world.add_default_bundle.call(entity, bundle_type);
assert(world.has_bundle.call(entity, bundle_type) == true, "Expected the default bundle to be inserted");
world.remove_bundle.call(entity, bundle_type);
assert(world.has_component.call(entity, script_comp_type) == false, "Expected remove_bundle to remove the components");

assert_throws(||{
    world.insert_bundle.call(entity, bundle_type, DynamicBundle.new_.call(#{ data: #{ Typo: 1 } }));
}, ".*unknown members: Typo.*");
assert(world.has_component.call(entity, simple_type) == false, "Expected a failed insert to not insert any members");
