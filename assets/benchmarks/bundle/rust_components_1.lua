-- spawns entities with a script registered bundle of 1 rust components
local members = {}
local values = {}
for i = 0, 0 do
    local name = "BenchComponent" .. i
    members[#members + 1] = types[name]
    values[name] = construct(types[name], { i })
end

local bundle = world.register_new_bundle("script.test", "BenchRustBundle1", members)
local value = DynamicBundle.new({ data = values })

function bench()
    world.spawn_bundle(bundle, value)
end
