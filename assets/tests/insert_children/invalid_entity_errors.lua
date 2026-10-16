local world = import_namespace("bms").world
local test_utils = import_namespace("test_utils")
local fake_entity = test_utils._entity_from_index(0)
local fake_entity_valid = test_utils._make_invalid_entity()

assert_throws(function()
    world.insert_children(fake_entity_valid, 1, { fake_entity_valid })
end, "Missing or invalid entity")

local entity = world.spawn()
assert_throws(function()
    world.insert_children(entity, 1, { fake_entity })
end, "Are you trying to use an entity in a callback in which it's unavailable?")
