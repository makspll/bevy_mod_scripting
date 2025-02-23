# elapsed

>  Returns the time elapsed on the timer. Guaranteed to be between 0.0 and `duration`.
>  Will only equal `duration` when the timer is finished and non repeating.
>  See also [`Stopwatch::elapsed`](Stopwatch::elapsed).
>  # Examples
>  ```
>  # use bevy_time::*;
>  use std::time::Duration;
>  let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
>  timer.tick(Duration::from_secs_f32(0.5));
>  assert_eq!(timer.elapsed(), Duration::from_secs_f32(0.5));
>  ```

#### Arguments

- **\_self** : `Timer` \- No Documentation 🚧

#### Returns

- **arg0** : `Duration` \- No Documentation 🚧