local EulerRot = import_namespace("math").EulerRot
local IVec2 = import_namespace("math").IVec2
local Mat2 = import_namespace("math").Mat2
local Mat3 = import_namespace("math").Mat3
local Mat4 = import_namespace("math").Mat4
local Quat = import_namespace("math").Quat
local DQuat = import_namespace("math").DQuat
local UVec3 = import_namespace("math").UVec3
local Vec2 = import_namespace("math").Vec2
local Vec3 = import_namespace("math").Vec3
local DVec2 = import_namespace("math").DVec2
local DVec3 = import_namespace("math").DVec3
local Affine2 = import_namespace("math").Affine2
local Affine3 = import_namespace("math").Affine3
local DAffine2 = import_namespace("math").DAffine2
local DAffine3 = import_namespace("math").DAffine3

local function approx(a, b)
    return math.abs(a - b) < 0.0001
end

local function approx_vec3(a, b)
    return approx(a.x, b.x) and approx(a.y, b.y) and approx(a.z, b.z)
end

-- quaternions
local q = Quat.from_rotation_y(math.pi / 2)
assert(approx_vec3(q * Vec3.unit_x(), Vec3.new(0, 0, -1)), "quat rotate a vector")
assert(approx_vec3(q:mul_vec3(Vec3.unit_x()), Vec3.new(0, 0, -1)), "quat mul_vec3")
assert(approx_vec3((q * q) * Vec3.unit_x(), Vec3.new(-1, 0, 0)), "quat compose rotations")
assert(approx_vec3((q * q:inverse()) * Vec3.unit_x(), Vec3.unit_x()), "quat inverse")
assert(Quat.identity() * Vec3.unit_x() == Vec3.unit_x(), "quat identity")
assert(approx(q:angle(), math.pi / 2), "quat angle")
assert(approx_vec3(q:axis(), Vec3.unit_y()), "quat axis")
assert(approx(Quat.identity():angle_between(q), math.pi / 2), "quat angle_between")
assert(approx_vec3(Quat.from_axis_angle(Vec3.new(0, 2, 0), math.pi / 2) * Vec3.unit_x(), Vec3.new(0, 0, -1)),
    "quat from_axis_angle normalizes the axis")

local dq = DQuat.from_rotation_y(math.pi / 2)
assert(approx_vec3(dq * DVec3.unit_x(), DVec3.new(0, 0, -1)), "dquat rotate a vector")
assert(approx_vec3(dq:mul_vec3(DVec3.unit_x()), DVec3.new(0, 0, -1)), "dquat mul_vec3")
assert(approx_vec3((dq * dq) * DVec3.unit_x(), DVec3.new(-1, 0, 0)), "dquat compose rotations")
assert(approx_vec3((dq * dq:inverse()) * DVec3.unit_x(), DVec3.unit_x()), "dquat inverse")
assert(DQuat.identity() * DVec3.unit_x() == DVec3.unit_x(), "dquat identity")
assert(approx(dq:angle(), math.pi / 2), "dquat angle")
assert(approx_vec3(dq:axis(), DVec3.unit_y()), "dquat axis")
assert(approx(DQuat.identity():angle_between(dq), math.pi / 2), "dquat angle_between")
assert(approx_vec3(DQuat.from_axis_angle(DVec3.new(0, 2, 0), math.pi / 2) * DVec3.unit_x(), DVec3.new(0, 0, -1)),
    "dquat from_axis_angle normalizes the axis")

-- enum variants can be passed directly where a value is expected
local euler = Quat.from_euler(EulerRot.YXZ, math.pi / 2, 0, 0)
assert(approx_vec3(euler * Vec3.unit_x(), Vec3.new(0, 0, -1)), "from_euler")
local angles = euler:to_euler(EulerRot.YXZ)
assert(approx(angles[1], math.pi / 2), "to_euler")

local half = Quat.identity():slerp(q, 0.5)
assert(approx(half:angle(), math.pi / 4), "slerp")

-- matrices
local m2 = Mat2.from_cols(Vec2.new(1, 0), Vec2.new(0, 2))
assert(m2 * Vec2.one() == Vec2.new(1, 2), "mat2 * vec2")
assert(m2:determinant() == 2, "mat2 determinant")
assert(m2:transpose() == m2, "mat2 transpose")
assert(m2 * m2:inverse() == Mat2.identity(), "mat2 inverse")
assert(m2:col(1) == Vec2.new(0, 2), "mat2 col")
-- matrices are values: a column is read as a read-only copy and written whole
local n2 = m2:clone()
assert(n2.y_axis == Vec2.new(0, 2) and not pcall(function(c) c.x = 1 end, n2.y_axis), "a column is a read-only copy")
n2.y_axis = Vec2.new(0, 5)
assert(n2:col(1) == Vec2.new(0, 5) and m2:col(1) == Vec2.new(0, 2), "a column is written whole, to the clone only")
---@diagnostic disable-next-line: assign-type-mismatch
assert(not pcall(function() n2.y_axis = 5 end) and n2.y_axis == Vec2.new(0, 5), "a column is written with a vector")
assert(m2 + m2 == m2 * 2, "mat2 + mat2")
assert(m2 - m2 == m2 * 0, "mat2 - mat2")
assert(m2 / 2 == m2 * 0.5, "mat2 / number")
-- a matrix and a vector have different shapes, so adding them is an error rather than a guess
assert(not pcall(function() return m2 + Vec2.one() end), "mat2 + vec2 errors")

