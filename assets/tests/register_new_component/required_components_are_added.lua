function on_test()
    local Base = world.register_new_component("RequiredBase")
    local Extra = world.register_new_component("RequiredExtra")
    local Top = world.register_new_component("RequiredTop", false, { Base, Extra })

    local entity = world.spawn()
    world.add_default_component(entity, Top)
    assert(world.has_component(entity, Base), "required Base should have been added")
    assert(world.has_component(entity, Extra), "required Extra should have been added")

    -- existing required components are not replaced and native ones work too
    local other = world.spawn()
    world.add_default_component(other, Base)
    world.add_default_component(other, Top)
    assert(world.has_component(other, Extra))

    -- requirements chain
    local A = world.register_new_component("RequiredCycleA")
    local B = world.register_new_component("RequiredCycleB", false, { A })
    local C = world.register_new_component("RequiredCycleC", false, { B })
    local cyc = world.spawn()
    world.add_default_component(cyc, C)
    assert(world.has_component(cyc, A) and world.has_component(cyc, B))
end
