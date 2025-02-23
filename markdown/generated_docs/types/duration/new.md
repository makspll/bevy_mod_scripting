# new

>  Creates a new `Duration` from the specified number of whole seconds and
>  additional nanoseconds.
>  If the number of nanoseconds is greater than 1 billion (the number of
>  nanoseconds in a second), then it will carry over into the seconds provided.
>  # Panics
>  This constructor will panic if the carry from the nanoseconds overflows
>  the seconds counter.
>  # Examples
>  ```
>  use std::time::Duration;
>  let five_seconds = Duration::new(5, 0);
>  ```

#### Arguments

- **secs** : `u64` \- No Documentation 🚧
- **nanos** : `u32` \- No Documentation 🚧

#### Returns

- **arg0** : `Duration` \- No Documentation 🚧