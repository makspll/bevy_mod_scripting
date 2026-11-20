-- 100 operations per call: the cost of one is (this - `empty loop`) / 100
local Mat3 = import_namespace("math").Mat3
local Vec3 = import_namespace("math").Vec3
local m = Mat3.from_cols(Vec3.new(1, 0, 0), Vec3.new(0, 2, 0), Vec3.new(0, 0, 3))

function bench()
    for i = 1, 100 do
        local n = Mat3.identity()
    end
end
