-- on_script_loaded is too late, gotta do it here to slow down loading
world._sleep(1)


function on_test()
    return "loaded after 1 sec"
end
