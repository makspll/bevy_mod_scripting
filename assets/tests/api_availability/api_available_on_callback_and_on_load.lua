local world = import_namespace("bms").world
local test_utils = import_namespace("test_utils")
assert(world ~= nil, "World was not found")
assert(world.get_type_by_name("test_utils::test_data::TestComponent") ~= nil, "Could not find TestComponent type")
local global_invocation = test_utils._entity_from_index(1)

function on_test()
    assert(world ~= nil, "World was not found")
    assert(world.get_type_by_name("test_utils::test_data::TestComponent") ~= nil, "Could not find TestComponent type")
    test_utils._entity_from_index(1)

    -- assert global_invocation happened
    assert(global_invocation ~= nil, "Global invocation did not happen")

    return true
end

function on_test_post_update()
    return true
end

function on_test_last()
    return true
end
