# paused

>  Returns `true` if the timer is paused.
>  See also [`Stopwatch::is_paused`](Stopwatch::is_paused).
>  # Examples
>  ```
>  # use bevy_time::*;
>  let mut timer = Timer::from_seconds(1.0, TimerMode::Once);
>  assert!(!timer.paused());
>  timer.pause();
>  assert!(timer.paused());
>  timer.unpause();
>  assert!(!timer.paused());
>  ```

#### Arguments

- **\_self** : `Timer` \- No Documentation 🚧

#### Returns

- **arg0** : `bool` \- No Documentation 🚧