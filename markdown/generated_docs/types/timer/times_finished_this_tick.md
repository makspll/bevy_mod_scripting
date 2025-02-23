# times\_finished\_this\_tick

>  Returns the number of times a repeating timer
>  finished during the last [`tick`](Timer<T>::tick) call.
>  For non repeating-timers, this method will only ever
>  return 0 or 1.
>  # Examples
>  ```
>  # use bevy_time::*;
>  use std::time::Duration;
>  let mut timer = Timer::from_seconds(1.0, TimerMode::Repeating);
>  timer.tick(Duration::from_secs_f32(6.0));
>  assert_eq!(timer.times_finished_this_tick(), 6);
>  timer.tick(Duration::from_secs_f32(2.0));
>  assert_eq!(timer.times_finished_this_tick(), 2);
>  timer.tick(Duration::from_secs_f32(0.5));
>  assert_eq!(timer.times_finished_this_tick(), 0);
>  ```

#### Arguments

- **\_self** : `Timer` \- No Documentation 🚧

#### Returns

- **arg0** : `u32` \- No Documentation 🚧