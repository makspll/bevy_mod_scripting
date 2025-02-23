# saturating\_mul

>  Saturating `Duration` multiplication. Computes `self * other`, returning
>  [`Duration::MAX`] if overflow occurred.
>  # Examples
>  ```
>  #![feature(duration_constants)]
>  use std::time::Duration;
>  assert_eq!(Duration::new(0, 500_000_001).saturating_mul(2), Duration::new(1, 2));
>  assert_eq!(Duration::new(u64::MAX - 1, 0).saturating_mul(2), Duration::MAX);
>  ```

#### Arguments

- **\_self** : `Duration` \- No Documentation 🚧
- **rhs** : `u32` \- No Documentation 🚧

#### Returns

- **arg0** : `Duration` \- No Documentation 🚧