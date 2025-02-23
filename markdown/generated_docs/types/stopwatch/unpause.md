# unpause

>  Unpauses the stopwatch. Resume the effect of ticking on elapsed time.
>  # Examples
>  ```
>  # use bevy_time::*;
>  use std::time::Duration;
>  let mut stopwatch = Stopwatch::new();
>  stopwatch.pause();
>  stopwatch.tick(Duration::from_secs_f32(1.0));
>  stopwatch.unpause();
>  stopwatch.tick(Duration::from_secs_f32(1.0));
>  assert!(!stopwatch.is_paused());
>  assert_eq!(stopwatch.elapsed_secs(), 1.0);
>  ```

#### Arguments

- **\_self** : `Stopwatch` \- No Documentation 🚧

#### Returns

- **arg0** : `()` \- No Documentation 🚧