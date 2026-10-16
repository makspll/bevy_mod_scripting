-- the test app may have no gizmo plugin: every function either draws, or fails with the same error

local function draws(f)
    local ok, err = pcall(f)
    if not ok then
        assert(string.find(tostring(err), "gizmos are not available", 1, true) ~= nil, "unexpected error: " .. tostring(err))
    end
    return ok
end

assert(Gizmos ~= nil, "Gizmos exists")

local calls = {
    line = function() Gizmos.line(Vec3.zero(), Vec3.one(), Color.white()) end,
    line_2d = function() Gizmos.line_2d(Vec2.zero(), Vec2.one(), Color.white()) end,
    ray = function() Gizmos.ray(Vec3.zero(), Vec3.new(1, 0, 0), Color.white()) end,
    ray_2d = function() Gizmos.ray_2d(Vec2.zero(), Vec2.new(1, 0), Color.white()) end,
    arrow = function() Gizmos.arrow(Vec3.zero(), Vec3.one(), Color.white()) end,
    arrow_2d = function() Gizmos.arrow_2d(Vec2.zero(), Vec2.one(), Color.white()) end,
    circle = function() Gizmos.circle(Vec3.zero(), 1.0, Color.white()) end,
    circle_2d = function() Gizmos.circle_2d(Vec2.zero(), 1.0, Color.white()) end,
    sphere = function() Gizmos.sphere(Vec3.zero(), 1.0, Color.white()) end,
    rect = function() Gizmos.rect(Vec3.zero(), Vec2.one(), Color.white()) end,
    rect_2d = function() Gizmos.rect_2d(Vec2.zero(), Vec2.one(), Color.white()) end,
    cuboid = function() Gizmos.cuboid(Transform.from_xyz(1, 2, 3), Color.white()) end,
    axes = function() Gizmos.axes(Transform.identity(), 1.0) end,
    cross = function() Gizmos.cross(Vec3.zero(), 0.5, Color.white()) end,
    linestrip = function() Gizmos.linestrip({ Vec3.zero(), Vec3.one(), Vec3.new(2, 0, 0) }, Color.white()) end,
    linestrip_2d = function() Gizmos.linestrip_2d({ Vec2.zero(), Vec2.one() }, Color.white()) end,
}

local available = draws(calls.line)
for name, call in pairs(calls) do
    assert(draws(call) == available, "gizmos " .. name .. " succeeds differently to gizmos line")
end
