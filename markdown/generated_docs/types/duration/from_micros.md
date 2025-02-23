# from\_micros

>  Creates a new `Duration` from the specified number of microseconds.
>  # Examples
>  ```
>  use std::time::Duration;
>  let duration = Duration::from_micros(1_000_002);
>  assert_eq!(1, duration.as_secs());
>  assert_eq!(2_000, duration.subsec_nanos());
>  ```

#### Arguments

- **micros** : `u64` \- No Documentation 🚧

#### Returns

- **arg0** : `Duration` \- No Documentation 🚧