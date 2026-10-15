let Vec3 = import_namespace.call("math").Vec3;
let a = Vec3.new_.call(2.0, -4.0, 6.0);

assert((-a).x == -2.0, "Negation did not work");
assert((-a).y == 4.0, "Negation did not work");
assert((-a).z == -6.0, "Negation did not work");