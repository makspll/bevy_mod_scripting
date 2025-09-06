local type = world.get_type_by_name('TestResource')

local expected = {
    type_name = 'test_utils::test_data::TestResource',
    short_name = 'TestResource',
}

local received = {
    type_name = type:type_name(),
    short_name = type:short_name(),
}

assert(type ~= nil, 'Type not found')
assert(received.type_name == expected.type_name,
    'type_name mismatch, expected: ' .. expected.type_name .. ', got: ' .. received.type_name)
assert(received.short_name == expected.short_name,
    'short_name mismatch, expected: ' .. expected.short_name .. ', got: ' .. received.short_name)
local type_ref = type:display()
--  check contains ScriptResourceRegistration
assert(string.find(type_ref, "ScriptResourceRegistration") ~= nil,
    "ScriptResourceRegistration not found in type_ref. got: " .. type_ref)
