local Vec2 = import_namespace("math").Vec2
local Vec3 = import_namespace("math").Vec3
local Vec4 = import_namespace("math").Vec4
local DVec2 = import_namespace("math").DVec2
local DVec3 = import_namespace("math").DVec3
local DVec4 = import_namespace("math").DVec4
local IVec2 = import_namespace("math").IVec2
local IVec3 = import_namespace("math").IVec3
local IVec4 = import_namespace("math").IVec4
local UVec2 = import_namespace("math").UVec2
local UVec3 = import_namespace("math").UVec3
local UVec4 = import_namespace("math").UVec4
local function approx(a, b)
    return math.abs(a - b) < 0.0001
end

-- constructors
local v2 = Vec2.new(1, 2)
assert(v2.x == 1 and v2.y == 2, "vec2 new")
local v3 = Vec3.new(1, 2, 3)
assert(v3.x == 1 and v3.y == 2 and v3.z == 3, "vec3 new")
local v4 = Vec4.new(1, 2, 3, 4)
assert(v4.x == 1 and v4.y == 2 and v4.z == 3 and v4.w == 4, "vec4 new")

local dv2 = DVec2.new(1, 2)
assert(dv2.x == 1 and dv2.y == 2, "dvec2 new")
local dv3 = DVec3.new(1, 2, 3)
assert(dv3.x == 1 and dv3.y == 2 and dv3.z == 3, "dvec3 new")
local dv4 = DVec4.new(1, 2, 3, 4)
assert(dv4.x == 1 and dv4.y == 2 and dv4.z == 3 and dv4.w == 4, "dvec4 new")

local iv2 = IVec2.new(1, 2)
assert(iv2.x == 1 and iv2.y == 2, "ivec2 new")
local iv3 = IVec3.new(1, 2, 3)
assert(iv3.x == 1 and iv3.y == 2 and iv3.z == 3, "ivec3 new")
local iv4 = IVec4.new(1, 2, 3, 4)
assert(iv4.x == 1 and iv4.y == 2 and iv4.z == 3 and iv4.w == 4, "ivec4 new")

local uv2 = UVec2.new(1, 2)
assert(uv2.x == 1 and uv2.y == 2, "uvec2 new")
local uv3 = UVec3.new(1, 2, 3)
assert(uv3.x == 1 and uv3.y == 2 and uv3.z == 3, "uvec3 new")
local uv4 = UVec4.new(1, 2, 3, 4)
assert(uv4.x == 1 and uv4.y == 2 and uv4.z == 3 and uv4.w == 4, "uvec4 new")

assert(Vec2.splat(2) == Vec2.new(2, 2), "vec2 splat")
assert(Vec3.splat(2) == Vec3.new(2, 2, 2), "vec3 splat")
assert(Vec4.splat(2) == Vec4.new(2, 2, 2, 2), "vec4 splat")

assert(DVec2.splat(2) == DVec2.new(2, 2), "dvec2 splat")
assert(DVec3.splat(2) == DVec3.new(2, 2, 2), "dvec3 splat")
assert(DVec4.splat(2) == DVec4.new(2, 2, 2, 2), "dvec4 splat")

assert(IVec2.splat(2) == IVec2.new(2, 2), "ivec2 splat")
assert(IVec3.splat(2) == IVec3.new(2, 2, 2), "ivec3 splat")
assert(IVec4.splat(2) == IVec4.new(2, 2, 2, 2), "ivec4 splat")

assert(UVec2.splat(2) == UVec2.new(2, 2), "uvec2 splat")
assert(UVec3.splat(2) == UVec3.new(2, 2, 2), "uvec3 splat")
assert(UVec4.splat(2) == UVec4.new(2, 2, 2, 2), "uvec4 splat")

assert(Vec2.zero() == Vec2.splat(0), "vec2 zero")
assert(Vec3.zero() == Vec3.splat(0), "vec3 zero")
assert(Vec4.zero() == Vec4.splat(0), "vec4 zero")

assert(DVec2.zero() == DVec2.splat(0), "dvec2 zero")
assert(DVec3.zero() == DVec3.splat(0), "dvec3 zero")
assert(DVec4.zero() == DVec4.splat(0), "dvec4 zero")

assert(IVec2.zero() == IVec2.splat(0), "ivec2 zero")
assert(IVec3.zero() == IVec3.splat(0), "ivec3 zero")
assert(IVec4.zero() == IVec4.splat(0), "ivec4 zero")

