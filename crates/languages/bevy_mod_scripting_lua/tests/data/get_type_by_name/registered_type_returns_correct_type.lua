local type = world:get_type_by_name('TestComponent')

local expected = {
    type_name = 'test_utils::test_data::TestComponent',
    short_name = 'TestComponent',
}

assert(type ~= nil, 'Type not found')
assert(type.type_name == expected.type_name, 'type_name mismatch, expected: ' .. expected.type_name .. ', got: ' .. type.type_name)
assert(type.short_name == expected.short_name, 'short_name mismatch, expected: ' .. expected.short_name .. ', got: ' .. type.short_name)
