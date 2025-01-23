local res_type = world.get_type_by_name("TestResourceWithVariousFields")
local res = world.get_resource(res_type)

local popped = res.vec_usize:pop()

assert(popped == 5, "Pop did not work")