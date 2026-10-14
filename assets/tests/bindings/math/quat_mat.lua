local function approx(a, b)
    return math.abs(a - b) < 0.0001
end

local function approx_vec3(a, b)
    return approx(a.x, b.x) and approx(a.y, b.y) and approx(a.z, b.z)
end

-- quaternions
local q = Quat.from_rotation_y(math.pi / 2)
assert(approx_vec3(q * Vec3.unit_x(), Vec3.new(0, 0, -1)), "rotate a vector")
assert(approx_vec3(q:mul_vec3(Vec3.unit_x()), Vec3.new(0, 0, -1)), "mul_vec3")
assert(approx_vec3((q * q) * Vec3.unit_x(), Vec3.new(-1, 0, 0)), "compose rotations")
assert(approx_vec3((q * q:inverse()) * Vec3.unit_x(), Vec3.unit_x()), "inverse")
assert(Quat.identity() * Vec3.unit_x() == Vec3.unit_x(), "identity")
assert(approx(q:angle(), math.pi / 2), "angle")
assert(approx_vec3(q:axis(), Vec3.unit_y()), "axis")
assert(approx(Quat.identity():angle_between(q), math.pi / 2), "angle_between")
assert(approx_vec3(Quat.from_axis_angle(Vec3.new(0, 2, 0), math.pi / 2) * Vec3.unit_x(), Vec3.new(0, 0, -1)),
    "from_axis_angle normalizes the axis")

-- enum variants can be passed directly where a value is expected
local euler = Quat.from_euler(EulerRot.YXZ, math.pi / 2, 0, 0)
assert(approx_vec3(euler * Vec3.unit_x(), Vec3.new(0, 0, -1)), "from_euler")
local angles = euler:to_euler(EulerRot.YXZ)
assert(approx(angles[1], math.pi / 2), "to_euler")

local half = Quat.identity():slerp(q, 0.5)
assert(approx(half:angle(), math.pi / 4), "slerp")

-- matrices
local m = Mat3.from_cols(Vec3.new(1, 0, 0), Vec3.new(0, 2, 0), Vec3.new(0, 0, 3))
assert(m * Vec3.one() == Vec3.new(1, 2, 3), "mat3 * vec3")
assert(m:determinant() == 6, "determinant")
assert(m:transpose() == m, "transpose")
assert(m * m:inverse() == Mat3.identity(), "inverse")
assert(m:col(1) == Vec3.new(0, 2, 0), "col")

local t = Mat4.from_translation(Vec3.new(1, 2, 3))
assert(t:transform_point3(Vec3.zero()) == Vec3.new(1, 2, 3), "transform_point3")
assert(t:transform_vector3(Vec3.unit_x()) == Vec3.unit_x(), "transform_vector3 ignores translation")
local srt = Mat4.from_scale_rotation_translation(Vec3.one(), Quat.identity(), Vec3.new(1, 2, 3))
assert(srt == t, "from_scale_rotation_translation")

-- integer vectors
local i = IVec2.new(3, -4)
assert(i + 1 == IVec2.new(4, -3), "ivec add")
assert(i:abs() == IVec2.new(3, 4), "ivec abs")
assert(i:as_vec2() == Vec2.new(3, -4), "ivec as_vec2")
assert(not pcall(function() return i / 0 end), "division by zero errors")
assert(UVec3.new(1, 2, 3):dot(UVec3.one()) == 6, "uvec dot")
