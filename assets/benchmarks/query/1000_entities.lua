local entity_a = world.spawn()
local entity_b = world.spawn()
local entity_c = world.spawn()

local components = {
    types.CompWithDefaultAndComponentData,
    types.CompWithFromWorldAndComponentData,
    types.SimpleTupleStruct,
    types.SimpleEnum,
}

reseed()

for i = 1, 1000 do
    local entity = world.spawn()
    -- spawn 1000 entities with random components
    local left_to_pick = {1,2,3,4}
    for j = 1, 3 do
        local index = random_int(1, #left_to_pick)
        local component = components[left_to_pick[index]]
        table.remove(left_to_pick, index)
        world.add_default_component(entity, component)
    end
end

function bench() 
    for i,result in pairs(world.query()
        :component(types.CompWithFromWorldAndComponentData)
        :component(types.SimpleTupleStruct)
        :with(types.SimpleEnum)
        :without(types.CompWithDefaultAndComponentData)
        :build()) do 
    
    end 
end