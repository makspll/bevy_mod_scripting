let dir = Dir3.new_.call(Vec3.new_.call(0.0, 0.0, 5.0));
assert(dir.as_vec3.call(dir) == Vec3.unit_z.call(), "dir is normalized");
assert(type_of(Dir3.new_.call(Vec3.zero.call())) == "()", "zero vector is not a direction");

let rect = Rect.new_.call(0.0, 0.0, 4.0, 2.0);
assert(rect.width.call(rect) == 4.0, "rect width");
assert(rect.contains.call(rect, Vec2.new_.call(1.0, 1.0)), "rect contains");

let ray = Ray3d.new_.call(Vec3.new_.call(0.0, 10.0, 0.0), Vec3.new_.call(0.0, -1.0, 0.0));
assert(ray.get_point.call(ray, 4.0) == Vec3.new_.call(0.0, 6.0, 0.0), "ray point");

assert(Sphere.new_.call(2.0).radius == 2.0, "sphere");

assert(EaseFunction.sample.call(EaseFunction.Linear, 0.25) == 0.25, "linear easing");
