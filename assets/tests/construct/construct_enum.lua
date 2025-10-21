local type = world.get_type_by_name("SimpleEnum")

-- Struct Variant
local constructed = construct(type, {
    variant = "Struct",
    foo = 123
})

assert(constructed:variant_name() == "Struct",
    "Value was constructed incorrectly, expected constructed.variant to be Struct but got " .. constructed:variant_name())
assert(constructed.foo == 123,
    "Value was constructed incorrectly, expected constructed.foo to be 123 but got " .. constructed.foo)


-- TupleStruct Variant
local constructed = construct(type, {
    variant = "TupleStruct",
    ["1"] = 123
})

assert(constructed:variant_name() == "TupleStruct",
    "Value was constructed incorrectly, expected constructed.variant to be TupleStruct but got " ..
    constructed:variant_name())
assert(constructed[1] == 123,
    "Value was constructed incorrectly, expected constructed._1 to be 123 but got " .. constructed[1])

-- Unit Variant
local constructed = construct(type, {
    variant = "Unit"
})

assert(constructed:variant_name() == "Unit",
    "Value was constructed incorrectly, expected constructed.variant to be Unit but got " .. constructed:variant_name())
