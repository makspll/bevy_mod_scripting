
reseed.call();

let matrix = ();
let vector = ();
fn pre_bench(){
    vector = Vec3.new_.call(random.call(1,999), random.call(1,999), random.call(1,999));
    matrix = Mat3.from_cols.call(
        Vec3.new_.call(random.call(1,999), random.call(1,999), random.call(1,999)),
        Vec3.new_.call(random.call(1,999), random.call(1,999), random.call(1,999)),
        Vec3.new_.call(random.call(1,999), random.call(1,999), random.call(1,999))
    );
}

fn bench() {
    let mul = matrix * vector;
    let add = matrix + vector;
    let div = vector / matrix;
}