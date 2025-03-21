local NewComponent = world.register_new_component("ScriptComponentA")
assert(NewComponent ~= nil, "Failed to register new component")
assert(NewComponent:short_name() == "DynamicComponent", "Unexpected component type")


local new_entity = world.spawn()

world.add_default_component(new_entity, NewComponent)

local component_intance = world.get_component(new_entity, NewComponent)

assert(component_intance ~= nil, "Failed to get component instance")