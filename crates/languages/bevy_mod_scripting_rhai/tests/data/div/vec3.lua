local a = Vec3.new(2.0, 4.0, 6.0)
local b = Vec3.new(1.0, 2.0, 3.0)

assert((a / 2).x == 1.0, "Division did not work")
assert((a / 2).y == 2.0, "Division did not work")
assert((a / 2).z == 3.0, "Division did not work")

assert((a / b).x == 2.0, "Division did not work")
assert((a / b).y == 2.0, "Division did not work")
assert((a / b).z == 2.0, "Division did not work")