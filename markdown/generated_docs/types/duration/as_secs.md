# as\_secs

>  Returns the number of _whole_ seconds contained by this `Duration`.
>  The returned value does not include the fractional (nanosecond) part of the
>  duration, which can be obtained using [`subsec_nanos`].
>  # Examples
>  ```
>  use std::time::Duration;
>  let duration = Duration::new(5, 730_023_852);
>  assert_eq!(duration.as_secs(), 5);
>  ```
>  To determine the total number of seconds represented by the `Duration`
>  including the fractional part, use [`as_secs_f64`] or [`as_secs_f32`]
>  [`as_secs_f64`]: Duration::as_secs_f64
>  [`as_secs_f32`]: Duration::as_secs_f32
>  [`subsec_nanos`]: Duration::subsec_nanos

#### Arguments

- **\_self** : `Duration` \- No Documentation 🚧

#### Returns

- **arg0** : `u64` \- No Documentation 🚧