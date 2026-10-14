function on_test()
    assert(types.ScriptTypesEntryComp == nil, "not registered yet")
    world.register_new_component("script.test", "ScriptTypesEntryComp")
end

function on_test_post_update()
    -- `types` is a snapshot taken when the context was created, script components registered later are imported instead
    assert(types.ScriptTypesEntryComp == nil, "types is not resynced")
    assert(types.Transform ~= nil, "native types stay available")

    local comp = import_namespace("script.test").ScriptTypesEntryComp
    assert(comp ~= nil, "script component should be importable")
    local entity = world.spawn()
    world.add_default_component(entity, comp)
    assert(world.has_component(entity, comp))
end
