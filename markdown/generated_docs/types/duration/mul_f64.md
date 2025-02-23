# mul\_f64

>  Multiplies `Duration` by `f64`.
>  # Panics
>  This method will panic if result is negative, overflows `Duration` or not finite.
>  # Examples
>  ```
>  use std::time::Duration;
>  let dur = Duration::new(2, 700_000_000);
>  assert_eq!(dur.mul_f64(3.14), Duration::new(8, 478_000_000));
>  assert_eq!(dur.mul_f64(3.14e5), Duration::new(847_800, 0));
>  ```

#### Arguments

- **\_self** : `Duration` \- No Documentation 🚧
- **rhs** : `f64` \- No Documentation 🚧

#### Returns

- **arg0** : `Duration` \- No Documentation 🚧