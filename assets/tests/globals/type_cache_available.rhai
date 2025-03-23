fn on_test() {
    let my_type = types.TestResource;
    assert(type_of(my_type) != "()", "Type TestResource is not available in type cache");
    assert(my_type.short_name.call() == "TestResource", "Type t.TestResource:short_name() is not correct: " + my_type.short_name.call());

    let my_type = types["GenericComponent<String>"];
    assert(type_of(my_type) != "()", "Type GenericComponent<String> is not available in type cache");
    assert(my_type.short_name.call() == "GenericComponent<String>", "Type t.GenericComponent<String>:short_name() is not correct: " + my_type.short_name.call());
}