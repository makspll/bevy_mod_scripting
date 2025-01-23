let a = Vec3.new_.call(2.0, -4.0, 6.0);
let b = Vec3.new_.call(4.0, 5.0, 6.0);


assert((a == b) == false, "Equality did not work");
assert((a != b) == true, "Inequality did not work");
assert((a == a) == true, "Equality did not work");