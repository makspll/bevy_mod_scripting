local world = import_namespace("bms").world
local TestComponent = import_namespace("test_utils").TestComponent
local entity_a = world.spawn()

local componentA = world.get_type_by_name("test_utils::test_data::CompWithFromWorldAndComponentData")
local componentB = world.get_type_by_name("test_utils::test_data::CompWithDefaultAndComponentData")
local componentC = world.get_type_by_name("test_utils::test_data::TestComponent")

world.add_default_component(entity_a, componentA)
world.add_default_component(entity_a, componentB)
world.insert_component(entity_a, componentC, TestComponent.new({
    strings = { "asd" }
}))

local query_result = world.query():component(componentA):component(componentA):component(componentC):build()

assert(#query_result == 1, "Expected 1 result, got " .. #query_result)
for i, result in pairs(query_result) do
    assert(result[1]:index() == entity_a:index(),
        "Expected entity_a, got " .. result[1]:index())
    -- the entity, then the components
    assert(#result == 4, "Expected the entity and 3 components, got " .. #result)
    A = result[2]
    B = result[3]
    C = result[4]
    assert(A[1] == "Default", "Expected 'Default', got: " .. A[1])
    assert(B[1] == "Default", "Expected 'Default', got: " .. B[1])
    assert(C.strings[1] == "asd", "Expected 'asd', got: " .. C.strings[1])
end
