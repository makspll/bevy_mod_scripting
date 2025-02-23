# is\_zero

>  Returns true if this `Duration` spans no time.
>  # Examples
>  ```
>  use std::time::Duration;
>  assert!(Duration::ZERO.is_zero());
>  assert!(Duration::new(0, 0).is_zero());
>  assert!(Duration::from_nanos(0).is_zero());
>  assert!(Duration::from_secs(0).is_zero());
>  assert!(!Duration::new(1, 1).is_zero());
>  assert!(!Duration::from_nanos(1).is_zero());
>  assert!(!Duration::from_secs(1).is_zero());
>  ```

#### Arguments

- **\_self** : `Duration` \- No Documentation 🚧

#### Returns

- **arg0** : `bool` \- No Documentation 🚧