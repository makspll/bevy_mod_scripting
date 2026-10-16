let Vec3 = import_namespace.call("math").Vec3;
let a = Vec3.new_.call(1.0, 2.0, 3.0);
let b = Vec3.new_.call(4.0, 5.0, 6.0);


assert((a - 1).x == 0.0, "Subtraction did not work");
assert((a - 1).y == 1.0, "Subtraction did not work");
assert((a - 1).z == 2.0, "Subtraction did not work");

assert((a - b).x == -3.0, "Subtraction did not work");
assert((a - b).y == -3.0, "Subtraction did not work");
assert((a - b).z == -3.0, "Subtraction did not work");

// a number on the left calls `rsub` on the vector
assert((10 - a) == Vec3.new_.call(9.0, 8.0, 7.0), "Subtraction with a number on the left did not work");