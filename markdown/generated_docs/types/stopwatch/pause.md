# pause

>  Pauses the stopwatch. Any call to [`tick`](Stopwatch::tick) while
>  paused will not have any effect on the elapsed time.
>  # Examples
>  ```
>  # use bevy_time::*;
>  use std::time::Duration;
>  let mut stopwatch = Stopwatch::new();
>  stopwatch.pause();
>  stopwatch.tick(Duration::from_secs_f32(1.5));
>  assert!(stopwatch.is_paused());
>  assert_eq!(stopwatch.elapsed_secs(), 0.0);
>  ```

#### Arguments

- **\_self** : `Stopwatch` \- No Documentation 🚧

#### Returns

- **arg0** : `()` \- No Documentation 🚧