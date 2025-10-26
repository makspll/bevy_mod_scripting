let type = world.get_type_by_name.call("SimpleEnum");

// Struct Variant
let constructed = construct.call(type, #{ variant: "Struct", foo: 123 });

assert(constructed.variant_name.call() == "Struct", "Value was constructed incorrectly, expected constructed.variant to be Struct but got " + constructed.variant_name.call());
assert(constructed.foo == 123, "Value was constructed incorrectly, expected constructed.foo to be 123 but got " + constructed.foo);

// TupleStruct Variant
constructed = construct.call(type, #{ variant: "TupleStruct", "0": 123 });

assert(constructed.variant_name.call() == "TupleStruct", "Value was constructed incorrectly, expected constructed.variant to be TupleStruct but got " + constructed.variant_name.call());
assert(constructed[0] == 123, "Value was constructed incorrectly, expected constructed[0] to be 123 but got " + constructed[0]);

// Unit Variant
constructed = construct.call(type, #{ variant: "Unit" });

assert(constructed.variant_name.call() == "Unit", "Value was constructed incorrectly, expected constructed.variant to be Unit but got " + constructed.variant_name.call());