# div\_f64

>  Divides `Duration` by `f64`.
>  # Panics
>  This method will panic if result is negative, overflows `Duration` or not finite.
>  # Examples
>  ```
>  use std::time::Duration;
>  let dur = Duration::new(2, 700_000_000);
>  assert_eq!(dur.div_f64(3.14), Duration::new(0, 859_872_611));
>  assert_eq!(dur.div_f64(3.14e5), Duration::new(0, 8_599));
>  ```

#### Arguments

- **\_self** : `Duration` \- No Documentation 🚧
- **rhs** : `f64` \- No Documentation 🚧

#### Returns

- **arg0** : `Duration` \- No Documentation 🚧