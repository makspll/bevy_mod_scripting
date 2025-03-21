local NewComponent = world.register_new_component("ScriptComponentA")
local entity = world.spawn()

assert(world.has_component(entity, NewComponent) == false, "Entity should not have component")
world.add_default_component(entity, NewComponent)
assert(world.has_component(entity, NewComponent) == true, "Entity should have component")