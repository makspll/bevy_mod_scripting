local seen = {}
local Marker
local observer
local with, again

register_callback("on_marker_added", function(event)
    table.insert(seen, event.entity)
end)

function on_test()
    Marker = world.register_new_component("ObserverMarker")
    observer = world.add_observer(types.AddEvent, { Marker }, "on_marker_added")
    assert(observer ~= nil, "observer entity should be returned")

    with = world.spawn()
    local without = world.spawn()
    world.add_default_component(with, Marker)
    world.add_default_component(without, world.get_type_by_name("Transform"))

    -- observers queue their callbacks, they do not run in the middle of this one
    assert(#seen == 0, "callbacks must wait for the triggering callback to finish")
end

function on_test_post_update()
    assert(#seen == 1, "observer should have run exactly once, ran: " .. #seen)
    assert(seen[1] == with, "observer should receive the entity")

    -- despawning the observer removes it
    world.despawn(observer)
    again = world.spawn()
    world.add_default_component(again, Marker)
end

function on_test_last()
    assert(#seen == 1, "despawned observer must not run")
end
