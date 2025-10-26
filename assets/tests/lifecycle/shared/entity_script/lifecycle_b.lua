function on_script_loaded()
    return "B: loaded!"
end

function on_script_unloaded()
    return "B: unloaded!"
end

function on_script_reloaded(val)
    return "B: reloaded with: " .. val
end
