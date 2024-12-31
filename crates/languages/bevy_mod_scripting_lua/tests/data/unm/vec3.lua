local a = Vec3.new(2.0, -4.0, 6.0)

assert(-a.x == -2.0, "Negation did not work")
assert(-a.y == 4.0, "Negation did not work")
assert(-a.z == -6.0, "Negation did not work")