# set\_elapsed

>  Sets the elapsed time of the timer without any other considerations.
>  See also [`Stopwatch::set`](Stopwatch::set).
>  #
>  ```
>  # use bevy_time::*;
>  use std::time::Duration;
>  let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
>  timer.set_elapsed(Duration::from_secs(2));
>  assert_eq!(timer.elapsed(), Duration::from_secs(2));
>  // the timer is not finished even if the elapsed time is greater than the duration.
>  assert!(!timer.finished());
>  ```

#### Arguments

- **\_self** : `Timer` \- No Documentation 🚧
- **time** : `Duration` \- No Documentation 🚧

#### Returns

- **arg0** : `()` \- No Documentation 🚧