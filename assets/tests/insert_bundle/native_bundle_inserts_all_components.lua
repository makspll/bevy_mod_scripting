local Bundle = world.get_type_by_name("TestBundle")
local Comp = world.get_type_by_name("CompWithDefaultAndComponentData")
local Simple = world.get_type_by_name("SimpleStruct")

local entity = world.spawn()
assert(world.has_bundle(entity, Bundle) == false, "Expected entity to not have the bundle before inserting, test invalid")

-- unspecified fields, including the nested bundle, are defaulted by the constructor
world.insert_bundle(entity, Bundle, TestBundle.new({ comp = CompWithDefaultAndComponentData.new({ "bundled" }) }))

assert(world.has_bundle(entity, Bundle) == true, "Expected entity to have the bundle after inserting")
assert(world.get_component(entity, Comp)[1] == "bundled", "Expected the bundled component value to be inserted")
assert(world.has_component(entity, Simple) == true, "Expected the nested bundle's component to be inserted")
assert(#Bundle:components() == 2, "Expected the nested bundle to be flattened into the bundle's components")

local taken = world.take_bundle(entity, Bundle)
assert(taken.comp[1] == "bundled", "Expected the taken bundle to contain the component value")
assert(world.has_component(entity, Comp) == false, "Expected take_bundle to remove the components")
assert(world.take_bundle(entity, Bundle) == nil, "Expected taking a missing bundle to return nil")

world.add_default_bundle(entity, Bundle)
assert(world.get_component(entity, Comp)[1] == "Default", "Expected the default bundle to be inserted")
world.remove_bundle(entity, Bundle)
assert(world.has_bundle(entity, Bundle) == false, "Expected remove_bundle to remove the components")

local spawned = world.spawn_bundle(Bundle, TestBundle.new({}))
assert(world.has_bundle(spawned, Bundle) == true, "Expected spawn_bundle to spawn an entity with the bundle")

-- the `types` table holds bundle registrations too
assert(world.has_bundle(spawned, types.TestBundle) == true, "Expected types.TestBundle to be a bundle registration")