local m3 = Mat3.from_cols(Vec3.new(1, 0, 0), Vec3.new(0, 2, 0), Vec3.new(0, 0, 3))
assert(m3 * Vec3.one() == Vec3.new(1, 2, 3), "mat3 * vec3")
assert(m3:determinant() == 6, "mat3 determinant")
assert(m3:transpose() == m3, "mat3 transpose")
assert(m3 * m3:inverse() == Mat3.identity(), "mat3 inverse")
assert(m3:col(1) == Vec3.new(0, 2, 0), "mat3 col")
-- matrices are values: a column is read as a read-only copy and written whole
local n3 = m3:clone()
assert(n3.y_axis == Vec3.new(0, 2, 0) and not pcall(function(c) c.x = 1 end, n3.y_axis), "a column is a read-only copy")
n3.y_axis = Vec3.new(0, 5, 0)
assert(n3:col(1) == Vec3.new(0, 5, 0) and m3:col(1) == Vec3.new(0, 2, 0), "a column is written whole, to the clone only")
---@diagnostic disable-next-line: assign-type-mismatch
assert(not pcall(function() n3.y_axis = 5 end) and n3.y_axis == Vec3.new(0, 5, 0), "a column is written with a vector")
assert(m3 + m3 == m3 * 2, "mat3 + mat3")
assert(m3 - m3 == m3 * 0, "mat3 - mat3")
assert(m3 / 2 == m3 * 0.5, "mat3 / number")
-- a matrix and a vector have different shapes, so adding them is an error rather than a guess
assert(not pcall(function() return m3 + Vec3.one() end), "mat3 + vec3 errors")

assert(m3:to_mat3a():to_mat3() == m3, "mat3a round trip")
assert(Mat4.identity() / 2 == Mat4.identity() * 0.5, "mat4 / number")

local t = Mat4.from_translation(Vec3.new(1, 2, 3))
assert(t:transform_point3(Vec3.zero()) == Vec3.new(1, 2, 3), "transform_point3")
assert(t.w_axis.x == 1 and t.w_axis.z == 3 and t.w_axis.w == 1, "mat4 columns")
assert(t:transform_vector3(Vec3.unit_x()) == Vec3.unit_x(), "transform_vector3 ignores translation")
local srt = Mat4.from_scale_rotation_translation(Vec3.one(), Quat.identity(), Vec3.new(1, 2, 3))
assert(srt == t, "from_scale_rotation_translation")

-- transforms
local a2 = Affine2.from_mat2(m2)
a2.translation = Vec2.one()
assert(a2:transform_point2(Vec2.one()) == Vec2.new(2, 3), "affine2 * point2")
assert(a2:transform_vector2(Vec2.one()) == Vec2.new(1, 2), "affine2 * vec2")
assert(a2 * a2:inverse() == Affine2.identity(), "affine2 inverse")

local a3 = Affine3.from_mat3(m3)
a3.translation = Vec3.one()
assert(a3:transform_point3(Vec3.one()) == Vec3.new(2, 3, 4), "affine3 * point3")
assert(a3:transform_vector3(Vec3.one()) == Vec3.new(1, 2, 3), "affine3 * vec3")
assert(a3 * a3:inverse() == Affine3.identity(), "affine3 inverse")

local da2 = DAffine2.from_mat2(m2:as_dmat2())
da2.translation = DVec2.one()
assert(da2:transform_point2(DVec2.one()) == DVec2.new(2, 3), "daffine2 * dpoint2")
assert(da2:transform_vector2(DVec2.one()) == DVec2.new(1, 2), "daffine2 * dvec2")
assert(da2 * da2:inverse() == DAffine2.identity(), "daffine2 inverse")

local da3 = DAffine3.from_mat3(m3:as_dmat3())
da3.translation = DVec3.one()
assert(da3:transform_point3(DVec3.one()) == DVec3.new(2, 3, 4), "daffine3 * dpoint3")
assert(da3:transform_vector3(DVec3.one()) == DVec3.new(1, 2, 3), "daffine3 * dvec3")
assert(da3 * da3:inverse() == DAffine3.identity(), "daffine3 inverse")

-- integer vectors
local i = IVec2.new(3, -4)
assert(i + 1 == IVec2.new(4, -3), "ivec add")
assert(i:abs() == IVec2.new(3, 4), "ivec abs")
assert(i:as_vec2() == Vec2.new(3, -4), "ivec as_vec2")
assert(not pcall(function() return i / 0 end), "division by zero errors")
assert(UVec3.new(1, 2, 3):dot(UVec3.one()) == 6, "uvec dot")
