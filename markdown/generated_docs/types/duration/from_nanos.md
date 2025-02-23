# from\_nanos

>  Creates a new `Duration` from the specified number of nanoseconds.
>  Note: Using this on the return value of `as_nanos()` might cause unexpected behavior:
>  `as_nanos()` returns a u128, and can return values that do not fit in u64, e.g. 585 years.
>  Instead, consider using the pattern `Duration::new(d.as_secs(), d.subsec_nanos())`
>  if you cannot copy/clone the Duration directly.
>  # Examples
>  ```
>  use std::time::Duration;
>  let duration = Duration::from_nanos(1_000_000_123);
>  assert_eq!(1, duration.as_secs());
>  assert_eq!(123, duration.subsec_nanos());
>  ```

#### Arguments

- **nanos** : `u64` \- No Documentation 🚧

#### Returns

- **arg0** : `Duration` \- No Documentation 🚧