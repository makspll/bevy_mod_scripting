let res_type = world.get_type_by_name.call("test_utils::test_data::TestResourceWithVariousFields");
let res = world.get_resource.call(res_type);

assert(res.vec_usize.len.call() == 5, "Length is not 5");