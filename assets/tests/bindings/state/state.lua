-- the test app sets no states up, so a state type without a `State` resource gives nil
assert(world.get_state(world.get_type_by_name("usize")) == nil, "get_state without a state gives nil")
