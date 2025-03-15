
function on_test()
    local my_type = types.TestResource;
    assert(my_type ~= nil, "Type TestResource is not available in type cache");
    assert(my_type:short_name() == "TestResource", "Type t.TestResource:short_name() is not correct: " .. my_type:short_name());
end