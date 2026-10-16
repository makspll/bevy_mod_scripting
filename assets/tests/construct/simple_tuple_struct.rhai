let bms = import_namespace.call("bms");
let type = world.get_type_by_name.call("test_utils::test_data::SimpleTupleStruct");
let constructed = bms.construct.call(type, #{ "0": 123 });

assert(constructed[0] == 123, "Value was constructed incorrectly, expected constructed.foo to be 123 but got " + constructed[0]);