assert(UVec2.zero() == UVec2.splat(0), "uvec2 zero")
assert(UVec3.zero() == UVec3.splat(0), "uvec3 zero")
assert(UVec4.zero() == UVec4.splat(0), "uvec4 zero")

assert(Vec2.one() == Vec2.splat(1), "vec2 one")
assert(Vec3.one() == Vec3.splat(1), "vec3 one")
assert(Vec4.one() == Vec4.splat(1), "vec4 one")

assert(DVec2.one() == DVec2.splat(1), "dvec2 one")
assert(DVec3.one() == DVec3.splat(1), "dvec3 one")
assert(DVec4.one() == DVec4.splat(1), "dvec4 one")

assert(IVec2.one() == IVec2.splat(1), "ivec2 one")
assert(IVec3.one() == IVec3.splat(1), "ivec3 one")
assert(IVec4.one() == IVec4.splat(1), "ivec4 one")

assert(UVec2.one() == UVec2.splat(1), "uvec2 one")
assert(UVec3.one() == UVec3.splat(1), "uvec3 one")
assert(UVec4.one() == UVec4.splat(1), "uvec4 one")

assert(Vec2.unit_x() == Vec2.new(1, 0), "vec2 unit_x")
assert(Vec2.unit_y() == Vec2.new(0, 1), "vec2 unit_y")
assert(Vec3.unit_x() == Vec3.new(1, 0, 0), "vec3 unit_x")
assert(Vec3.unit_y() == Vec3.new(0, 1, 0), "vec3 unit_y")
assert(Vec3.unit_z() == Vec3.new(0, 0, 1), "vec3 unit_z")
assert(Vec4.unit_x() == Vec4.new(1, 0, 0, 0), "vec4 unit_x")
assert(Vec4.unit_y() == Vec4.new(0, 1, 0, 0), "vec4 unit_y")
assert(Vec4.unit_z() == Vec4.new(0, 0, 1, 0), "vec4 unit_z")
assert(Vec4.unit_w() == Vec4.new(0, 0, 0, 1), "vec4 unit_w")

assert(DVec2.unit_x() == DVec2.new(1, 0), "dvec2 unit_x")
assert(DVec2.unit_y() == DVec2.new(0, 1), "dvec2 unit_y")
assert(DVec3.unit_x() == DVec3.new(1, 0, 0), "dvec3 unit_x")
assert(DVec3.unit_y() == DVec3.new(0, 1, 0), "dvec3 unit_y")
assert(DVec3.unit_z() == DVec3.new(0, 0, 1), "dvec3 unit_z")
assert(DVec4.unit_x() == DVec4.new(1, 0, 0, 0), "dvec4 unit_x")
assert(DVec4.unit_y() == DVec4.new(0, 1, 0, 0), "dvec4 unit_y")
assert(DVec4.unit_z() == DVec4.new(0, 0, 1, 0), "dvec4 unit_z")
assert(DVec4.unit_w() == DVec4.new(0, 0, 0, 1), "dvec4 unit_w")

assert(IVec2.unit_x() == IVec2.new(1, 0), "ivec2 unit_x")
assert(IVec2.unit_y() == IVec2.new(0, 1), "ivec2 unit_y")
assert(IVec3.unit_x() == IVec3.new(1, 0, 0), "ivec3 unit_x")
assert(IVec3.unit_y() == IVec3.new(0, 1, 0), "ivec3 unit_y")
assert(IVec3.unit_z() == IVec3.new(0, 0, 1), "ivec3 unit_z")
assert(IVec4.unit_x() == IVec4.new(1, 0, 0, 0), "ivec4 unit_x")
assert(IVec4.unit_y() == IVec4.new(0, 1, 0, 0), "ivec4 unit_y")
assert(IVec4.unit_z() == IVec4.new(0, 0, 1, 0), "ivec4 unit_z")
assert(IVec4.unit_w() == IVec4.new(0, 0, 0, 1), "ivec4 unit_w")

assert(UVec2.unit_x() == UVec2.new(1, 0), "uvec2 unit_x")
assert(UVec2.unit_y() == UVec2.new(0, 1), "uvec2 unit_y")
assert(UVec3.unit_x() == UVec3.new(1, 0, 0), "uvec3 unit_x")
assert(UVec3.unit_y() == UVec3.new(0, 1, 0), "uvec3 unit_y")
assert(UVec3.unit_z() == UVec3.new(0, 0, 1), "uvec3 unit_z")
assert(UVec4.unit_x() == UVec4.new(1, 0, 0, 0), "uvec4 unit_x")
assert(UVec4.unit_y() == UVec4.new(0, 1, 0, 0), "uvec4 unit_y")
assert(UVec4.unit_z() == UVec4.new(0, 0, 1, 0), "uvec4 unit_z")
assert(UVec4.unit_w() == UVec4.new(0, 0, 0, 1), "uvec4 unit_w")

