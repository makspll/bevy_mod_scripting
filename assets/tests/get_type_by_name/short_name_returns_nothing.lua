assert(world.get_type_by_name('TestComponent') == nil, 'Short names should not resolve')
assert(world.get_type_by_name('test_utils::test_data::TestComponent') ~= nil, 'Full path should resolve')
