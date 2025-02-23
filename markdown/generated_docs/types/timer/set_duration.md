# set\_duration

>  Sets the duration of the timer.
>  # Examples
>  ```
>  # use bevy_time::*;
>  use std::time::Duration;
>  let mut timer = Timer::from_seconds(1.5, TimerMode::Once);
>  timer.set_duration(Duration::from_secs(1));
>  assert_eq!(timer.duration(), Duration::from_secs(1));
>  ```

#### Arguments

- **\_self** : `Timer` \- No Documentation 🚧
- **duration** : `Duration` \- No Documentation 🚧

#### Returns

- **arg0** : `()` \- No Documentation 🚧