-- types are tables which keep the functions found in them, read-only
assert(type(Vec3) == "table" and getmetatable(Vec3) == false and Vec3.new == Vec3.new, "a type keeps its functions")
---@diagnostic disable-next-line: assign-type-mismatch
assert(not pcall(function() Vec3.new = 1 end) and Vec3.new(1, 2, 3) == v3, "types are read-only")

-- operators, with vectors and numbers
assert(v3 + v3 == Vec3.new(2, 4, 6), "vec3 add vector")
assert(v3 + 1 == Vec3.new(2, 3, 4), "vec3 add number")
assert(v3 - 1 == Vec3.new(0, 1, 2), "vec3 sub number")
assert(v3 * 2 == Vec3.new(2, 4, 6), "vec3 mul number")
assert(v3 * v3 == Vec3.new(1, 4, 9), "vec3 mul vector")
assert(v3 / 2 == Vec3.new(0.5, 1, 1.5), "vec3 div number")
assert(v3 % 2 == Vec3.new(1, 0, 1), "vec3 rem number")
assert(-v3 == Vec3.new(-1, -2, -3), "vec3 neg")
assert(v3 ~= Vec3.zero(), "vec3 not eq")

assert(dv3 + dv3 == DVec3.new(2, 4, 6), "dvec3 add vector")
assert(dv3 + 1 == DVec3.new(2, 3, 4), "dvec3 add number")
assert(dv3 - 1 == DVec3.new(0, 1, 2), "dvec3 sub number")
assert(dv3 * 2 == DVec3.new(2, 4, 6), "dvec3 mul number")
assert(dv3 * dv3 == DVec3.new(1, 4, 9), "dvec3 mul vector")
assert(dv3 / 2 == DVec3.new(0.5, 1, 1.5), "dvec3 div number")
assert(dv3 % 2 == DVec3.new(1, 0, 1), "dvec3 rem number")
assert(-dv3 == DVec3.new(-1, -2, -3), "dvec3 neg")
assert(dv3 ~= DVec3.zero(), "dvec3 not eq")

assert(iv3 + iv3 == IVec3.new(2, 4, 6), "ivec3 add vector")
assert(iv3 + 1 == IVec3.new(2, 3, 4), "ivec3 add number")
assert(iv3 - 1 == IVec3.new(0, 1, 2), "ivec3 sub number")
assert(iv3 * 2 == IVec3.new(2, 4, 6), "ivec3 mul number")
assert(iv3 * iv3 == IVec3.new(1, 4, 9), "ivec3 mul vector")
assert(iv3 / 2 == IVec3.new(0, 1, 1), "ivec3 div number")
assert(iv3 % 2 == IVec3.new(1, 0, 1), "ivec3 rem number")
assert(-iv3 == IVec3.new(-1, -2, -3), "ivec3 neg")
assert(iv3 ~= IVec3.zero(), "ivec3 not eq")

assert(uv3 + uv3 == UVec3.new(2, 4, 6), "uvec3 add vector")
assert(uv3 + 1 == UVec3.new(2, 3, 4), "uvec3 add number")
assert(uv3 - 1 == UVec3.new(0, 1, 2), "uvec3 sub number")
assert(uv3 * 2 == UVec3.new(2, 4, 6), "uvec3 mul number")
assert(uv3 * uv3 == UVec3.new(1, 4, 9), "uvec3 mul vector")
assert(uv3 / 2 == UVec3.new(0, 1, 1), "uvec3 div number")
assert(uv3 % 2 == UVec3.new(1, 0, 1), "uvec3 rem number")
assert(uv3 ~= UVec3.zero(), "uvec3 not eq")

-- lengths and products
assert(approx(Vec2.new(3, 4):length(), 5), "vec2 length")
assert(approx(DVec2.new(3, 4):length(), 5), "dvec2 length")
assert(Vec2.new(3, 4):length_squared() == 25, "vec2 length_squared")
assert(DVec2.new(3, 4):length_squared() == 25, "dvec2 length_squared")
assert(IVec2.new(3, 4):length_squared() == 25, "ivec2 length_squared")
assert(UVec2.new(3, 4):length_squared() == 25, "uvec2 length_squared")

