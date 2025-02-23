# saturating\_add

>  Saturating `Duration` addition. Computes `self + other`, returning [`Duration::MAX`]
>  if overflow occurred.
>  # Examples
>  ```
>  #![feature(duration_constants)]
>  use std::time::Duration;
>  assert_eq!(Duration::new(0, 0).saturating_add(Duration::new(0, 1)), Duration::new(0, 1));
>  assert_eq!(Duration::new(1, 0).saturating_add(Duration::new(u64::MAX, 0)), Duration::MAX);
>  ```

#### Arguments

- **\_self** : `Duration` \- No Documentation 🚧
- **rhs** : `Duration` \- No Documentation 🚧

#### Returns

- **arg0** : `Duration` \- No Documentation 🚧