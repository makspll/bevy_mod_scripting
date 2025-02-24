local a = Vec3.new(2.0, -4.0, 6.0)
local b = Vec3.new(4.0, 5.0, 6.0)


assert((a == b) == false, "Equality did not work")
assert((a ~= b) == true, "Inequality did not work")
assert((a == a) == true, "Equality did not work")