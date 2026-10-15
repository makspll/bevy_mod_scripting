let EulerRot = import_namespace.call("math").EulerRot;
let IVec2 = import_namespace.call("math").IVec2;
let Mat3 = import_namespace.call("math").Mat3;
let Mat4 = import_namespace.call("math").Mat4;
let Quat = import_namespace.call("math").Quat;
let Vec3 = import_namespace.call("math").Vec3;

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
assert(m.determinant.call() == 6.0, "determinant");
assert(m + m == m * 2.0, "mat3 + mat3");
assert(m / 2.0 == m * 0.5, "mat3 / number");

let t = Mat4.from_translation.call(Vec3.new_.call(1.0, 2.0, 3.0));
assert(t.transform_point3.call(Vec3.zero.call()) == Vec3.new_.call(1.0, 2.0, 3.0), "transform_point3");

let i = IVec2.new_.call(3, -4);
assert(i.abs.call() == IVec2.new_.call(3, 4), "ivec abs");
