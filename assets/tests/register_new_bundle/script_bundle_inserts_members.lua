local ScriptComp = world.register_new_component("ScriptBundleComp")
local Comp = world.get_type_by_name("CompWithDefaultAndComponentData")
local Simple = world.get_type_by_name("SimpleStruct")
local Nested = world.get_type_by_name("TestNestedBundle")

-- bundles passed as members are flattened into their components
local Bundle = world.register_new_bundle("ScriptBundle", { ScriptComp, Comp, Nested })
assert(Bundle ~= nil, "Failed to register new bundle")
assert(#Bundle:components() == 3, "Expected the nested bundle to be flattened")
assert(world.get_type_by_name("ScriptBundle") ~= nil, "Expected the bundle to be retrievable by name")

-- members without a value are default constructed
local entity = world.spawn_bundle(Bundle, DynamicBundle.new({
    data = {
        ScriptBundleComp = "hello",
        CompWithDefaultAndComponentData = construct(Comp, { ["1"] = "given" })
    }
}))

assert(world.has_bundle(entity, Bundle) == true, "Expected entity to have the bundle")
assert(world.get_component(entity, ScriptComp).data == "hello", "Expected the script component value to be inserted")
assert(world.get_component(entity, Comp)[1] == "given", "Expected the native component value to be inserted")
assert(world.has_component(entity, Simple) == true, "Expected the missing member to be default constructed")

local taken = world.take_bundle(entity, Bundle)
assert(taken.data.ScriptBundleComp == "hello", "Expected the taken bundle to contain the script component value")
assert(world.has_bundle(entity, Bundle) == false, "Expected take_bundle to remove the components")
assert(world.has_component(entity, Simple) == false, "Expected take_bundle to remove all the components")

world.add_default_bundle(entity, Bundle)
assert(world.has_bundle(entity, Bundle) == true, "Expected the default bundle to be inserted")
world.remove_bundle(entity, Bundle)
assert(world.has_component(entity, ScriptComp) == false, "Expected remove_bundle to remove the components")

assert_throws(function()
    world.insert_bundle(entity, Bundle, DynamicBundle.new({ data = { Typo = 1 } }))
end, ".*unknown members: Typo.*")
assert(world.has_component(entity, Simple) == false, "Expected a failed insert to not insert any members")
