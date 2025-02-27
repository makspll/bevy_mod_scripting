let type = world.get_type_by_name.call("SimpleStruct");
let constructed = construct.call(type, #{ foo: 123 });

assert(constructed.foo == 123, "Value was constructed incorrectly, expected constructed.foo to be 123 but got " + constructed.foo);