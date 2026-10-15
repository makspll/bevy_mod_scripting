local calls = 0
local first_payload

function on_hit(payload)
    calls = calls + 1
    if calls == 1 then
        first_payload = payload
    end
end

function on_test()
    -- script events take any payload, or none
    local Hit = world.register_new_event("script.test", "TriggerEventHit")
    world.add_observer(Hit, "on_hit")
    world.trigger_event(Hit, 5)
    world.trigger_event(Hit)

    -- rust events need a value
    local AddEvent = import_namespace("ecs").AddEvent
    local ok, err = pcall(world.trigger_event, AddEvent)
    assert(not ok and tostring(err):find("needs an event value", 1, true), tostring(err))
end

function on_test_post_update()
    assert(calls == 2, "both triggers should reach the observer, got " .. calls)
    assert(first_payload == 5, "payload should arrive")
end
