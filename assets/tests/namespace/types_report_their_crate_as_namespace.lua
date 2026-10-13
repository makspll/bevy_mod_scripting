function on_test()
    assert(types.Transform:namespace() == "bevy_transform", "got: " .. types.Transform:namespace())
    assert(types.Entity:namespace() == "bevy_ecs", "got: " .. types.Entity:namespace())

    -- script components are not in any native namespace
    local Comp = world.register_new_component("NamespaceTestComp")
    assert(Comp:namespace() == "", "script components live in the root namespace")
end
