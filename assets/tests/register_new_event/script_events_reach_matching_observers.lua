local got_a = {}
local got_b = {}

function on_test()
    local EventA = world.register_new_event("TestEventA")
    local EventB = world.register_new_event("TestEventB")
    -- registering again returns the same event
    assert(world.register_new_event("TestEventA", true) ~= nil)

    local observer_a = world.add_observer(EventA, {}, function(payload)
        table.insert(got_a, payload)
    end)
    world.add_observer(EventB, {}, function(payload)
        table.insert(got_b, payload)
    end)

    world.trigger(EventA, { value = 1 })
    world.trigger(EventB, "hello")
    world.trigger(EventA, 3)

    assert(#got_a == 2, "A observers should see both A triggers, saw " .. #got_a)
    assert(got_a[1].value == 1, "table payload should arrive")
    assert(got_a[2] == 3, "number payload should arrive")
    assert(#got_b == 1 and got_b[1] == "hello", "B observer sees only B")

    world.despawn(observer_a)
    world.trigger(EventA, 5)
    assert(#got_a == 2, "despawned observer must not run")
end
