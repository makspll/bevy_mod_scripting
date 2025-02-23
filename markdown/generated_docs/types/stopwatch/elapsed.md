# elapsed

>  Returns the elapsed time since the last [`reset`](Stopwatch::reset)
>  of the stopwatch.
>  # Examples
>  ```
>  # use bevy_time::*;
>  use std::time::Duration;
>  let mut stopwatch = Stopwatch::new();
>  stopwatch.tick(Duration::from_secs(1));
>  assert_eq!(stopwatch.elapsed(), Duration::from_secs(1));
>  ```
>  # See Also
>  [`elapsed_secs`](Stopwatch::elapsed_secs) - if an `f32` value is desirable instead.
>  [`elapsed_secs_f64`](Stopwatch::elapsed_secs_f64) - if an `f64` is desirable instead.

#### Arguments

- **\_self** : `Stopwatch` \- No Documentation 🚧

#### Returns

- **arg0** : `Duration` \- No Documentation 🚧