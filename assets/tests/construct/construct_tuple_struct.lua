local type = world.get_type_by_name("SimpleTupleStruct")
local constructed = construct(type, {
    ["1"] = 123
})

assert(constructed[1] == 123,
    "Value was constructed incorrectly, expected constructed.foo to be 123 but got " .. constructed[1])
