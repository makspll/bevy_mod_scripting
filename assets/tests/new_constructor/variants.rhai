let value = SimpleEnum.new_.call(#{ variant: "Struct", foo: 1 });

assert(value.variant.call() == SimpleEnum.Struct, "expected value to be the Struct variant");
assert(value.variant.call() != SimpleEnum.Unit, "expected value not to be the Unit variant");

let constructed = SimpleEnum.Struct.new_.call(#{ foo: 5 });
assert(constructed.variant.call() == SimpleEnum.Struct, "expected Struct variant");
assert(constructed.foo == 5, "expected foo to be 5 but got " + constructed.foo);
