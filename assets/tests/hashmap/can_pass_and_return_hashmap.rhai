let my_map = make_hashmap.call(#{
    key1: 2,
    key2: 3,
});

assert(my_map["key1"] == 2, "map[\"key1\"] should be 2");
assert(my_map["key2"] == 3, "map[\"key2\"] should be 3");