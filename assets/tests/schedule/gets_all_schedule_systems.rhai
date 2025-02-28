let schedule = world.get_schedule_by_name.call("Startup");
let systems = schedule.systems.call();

assert(systems.len() == 1, "Schedule does not contain all systems");
assert(schedule.get_system_by_name.call("dummy_startup_system").identifier.call() == "dummy_startup_system", "System identifier was wrong");