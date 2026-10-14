let res_type = world.get_type_by_name.call("test_utils::test_data::TestResourceWithVariousFields");
let res = world.get_resource.call(res_type);

let popped = res.vec_usize.pop.call();

assert(popped == 5, "Pop did not work, got " + popped);