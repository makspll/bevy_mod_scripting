local type = world.get_type_by_name("SimpleTupleStruct")
local constructed = construct(type, {
    _1 = 123
})

print(constructed:display_value())

assert(constructed._1 == 123, "Value was constructed incorrectly, expected constructed.foo to be 123 but got " .. constructed._1)