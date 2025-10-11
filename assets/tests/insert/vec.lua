local res_type = world.get_type_by_name("TestResourceWithVariousFields")
local res = world.get_resource(res_type)

res.vec_usize[2] = 42

assert(res.vec_usize[2] == 42, "insert did not work")
