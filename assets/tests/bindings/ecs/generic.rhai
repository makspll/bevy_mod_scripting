// `==` works for every reflected type, even without its own `eq`
let SimpleStruct = import_namespace.call("test_utils").SimpleStruct;
let a = SimpleStruct.new_.call(#{ foo: 1 });
let b = SimpleStruct.new_.call(#{ foo: 1 });
assert(a == b, "equal values compare equal");
b.foo = 2;
assert(a != b, "different values are not equal");

// clone makes an independent copy
let copy = b.clone.call();
assert(copy == b, "a copy is equal");
copy.foo = 3;
assert(b.foo == 2, "changing the copy doesn't change the original");
