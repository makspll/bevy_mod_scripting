# reset

>  Resets the timer. The reset doesn't affect the `paused` state of the timer.
>  See also [`Stopwatch::reset`](Stopwatch::reset).
>  Examples
>  ```
>  # use bevy_time::*;
>  use std::time::Duration;
>  let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
>  timer.tick(Duration::from_secs_f32(1.5));
>  timer.reset();
>  assert!(!timer.finished());
>  assert!(!timer.just_finished());
>  assert_eq!(timer.elapsed_secs(), 0.0);
>  ```

#### Arguments

- **\_self** : `Timer` \- No Documentation 🚧

#### Returns

- **arg0** : `()` \- No Documentation 🚧