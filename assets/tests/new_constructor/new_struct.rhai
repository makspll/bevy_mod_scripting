let SimpleStruct = import_namespace.call("test_utils").SimpleStruct;
let SimpleTupleStruct = import_namespace.call("test_utils").SimpleTupleStruct;

let constructed = SimpleStruct.new_.call(#{ foo: 123 });
assert(constructed.foo == 123, "expected foo to be 123 but got " + constructed.foo);

let defaulted = SimpleStruct.new_.call(#{});
assert(defaulted.foo == 0, "expected foo to be 0 but got " + defaulted.foo);

// a list is taken as the fields in order
let positional = SimpleTupleStruct.new_.call([9]);
assert(positional[0] == 9, "expected first field to be 9 but got " + positional[0]);
