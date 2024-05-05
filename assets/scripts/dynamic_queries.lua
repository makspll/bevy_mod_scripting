function on_event()
	local component_a = world:get_type_by_name("ComponentA")
	local component_b = world:get_type_by_name("ComponentB")
	local component_c = world:get_type_by_name("ComponentC")

	for entity, _ in world:query(component_a):with(component_b):without(component_c):iter() do
		print(entity)
	end
end
