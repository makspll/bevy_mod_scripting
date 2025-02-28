local schedule = world.get_schedule_by_name("Startup")
local systems = schedule:systems()

-- contains event_handler system
assert(#systems == 1, "Schedule does not contain all systems")

assert(schedule:get_system_by_name("dummy_startup_system"):identifier() == "dummy_startup_system", "System identifier was wrong")