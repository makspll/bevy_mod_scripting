# saturating\_sub

>  Saturating `Duration` subtraction. Computes `self - other`, returning [`Duration::ZERO`]
>  if the result would be negative or if overflow occurred.
>  # Examples
>  ```
>  use std::time::Duration;
>  assert_eq!(Duration::new(0, 1).saturating_sub(Duration::new(0, 0)), Duration::new(0, 1));
>  assert_eq!(Duration::new(0, 0).saturating_sub(Duration::new(0, 1)), Duration::ZERO);
>  ```

#### Arguments

- **\_self** : `Duration` \- No Documentation 🚧
- **rhs** : `Duration` \- No Documentation 🚧

#### Returns

- **arg0** : `Duration` \- No Documentation 🚧