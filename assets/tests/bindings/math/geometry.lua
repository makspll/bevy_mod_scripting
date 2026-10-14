local function approx(a, b)
    return math.abs(a - b) < 0.0001
end

-- directions
local dir = Dir3.new(Vec3.new(0, 0, 5))
assert(dir:as_vec3() == Vec3.unit_z(), "dir is normalized")
assert(Dir3.new(Vec3.zero()) == nil, "zero vector is not a direction")
assert(Dir3.y():as_vec3() == Vec3.unit_y(), "dir constant")
assert(dir * 2 == Vec3.new(0, 0, 2), "scaled direction")

-- 2d rotations
local rot = Rot2.degrees(90)
assert(approx(rot:as_radians(), math.pi / 2), "rot2 degrees")
local rotated = rot * Vec2.unit_x()
assert(approx(rotated.x, 0) and approx(rotated.y, 1), "rot2 * vec2")

-- rectangles
local rect = Rect.new(0, 0, 4, 2)
assert(rect:width() == 4 and rect:height() == 2, "rect size")
assert(rect:center() == Vec2.new(2, 1), "rect center")
assert(rect:contains(Vec2.new(1, 1)), "rect contains")
assert(not rect:contains(Vec2.new(5, 1)), "rect does not contain")
assert(rect:intersect(Rect.new(2, 0, 6, 2)):width() == 2, "rect intersect")

-- rays
local ray = Ray3d.new(Vec3.new(0, 10, 0), Vec3.new(0, -1, 0))
assert(ray:get_point(4) == Vec3.new(0, 6, 0), "ray point")
assert(approx(ray:intersect_plane(Vec3.zero(), Vec3.unit_y()), 10), "ray hits plane")
assert(ray:intersect_plane(Vec3.new(0, 20, 0), Vec3.unit_y()) == nil, "ray misses plane behind it")
assert(not pcall(function() return Ray3d.new(Vec3.zero(), Vec3.zero()) end), "zero direction errors")

-- shapes
assert(Sphere.new(2).radius == 2, "sphere")
assert(Cuboid.new(2, 4, 6).half_size == Vec3.new(1, 2, 3), "cuboid")
assert(Rectangle.new(2, 4).half_size == Vec2.new(1, 2), "rectangle")

-- easing, variants can be passed directly
assert(EaseFunction.sample(EaseFunction.Linear, 0.25) == 0.25, "linear easing")
assert(approx(EaseFunction.sample(EaseFunction.QuadraticIn, 0.5), 0.25), "quadratic easing")
assert(EaseFunction.sample(EaseFunction.QuadraticIn, 2) == 1, "easing is clamped")
