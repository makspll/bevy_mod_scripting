-- 100 operations per call: the cost of one is (this - `empty loop`) / 100
local Vec3 = import_namespace("math").Vec3
local v = Vec3.new(1.0, 2.0, 3.0)
local w = Vec3.new(4.0, 5.0, 6.0)

function bench()
    for i = 1, 100 do
        local c = Vec3.new(1.0, 2.0, 3.0)
    end
end
