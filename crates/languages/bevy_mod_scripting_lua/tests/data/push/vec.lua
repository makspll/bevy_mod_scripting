local res_type = world.get_type_by_name("TestResourceWithVariousFields")
local res = world.get_resource(res_type)

res.vec_usize:push(42)

assert(res.vec_usize[6] == 42, "Push did not work")