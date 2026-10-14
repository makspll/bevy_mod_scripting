function on_test()
    local transform = world.get_type_by_name("bevy_transform::components::transform::Transform")
    local entity = world.get_type_by_name("bevy_ecs::entity::Entity")
    assert(transform:namespace() == "bevy_transform", "got: " .. transform:namespace())
    assert(entity:namespace() == "bevy_ecs", "got: " .. entity:namespace())

    -- BMS' own types opt in to the `bms` namespace explicitly
    assert(world.get_type_by_name("bevy_mod_scripting_bindings::query::ScriptTypeRegistration"):namespace() == "bms")
    assert(world.get_type_by_name("bevy_mod_scripting_script::ScriptAttachment"):namespace() == "bms")
    assert(world.get_type_by_name("bevy_mod_scripting_bindings::script_component::DynamicComponent"):namespace() == "bms")

    -- script components are in the namespace they were registered in
    local Comp = world.register_new_component("script.test", "NamespaceTestComp")
    assert(Comp:namespace() == "script.test", "got: " .. Comp:namespace())
end
