let bms = import_namespace.call("bms");
let world = bms.world;
let type = world.get_type_by_name.call("test_utils::test_data::SimpleStruct");
let constructed = bms.construct.call(type, #{ foo: 123 });

assert(constructed.foo == 123, "Value was constructed incorrectly, expected constructed.foo to be 123 but got " + constructed.foo);