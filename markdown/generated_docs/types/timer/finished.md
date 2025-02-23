# finished

>  Returns `true` if the timer has reached its duration.
>  For repeating timers, this method behaves identically to [`Timer::just_finished`].
>  # Examples
>  ```
>  # use bevy_time::*;
>  use std::time::Duration;
>  let mut timer_once = Timer::from_seconds(1.0, TimerMode::Once);
>  timer_once.tick(Duration::from_secs_f32(1.5));
>  assert!(timer_once.finished());
>  timer_once.tick(Duration::from_secs_f32(0.5));
>  assert!(timer_once.finished());
>  let mut timer_repeating = Timer::from_seconds(1.0, TimerMode::Repeating);
>  timer_repeating.tick(Duration::from_secs_f32(1.1));
>  assert!(timer_repeating.finished());
>  timer_repeating.tick(Duration::from_secs_f32(0.8));
>  assert!(!timer_repeating.finished());
>  timer_repeating.tick(Duration::from_secs_f32(0.6));
>  assert!(timer_repeating.finished());
>  ```

#### Arguments

- **\_self** : `Timer` \- No Documentation 🚧

#### Returns

- **arg0** : `bool` \- No Documentation 🚧