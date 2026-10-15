let Cuboid = import_namespace.call("math").Cuboid;
let Dir3 = import_namespace.call("math").Dir3;
let EaseFunction = import_namespace.call("math").EaseFunction;
let Ray3d = import_namespace.call("math").Ray3d;
let Rect = import_namespace.call("math").Rect;
let Vec2 = import_namespace.call("math").Vec2;
let Vec3 = import_namespace.call("math").Vec3;
let dir = Dir3.new_.call(Vec3.new_.call(0.0, 0.0, 5.0));
assert(dir.as_vec3.call() == Vec3.unit_z.call(), "dir is normalized");
assert(type_of(Dir3.new_.call(Vec3.zero.call())) == "()", "zero vector is not a direction");

let rect = Rect.new_.call(0.0, 0.0, 4.0, 2.0);
assert(rect.width.call() == 4.0, "rect width");
assert(rect.contains.call(Vec2.new_.call(1.0, 1.0)), "rect contains");

let ray = Ray3d.new_.call(Vec3.new_.call(0.0, 10.0, 0.0), Vec3.new_.call(0.0, -1.0, 0.0));
assert(ray.get_point.call(4.0) == Vec3.new_.call(0.0, 6.0, 0.0), "ray point");

assert(Cuboid.new_.call(2.0, 4.0, 6.0).half_size == Vec3.new_.call(1.0, 2.0, 3.0), "cuboid");

assert(EaseFunction.sample.call(EaseFunction.Linear, 0.25) == 0.25, "linear easing");
