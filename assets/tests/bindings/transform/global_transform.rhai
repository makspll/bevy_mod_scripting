let GlobalTransform = import_namespace.call("transform").GlobalTransform;
let Quat = import_namespace.call("math").Quat;
let Transform = import_namespace.call("transform").Transform;
let Vec3 = import_namespace.call("math").Vec3;

fn approx(a, b) {
    (a - b).abs() < 0.0001
}

fn approx_vec3(a, b) {
    approx(a.x, b.x) && approx(a.y, b.y) && approx(a.z, b.z)
}

// constructors
let g = GlobalTransform.identity.call();
assert(approx_vec3(g.translation.call(), Vec3.zero.call()), "identity translation");
assert(g.rotation.call() == Quat.identity.call(), "identity rotation");
assert(approx_vec3(g.scale.call(), Vec3.one.call()), "identity scale");

let xyz = GlobalTransform.from_xyz.call(1.0, 2.0, 3.0);
assert(approx_vec3(xyz.translation.call(), Vec3.new_.call(1.0, 2.0, 3.0)), "from_xyz");
assert(approx_vec3(xyz.compute_transform.call().translation, Vec3.new_.call(1.0, 2.0, 3.0)), "compute_transform keeps the translation");

// accessors on a transform that has a rotation and a scale
let posed = GlobalTransform.identity.call() * Transform.from_scale.call(Vec3.new_.call(2.0, 2.0, 2.0));
assert(approx_vec3(posed.scale.call(), Vec3.new_.call(2.0, 2.0, 2.0)), "scale");
let turned = GlobalTransform.identity.call() * Transform.from_rotation.call(Quat.from_rotation_y.call(PI() / 2.0));
assert(approx_vec3(turned.rotation.call() * Vec3.unit_x.call(), Vec3.new_.call(0.0, 0.0, -1.0)), "rotation");

// directions
let id = GlobalTransform.identity.call();
assert(approx_vec3(id.forward.call(), Vec3.new_.call(0.0, 0.0, -1.0)), "forward");
assert(approx_vec3(id.back.call(), Vec3.new_.call(0.0, 0.0, 1.0)), "back");
assert(approx_vec3(id.left.call(), Vec3.new_.call(-1.0, 0.0, 0.0)), "left");
assert(approx_vec3(id.right.call(), Vec3.new_.call(1.0, 0.0, 0.0)), "right");
assert(approx_vec3(id.up.call(), Vec3.new_.call(0.0, 1.0, 0.0)), "up");
assert(approx_vec3(id.down.call(), Vec3.new_.call(0.0, -1.0, 0.0)), "down");
assert(approx_vec3(turned.forward.call(), Vec3.new_.call(-1.0, 0.0, 0.0)), "directions follow the rotation");

// points, matrices and conversions
assert(approx_vec3(xyz.transform_point.call(Vec3.new_.call(1.0, 0.0, 0.0)), Vec3.new_.call(2.0, 2.0, 3.0)), "transform_point");
assert(approx_vec3(xyz.to_matrix.call().transform_point3.call(Vec3.zero.call()), Vec3.new_.call(1.0, 2.0, 3.0)), "to_matrix");

// a local transform relative to a parent
let child = GlobalTransform.from_xyz.call(3.0, 0.0, 0.0);
let parent = GlobalTransform.from_xyz.call(1.0, 0.0, 0.0);
assert(approx_vec3(child.reparented_to.call(parent).translation, Vec3.new_.call(2.0, 0.0, 0.0)), "reparented_to");

// composition
let g1 = GlobalTransform.from_xyz.call(1.0, 0.0, 0.0);
let g2 = GlobalTransform.from_xyz.call(0.0, 2.0, 0.0);
assert(approx_vec3((g1 * g2).translation.call(), Vec3.new_.call(1.0, 2.0, 0.0)), "mul with a global transform");
assert(approx_vec3((g1 * Transform.from_xyz.call(0.0, 0.0, 5.0)).translation.call(), Vec3.new_.call(1.0, 0.0, 5.0)), "mul with a transform");

// equality
assert(GlobalTransform.from_xyz.call(1.0, 2.0, 3.0).eq.call(xyz), "eq");
assert(GlobalTransform.from_xyz.call(1.0, 2.0, 3.0) == xyz, "== compares global transforms");
assert(!g1.eq.call(g2), "eq is false for different transforms");
