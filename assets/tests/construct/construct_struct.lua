local type = world.get_type_by_name("SimpleStruct")
local constructed = construct(type, {
    foo = 123
})

assert(constructed.foo == 123, "Value was constructed incorrectly, expected constructed.foo to be 123 but got " .. constructed.foo)