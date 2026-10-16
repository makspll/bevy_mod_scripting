
let test_utils = import_namespace.call("test_utils");
test_utils.reseed.call();

let matrix = ();
let vector = ();
fn pre_bench(){
    let Mat3 = import_namespace.call("math").Mat3;
    let Vec3 = import_namespace.call("math").Vec3;
    vector = Vec3.new_.call(test_utils.random.call(1,999), test_utils.random.call(1,999), test_utils.random.call(1,999));
    matrix = Mat3.from_cols.call(
        Vec3.new_.call(test_utils.random.call(1,999), test_utils.random.call(1,999), test_utils.random.call(1,999)),
        Vec3.new_.call(test_utils.random.call(1,999), test_utils.random.call(1,999), test_utils.random.call(1,999)),
        Vec3.new_.call(test_utils.random.call(1,999), test_utils.random.call(1,999), test_utils.random.call(1,999))
    );
}

fn bench() {
    let mul = matrix * vector;
    let add = matrix + matrix;
    let compose = matrix * matrix;
}