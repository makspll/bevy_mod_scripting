local test_utils = import_namespace("test_utils")
local my_fn = test_utils.into_script_function(
    function(string, list)
        print(string, list)
        assert(string == "test", "string is not test got: " .. string)
        assert(list[1] == "test", "list[1] is not test, got: ".. list[1])
    end
)

my_fn("test", {"test"})

