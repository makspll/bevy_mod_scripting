local Duration = import_namespace("time").Duration

-- the default clock, and the clocks behind it
local time = world.time()
assert(time:delta_secs() >= 0.0, "delta_secs")
assert(time:delta():as_secs() >= 0.0, "delta")
assert(time:delta_secs_f64() >= 0.0, "delta_secs_f64")
assert(time:elapsed_secs() >= 0.0, "elapsed_secs")
assert(time:elapsed():as_secs() >= 0.0, "elapsed")
assert(time:elapsed_secs_f64() >= 0.0, "elapsed_secs_f64")
assert(time:elapsed_secs_wrapped() >= 0.0, "elapsed_secs_wrapped")

local real = world.real_time()
assert(real:delta_secs() >= 0.0, "real clock delta_secs")
assert(real:elapsed_secs_wrapped() >= 0.0, "real clock elapsed_secs_wrapped")

-- pausing the virtual clock persists through the reference
local virtual = world.virtual_time()
virtual:unpause()
assert(virtual:is_paused() == false, "unpaused")
world.virtual_time():pause()
assert(world.virtual_time():is_paused() == true, "pause persists through the resource reference")
virtual:unpause()
assert(virtual:is_paused() == false, "unpause persists through the resource reference")

-- virtual clock speed and maximum delta
virtual:set_relative_speed(2.0)
assert(virtual:relative_speed() == 2.0, "relative_speed")
assert(not pcall(function() virtual:set_relative_speed(-1.0) end), "set_relative_speed rejects negative speeds")
virtual:set_relative_speed(1.0)
virtual:set_max_delta(Duration.from_millis(100))
assert(virtual:max_delta():as_millis() == 100, "max_delta")

-- fixed clock timestep
local fixed = world.fixed_time()
fixed:set_timestep_seconds(0.25)
assert(fixed:timestep():as_secs() == 0.25, "timestep")
assert(world.fixed_time():timestep():as_secs() == 0.25, "timestep persists through the resource reference")
assert(not pcall(function() fixed:set_timestep_seconds(0.0) end), "set_timestep_seconds rejects zero")
assert(not pcall(function() fixed:set_timestep_seconds(-1.0) end), "set_timestep_seconds rejects negative seconds")
assert(fixed:overstep_fraction() >= 0.0, "overstep_fraction")
fixed:set_timestep_seconds(0.015625)
