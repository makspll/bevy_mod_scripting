local world = import_namespace("bms").world
local test_utils = import_namespace("test_utils")
assert_throws(function()
    world.get_parent(test_utils._make_invalid_entity())
end, "Missing or invalid entity")
