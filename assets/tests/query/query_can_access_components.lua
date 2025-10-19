local entity_a = world.spawn()

local componentA = world.get_type_by_name("CompWithFromWorldAndComponentData")
local componentB = world.get_type_by_name("CompWithDefaultAndComponentData")
local componentC = world.get_type_by_name("TestComponent")

world.add_default_component(entity_a, componentA)
world.add_default_component(entity_a, componentB)
world.insert_component(entity_a, componentC, construct(componentC, {
    strings = { [1] = "asd" }
}))

local query_result = world.query():component(componentA):component(componentA):component(componentC):build()

assert(#query_result == 1, "Expected 1 result, got " .. #query_result)
for i, result in pairs(query_result) do
    assert(result:entity():index() == entity_a:index(), "Expected entity_a, got " .. result:entity():index())
    components = result:components()
    assert(#components == 3, "Expected 3 components, got " .. #components)
    A = components[1]
    B = components[2]
    C = components[3]
    assert(A[1] == "Default", "Expected 'Default', got: " .. A[1])
    assert(B[1] == "Default", "Expected 'Default', got: " .. B[1])
    assert(C.strings[1] == "asd", "Expected 'asd', got: " .. C.strings[1])
end
