local test_utils = import_namespace("test_utils")
local my_fn = test_utils.into_script_function(
    function(string, list)
        print(string, list)
        assert(string == "test", "string is not test got: " .. string)
        assert(list[1] == "test", "list[1] is not test, got: ".. list[1])
    end
)

my_fn("test", {"test"})

-- return values keep their count and order
local one = test_utils.into_script_function(function() return 1 end)
assert(one() == 1, "single return value was lost")
local two = test_utils.into_script_function(function() return 1, 2 end)
local a, b = two()
assert(a == 1 and b == 2, "two return values were mangled")
local three = test_utils.into_script_function(function() return 1, 2, 3 end)
local x, y, z = three()
assert(x == 1 and y == 2 and z == 3, "three return values were mangled")

