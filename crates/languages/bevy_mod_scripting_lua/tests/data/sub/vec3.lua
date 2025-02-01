

local a = Vec3.new(1.0, 2.0, 3.0)
local b = Vec3.new(4.0, 5.0, 6.0)


assert((a - 1).x == 0.0, "Subtraction did not work")
assert((a - 1).y == 1.0, "Subtraction did not work")
assert((a - 1).z == 2.0, "Subtraction did not work")

assert((a - b).x == -3.0, "Subtraction did not work")
assert((a - b).y == -3.0, "Subtraction did not work")
assert((a - b).z == -3.0, "Subtraction did not work")