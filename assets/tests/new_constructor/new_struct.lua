local constructed = SimpleStruct.new({ foo = 123 })
assert(constructed.foo == 123, "expected foo to be 123 but got " .. tostring(constructed.foo))

-- defaults fill in missing fields
local defaulted = SimpleStruct.new({})
assert(defaulted.foo == 0, "expected foo to be 0 but got " .. tostring(defaulted.foo))

-- tuple structs use positional keys
local tuple_struct = SimpleTupleStruct.new({ ["1"] = 5 })
assert(tuple_struct[1] == 5, "expected first field to be 5 but got " .. tostring(tuple_struct[1]))

-- a list is taken as the fields in order
local positional = SimpleTupleStruct.new({ 9 })
assert(positional[1] == 9, "expected first field to be 9 but got " .. tostring(positional[1]))

-- `construct` works for every type, the same way
local struct_type = world.get_type_by_name("test_utils::test_data::SimpleStruct")
local constructed_via_global = construct(struct_type, { foo = 3 })
assert(constructed_via_global.foo == 3, "expected foo to be 3 but got " .. tostring(constructed_via_global.foo))
local tuple_type = world.get_type_by_name("test_utils::test_data::SimpleTupleStruct")
assert(construct(tuple_type, { 4 })[1] == 4, "expected construct to accept positional fields")
