fn approx(a, b) {
    (a - b).abs() < 0.0001
}

// constructors
let v = Vec3.new_.call(1.0, 2.0, 3.0);
assert(v.x == 1.0 && v.y == 2.0 && v.z == 3.0, "new");
assert(Vec3.splat.call(2.0) == Vec3.new_.call(2.0, 2.0, 2.0), "splat");
assert(Vec3.zero.call() == Vec3.new_.call(0.0, 0.0, 0.0), "zero");
assert(Vec3.unit_y.call() == Vec3.new_.call(0.0, 1.0, 0.0), "unit_y");

// operators, with vectors and numbers
assert(v + v == Vec3.new_.call(2.0, 4.0, 6.0), "add vector");
assert(v + 1.0 == Vec3.new_.call(2.0, 3.0, 4.0), "add number");
assert(v * 2.0 == Vec3.new_.call(2.0, 4.0, 6.0), "mul number");
assert(v - v == Vec3.zero.call(), "sub");

// lengths and products
let v2 = Vec2.new_.call(3.0, 4.0);
assert(approx(v2.length.call(v2), 5.0), "length");
assert(v.dot.call(v, v) == 14.0, "dot");
let x = Vec3.unit_x.call();
assert(x.cross.call(x, Vec3.unit_y.call()) == Vec3.unit_z.call(), "cross");

// normalization
let n = v.normalize.call(v);
assert(approx(n.length.call(n), 1.0), "normalize");
let zero = Vec3.zero.call();
assert(type_of(zero.try_normalize.call(zero)) == "()", "try_normalize");

// component wise
assert(zero.lerp.call(zero, Vec3.splat.call(2.0), 0.5) == Vec3.one.call(), "lerp");
assert(v.min_element.call(v) == 1.0, "min_element");
assert(v.truncate.call(v) == Vec2.new_.call(1.0, 2.0), "truncate");
