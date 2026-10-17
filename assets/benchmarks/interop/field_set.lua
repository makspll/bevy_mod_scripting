local Vec3 = import_namespace("math").Vec3
local v = Vec3.new(1.0, 2.0, 3.0)

function bench()
    v.x = 2.0
end
