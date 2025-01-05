local component_a = world.get_type_by_name("TestComponent")

for i,result in pairs(world.query():component(component_a):without(component_a):build()) do
    assert(false, "This should not be reached")
end