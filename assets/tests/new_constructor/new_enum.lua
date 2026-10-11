local constructed = SimpleEnum.new({ variant = "Struct", foo = 123 })
assert(constructed:variant_name() == "Struct", "expected variant Struct but got " .. tostring(constructed:variant_name()))
assert(constructed.foo == 123, "expected foo to be 123 but got " .. tostring(constructed.foo))

local unit = SimpleEnum.new({ variant = "Unit" })
assert(unit:variant_name() == "Unit", "expected variant Unit but got " .. tostring(unit:variant_name()))
