local res_type = world.get_type_by_name("TestResourceWithVariousFields")
local res = world.get_resource(res_type)

local set = res.string_set

local iterator = set:iter()
local count = 0
local found_values = {}

local result = iterator()
while result ~= nil do
    count = count + 1
    found_values[result] = true
    result = iterator()
end

assert(count == 2, "Expected 2 entries, got " .. count)
assert(found_values["apple"] == true, "Expected to find 'apple'")
assert(found_values["banana"] == true, "Expected to find 'banana'")
