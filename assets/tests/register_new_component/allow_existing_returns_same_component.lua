local first = world.register_new_component("ScriptComponentAllowExisting")
local second = world.register_new_component("ScriptComponentAllowExisting", true)

local entity = world.spawn()
world.add_default_component(entity, first)

assert(world.has_component(entity, second), "allow_existing should return the original component")
