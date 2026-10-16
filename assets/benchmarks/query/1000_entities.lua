local world = import_namespace("bms").world
local test_utils = import_namespace("test_utils")
local entity_a = world.spawn()
local entity_b = world.spawn()
local entity_c = world.spawn()

local with_default = world.get_type_by_name("test_utils::test_data::CompWithDefaultAndComponentData")
local with_from_world = world.get_type_by_name("test_utils::test_data::CompWithFromWorldAndComponentData")
local tuple_struct = world.get_type_by_name("test_utils::test_data::SimpleTupleStruct")
local simple_enum = world.get_type_by_name("test_utils::test_data::SimpleEnum")

local components = {
    with_default,
    with_from_world,
    tuple_struct,
    simple_enum,
}

test_utils.reseed()

for i = 1, 1000 do
    local entity = world.spawn()
    -- spawn 1000 entities with random components
    local left_to_pick = {1,2,3,4}
    for j = 1, 3 do
        local index = test_utils.random_int(1, #left_to_pick)
        local component = components[left_to_pick[index]]
        table.remove(left_to_pick, index)
        world.add_default_component(entity, component)
    end
end

function bench() 
    for i,result in pairs(world.query()
        :component(with_from_world)
        :component(tuple_struct)
        :with(simple_enum)
        :without(with_default)
        :build()) do 
    
    end 
end