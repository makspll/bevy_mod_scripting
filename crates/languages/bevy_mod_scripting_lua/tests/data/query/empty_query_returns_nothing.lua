local component_a = world:get_type_by_name("TestComponent")

for entity,_ in world:query(component_a):with(component_a):without(component_a):iter() do
    assert(false, "This should not be reached")
end