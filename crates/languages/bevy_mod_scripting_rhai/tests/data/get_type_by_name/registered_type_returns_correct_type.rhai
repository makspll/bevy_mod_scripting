let type = world.get_type_by_name.call("TestComponent");

assert(type_of(type) != "()", "Registered type was not found");

let expected_type_name = "test_utils::test_data::TestComponent";
let expected_short_name = "TestComponent";

let received_type_name = type.type_name.call(type);
let received_short_name = type.short_name.call(type);


// assert(type != (), 'Type not found')
// assert(received.type_name == expected.type_name, 'type_name mismatch, expected: ' .. expected.type_name .. ', got: ' .. received.type_name)
// assert(received.short_name == expected.short_name, 'short_name mismatch, expected: ' .. expected.short_name .. ', got: ' .. received.short_name)

assert(received_type_name == expected_type_name, "type_name mismatch, expected: " + expected_type_name + ", got: " + received_type_name);
assert(received_short_name == expected_short_name, "short_name mismatch, expected: " + expected_short_name + ", got: " + received_short_name);