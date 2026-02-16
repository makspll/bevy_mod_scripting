function on_script_loaded(arg1)
    print("running")
    world._sleep(1)
    print("run")
    return "got: " .. arg1
end
