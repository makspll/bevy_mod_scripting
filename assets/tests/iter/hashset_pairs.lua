local res_type = world.get_type_by_name("TestResourceWithVariousFields")
local res = world.get_resource(res_type)

local set = res.string_set

local count = 0
local found_values = {}

for value in pairs(set) do
    count = count + 1
    found_values[value] = true
end

if count ~= 2 then
    error(string.format("Expected 2 entries, got %d", count))
end
if found_values["apple"] ~= true then
    error("Expected to find 'apple'")
end
if found_values["banana"] ~= true then
    error("Expected to find 'banana'")
end
