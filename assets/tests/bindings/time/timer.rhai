// constructors
let Duration = import_namespace.call("time").Duration;
let Stopwatch = import_namespace.call("time").Stopwatch;
let Timer = import_namespace.call("time").Timer;
let TimerMode = import_namespace.call("time").TimerMode;
let timer = Timer.from_seconds.call(2.0, TimerMode.Once);
assert(timer.duration.call().as_secs.call() == 2.0, "from_seconds sets the duration");
assert(timer.mode.call() != (), "mode");

let from_duration = Timer.new_.call(Duration.from_secs.call(1.0), TimerMode.Repeating);
assert(from_duration.duration.call().as_secs.call() == 1.0, "new sets the duration");
from_duration.tick.call(2.5);
assert(from_duration.elapsed_secs.call() == 0.5, "new sets the mode, a repeating timer wraps around");

// a timer running once stops at its duration
let once = Timer.from_seconds.call(1.0, TimerMode.Once);
once.tick.call(2.5);
assert(once.elapsed_secs.call() == 1.0, "a timer running once stops at its duration");
assert(once.times_finished_this_tick.call() == 1, "a timer running once finishes once");

// ticking a timer that runs once
assert(!timer.finished.call(), "not finished at the start");
timer.tick.call(1.0);
assert(timer.elapsed_secs.call() == 1.0, "elapsed_secs");
assert(timer.elapsed.call().as_secs.call() == 1.0, "elapsed");
assert(timer.remaining_secs.call() == 1.0, "remaining_secs");
assert(timer.remaining.call().as_secs.call() == 1.0, "remaining");
assert(timer.fraction.call() == 0.5, "fraction");
assert(timer.fraction_remaining.call() == 0.5, "fraction_remaining");
assert(!timer.just_finished.call(), "not just finished before the end");
timer.tick.call(Duration.from_secs.call(1.0));
assert(timer.finished.call(), "finished once the duration has passed");
assert(timer.just_finished.call(), "just finished on the tick that finished it");
assert(timer.times_finished_this_tick.call() == 1, "times_finished_this_tick");

timer.tick.call(1.0);
assert(!timer.just_finished.call(), "not just finished on later ticks");
assert(timer.finished.call(), "still finished on later ticks");

// pause, reset and almost_finish
timer.reset.call();
assert(!timer.finished.call(), "reset clears the finished state");
assert(timer.elapsed_secs.call() == 0.0, "reset clears the elapsed time");
timer.pause.call();
assert(timer.paused.call(), "paused");
timer.tick.call(1.0);
assert(timer.elapsed_secs.call() == 0.0, "a paused timer does not advance");
timer.unpause.call();
assert(!timer.paused.call(), "unpaused");
timer.almost_finish.call();
assert(timer.fraction.call() > 0.99, "almost_finish moves the timer to its end");

// setters
timer.set_elapsed.call(Duration.from_secs.call(5.0));
assert(timer.elapsed_secs.call() == 5.0, "set_elapsed");
timer.set_duration.call(Duration.from_secs.call(20.0));
assert(timer.duration.call().as_secs.call() == 20.0, "set_duration");
timer.set_mode.call(TimerMode.Repeating);
timer.set_duration.call(Duration.from_secs.call(1.0));
timer.tick.call(2.5);
assert(timer.elapsed_secs.call() == 0.5, "set_mode, a repeating timer wraps around");

// a repeating timer counts how often it finished
let repeating = Timer.new_.call(Duration.from_secs.call(1.0), TimerMode.Repeating);
repeating.tick.call(2.5);
assert(repeating.times_finished_this_tick.call() == 2, "repeating timers finish once per duration");
assert(repeating.elapsed_secs.call() == 0.5, "repeating timers keep the time past their last finish");

// stopwatches
let stopwatch = Stopwatch.new_.call();
assert(stopwatch.elapsed_secs.call() == 0.0, "stopwatch starts at zero");
assert(!stopwatch.is_paused.call(), "stopwatch starts running");
stopwatch.tick.call(1.5);
assert(stopwatch.elapsed_secs.call() == 1.5, "stopwatch tick");
assert(stopwatch.elapsed.call().as_secs.call() == 1.5, "stopwatch elapsed");
stopwatch.pause.call();
assert(stopwatch.is_paused.call(), "stopwatch paused");
stopwatch.tick.call(1.0);
assert(stopwatch.elapsed_secs.call() == 1.5, "a paused stopwatch does not advance");
stopwatch.unpause.call();
stopwatch.tick.call(Duration.from_secs.call(0.5));
assert(stopwatch.elapsed_secs.call() == 2.0, "stopwatch tick with a duration");
stopwatch.set_elapsed.call(Duration.from_secs.call(3.0));
assert(stopwatch.elapsed_secs.call() == 3.0, "stopwatch set_elapsed");
stopwatch.reset.call();
assert(stopwatch.elapsed_secs.call() == 0.0, "stopwatch reset");

// enum values compare equal to their variant, both ways
let mode_timer = Timer.from_seconds.call(1.0, TimerMode.Repeating);
assert(mode_timer.mode.call() == TimerMode.Repeating, "value == variant");
assert(TimerMode.Repeating == mode_timer.mode.call(), "variant == value");
assert(mode_timer.mode.call() != TimerMode.Once, "value != other variant");
