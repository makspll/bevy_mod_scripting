local res_type = world.get_type_by_name("TestResourceWithVariousFields")
local res = world.get_resource(res_type)

local iterated_vals = {}
for i, v in res.vec_usize:ipairs_clone() do
    assert(i == #iterated_vals + 1, "Index mismatch: expected " .. (#iterated_vals + 1) .. ", got " .. i)
    iterated_vals[#iterated_vals + 1] = v
end

assert(#iterated_vals == 5, "Length is not 5")
assert(iterated_vals[1] == 1, "First value is not 1")
assert(iterated_vals[2] == 2, "Second value is not 2")
assert(iterated_vals[3] == 3, "Third value is not 3")
assert(iterated_vals[4] == 4, "Fourth value is not 4")
assert(iterated_vals[5] == 5, "Fifth value is not 5")
