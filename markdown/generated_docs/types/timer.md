# Timer

### Timer

- **stopwatch** : bevy\_time::stopwatch::Stopwatch
- **duration** : bevy\_utils::Duration
- **mode** : bevy\_time::timer::TimerMode
- **finished** : bool
- **times\_finished\_this\_tick** : u32

## Description

>  Tracks elapsed time. Enters the finished state once `duration` is reached.
> 
>  Non repeating timers will stop tracking and stay in the finished state until reset.
>  Repeating timers will only be in the finished state on each tick `duration` is reached or
>  exceeded, and can still be reset at any given point.
> 
>  Paused timers will not have elapsed time increased.
> 
>  Note that in order to advance the timer [`tick`](Timer::tick) **MUST** be called.

## Functions

| Function | Summary |
| --- | --- |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./timer/assert_receiver_is_total_eq.md) |
| `clone(_self)` | [No Documentation 🚧](./timer/clone.md) |
| `duration(_self)` | [ Returns the duration of the timer\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  use std::time::Duration;  let timer = Timer::new\(Duration::from\_secs\(1\), TimerMode::Once\);  assert\_eq\!\(timer\.duration\(\), Duration::from\_secs\(1\)\);  \`\`\`](./timer/duration.md) |
| `elapsed(_self)` | [ Returns the time elapsed on the timer\. Guaranteed to be between 0\.0 and \`duration\`\.  Will only equa](./timer/elapsed.md) |
| `elapsed_secs(_self)` | [ Returns the time elapsed on the timer as an \`f32\`\.  See also \[\`Timer::elapsed\`\]\(Timer::elapsed\)\.](./timer/elapsed_secs.md) |
| `elapsed_secs_f64(_self)` | [ Returns the time elapsed on the timer as an \`f64\`\.  See also \[\`Timer::elapsed\`\]\(Timer::elapsed\)\.](./timer/elapsed_secs_f64.md) |
| `eq(_self, other)` | [No Documentation 🚧](./timer/eq.md) |
| `finished(_self)` | [ Returns \`true\` if the timer has reached its duration\.  For repeating timers, this method behaves id](./timer/finished.md) |
| `fraction(_self)` | [ Returns the fraction of the timer elapsed time \(goes from 0\.0 to 1\.0\)\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  use std::time::Duration;  let mut timer = Timer::from\_seconds\(2\.0, TimerMode::Once\);  timer\.tick\(Duration::from\_secs\_f32\(0\.5\)\);  assert\_eq\!\(timer\.fraction\(\), 0\.25\);  \`\`\`](./timer/fraction.md) |
| `fraction_remaining(_self)` | [ Returns the fraction of the timer remaining time \(goes from 1\.0 to 0\.0\)\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  use std::time::Duration;  let mut timer = Timer::from\_seconds\(2\.0, TimerMode::Once\);  timer\.tick\(Duration::from\_secs\_f32\(0\.5\)\);  assert\_eq\!\(timer\.fraction\_remaining\(\), 0\.75\);  \`\`\`](./timer/fraction_remaining.md) |
| `from_seconds(duration, mode)` | [ Creates a new timer with a given duration in seconds\.  \# Example  \`\`\`  \# use bevy\_time::\*;  let mut timer = Timer::from\_seconds\(1\.0, TimerMode::Once\);  \`\`\`](./timer/from_seconds.md) |
| `just_finished(_self)` | [ Returns \`true\` only on the tick the timer reached its duration\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  use std::time::Duration;  let mut timer = Timer::from\_seconds\(1\.0, TimerMode::Once\);  timer\.tick\(Duration::from\_secs\_f32\(1\.5\)\);  assert\!\(timer\.just\_finished\(\)\);  timer\.tick\(Duration::from\_secs\_f32\(0\.5\)\);  assert\!\(\!timer\.just\_finished\(\)\);  \`\`\`](./timer/just_finished.md) |
| `mode(_self)` | [ Returns the mode of the timer\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  let mut timer = Timer::from\_seconds\(1\.0, TimerMode::Repeating\);  assert\_eq\!\(timer\.mode\(\), TimerMode::Repeating\);  \`\`\`](./timer/mode.md) |
| `new(duration, mode)` | [ Creates a new timer with a given duration\.  See also \[\`Timer::from\_seconds\`\]\(Timer::from\_seconds\)\.](./timer/new.md) |
| `pause(_self)` | [ Pauses the Timer\. Disables the ticking of the timer\.  See also \[\`Stopwatch::pause\`\]\(Stopwatch::pause\)\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  use std::time::Duration;  let mut timer = Timer::from\_seconds\(1\.0, TimerMode::Once\);  timer\.pause\(\);  timer\.tick\(Duration::from\_secs\_f32\(0\.5\)\);  assert\_eq\!\(timer\.elapsed\_secs\(\), 0\.0\);  \`\`\`](./timer/pause.md) |
| `paused(_self)` | [ Returns \`true\` if the timer is paused\.  See also \[\`Stopwatch::is\_paused\`\]\(Stopwatch::is\_paused\)\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  let mut timer = Timer::from\_seconds\(1\.0, TimerMode::Once\);  assert\!\(\!timer\.paused\(\)\);  timer\.pause\(\);  assert\!\(timer\.paused\(\)\);  timer\.unpause\(\);  assert\!\(\!timer\.paused\(\)\);  \`\`\`](./timer/paused.md) |
| `remaining(_self)` | [ Returns the remaining time using Duration  \# Examples  \`\`\`  \# use bevy\_time::\*;  use std::time::Duration;  let mut timer = Timer::from\_seconds\(2\.0, TimerMode::Once\);  timer\.tick\(Duration::from\_secs\_f32\(0\.5\)\);  assert\_eq\!\(timer\.remaining\(\), Duration::from\_secs\_f32\(1\.5\)\);  \`\`\`](./timer/remaining.md) |
| `remaining_secs(_self)` | [ Returns the remaining time in seconds  \# Examples  \`\`\`  \# use bevy\_time::\*;  use std::cmp::Ordering;  use std::time::Duration;  let mut timer = Timer::from\_seconds\(2\.0, TimerMode::Once\);  timer\.tick\(Duration::from\_secs\_f32\(0\.5\)\);  let result = timer\.remaining\_secs\(\)\.total\_cmp\(&1\.5\);  assert\_eq\!\(Ordering::Equal, result\);  \`\`\`](./timer/remaining_secs.md) |
| `reset(_self)` | [ Resets the timer\. The reset doesn't affect the \`paused\` state of the timer\.  See also \[\`Stopwatch::reset\`\]\(Stopwatch::reset\)](./timer/reset.md) |
| `set_duration(_self, duration)` | [ Sets the duration of the timer\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  use std::time::Duration;  let mut timer = Timer::from\_seconds\(1\.5, TimerMode::Once\);  timer\.set\_duration\(Duration::from\_secs\(1\)\);  assert\_eq\!\(timer\.duration\(\), Duration::from\_secs\(1\)\);  \`\`\`](./timer/set_duration.md) |
| `set_elapsed(_self, time)` | [ Sets the elapsed time of the timer without any other considerations\.  See also \[\`Stopwatch::set\`\]\(Stopwatch::set\)\.  \#  \`\`\`  \# use bevy\_time::\*;  use std::time::Duration;  let mut timer = Timer::from\_seconds\(1\.0, TimerMode::Once\);  timer\.set\_elapsed\(Duration::from\_secs\(2\)\);  assert\_eq\!\(timer\.elapsed\(\), Duration::from\_secs\(2\)\);  // the timer is not finished even if the elapsed time is greater than the duration\.  assert\!\(\!timer\.finished\(\)\);  \`\`\`](./timer/set_elapsed.md) |
| `set_mode(_self, mode)` | [ Sets the mode of the timer\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  let mut timer = Timer::from\_seconds\(1\.0, TimerMode::Repeating\);  timer\.set\_mode\(TimerMode::Once\);  assert\_eq\!\(timer\.mode\(\), TimerMode::Once\);  \`\`\`](./timer/set_mode.md) |
| `times_finished_this_tick(_self)` | [ Returns the number of times a repeating timer  finished during the last \[\`tick\`\]\(Timer<T>::tick\) call\.  For non repeating\-timers, this method will only ever  return 0 or 1\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  use std::time::Duration;  let mut timer = Timer::from\_seconds\(1\.0, TimerMode::Repeating\);  timer\.tick\(Duration::from\_secs\_f32\(6\.0\)\);  assert\_eq\!\(timer\.times\_finished\_this\_tick\(\), 6\);  timer\.tick\(Duration::from\_secs\_f32\(2\.0\)\);  assert\_eq\!\(timer\.times\_finished\_this\_tick\(\), 2\);  timer\.tick\(Duration::from\_secs\_f32\(0\.5\)\);  assert\_eq\!\(timer\.times\_finished\_this\_tick\(\), 0\);  \`\`\`](./timer/times_finished_this_tick.md) |
| `unpause(_self)` | [ Unpauses the Timer\. Resumes the ticking of the timer\.  See also \[\`Stopwatch::unpause\(\)\`\]\(Stopwatch::unpause\)\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  use std::time::Duration;  let mut timer = Timer::from\_seconds\(1\.0, TimerMode::Once\);  timer\.pause\(\);  timer\.tick\(Duration::from\_secs\_f32\(0\.5\)\);  timer\.unpause\(\);  timer\.tick\(Duration::from\_secs\_f32\(0\.5\)\);  assert\_eq\!\(timer\.elapsed\_secs\(\), 0\.5\);  \`\`\`](./timer/unpause.md) |