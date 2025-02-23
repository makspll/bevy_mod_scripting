# from\_secs\_f32

>  Creates a new `Duration` from the specified number of seconds represented
>  as `f32`.
>  # Panics
>  This constructor will panic if `secs` is negative, overflows `Duration` or not finite.
>  # Examples
>  ```
>  use std::time::Duration;
>  let res = Duration::from_secs_f32(0.0);
>  assert_eq!(res, Duration::new(0, 0));
>  let res = Duration::from_secs_f32(1e-20);
>  assert_eq!(res, Duration::new(0, 0));
>  let res = Duration::from_secs_f32(4.2e-7);
>  assert_eq!(res, Duration::new(0, 420));
>  let res = Duration::from_secs_f32(2.7);
>  assert_eq!(res, Duration::new(2, 700_000_048));
>  let res = Duration::from_secs_f32(3e10);
>  assert_eq!(res, Duration::new(30_000_001_024, 0));
>  // subnormal float
>  let res = Duration::from_secs_f32(f32::from_bits(1));
>  assert_eq!(res, Duration::new(0, 0));
>  // conversion uses rounding
>  let res = Duration::from_secs_f32(0.999e-9);
>  assert_eq!(res, Duration::new(0, 1));
>  ```

#### Arguments

- **secs** : `f32` \- No Documentation 🚧

#### Returns

- **arg0** : `Duration` \- No Documentation 🚧