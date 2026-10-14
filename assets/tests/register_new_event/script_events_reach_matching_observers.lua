local got_a = {}
local got_b = {}
local EventA, EventB, observer_a

register_callback("on_a", function(payload)
    table.insert(got_a, payload)
end)
-- a freestanding global function works as a callback too
function on_b(payload)
    table.insert(got_b, payload)
end

function on_test()
    EventA = world.register_new_event("script.test", "TestEventA")
    EventB = world.register_new_event("script.test", "TestEventB")
    -- registering again returns the same event
    assert(world.register_new_event("script.test", "TestEventA", true) ~= nil)
    assert(EventA:short_name() == "TestEventA", "got: " .. EventA:short_name())
    assert(EventA:namespace() == "script.test", "got: " .. EventA:namespace())

    observer_a = world.add_observer(EventA, "on_a")
    world.add_observer(EventB, "on_b")

    world.trigger_event(EventA, { value = 1 })
    world.trigger_event(EventB, "hello")
    world.trigger_event(EventA, 3)
end

function on_test_post_update()
    assert(#got_a == 2, "A observers should see both A triggers, saw " .. #got_a)
    assert(got_a[1].value == 1, "table payload should arrive")
    assert(got_a[2] == 3, "number payload should arrive")
    assert(#got_b == 1 and got_b[1] == "hello", "B observer sees only B")

    world.despawn(observer_a)
    world.trigger_event(EventA, 5)
end

function on_test_last()
    assert(#got_a == 2, "despawned observer must not run")
end
