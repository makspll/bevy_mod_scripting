local world = import_namespace("bms").world
local Mat4 = import_namespace("math").Mat4
local Quat = import_namespace("math").Quat
local Transform = import_namespace("transform").Transform
local Vec3 = import_namespace("math").Vec3
local function approx(a, b)
    return math.abs(a - b) < 0.0001
end

local function approx_vec3(a, b)
    return approx(a.x, b.x) and approx(a.y, b.y) and approx(a.z, b.z)
end

-- constructors
local t = Transform.identity()
assert(approx_vec3(t.translation, Vec3.zero()), "identity translation")
assert(t.rotation == Quat.identity(), "identity rotation")
assert(approx_vec3(t.scale, Vec3.one()), "identity scale")
assert(t == Transform.identity(), "identity is equal to identity")

local xyz = Transform.from_xyz(1, 2, 3)
assert(approx_vec3(xyz.translation, Vec3.new(1, 2, 3)), "from_xyz")
assert(approx_vec3(Transform.from_translation(Vec3.new(1, 2, 3)).translation, Vec3.new(1, 2, 3)), "from_translation")
assert(Transform.from_rotation(Quat.from_rotation_y(math.pi / 2)).rotation == Quat.from_rotation_y(math.pi / 2),
    "from_rotation")
assert(approx_vec3(Transform.from_scale(Vec3.new(2, 2, 2)).scale, Vec3.new(2, 2, 2)), "from_scale")
local from_matrix = Transform.from_matrix(Mat4.from_translation(Vec3.new(1, 2, 3)))
assert(approx_vec3(from_matrix.translation, Vec3.new(1, 2, 3)), "from_matrix")

-- builders return new transforms and leave the original alone
local moved = xyz:with_translation(Vec3.new(4, 5, 6))
assert(approx_vec3(moved.translation, Vec3.new(4, 5, 6)), "with_translation")
assert(approx_vec3(xyz.translation, Vec3.new(1, 2, 3)), "with_translation keeps the original")
local rotated = xyz:with_rotation(Quat.from_rotation_y(math.pi / 2))
assert(approx_vec3(rotated:forward(), Vec3.new(-1, 0, 0)), "with_rotation")
assert(approx_vec3(xyz:with_scale(Vec3.new(3, 3, 3)).scale, Vec3.new(3, 3, 3)), "with_scale")

local looking = Transform.from_xyz(0, 0, 0):looking_at(Vec3.new(0, 0, -5), Vec3.unit_y())
assert(approx_vec3(looking:forward(), Vec3.new(0, 0, -1)), "looking_at")
local looking_to = Transform.from_xyz(0, 0, 0):looking_to(Vec3.new(1, 0, 0), Vec3.unit_y())
assert(approx_vec3(looking_to:forward(), Vec3.new(1, 0, 0)), "looking_to")
assert(not pcall(function() return Transform.identity():looking_at(Vec3.new(0, 0, -5), Vec3.zero()) end),
    "looking_at errors on a zero up direction")
assert(not pcall(function() return Transform.identity():looking_to(Vec3.zero(), Vec3.unit_y()) end),
    "looking_to errors on a zero direction")

-- in-place rotations
local r = Transform.identity()
r:rotate(Quat.from_rotation_y(math.pi / 2))
assert(approx_vec3(r:forward(), Vec3.new(-1, 0, 0)), "rotate")

r = Transform.identity()
r:rotate_x(math.pi / 2)
assert(approx_vec3(r:up(), Vec3.new(0, 0, 1)), "rotate_x")

r = Transform.identity()
r:rotate_y(math.pi / 2)
assert(approx_vec3(r:forward(), Vec3.new(-1, 0, 0)), "rotate_y")

r = Transform.identity()
r:rotate_z(math.pi / 2)
assert(approx_vec3(r:right(), Vec3.new(0, 1, 0)), "rotate_z")

r = Transform.identity()
r:rotate_axis(Vec3.unit_x(), math.pi / 2)
assert(approx_vec3(r:forward(), Vec3.new(0, 1, 0)), "rotate_axis")
assert(not pcall(function() r:rotate_axis(Vec3.zero(), 1.0) end), "rotate_axis errors on a zero axis")

-- local rotations apply after the current rotation, world rotations before it
r = Transform.from_rotation(Quat.from_rotation_y(math.pi / 2))
r:rotate_local_x(math.pi / 2)
assert(approx_vec3(r:forward(), Vec3.new(0, 1, 0)), "rotate_local_x")

r = Transform.from_rotation(Quat.from_rotation_y(math.pi / 2))
r:rotate_x(math.pi / 2)
assert(approx_vec3(r:forward(), Vec3.new(-1, 0, 0)), "rotate_x after another rotation")

r = Transform.identity()
r:rotate_local(Quat.from_rotation_x(math.pi / 2))
assert(approx_vec3(r:forward(), Vec3.new(0, 1, 0)), "rotate_local")

