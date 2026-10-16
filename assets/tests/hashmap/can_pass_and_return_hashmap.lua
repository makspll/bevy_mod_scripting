local test_utils = import_namespace("test_utils")
local map = test_utils.make_hashmap({
    key1 = 2,
    key2 = 3,
})

assert(map["key1"] == 2, "map[\"key1\"] should be 2")
assert(map["key2"] == 3, "map[\"key2\"] should be 3")