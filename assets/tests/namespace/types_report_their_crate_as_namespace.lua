function on_test()
    local transform = world.get_type_by_name("Transform")
    local entity = world.get_type_by_name("Entity")
    assert(transform:namespace() == "bevy_transform", "got: " .. transform:namespace())
    assert(entity:namespace() == "bevy_ecs", "got: " .. entity:namespace())

    -- BMS' own types opt in to the `bms` namespace explicitly
    assert(world.get_type_by_name("ScriptTypeRegistration"):namespace() == "bms")
    assert(world.get_type_by_name("ScriptAttachment"):namespace() == "bms")
    assert(world.get_type_by_name("DynamicComponent"):namespace() == "bms")

    -- script components are in the namespace they were registered in
    local Comp = world.register_new_component("script.test", "NamespaceTestComp")
    assert(Comp:namespace() == "script.test", "got: " .. Comp:namespace())
end
