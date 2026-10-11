local value = SimpleEnum.new({ variant = "Struct", foo = 1 })

-- discriminating variants
assert(value:variant() == SimpleEnum.Struct, "expected value to be the Struct variant")
assert(value:variant() ~= SimpleEnum.Unit, "expected value not to be the Unit variant")
assert(SimpleEnum.Unit:variant_name() == "Unit", "expected variant name Unit")

-- constructing from variants
local constructed = SimpleEnum.Struct:new({ foo = 5 })
assert(constructed:variant() == SimpleEnum.Struct, "expected Struct variant")
assert(constructed.foo == 5, "expected foo to be 5 but got " .. tostring(constructed.foo))

local tuple = SimpleEnum.TupleStruct:new({ ["1"] = 7 })
assert(tuple:variant() == SimpleEnum.TupleStruct, "expected TupleStruct variant")
assert(tuple[1] == 7, "expected first field to be 7 but got " .. tostring(tuple[1]))

local unit = SimpleEnum.Unit:new({})
assert(unit:variant() == SimpleEnum.Unit, "expected Unit variant")

-- the escape hatch for names shadowed by functions
local type = world.get_type_by_name("SimpleEnum")
assert(world.get_enum_variant(type, "Unit") == SimpleEnum.Unit, "expected get_enum_variant to find Unit")
assert(world.get_enum_variant(type, "Missing") == nil, "expected no such variant")

-- non enums have no variant
assert(SimpleStruct.new({}):variant() == nil, "expected structs to have no variant")

-- variants take positional fields too
local positional_tuple = SimpleEnum.TupleStruct:new({ 8 })
assert(positional_tuple[1] == 8, "expected first field to be 8 but got " .. tostring(positional_tuple[1]))
