# reset

>  Resets the stopwatch. The reset doesn't affect the paused state of the stopwatch.
>  # Examples
>  ```
>  # use bevy_time::*;
>  use std::time::Duration;
>  let mut stopwatch = Stopwatch::new();
>  stopwatch.tick(Duration::from_secs_f32(1.5));
>  stopwatch.reset();
>  assert_eq!(stopwatch.elapsed_secs(), 0.0);
>  ```

#### Arguments

- **\_self** : `Stopwatch` \- No Documentation 🚧

#### Returns

- **arg0** : `()` \- No Documentation 🚧