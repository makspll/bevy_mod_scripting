let res_type = world.get_type_by_name.call("TestResourceWithVariousFields");
let res = world.get_resource.call(res_type);

let removed = res.vec_usize.remove.call(4);

assert(removed == 5, "Remove did not work");
