local constructed = SimpleStruct.new({ foo = 123 })
assert(constructed.foo == 123, "expected foo to be 123 but got " .. tostring(constructed.foo))

-- the type's default fills in missing fields
local defaulted = SimpleStruct.new({})
assert(defaulted.foo == 0, "expected foo to be 0 but got " .. tostring(defaulted.foo))

-- tuple structs use positional keys
local tuple_struct = SimpleTupleStruct.new({ ["1"] = 5 })
assert(tuple_struct[1] == 5, "expected first field to be 5 but got " .. tostring(tuple_struct[1]))

-- missing fields without any default are an error
local ok = pcall(function()
    return SimpleStruct.new({}, false, false)
end)
assert(not ok, "expected construction to fail with defaults disabled")

-- field defaults still apply without the type default
local field_defaulted = SimpleStruct.new({}, false, true)
assert(field_defaulted.foo == 0, "expected foo to be 0 but got " .. tostring(field_defaulted.foo))
