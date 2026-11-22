local DVec3 = import_namespace("math").DVec3
local IVec3 = import_namespace("math").IVec3
local Vec2 = import_namespace("math").Vec2
local Vec3 = import_namespace("math").Vec3
local Vec4 = import_namespace("math").Vec4
local function approx(a, b)
    return math.abs(a - b) < 0.0001
end

-- constructors
local v = Vec3.new(1, 2, 3)
assert(v.x == 1 and v.y == 2 and v.z == 3, "new")
assert(Vec3.splat(2) == Vec3.new(2, 2, 2), "splat")
assert(Vec3.zero() == Vec3.new(0, 0, 0), "zero")
assert(Vec3.one() == Vec3.new(1, 1, 1), "one")
assert(Vec3.unit_y() == Vec3.new(0, 1, 0), "unit_y")
assert(Vec2.new(1, 2).y == 2, "vec2 new")
assert(Vec4.new(1, 2, 3, 4).w == 4, "vec4 new")
-- types are tables which keep the functions found in them, read-only
assert(type(Vec3) == "table" and getmetatable(Vec3) == false and Vec3.new == Vec3.new, "a type keeps its functions")
---@diagnostic disable-next-line: assign-type-mismatch
assert(not pcall(function() Vec3.new = 1 end) and Vec3.new(1, 2, 3) == v, "types are read-only")

-- operators, with vectors and numbers
assert(v + v == Vec3.new(2, 4, 6), "add vector")
assert(v + 1 == Vec3.new(2, 3, 4), "add number")
assert(v - 1 == Vec3.new(0, 1, 2), "sub")
assert(v * 2 == Vec3.new(2, 4, 6), "mul number")
assert(v * v == Vec3.new(1, 4, 9), "mul vector")
assert(v / 2 == Vec3.new(0.5, 1, 1.5), "div")
assert(v % 2 == Vec3.new(1, 0, 1), "rem")
assert(-v == Vec3.new(-1, -2, -3), "neg")
assert(v ~= Vec3.zero(), "not eq")

-- lengths and products
assert(approx(Vec2.new(3, 4):length(), 5), "length")
assert(Vec2.new(3, 4):length_squared() == 25, "length_squared")
assert(approx(Vec3.zero():distance(Vec3.new(0, 3, 4)), 5), "distance")
assert(v:dot(v) == 14, "dot")
assert(Vec3.unit_x():cross(Vec3.unit_y()) == Vec3.unit_z(), "cross")
assert(approx(Vec3.unit_x():angle_between(Vec3.unit_y()), math.pi / 2), "angle_between")

-- normalization
assert(approx(v:normalize():length(), 1), "normalize")
local normalized, err = pcall(function() return Vec3.zero():normalize() end)
assert(not normalized and tostring(err):match("cannot normalize") and tostring(err):match("at line: [1-9]"),
    "normalize errors on zero, at the line of the script")
assert(Vec3.zero():normalize_or_zero() == Vec3.zero(), "normalize_or_zero")
assert(Vec3.zero():try_normalize() == nil, "try_normalize")

-- component wise
assert(Vec3.zero():lerp(Vec3.new(2, 2, 2), 0.5) == Vec3.one(), "lerp")
assert(Vec3.zero():move_towards(Vec3.new(10, 0, 0), 2) == Vec3.new(2, 0, 0), "move_towards")
assert((-v):abs() == v, "abs")
assert(Vec2.new(1.5, -1.5):floor() == Vec2.new(1, -2), "floor")
assert(Vec2.new(1.5, -1.5):ceil() == Vec2.new(2, -1), "ceil")
assert(v:min(Vec3.splat(2)) == Vec3.new(1, 2, 2), "min")
assert(v:max(Vec3.splat(2)) == Vec3.new(2, 2, 3), "max")
assert(v:clamp(Vec3.splat(2), Vec3.splat(2)) == Vec3.splat(2), "clamp")
assert(approx(Vec3.new(10, 0, 0):clamp_length_max(2):length(), 2), "clamp_length_max")
assert(v:min_element() == 1 and v:max_element() == 3, "min/max element")
local array = v:to_array()
assert(#array == 3 and array[3] == 3, "to_array")

-- dimension specific
assert(Vec2.new(1, 2):extend(3) == v, "extend")
assert(v:truncate() == Vec2.new(1, 2), "truncate")
assert(Vec2.unit_x():perp() == Vec2.unit_y(), "perp")
assert(approx(Vec2.from_angle(0):to_angle(), 0), "from_angle/to_angle")
assert(v:as_ivec3() == IVec3.new(1, 2, 3), "as_ivec3")
assert(v:to_vec3a():to_vec3() == v, "vec3a round trip")
assert(DVec3.new(1, 2, 3):as_vec3() == v, "dvec3")

-- Vec3 is stored natively, it mixes with references to vectors in the world
local w = Vec3.new(1, 2, 3)
w.y = 5
assert(w.y == 5 and v.y == 2, "set a field of a native value")
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