r = Transform.identity()
r:rotate_local_y(math.pi / 2)
assert(approx_vec3(r:forward(), Vec3.new(-1, 0, 0)), "rotate_local_y")

r = Transform.identity()
r:rotate_local_z(math.pi / 2)
assert(approx_vec3(r:right(), Vec3.new(0, 1, 0)), "rotate_local_z")

-- rotations around a point move the translation
r = Transform.from_xyz(1, 0, 0)
r:rotate_around(Vec3.zero(), Quat.from_rotation_y(math.pi / 2))
assert(approx_vec3(r.translation, Vec3.new(0, 0, -1)), "rotate_around moves the translation")
assert(approx_vec3(r:forward(), Vec3.new(-1, 0, 0)), "rotate_around rotates")

r = Transform.from_xyz(1, 0, 0)
r:translate_around(Vec3.zero(), Quat.from_rotation_y(math.pi / 2))
assert(approx_vec3(r.translation, Vec3.new(0, 0, -1)), "translate_around moves the translation")
assert(approx_vec3(r:forward(), Vec3.new(0, 0, -1)), "translate_around keeps the rotation")

-- in-place looking
r = Transform.from_xyz(0, 0, 0)
r:look_at(Vec3.new(1, 0, 0), Vec3.unit_y())
assert(approx_vec3(r:forward(), Vec3.new(1, 0, 0)), "look_at")
r:look_to(Vec3.new(0, 0, 1), Vec3.unit_y())
assert(approx_vec3(r:forward(), Vec3.new(0, 0, 1)), "look_to")
assert(not pcall(function() r:look_at(Vec3.new(1, 0, 0), Vec3.zero()) end), "look_at errors on a zero up direction")
assert(not pcall(function() r:look_to(Vec3.new(1, 0, 0), Vec3.zero()) end), "look_to errors on a zero up direction")

-- directions of the identity transform
local id = Transform.identity()
assert(approx_vec3(id:forward(), Vec3.new(0, 0, -1)), "forward")
assert(approx_vec3(id:back(), Vec3.new(0, 0, 1)), "back")
assert(approx_vec3(id:left(), Vec3.new(-1, 0, 0)), "left")
assert(approx_vec3(id:right(), Vec3.new(1, 0, 0)), "right")
assert(approx_vec3(id:up(), Vec3.new(0, 1, 0)), "up")
assert(approx_vec3(id:down(), Vec3.new(0, -1, 0)), "down")
assert(approx_vec3(id:local_x(), Vec3.unit_x()), "local_x")
assert(approx_vec3(id:local_y(), Vec3.unit_y()), "local_y")
assert(approx_vec3(id:local_z(), Vec3.unit_z()), "local_z")

-- points and composition
local a = Transform.from_translation(Vec3.new(1, 0, 0)):with_scale(Vec3.new(2, 2, 2))
assert(approx_vec3(a:transform_point(Vec3.new(1, 0, 0)), Vec3.new(3, 0, 0)), "transform_point scales then translates")
assert(approx_vec3(a * Vec3.new(1, 0, 0), Vec3.new(3, 0, 0)), "mul by a point transforms it")

local b = Transform.from_translation(Vec3.new(0, 2, 0))
assert(approx_vec3((Transform.from_translation(Vec3.new(1, 0, 0)) * b).translation, Vec3.new(1, 2, 0)), "mul composes")
assert(approx_vec3(Transform.from_translation(Vec3.new(1, 0, 0)):mul_transform(b).translation, Vec3.new(1, 2, 0)),
    "mul_transform composes")

assert(approx(Transform.from_xyz(0, 0, 0):to_matrix():transform_point3(Vec3.zero()).x, 0), "to_matrix")
local matrix_point = Transform.from_xyz(1, 2, 3):to_matrix():transform_point3(Vec3.zero())
assert(approx_vec3(matrix_point, Vec3.new(1, 2, 3)), "to_matrix keeps the translation")

-- equality and finiteness
assert(Transform.from_xyz(0, 0, 0):eq(Transform.identity()), "eq")
assert(not xyz:eq(Transform.identity()), "eq is false for different transforms")
assert(Transform.identity():is_finite(), "is_finite")
assert(not Transform.from_xyz(0 / 0, 0, 0):is_finite(), "NaN is not finite")

-- components referenced from the world: changes made through a reference persist
local transform_type = world.get_type_by_name("bevy_transform::components::transform::Transform")
local entity = world.spawn()
world.insert_component(entity, transform_type, Transform.from_xyz(1, 2, 3))

local reference = world.get_component(entity, transform_type)
reference:rotate_y(1.0)
reference.translation.x = 5

local fetched = world.get_component(entity, transform_type)
assert(approx(fetched.translation.x, 5), "a field change persists")
assert(approx(fetched.translation.y, 2), "other fields are unchanged")
assert(approx_vec3(fetched:forward(), Vec3.new(-math.sin(1.0), 0, -math.cos(1.0))), "a rotation persists")
