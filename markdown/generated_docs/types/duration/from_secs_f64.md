# from\_secs\_f64

>  Creates a new `Duration` from the specified number of seconds represented
>  as `f64`.
>  # Panics
>  This constructor will panic if `secs` is negative, overflows `Duration` or not finite.
>  # Examples
>  ```
>  use std::time::Duration;
>  let res = Duration::from_secs_f64(0.0);
>  assert_eq!(res, Duration::new(0, 0));
>  let res = Duration::from_secs_f64(1e-20);
>  assert_eq!(res, Duration::new(0, 0));
>  let res = Duration::from_secs_f64(4.2e-7);
>  assert_eq!(res, Duration::new(0, 420));
>  let res = Duration::from_secs_f64(2.7);
>  assert_eq!(res, Duration::new(2, 700_000_000));
>  let res = Duration::from_secs_f64(3e10);
>  assert_eq!(res, Duration::new(30_000_000_000, 0));
>  // subnormal float
>  let res = Duration::from_secs_f64(f64::from_bits(1));
>  assert_eq!(res, Duration::new(0, 0));
>  // conversion uses rounding
>  let res = Duration::from_secs_f64(0.999e-9);
>  assert_eq!(res, Duration::new(0, 1));
>  ```

#### Arguments

- **secs** : `f64` \- No Documentation 🚧

#### Returns

- **arg0** : `Duration` \- No Documentation 🚧