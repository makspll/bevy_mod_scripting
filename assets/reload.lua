-- reload.lua
--
-- An example of the script reload feature. Exercise with this command:
-- ```sh
-- cargo run --features lua54,bevy/file_watcher,bevy/multi_threaded --example run-script -- reload.lua
-- ```
function on_script_loaded()
    world.info("Hello world")
end

function on_script_unloaded()
    world.info("Goodbye world")
    return "house"
end

function on_script_reloaded(value)
    if value then
        world.info("I'm back. Thanks for the "..value.." keys!")
    else
        world.info('I have not saved any state before unloading')
    end
end
