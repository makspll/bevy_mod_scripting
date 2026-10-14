-- `==` works for every reflected type, even without its own `eq`
local a = SimpleStruct.new({ foo = 1 })
local b = SimpleStruct.new({ foo = 1 })
assert(a == b, "equal values compare equal")
b.foo = 2
assert(a ~= b, "different values are not equal")
assert(a:eq(a), "eq can be called directly")
assert(not a:eq(5), "values of different kinds are not equal")

-- clone makes an independent copy
local copy = b:clone()
assert(copy == b, "a copy is equal")
copy.foo = 3
assert(b.foo == 2, "changing the copy doesn't change the original")
