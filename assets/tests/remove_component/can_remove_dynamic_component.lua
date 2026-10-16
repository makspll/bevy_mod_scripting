local world = import_namespace("bms").world
local NewComponent = world.register_new_component("script.test", "ScriptComponentA")
local new_entity = world.spawn()
world.add_default_component(new_entity, NewComponent)

local component_instance = world.get_component(new_entity, NewComponent)
assert(component_instance ~= nil, "unexpected value: " .. tostring(component_instance.data))

world.remove_component(new_entity, NewComponent)
local component_instance = world.get_component(new_entity, NewComponent)

assert(component_instance == nil, "unexpected value: " .. tostring(component_instance))
