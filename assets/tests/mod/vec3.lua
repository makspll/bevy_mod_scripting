local a = Vec3.new(2.0, 5.0, 6.0)
local b = Vec3.new(1.0, 2.0, 3.0)

assert((a % 2).x == 0.0, "Modulus did not work")
assert((a % 2).y == 1.0, "Modulus did not work")
assert((a % 2).z == 0.0, "Modulus did not work")

assert((a % b).x == 0.0, "Modulus did not work")
assert((a % b).y == 1.0, "Modulus did not work")
assert((a % b).z == 0.0, "Modulus did not work")