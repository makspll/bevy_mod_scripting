let constructed = SimpleStruct.new_.call(#{ foo: 123 });
assert(constructed.foo == 123, "expected foo to be 123 but got " + constructed.foo);

let defaulted = SimpleStruct.new_.call(#{});
assert(defaulted.foo == 0, "expected foo to be 0 but got " + defaulted.foo);
