local res_type = world.get_type_by_name("TestResourceWithVariousFields")
local res = world.get_resource(res_type)

local map = res.string_map

local count = 0
local found_keys = {}
for key, value in pairs(map) do
    count = count + 1
    found_keys[key] = value
end

assert(count == 2, "Expected 2 entries, got " .. count)
assert(found_keys["foo"] == "bar", "Expected foo=>bar")
assert(found_keys["zoo"] == "zed", "Expected zoo=>zed")
