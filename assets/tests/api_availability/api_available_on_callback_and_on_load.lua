assert(world ~= nil, "World was not found")
assert(world.get_type_by_name("TestComponent") ~= nil, "Could not find TestComponent type")
local global_invocation = Entity.from_raw(1)

function on_test()
    assert(world ~= nil, "World was not found")
    assert(world.get_type_by_name("TestComponent") ~= nil, "Could not find TestComponent type")
    Entity.from_raw(1)

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