local world = import_namespace("bms").world
local test_utils = import_namespace("test_utils")
local fake_entity = test_utils._make_invalid_entity()

assert_throws(function()
    world.push_children(fake_entity, { fake_entity })
end, "Missing or invalid entity")

local entity = world.spawn()
assert_throws(function()
    world.push_children(entity, { fake_entity })
end, "Missing or invalid entity")
