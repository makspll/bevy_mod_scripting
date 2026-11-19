local Vec3 = import_namespace("math").Vec3
local Mat3 = import_namespace("math").Mat3
local m = Mat3.from_cols(Vec3.new(1.0, 2.0, 3.0), Vec3.new(4.0, 5.0, 6.0), Vec3.new(7.0, 8.0, 9.0))
local x_axis = m.x_axis

function bench()
    local l = x_axis:length()
end
