function on_test()
    assert(types.Transform:namespace() == "bevy_transform", "got: " .. types.Transform:namespace())
    assert(types.Entity:namespace() == "bevy_ecs", "got: " .. types.Entity:namespace())

    -- BMS' own types opt in to the `bms` namespace explicitly
    assert(types.ScriptTypeRegistration:namespace() == "bms")
    assert(types.ScriptAttachment:namespace() == "bms")
    assert(types.DynamicComponent:namespace() == "bms")

    -- script components are in the namespace they were registered in
    local Comp = world.register_new_component("script.test", "NamespaceTestComp")
    assert(Comp:namespace() == "script.test", "got: " .. Comp:namespace())
end
