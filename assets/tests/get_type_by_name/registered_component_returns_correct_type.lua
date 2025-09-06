local type = world.get_type_by_name('TestComponent')

local expected = {
    type_name = 'test_utils::test_data::TestComponent',
    short_name = 'TestComponent',
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
--  check contains ScriptComponentRegistration
assert(string.find(type_ref, "ScriptComponentRegistration") ~= nil,
    "ScriptComponentRegistration not found in type_ref. got: " .. type_ref)