assert(approx(Vec3.zero():distance(Vec3.new(0, 3, 4)), 5), "vec3 distance")
assert(approx(DVec3.zero():distance(DVec3.new(0, 3, 4)), 5), "dvec3 distance")
assert(v3:dot(v3) == 14, "vec3 dot")
assert(dv3:dot(dv3) == 14, "dvec3 dot")
assert(Vec3.unit_x():cross(Vec3.unit_y()) == Vec3.unit_z(), "vec3 cross")
assert(DVec3.unit_x():cross(DVec3.unit_y()) == DVec3.unit_z(), "dvec3 cross")
assert(approx(Vec3.unit_x():angle_between(Vec3.unit_y()), math.pi / 2), "vec3 angle_between")
assert(approx(DVec3.unit_x():angle_between(DVec3.unit_y()), math.pi / 2), "dvec3 angle_between")

-- normalization
assert(approx(v3:normalize():length(), 1), "normalize")
local normalized, err = pcall(function() return Vec3.zero():normalize() end)
assert(not normalized and tostring(err):match("cannot normalize") and tostring(err):match("at line: [1-9]"),
    "normalize errors on zero, at the line of the script")
assert(Vec3.zero():normalize_or_zero() == Vec3.zero(), "normalize_or_zero")
assert(Vec3.zero():try_normalize() == nil, "try_normalize")

