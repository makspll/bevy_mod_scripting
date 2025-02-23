# pause

>  Pauses the Timer. Disables the ticking of the timer.
>  See also [`Stopwatch::pause`](Stopwatch::pause).
>  # Examples
>  ```
>  # use bevy_time::*;
>  use std::time::Duration;
>  let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
>  timer.pause();
>  timer.tick(Duration::from_secs_f32(0.5));
>  assert_eq!(timer.elapsed_secs(), 0.0);
>  ```

#### Arguments

- **\_self** : `Timer` \- No Documentation 🚧

#### Returns

- **arg0** : `()` \- No Documentation 🚧