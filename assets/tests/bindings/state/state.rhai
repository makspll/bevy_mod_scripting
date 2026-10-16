// the test app sets no states up, so a state type without a `State` resource gives nil
assert(type_of(world.get_state.call(world.get_type_by_name.call("usize"))) == "()", "get_state without a state gives nil");
