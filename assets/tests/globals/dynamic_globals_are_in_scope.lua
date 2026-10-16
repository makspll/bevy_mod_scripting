local test_utils = import_namespace("test_utils")
assert(test_utils.global_hello_world() == "hi!", "global_hello_world() == 'hi!'")