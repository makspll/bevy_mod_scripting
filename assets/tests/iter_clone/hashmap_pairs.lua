local res_type = world.get_type_by_name("TestResourceWithVariousFields")
local res = world.get_resource(res_type)

local map = res.string_map

local count = 0
local found_keys = {}

-- Use pairs_clone to loop over Reflect values
for key, value in map:pairs_clone() do
    count = count + 1
    found_keys[key] = value
end

if count ~= 2 then
    error(string.format("Expected 2 entries, got %d", count))
end
if found_keys["foo"] ~= "bar" then
    error("Expected foo=>bar")
end
if found_keys["zoo"] ~= "zed" then
    error("Expected zoo=>zed")
end
