let world = import_namespace.call("bms").world;
let Duration = import_namespace.call("time").Duration;

// the default clock, and the clocks behind it
let time = world.time.call();
assert(time.delta_secs.call() >= 0.0, "delta_secs");
assert(time.delta.call().as_secs.call() >= 0.0, "delta");
assert(time.delta_secs_f64.call() >= 0.0, "delta_secs_f64");
assert(time.elapsed_secs.call() >= 0.0, "elapsed_secs");
assert(time.elapsed.call().as_secs.call() >= 0.0, "elapsed");
assert(time.elapsed_secs_f64.call() >= 0.0, "elapsed_secs_f64");
assert(time.elapsed_secs_wrapped.call() >= 0.0, "elapsed_secs_wrapped");

let real = world.real_time.call();
assert(real.delta_secs.call() >= 0.0, "real clock delta_secs");
assert(real.elapsed_secs_wrapped.call() >= 0.0, "real clock elapsed_secs_wrapped");

// pausing the virtual clock persists through the reference
let virtual = world.virtual_time.call();
virtual.unpause.call();
assert(!virtual.is_paused.call(), "unpaused");
world.virtual_time.call().pause.call();
assert(world.virtual_time.call().is_paused.call() == true, "pause persists through the resource reference");
virtual.unpause.call();
assert(!virtual.is_paused.call(), "unpause persists through the resource reference");

// virtual clock speed and maximum delta
virtual.set_relative_speed.call(2.0);
assert(virtual.relative_speed.call() == 2.0, "relative_speed");
virtual.set_relative_speed.call(1.0);
virtual.set_max_delta.call(Duration.from_millis.call(100));
assert(virtual.max_delta.call().as_millis.call() == 100, "max_delta");

// fixed clock timestep
let fixed = world.fixed_time.call();
fixed.set_timestep_seconds.call(0.25);
assert(fixed.timestep.call().as_secs.call() == 0.25, "timestep");
assert(world.fixed_time.call().timestep.call().as_secs.call() == 0.25, "timestep persists through the resource reference");

assert(fixed.overstep_fraction.call() >= 0.0, "overstep_fraction");
fixed.set_timestep_seconds.call(0.015625);
