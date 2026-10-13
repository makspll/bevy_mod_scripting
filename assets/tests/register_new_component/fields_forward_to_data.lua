function on_test()
    local Health = world.register_new_component("script.test", "ForwardedHealth")
    local entity = world.spawn()
    world.insert_component(entity, Health, DynamicComponent.new({ data = { current = 5, label = "hp" } }))

    local health = world.get_component(entity, Health)

    -- fields are readable directly, and through `data`
    assert(health.current == 5, "forwarded read failed: " .. tostring(health.current))
    assert(health.data.current == 5, "data read failed")
    assert(health.label == "hp")

    -- and writable directly
    health.current = 7
    assert(health.current == 7, "forwarded write failed: " .. tostring(health.current))
    assert(health.data.current == 7, "write not visible through data")

    -- adding new fields works
    health.max = 10
    assert(health.data.max == 10)

    -- `data` itself can still be replaced
    health.data = { current = 1 }
    assert(health.current == 1 and health.max == nil)
end
