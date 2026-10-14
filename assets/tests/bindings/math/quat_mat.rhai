fn approx(a, b) {
    (a - b).abs() < 0.0001
}

let x = Vec3.unit_x.call();
let q = Quat.from_rotation_y.call(PI() / 2.0);
let rotated = q * x;
assert(approx(rotated.z, -1.0), "rotate a vector");
assert(Quat.identity.call() * x == x, "identity");

// enum variants can be passed directly where a value is expected
let euler = Quat.from_euler.call(EulerRot.YXZ, PI() / 2.0, 0.0, 0.0);
let rotated_euler = euler * x;
assert(approx(rotated_euler.z, -1.0), "from_euler");

let m = Mat3.from_cols.call(Vec3.new_.call(1.0, 0.0, 0.0), Vec3.new_.call(0.0, 2.0, 0.0), Vec3.new_.call(0.0, 0.0, 3.0));
assert(m * Vec3.one.call() == Vec3.new_.call(1.0, 2.0, 3.0), "mat3 * vec3");
assert(m.determinant.call(m) == 6.0, "determinant");

let t = Mat4.from_translation.call(Vec3.new_.call(1.0, 2.0, 3.0));
assert(t.transform_point3.call(t, Vec3.zero.call()) == Vec3.new_.call(1.0, 2.0, 3.0), "transform_point3");

let i = IVec2.new_.call(3, -4);
assert(i.abs.call(i) == IVec2.new_.call(3, 4), "ivec abs");
