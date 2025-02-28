-- add system to PostUpdate, attach it to a `custom_system` callback, and assert it has run after the test
local post_update_schedule = world.get_schedule_by_name("PostUpdate")




local system = world.add_system(
    post_update_schedule,
    system_builder("custom_system", script_id))

local system_after = world.add_system(
    post_update_schedule,
    system_builder("custom_system_after", script_id)
        :after(system)
)
    
local system_before = world.add_system(
    post_update_schedule,
    system_builder("custom_system_before", script_id)
        :before(system)
)
    
        
        

local runs = {}

function custom_system()
    print("custom_system")
    runs[#runs + 1] = "custom_system"
end

function custom_system_before()
    print("custom_system_before")
    runs[#runs + 1] = "custom_system_before"
end

function custom_system_after()
    print("custom_system_after")
    runs[#runs + 1] = "custom_system_after"
end

-- runs in the `Last` bevy schedule
function on_test_last()
    assert(#runs == 3, "Expected 3 runs, got: " .. #runs)
    assert(runs[1] == "custom_system_before", "Expected custom_system_before to run first, got: " .. runs[1])
    assert(runs[2] == "custom_system", "Expected custom_system to run second, got: " .. runs[2])
    assert(runs[3] == "custom_system_after", "Expected custom_system_after to run third, got: " .. runs[3])
end

assert(system:identifier() == "script_system_custom_system", "System identifier is not correct got: ".. system:identifier())
assert(system_before:identifier() == "script_system_custom_system_before", "System identifier is not correct got: ".. system_before:identifier())
assert(system_after:identifier() == "script_system_custom_system_after", "System identifier is not correct got: ".. system_after:identifier())