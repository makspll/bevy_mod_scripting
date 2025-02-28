fn on_test() { 
    let startup_schedule = world.get_schedule_by_name.call("Startup");


    let expected_systems = [
        "dummy_startup_system"
    ];

    for system in expected_systems {
        let found_system = startup_schedule.get_system_by_name.call(system);
        assert(type_of(found_system) != "()", "Expected system not found: " + system);
    }
}