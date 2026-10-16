let test_utils = import_namespace.call("test_utils");
assert(test_utils.global_hello_world.call() == "hi!", "global_hello_world() == 'hi!'")