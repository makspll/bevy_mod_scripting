local Vec3 = import_namespace("math").Vec3
local a = Vec3.new(1.0, 2.0, 3.0)
local b = Vec3.new(4.0, 5.0, 6.0)

function bench()
    local c = a + b
end
