local world = import_namespace("bms").world
local CompWithDefaultAndComponentData = import_namespace("test_utils").CompWithDefaultAndComponentData
local TestBundle = import_namespace("test_utils").TestBundle

local Bundle = world.get_type_by_name("test_utils::test_data::TestBundle")
local Comp = world.get_type_by_name("test_utils::test_data::CompWithDefaultAndComponentData")
local Simple = world.get_type_by_name("test_utils::test_data::SimpleStruct")

local entity = world.spawn()

-- unspecified fields, including the nested bundle, are defaulted by the constructor
world.insert_bundle(entity, Bundle, TestBundle.new({ comp = CompWithDefaultAndComponentData.new({ "bundled" }) }))

assert(world.get_component(entity, Comp)[1] == "bundled", "Expected the bundled component value to be inserted")
assert(world.has_component(entity, Simple) == true, "Expected the nested bundle's component to be inserted")

local spawned = world.spawn_bundle(Bundle, TestBundle.new({}))
assert(world.has_component(spawned, Comp) == true, "Expected spawn_bundle to spawn an entity with the bundle")
assert(world.has_component(spawned, Simple) == true, "Expected spawn_bundle to insert the nested bundle")
