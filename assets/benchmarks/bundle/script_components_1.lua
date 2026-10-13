-- spawns entities with a script registered bundle of 1 script components
local members = {}
local values = {}
for i = 0, 0 do
    local name = "BenchScriptComponent" .. i
    members[#members + 1] = world.register_new_component(name)
    values[name] = i
end

local bundle = world.register_new_bundle("BenchScriptBundle1", members)
local value = DynamicBundle.new({ data = values })

function bench()
    world.spawn_bundle(bundle, value)
end
