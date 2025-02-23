# remaining\_secs

>  Returns the remaining time in seconds
>  # Examples
>  ```
>  # use bevy_time::*;
>  use std::cmp::Ordering;
>  use std::time::Duration;
>  let mut timer = Timer::from_seconds(2.0, TimerMode::Once);
>  timer.tick(Duration::from_secs_f32(0.5));
>  let result = timer.remaining_secs().total_cmp(&1.5);
>  assert_eq!(Ordering::Equal, result);
>  ```

#### Arguments

- **\_self** : `Timer` \- No Documentation 🚧

#### Returns

- **arg0** : `f32` \- No Documentation 🚧