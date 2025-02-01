let res_type = world.get_type_by_name.call("TestResourceWithVariousFields");
let res = world.get_resource.call(res_type);

res.vec_usize.insert.call(2, 42);

assert(res.vec_usize[2] == 42, "insert did not work");