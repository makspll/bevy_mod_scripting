local removed = {}
local added = {}

function on_test()
    local Marker = world.register_new_component("ObserverFilterMarker")
    local a = world.spawn()
    local b = world.spawn()

    -- only observes `a`
    world.add_observer(types.AddEvent, { Marker }, function(event)
        table.insert(added, event.entity)
    end, { a })
    world.add_observer(types.RemoveEvent, { Marker }, function(event)
        table.insert(removed, event.entity)
    end)

    world.add_default_component(a, Marker)
    world.add_default_component(b, Marker)
    assert(#added == 1 and added[1] == a, "entity filtered observer should only see a")

    world.remove_component(b, Marker)
    assert(#removed == 1 and removed[1] == b, "remove observer should see b")
end
