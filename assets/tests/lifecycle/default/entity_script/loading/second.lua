function on_script_loaded()
    return "2:loaded!"
end

function on_script_unloaded()
    return "2:unloaded!"
end

function on_script_reloaded(val)
    return "2:reloaded with: " .. val
end
