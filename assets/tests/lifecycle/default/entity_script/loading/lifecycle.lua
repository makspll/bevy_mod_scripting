function on_script_loaded()
    return "loaded!"
end

function on_script_unloaded()
    return "unloaded!"
end

function on_script_reloaded(val)
    return "reloaded with: " .. val
end
