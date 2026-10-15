-- the types of the crates of the math bindings are all in the namespace, including the ones without functions
local math_namespace = import_namespace("math")
assert(math_namespace.JumpAt ~= nil, "the jump of the step easing function is in the namespace")
assert(math_namespace.EaseFunction ~= nil, "the easing function is in the namespace")
