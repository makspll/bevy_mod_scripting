local seen = {}

function on_test()
    local Marker = world.register_new_component("ObserverMarker")
    local observer = world.add_observer(types.AddEvent, { Marker }, function(event)
        table.insert(seen, event.entity)
    end)
    assert(observer ~= nil, "observer entity should be returned")

    local with = world.spawn()
    local without = world.spawn()
    world.add_default_component(with, Marker)
    world.add_default_component(without, world.get_type_by_name("Transform"))

    assert(#seen == 1, "observer should have run exactly once, ran: " .. #seen)
    assert(seen[1] == with, "observer should receive the entity")

    -- despawning the observer removes it
    world.despawn(observer)
    local again = world.spawn()
    world.add_default_component(again, Marker)
    assert(#seen == 1, "despawned observer must not run")
end
