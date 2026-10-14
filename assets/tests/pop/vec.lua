local res_type = world.get_type_by_name("test_utils::test_data::TestResourceWithVariousFields")
local res = world.get_resource(res_type)

local popped = res.vec_usize:pop()

assert(popped == 5, "Pop did not work")