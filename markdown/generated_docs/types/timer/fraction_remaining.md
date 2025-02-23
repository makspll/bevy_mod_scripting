# fraction\_remaining

>  Returns the fraction of the timer remaining time (goes from 1.0 to 0.0).
>  # Examples
>  ```
>  # use bevy_time::*;
>  use std::time::Duration;
>  let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
>  timer.tick(Duration::from_secs_f32(0.5));
>  assert_eq!(timer.fraction_remaining(), 0.75);
>  ```

#### Arguments

- **\_self** : `Timer` \- No Documentation 🚧

#### Returns

- **arg0** : `f32` \- No Documentation 🚧