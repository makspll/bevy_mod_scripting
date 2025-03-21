function on_test()
    local NewComponent = world.register_new_component("ScriptComponentA")

    local new_entity = world.spawn()
    world.insert_component(new_entity, NewComponent, construct(types.DynamicComponent, {
        data = "Hello World"
    }))

    local component_instance = world.get_component(new_entity, NewComponent)
    assert(component_instance.data == "Hello World", "unexpected value: " .. component_instance.data)

    component_instance.data = {
        foo = "bar"
    }

    assert(component_instance.data.foo == "bar", "unexpected value: " .. component_instance.data.foo)
end