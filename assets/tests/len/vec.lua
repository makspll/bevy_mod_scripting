local res_type = world.get_type_by_name("test_utils::test_data::TestResourceWithVariousFields")
local res = world.get_resource(res_type)

assert(res.vec_usize:len() == 5, "Length is not 5")