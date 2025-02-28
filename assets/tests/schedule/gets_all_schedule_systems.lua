function on_test()
    local startup_schedule = world.get_schedule_by_name("Startup")


    local expected_systems = {
        "dummy_startup_system",
    }

    for i, system in ipairs(expected_systems) do
        local found_system = startup_schedule:get_system_by_name(system)
        assert(found_system ~= nil, "Expected system not found: " .. system)
    end
end