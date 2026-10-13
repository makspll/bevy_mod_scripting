local removed = {}
local added = {}
local a, b

register_callback("on_added", function(event)
    table.insert(added, event.entity)
end)
register_callback("on_removed", function(event)
    table.insert(removed, event.entity)
end)

function on_test()
    local Marker = world.register_new_component("script.test", "ObserverFilterMarker")
    a = world.spawn()
    b = world.spawn()

    -- only observes `a`
    world.add_observer(types.AddEvent, { Marker }, "on_added", { a })
    world.add_observer(types.RemoveEvent, { Marker }, "on_removed")

    world.add_default_component(a, Marker)
    world.add_default_component(b, Marker)
    world.remove_component(b, Marker)
end

function on_test_post_update()
    assert(#added == 1 and added[1] == a, "entity filtered observer should only see a")
    assert(#removed == 1 and removed[1] == b, "remove observer should see b")
end