-- component wise
assert(Vec3.zero():lerp(Vec3.new(2, 2, 2), 0.5) == Vec3.one(), "lerp")
assert(Vec3.zero():move_towards(Vec3.new(10, 0, 0), 2) == Vec3.new(2, 0, 0), "move_towards")
assert((-v3):abs() == v3, "abs")
assert(Vec2.new(1.5, -1.5):floor() == Vec2.new(1, -2), "floor")
assert(Vec2.new(1.5, -1.5):ceil() == Vec2.new(2, -1), "ceil")
assert(v3:min(Vec3.splat(2)) == Vec3.new(1, 2, 2), "min")
assert(v3:max(Vec3.splat(2)) == Vec3.new(2, 2, 3), "max")
assert(v3:clamp(Vec3.splat(2), Vec3.splat(2)) == Vec3.splat(2), "clamp")
assert(approx(Vec3.new(10, 0, 0):clamp_length_max(2):length(), 2), "clamp_length_max")
assert(v3:min_element() == 1 and v3:max_element() == 3, "min/max element")
local array = v3:to_array()
assert(#array == 3 and array[3] == 3, "to_array")

-- dimension specific
assert(v2:extend(3) == v3, "vec2 extend")
assert(v3:extend(4) == v4, "vec3 extend")
assert(v3:truncate() == v2, "vec3 truncate")
assert(v4:truncate() == v3, "vec4 truncate")

assert(dv2:extend(3) == dv3, "dvec2 extend")
assert(dv3:extend(4) == dv4, "dvec3 extend")
assert(dv3:truncate() == dv2, "dvec3 truncate")
assert(dv4:truncate() == dv3, "dvec4 truncate")

assert(iv2:extend(3) == iv3, "ivec2 extend")
assert(iv3:extend(4) == iv4, "ivec3 extend")
assert(iv3:truncate() == iv2, "ivec3 truncate")
assert(iv4:truncate() == iv3, "ivec4 truncate")

assert(uv2:extend(3) == uv3, "uvec2 extend")
assert(uv3:extend(4) == uv4, "uvec3 extend")
assert(uv3:truncate() == uv2, "uvec3 truncate")
assert(uv4:truncate() == uv3, "uvec4 truncate")

assert(Vec2.unit_x():perp() == Vec2.unit_y(), "perp")
assert(approx(Vec2.from_angle(0):to_angle(), 0), "from_angle/to_angle")

assert(v2:as_ivec2() == iv2, "vec2 as_ivec2")
assert(v2:as_uvec2() == uv2, "vec2 as_uvec2")
assert(v2:as_dvec2() == dv2, "vec2 as_dvec2")
assert(v3:as_ivec3() == iv3, "vec3 as_ivec3")
assert(v3:as_uvec3() == uv3, "vec3 as_uvec3")
assert(v3:as_dvec3() == dv3, "vec3 as_dvec3")
assert(v4:as_ivec4() == iv4, "vec4 as_ivec3")
assert(v4:as_uvec4() == uv4, "vec4 as_uvec3")
assert(v4:as_dvec4() == dv4, "vec4 as_dvec3")

assert(dv2:as_ivec2() == iv2, "dvec2 as_ivec2")
assert(dv2:as_uvec2() == uv2, "dvec2 as_uvec2")
assert(dv2:as_vec2() == v2, "dvec2 as_vec2")
assert(dv3:as_ivec3() == iv3, "dvec3 as_ivec3")
assert(dv3:as_uvec3() == uv3, "dvec3 as_uvec3")
assert(dv3:as_vec3() == v3, "dvec3 as_vec3")
assert(dv4:as_ivec4() == iv4, "dvec4 as_ivec3")
assert(dv4:as_uvec4() == uv4, "dvec4 as_uvec3")
assert(dv4:as_vec4() == v4, "dvec4 as_vec3")

assert(iv2:as_uvec2() == uv2, "ivec2 as_uvec2")
assert(iv2:as_vec2() == v2, "ivec2 as_vec2")
assert(iv2:as_dvec2() == dv2, "ivec2 as_dvec2")
assert(iv3:as_uvec3() == uv3, "ivec3 as_uvec3")
assert(iv3:as_vec3() == v3, "ivec3 as_vec3")
assert(iv3:as_dvec3() == dv3, "ivec3 as_dvec3")
assert(iv4:as_uvec4() == uv4, "ivec4 as_uvec4")
assert(iv4:as_vec4() == v4, "ivec4 as_vec3")
assert(iv4:as_dvec4() == dv4, "ivec4 as_dvec3")

assert(uv2:as_ivec2() == iv2, "uvec2 as_ivec2")
assert(uv2:as_vec2() == v2, "uvec2 as_vec2")
assert(uv2:as_dvec2() == dv2, "uvec2 as_dvec2")
assert(uv3:as_ivec3() == iv3, "uvec3 as_ivec3")
assert(uv3:as_vec3() == v3, "uvec3 as_vec3")
assert(uv3:as_dvec3() == dv3, "uvec3 as_dvec3")
assert(uv4:as_ivec4() == iv4, "uvec4 as_ivec4")
assert(uv4:as_vec4() == v4, "uvec4 as_vec3")
assert(uv4:as_dvec4() == dv4, "uvec4 as_dvec3")

assert(v3:to_vec3a():to_vec3() == v3, "vec3a round trip")

-- Vec3 is stored natively, it mixes with references to vectors in the world
local w = Vec3.new(1, 2, 3)
w.y = 5
assert(w.y == 5 and v3.y == 2, "set a field of a native value")
---@diagnostic disable-next-line: assign-type-mismatch
assert(not pcall(function() w.y = "a" end), "fields keep their type")
---@diagnostic disable-next-line: inject-field
assert(not pcall(function() w.w = 1 end), "unknown fields can't be set")
local Transform = import_namespace("transform").Transform
local t = Transform.from_translation(w)
assert(t.translation == w and w == t.translation, "compared with a reference")
assert(w:dot(t.translation) == 35, "a reference as an argument")
assert(t.translation + w == w * 2 and 2 * t.translation == w * 2, "operators of references, reflected too")
assert(-t.translation == -w, "unary operators of references")
t.translation = Vec3.new(7, 8, 9)
assert(t.translation.x == 7, "assigned to a field")
assert(tostring(w) == "Vec3 { x: 1.0, y: 5.0, z: 3.0 }", "display")
-- LuaJIT: Vec3 values are cdata, whose metatable is hidden as "ffi"
local hidden = getmetatable(w)
assert(hidden == false or hidden == "ffi", "the metatable of native values is hidden")
---@diagnostic disable-next-line: param-type-mismatch
local function fail() w:dot("a") end
local _, outside = pcall(fail)
local _, inside = coroutine.resume(coroutine.create(fail))
local at = tostring(outside):match("at line: %d+")
assert(at and tostring(inside):match("at line: %d+") == at, "errors in coroutines point at their line")

-- a finalizer marked before a native value which needs dropping runs after it: the value it brings back was dropped,
-- using it errors
local world = import_namespace("bms").world
local label = world._make_native_test_value("label")
assert(label.label == "label" and getmetatable(label) == false, "a native value which needs dropping")
-- (tables have no finalizers on Lua 5.1 and LuaJIT)
if not rawget(_G, "jit") then
    local resurrected
    do
        local holder = setmetatable({}, { __gc = function(self) resurrected = self.v end })
        holder.v = world._make_native_test_value("collected")
    end
    collectgarbage()
    collectgarbage()
    assert(resurrected ~= nil, "the finalizer ran")
    assert(not pcall(function() return resurrected.label end), "a dropped native value can't be used")
end
