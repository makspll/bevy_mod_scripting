let Mat4 = import_namespace.call("math").Mat4;
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
let t = Transform.identity.call();
assert(approx_vec3(t.translation, Vec3.zero.call()), "identity translation");
assert(t.rotation == Quat.identity.call(), "identity rotation");
assert(approx_vec3(t.scale, Vec3.one.call()), "identity scale");
assert(t == Transform.identity.call(), "identity is equal to identity");

let xyz = Transform.from_xyz.call(1.0, 2.0, 3.0);
assert(approx_vec3(xyz.translation, Vec3.new_.call(1.0, 2.0, 3.0)), "from_xyz");
assert(approx_vec3(Transform.from_translation.call(Vec3.new_.call(1.0, 2.0, 3.0)).translation, Vec3.new_.call(1.0, 2.0, 3.0)), "from_translation");
assert(Transform.from_rotation.call(Quat.from_rotation_y.call(PI() / 2.0)).rotation == Quat.from_rotation_y.call(PI() / 2.0), "from_rotation");
assert(approx_vec3(Transform.from_scale.call(Vec3.new_.call(2.0, 2.0, 2.0)).scale, Vec3.new_.call(2.0, 2.0, 2.0)), "from_scale");
let from_matrix = Transform.from_matrix.call(Mat4.from_translation.call(Vec3.new_.call(1.0, 2.0, 3.0)));
assert(approx_vec3(from_matrix.translation, Vec3.new_.call(1.0, 2.0, 3.0)), "from_matrix");

// builders return new transforms and leave the original alone
let moved = xyz.with_translation.call(Vec3.new_.call(4.0, 5.0, 6.0));
assert(approx_vec3(moved.translation, Vec3.new_.call(4.0, 5.0, 6.0)), "with_translation");
assert(approx_vec3(xyz.translation, Vec3.new_.call(1.0, 2.0, 3.0)), "with_translation keeps the original");
let rotated = xyz.with_rotation.call(Quat.from_rotation_y.call(PI() / 2.0));
assert(approx_vec3(rotated.forward.call(), Vec3.new_.call(-1.0, 0.0, 0.0)), "with_rotation");
assert(approx_vec3(xyz.with_scale.call(Vec3.new_.call(3.0, 3.0, 3.0)).scale, Vec3.new_.call(3.0, 3.0, 3.0)), "with_scale");

let looking = Transform.from_xyz.call(0.0, 0.0, 0.0).looking_at.call(Vec3.new_.call(0.0, 0.0, -5.0), Vec3.unit_y.call());
assert(approx_vec3(looking.forward.call(), Vec3.new_.call(0.0, 0.0, -1.0)), "looking_at");
let looking_to = Transform.from_xyz.call(0.0, 0.0, 0.0).looking_to.call(Vec3.new_.call(1.0, 0.0, 0.0), Vec3.unit_y.call());
assert(approx_vec3(looking_to.forward.call(), Vec3.new_.call(1.0, 0.0, 0.0)), "looking_to");

// in-place rotations
let r = Transform.identity.call();
r.rotate.call(Quat.from_rotation_y.call(PI() / 2.0));
assert(approx_vec3(r.forward.call(), Vec3.new_.call(-1.0, 0.0, 0.0)), "rotate");

r = Transform.identity.call();
r.rotate_x.call(PI() / 2.0);
assert(approx_vec3(r.up.call(), Vec3.new_.call(0.0, 0.0, 1.0)), "rotate_x");

r = Transform.identity.call();
r.rotate_y.call(PI() / 2.0);
assert(approx_vec3(r.forward.call(), Vec3.new_.call(-1.0, 0.0, 0.0)), "rotate_y");

r = Transform.identity.call();
r.rotate_z.call(PI() / 2.0);
assert(approx_vec3(r.right.call(), Vec3.new_.call(0.0, 1.0, 0.0)), "rotate_z");

r = Transform.identity.call();
r.rotate_axis.call(Vec3.unit_x.call(), PI() / 2.0);
assert(approx_vec3(r.forward.call(), Vec3.new_.call(0.0, 1.0, 0.0)), "rotate_axis");

// local rotations apply after the current rotation, world rotations before it
r = Transform.from_rotation.call(Quat.from_rotation_y.call(PI() / 2.0));
r.rotate_local_x.call(PI() / 2.0);
assert(approx_vec3(r.forward.call(), Vec3.new_.call(0.0, 1.0, 0.0)), "rotate_local_x");

r = Transform.from_rotation.call(Quat.from_rotation_y.call(PI() / 2.0));
r.rotate_x.call(PI() / 2.0);
assert(approx_vec3(r.forward.call(), Vec3.new_.call(-1.0, 0.0, 0.0)), "rotate_x after another rotation");

r = Transform.identity.call();
r.rotate_local.call(Quat.from_rotation_x.call(PI() / 2.0));
assert(approx_vec3(r.forward.call(), Vec3.new_.call(0.0, 1.0, 0.0)), "rotate_local");

r = Transform.identity.call();
r.rotate_local_y.call(PI() / 2.0);
assert(approx_vec3(r.forward.call(), Vec3.new_.call(-1.0, 0.0, 0.0)), "rotate_local_y");

r = Transform.identity.call();
r.rotate_local_z.call(PI() / 2.0);
assert(approx_vec3(r.right.call(), Vec3.new_.call(0.0, 1.0, 0.0)), "rotate_local_z");

