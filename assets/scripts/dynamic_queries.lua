local component_a = world:get_type_by_name("ComponentA")
local component_b = world:get_type_by_name("ComponentB")
local component_c = world:get_type_by_name("ComponentC")

print("Querying for entities with component_a and without component_c")
for entity, c in world:query(component_a):without(component_c):iter() do
	print("Entity with index: " .. entity:index() .. " component: " .. tostring(c))
end

print("Querying for entities with component_b and without component_a")
for entity, c in world:query(component_b):without(component_a):iter() do
	print("Entity with index: " .. entity:index() .. " component: " .. tostring(c))
end

print("Querying for all components at once")
for entity, c1,c2,c3 in world:query(component_a, component_b, component_c):iter() do
	print("Entity with index: " .. entity:index())
	print("\tComponentA: " .. tostring(c1))
	print("\tComponentB: " .. tostring(c2))
	print("\tComponentC: " .. tostring(c3))
end

world:exit()