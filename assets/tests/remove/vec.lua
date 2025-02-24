local res_type = world.get_type_by_name("TestResourceWithVariousFields")
local res = world.get_resource(res_type)

local removed = res.vec_usize:remove(5)

assert(removed == 5, "Remove did not work")
