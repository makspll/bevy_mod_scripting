# remaining

>  Returns the remaining time using Duration
>  # Examples
>  ```
>  # use bevy_time::*;
>  use std::time::Duration;
>  let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
>  timer.tick(Duration::from_secs_f32(0.5));
>  assert_eq!(timer.remaining(), Duration::from_secs_f32(1.5));
>  ```

#### Arguments

- **\_self** : `Timer` \- No Documentation 🚧

#### Returns

- **arg0** : `Duration` \- No Documentation 🚧