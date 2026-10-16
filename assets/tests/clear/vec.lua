local world = import_namespace("bms").world
local res_type = world.get_type_by_name("test_utils::test_data::TestResourceWithVariousFields")
local res = world.get_resource(res_type)

res.vec_usize:clear()

assert(res.vec_usize:len() == 0, "Clear did not work")