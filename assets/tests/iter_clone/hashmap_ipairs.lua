local res_type = world.get_type_by_name("TestResourceWithVariousFields")
local res = world.get_resource(res_type)

local map = res.string_map

local count = 0
local found_keys = {}

-- ipairs_clone on a map returns (index, [key, value]) where value is a list
for i, entry in map:ipairs_clone() do
    assert(i == count + 1, "Index should be sequential: expected " .. (count + 1) .. ", got " .. i)

    -- entry should be a list with [key, value]
    assert(entry ~= nil, "Entry should not be nil")
    assert(entry[1] ~= nil, "Key should not be nil")
    assert(entry[2] ~= nil, "Value should not be nil")

    local key = entry[1]
    local value = entry[2]

    count = count + 1
    found_keys[key] = value
end

assert(count == 2, "Expected 2 entries, got " .. count)
assert(found_keys["foo"] == "bar", "Expected foo=>bar, got " .. tostring(found_keys["foo"]))
assert(found_keys["zoo"] == "zed", "Expected zoo=>zed, got " .. tostring(found_keys["zoo"]))
