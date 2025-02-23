# just\_finished

>  Returns `true` only on the tick the timer reached its duration.
>  # Examples
>  ```
>  # use bevy_time::*;
>  use std::time::Duration;
>  let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
>  timer.tick(Duration::from_secs_f32(1.5));
>  assert!(timer.just_finished());
>  timer.tick(Duration::from_secs_f32(0.5));
>  assert!(!timer.just_finished());
>  ```

#### Arguments

- **\_self** : `Timer` \- No Documentation 🚧

#### Returns

- **arg0** : `bool` \- No Documentation 🚧