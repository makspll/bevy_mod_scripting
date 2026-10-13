local ran = 0
local Event

register_callback("on_instant", function()
    ran = ran + 1
end)

function on_test()
    Event = world.register_new_event("InstantEvent")
    -- an instant observer on the script that is triggering it cannot run: its context is busy
    world.add_observer(Event, {}, "on_instant", nil, true)
    world.trigger(Event)
end

function on_test_post_update()
    assert(ran == 0, "instant observer must not re-enter the running script, ran: " .. ran)
end
