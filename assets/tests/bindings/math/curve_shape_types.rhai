// the types of the crates of the math bindings are all in the namespace, including the ones without functions
let math_namespace = import_namespace.call("math");
assert(type_of(math_namespace.JumpAt) != "()", "the jump of the step easing function is in the namespace");
assert(type_of(math_namespace.EaseFunction) != "()", "the easing function is in the namespace");
