local function error_of(f, ...)
    local ok, err = pcall(f, ...)
    assert(not ok, "expected an error")
    return tostring(err)
end

register_callback("add_impl", function(a, b)
    return a + b
end)

function exported_global(name)
    return "hello " .. name
end

-- entity_a: exports functions
function on_test()
    export("script.exports", "add", "add_impl")
    export("script.exports", "greet", "exported_global")

    -- the exporting script can't call into itself while it is running
    local err = error_of(function()
        return import("script.exports").add(1, 2)
    end)
    assert(err:find("currently running", 1, true), err)

    -- exporting again from the same script replaces
    export("script.exports", "add", "add_impl")

    -- invalid namespaces are rejected
    assert(error_of(export, "exports", "add", "add_impl"):find("script.", 1, true))
    return "exported"
end

-- entity_b: calls them from its own context
function on_test_post_update()
    local exports = import("script.exports")
    assert(exports.add(40, 2) == 42, "exported function should return its result")
    assert(exports.greet("bms") == "hello bms")

    -- another script cannot take the same export
    local err = error_of(export, "script.exports", "add", "add_impl")
    assert(err:find("already exported by script", 1, true), err)

    -- exports are not types, and cannot share a path with components
    local err2 = error_of(world.register_new_component, "script.exports", "add")
    assert(err2:find("already exported", 1, true), err2)

    -- missing namespaces and the exports namespace get suggested
    local err3 = error_of(import, "script.export")
    assert(err3:find("script.exports", 1, true), err3)
    return "called"
end