// rotations around a point move the translation
r = Transform.from_xyz.call(1.0, 0.0, 0.0);
r.rotate_around.call(Vec3.zero.call(), Quat.from_rotation_y.call(PI() / 2.0));
assert(approx_vec3(r.translation, Vec3.new_.call(0.0, 0.0, -1.0)), "rotate_around moves the translation");
assert(approx_vec3(r.forward.call(), Vec3.new_.call(-1.0, 0.0, 0.0)), "rotate_around rotates");

r = Transform.from_xyz.call(1.0, 0.0, 0.0);
r.translate_around.call(Vec3.zero.call(), Quat.from_rotation_y.call(PI() / 2.0));
assert(approx_vec3(r.translation, Vec3.new_.call(0.0, 0.0, -1.0)), "translate_around moves the translation");
assert(approx_vec3(r.forward.call(), Vec3.new_.call(0.0, 0.0, -1.0)), "translate_around keeps the rotation");

// in-place looking
r = Transform.from_xyz.call(0.0, 0.0, 0.0);
r.look_at.call(Vec3.new_.call(1.0, 0.0, 0.0), Vec3.unit_y.call());
assert(approx_vec3(r.forward.call(), Vec3.new_.call(1.0, 0.0, 0.0)), "look_at");
r.look_to.call(Vec3.new_.call(0.0, 0.0, 1.0), Vec3.unit_y.call());
assert(approx_vec3(r.forward.call(), Vec3.new_.call(0.0, 0.0, 1.0)), "look_to");

// directions of the identity transform
let id = Transform.identity.call();
assert(approx_vec3(id.forward.call(), Vec3.new_.call(0.0, 0.0, -1.0)), "forward");
assert(approx_vec3(id.back.call(), Vec3.new_.call(0.0, 0.0, 1.0)), "back");
assert(approx_vec3(id.left.call(), Vec3.new_.call(-1.0, 0.0, 0.0)), "left");
assert(approx_vec3(id.right.call(), Vec3.new_.call(1.0, 0.0, 0.0)), "right");
assert(approx_vec3(id.up.call(), Vec3.new_.call(0.0, 1.0, 0.0)), "up");
assert(approx_vec3(id.down.call(), Vec3.new_.call(0.0, -1.0, 0.0)), "down");
assert(approx_vec3(id.local_x.call(), Vec3.unit_x.call()), "local_x");
assert(approx_vec3(id.local_y.call(), Vec3.unit_y.call()), "local_y");
assert(approx_vec3(id.local_z.call(), Vec3.unit_z.call()), "local_z");

// points and composition
let a = Transform.from_translation.call(Vec3.new_.call(1.0, 0.0, 0.0)).with_scale.call(Vec3.new_.call(2.0, 2.0, 2.0));
assert(approx_vec3(a.transform_point.call(Vec3.new_.call(1.0, 0.0, 0.0)), Vec3.new_.call(3.0, 0.0, 0.0)), "transform_point scales then translates");
assert(approx_vec3(a * Vec3.new_.call(1.0, 0.0, 0.0), Vec3.new_.call(3.0, 0.0, 0.0)), "mul by a point transforms it");

let b = Transform.from_translation.call(Vec3.new_.call(0.0, 2.0, 0.0));
let composed = Transform.from_translation.call(Vec3.new_.call(1.0, 0.0, 0.0)) * b;
assert(approx_vec3(composed.translation, Vec3.new_.call(1.0, 2.0, 0.0)), "mul composes");
let mul_composed = Transform.from_translation.call(Vec3.new_.call(1.0, 0.0, 0.0)).mul_transform.call(b);
assert(approx_vec3(mul_composed.translation, Vec3.new_.call(1.0, 2.0, 0.0)), "mul_transform composes");

let matrix_point = Transform.from_xyz.call(1.0, 2.0, 3.0).to_matrix.call().transform_point3.call(Vec3.zero.call());
assert(approx_vec3(matrix_point, Vec3.new_.call(1.0, 2.0, 3.0)), "to_matrix keeps the translation");

// equality and finiteness
assert(Transform.from_xyz.call(0.0, 0.0, 0.0).eq.call(Transform.identity.call()), "eq");
assert(!xyz.eq.call(Transform.identity.call()), "eq is false for different transforms");
assert(Transform.identity.call().is_finite.call(), "is_finite");
let nan = 0.0 / 0.0;
assert(!Transform.from_xyz.call(nan, 0.0, 0.0).is_finite.call(), "NaN is not finite");

// components referenced from the world: changes made through a reference persist
let transform_type = world.get_type_by_name.call("bevy_transform::components::transform::Transform");
let entity = world.spawn_.call();
world.insert_component.call(entity, transform_type, Transform.from_xyz.call(1.0, 2.0, 3.0));

// rhai writes a field of a component through the whole field, `reference.translation.x = 5.0` fails to claim access
let reference = world.get_component.call(entity, transform_type);
reference.rotate_y.call(1.0);
reference.translation = Vec3.new_.call(5.0, 2.0, 3.0);

let fetched = world.get_component.call(entity, transform_type);
assert(approx(fetched.translation.x, 5.0), "a field change persists");
assert(approx(fetched.translation.y, 2.0), "other fields are unchanged");
let one = 1.0;
assert(approx_vec3(fetched.forward.call(), Vec3.new_.call(-one.sin(), 0.0, -one.cos())), "a rotation persists");
