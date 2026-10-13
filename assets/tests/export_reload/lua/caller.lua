local old_handle

function on_test()
    old_handle = import("script.reload").double
    assert(old_handle(21) == 42)
    return "called"
end

function on_test_post_update()
    -- the exporter was reloaded, its export is gone: the old handle says so, and so does the namespace
    local ok, err = pcall(old_handle, 1)
    assert(not ok and tostring(err):find("does not exist (anymore)", 1, true), tostring(err))
    local ok2, err2 = pcall(import, "script.reload")
    assert(not ok2 and tostring(err2):find("does not exist", 1, true), tostring(err2))
    return "gone"
end

function on_test_last()
    -- the exporter ran again, so a fresh import works, and the old handle does too since it looks up by path
    assert(import("script.reload").double(4) == 8)
    assert(old_handle(5) == 10)
    return "called again"
end
