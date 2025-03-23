
function on_test()
    local my_type = types.TestResource;
    assert(my_type ~= nil, "Type TestResource is not available in type cache");
    assert(my_type:short_name() == "TestResource", "Type t.TestResource:short_name() is not correct: " .. my_type:short_name());

    local my_generic_type = types["GenericComponent<String>"];
    assert(my_generic_type ~= nil, "Type GenericComponent<String> is not available in type cache");
    assert(my_generic_type:short_name() == "GenericComponent<String>", "Type t.GenericComponent<String>:short_name() is not correct: " .. my_generic_type:short_name());
end