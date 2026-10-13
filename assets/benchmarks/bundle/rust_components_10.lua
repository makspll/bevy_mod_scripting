-- spawns entities with a script registered bundle of 10 rust components
local members = {}
local values = {}
for i = 0, 9 do
    local name = "BenchComponent" .. i
    members[#members + 1] = types[name]
    values[name] = construct(types[name], { i })
end

local bundle = world.register_new_bundle("BenchRustBundle10", members)
local value = DynamicBundle.new({ data = values })

function bench()
    world.spawn_bundle(bundle, value)
end
