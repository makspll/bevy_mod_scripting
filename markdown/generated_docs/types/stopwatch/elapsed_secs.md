# elapsed\_secs

>  Returns the elapsed time since the last [`reset`](Stopwatch::reset)
>  of the stopwatch, in seconds.
>  # Examples
>  ```
>  # use bevy_time::*;
>  use std::time::Duration;
>  let mut stopwatch = Stopwatch::new();
>  stopwatch.tick(Duration::from_secs(1));
>  assert_eq!(stopwatch.elapsed_secs(), 1.0);
>  ```
>  # See Also
>  [`elapsed`](Stopwatch::elapsed) - if a `Duration` is desirable instead.
>  [`elapsed_secs_f64`](Stopwatch::elapsed_secs_f64) - if an `f64` is desirable instead.

#### Arguments

- **\_self** : `Stopwatch` \- No Documentation 🚧

#### Returns

- **arg0** : `f32` \- No Documentation 🚧