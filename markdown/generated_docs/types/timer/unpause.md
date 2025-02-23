# unpause

>  Unpauses the Timer. Resumes the ticking of the timer.
>  See also [`Stopwatch::unpause()`](Stopwatch::unpause).
>  # Examples
>  ```
>  # use bevy_time::*;
>  use std::time::Duration;
>  let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
>  timer.pause();
>  timer.tick(Duration::from_secs_f32(0.5));
>  timer.unpause();
>  timer.tick(Duration::from_secs_f32(0.5));
>  assert_eq!(timer.elapsed_secs(), 0.5);
>  ```

#### Arguments

- **\_self** : `Timer` \- No Documentation 🚧

#### Returns

- **arg0** : `()` \- No Documentation 🚧