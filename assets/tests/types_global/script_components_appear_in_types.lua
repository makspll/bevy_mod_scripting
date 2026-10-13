function on_test()
    assert(types.ScriptTypesEntryComp == nil, "not registered yet")
    local comp = world.register_new_component("script.test", "ScriptTypesEntryComp")
    world.register_new_bundle("script.test", "ScriptTypesEntryBundle", { comp })
end

function on_test_post_update()
    -- the context was created before the registrations, `types` must have been resynced
    assert(types.ScriptTypesEntryComp ~= nil, "script component should appear in types")
    assert(types.ScriptTypesEntryBundle ~= nil, "script bundle should appear in types")
    assert(types.Transform ~= nil, "native types stay available")

    local entity = world.spawn()
    world.add_default_component(entity, types.ScriptTypesEntryComp)
    assert(world.has_component(entity, types.ScriptTypesEntryComp))
end
