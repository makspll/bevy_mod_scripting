local bms = import_namespace("bms")

-- functions which don't belong to a type are members of a namespace, not globals
assert(type(bms.log_info) == "function", "log_info is a member of bms")
assert(type(bms.construct) == "function", "construct is a member of bms")
assert(log_info == nil, "log_info is not a global")
assert(entity == nil, "entity is not a global")
assert(script_asset == nil, "script_asset is not a global")

function on_test()
    -- the running script's entity and handle are looked up when asked for
    assert(bms.current_entity() ~= nil, "the script is attached to an entity")
    assert(bms.current_script() ~= nil, "the running script has a handle")
    bms.log_info("logged through the bms namespace")
    return true
end
