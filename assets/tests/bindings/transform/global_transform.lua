local GlobalTransform = import_namespace("transform").GlobalTransform
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
local g = GlobalTransform.identity()
assert(approx_vec3(g:translation(), Vec3.zero()), "identity translation")
assert(g:rotation() == Quat.identity(), "identity rotation")
assert(approx_vec3(g:scale(), Vec3.one()), "identity scale")

local xyz = GlobalTransform.from_xyz(1, 2, 3)
assert(approx_vec3(xyz:translation(), Vec3.new(1, 2, 3)), "from_xyz")
assert(approx_vec3(xyz:compute_transform().translation, Vec3.new(1, 2, 3)), "compute_transform keeps the translation")

-- accessors on a transform that has a rotation and a scale
local posed = GlobalTransform.identity() * Transform.from_scale(Vec3.new(2, 2, 2))
assert(approx_vec3(posed:scale(), Vec3.new(2, 2, 2)), "scale")
local turned = GlobalTransform.identity() * Transform.from_rotation(Quat.from_rotation_y(math.pi / 2))
assert(approx_vec3(turned:rotation() * Vec3.unit_x(), Vec3.new(0, 0, -1)), "rotation")

-- directions
local id = GlobalTransform.identity()
assert(approx_vec3(id:forward(), Vec3.new(0, 0, -1)), "forward")
assert(approx_vec3(id:back(), Vec3.new(0, 0, 1)), "back")
assert(approx_vec3(id:left(), Vec3.new(-1, 0, 0)), "left")
assert(approx_vec3(id:right(), Vec3.new(1, 0, 0)), "right")
assert(approx_vec3(id:up(), Vec3.new(0, 1, 0)), "up")
assert(approx_vec3(id:down(), Vec3.new(0, -1, 0)), "down")
assert(approx_vec3(turned:forward(), Vec3.new(-1, 0, 0)), "directions follow the rotation")

-- points, matrices and conversions
assert(approx_vec3(xyz:transform_point(Vec3.new(1, 0, 0)), Vec3.new(2, 2, 3)), "transform_point")
assert(approx_vec3(xyz:to_matrix():transform_point3(Vec3.zero()), Vec3.new(1, 2, 3)), "to_matrix")

-- a local transform relative to a parent
local child = GlobalTransform.from_xyz(3, 0, 0)
local parent = GlobalTransform.from_xyz(1, 0, 0)
assert(approx_vec3(child:reparented_to(parent).translation, Vec3.new(2, 0, 0)), "reparented_to")

-- composition
local g1 = GlobalTransform.from_xyz(1, 0, 0)
local g2 = GlobalTransform.from_xyz(0, 2, 0)
assert(approx_vec3((g1 * g2):translation(), Vec3.new(1, 2, 0)), "mul with a global transform")
assert(approx_vec3((g1 * Transform.from_xyz(0, 0, 5)):translation(), Vec3.new(1, 0, 5)), "mul with a transform")

-- equality
assert(GlobalTransform.from_xyz(1, 2, 3):eq(xyz), "eq")
assert(GlobalTransform.from_xyz(1, 2, 3) == xyz, "== compares global transforms")
assert(not g1:eq(g2), "eq is false for different transforms")
