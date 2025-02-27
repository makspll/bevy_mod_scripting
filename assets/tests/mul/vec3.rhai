let a = Vec3.new_.call(1.0, 2.0, 3.0);
let b = Vec3.new_.call(4.0, 5.0, 6.0);

assert((a * 2).x == 2.0, "Multiplication did not work");
assert((a * 2).y == 4.0, "Multiplication did not work");
assert((a * 2).z == 6.0, "Multiplication did not work");

assert((a * b).x == 4.0, "Multiplication did not work");
assert((a * b).y == 10.0, "Multiplication did not work");
assert((a * b).z == 18.0, "Multiplication did not work");