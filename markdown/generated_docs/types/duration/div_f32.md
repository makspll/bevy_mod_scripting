# div\_f32

>  Divides `Duration` by `f32`.
>  # Panics
>  This method will panic if result is negative, overflows `Duration` or not finite.
>  # Examples
>  ```
>  use std::time::Duration;
>  let dur = Duration::new(2, 700_000_000);
>  // note that due to rounding errors result is slightly
>  // different from 0.859_872_611
>  assert_eq!(dur.div_f32(3.14), Duration::new(0, 859_872_580));
>  assert_eq!(dur.div_f32(3.14e5), Duration::new(0, 8_599));
>  ```

#### Arguments

- **\_self** : `Duration` \- No Documentation 🚧
- **rhs** : `f32` \- No Documentation 🚧

#### Returns

- **arg0** : `Duration` \- No Documentation 🚧