local type = world.get_type_by_name("UnitStruct")
local constructed = construct(type, {})

assert(constructed ~= nil, "Value was not constructed")
