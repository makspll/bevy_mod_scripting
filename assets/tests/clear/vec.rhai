let world = import_namespace.call("bms").world;
let res_type = world.get_type_by_name.call("test_utils::test_data::TestResourceWithVariousFields");
let res = world.get_resource.call(res_type);

res.vec_usize.clear.call();

assert(res.vec_usize.len.call() == 0, "Clear did not work");