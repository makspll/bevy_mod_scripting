local res_type = world.get_type_by_name("TestResourceWithVariousFields")
local res = world.get_resource(res_type)

iterated_vals = {}
for i,v in pairs(res.vec_usize) do
    iterated_vals[i] = v
end

print("Iterated vals:")
for i,v in pairs(iterated_vals) do
    print(i, v)
end
assert(iterated_vals[1] == 1)
assert(iterated_vals[2] == 2)
assert(iterated_vals[3] == 3)
assert(iterated_vals[4] == 4)
assert(iterated_vals[5] == 5)
