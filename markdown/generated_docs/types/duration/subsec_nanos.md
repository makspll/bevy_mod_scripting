# subsec\_nanos

>  Returns the fractional part of this `Duration`, in nanoseconds.
>  This method does **not** return the length of the duration when
>  represented by nanoseconds. The returned number always represents a
>  fractional portion of a second (i.e., it is less than one billion).
>  # Examples
>  ```
>  use std::time::Duration;
>  let duration = Duration::from_millis(5_010);
>  assert_eq!(duration.as_secs(), 5);
>  assert_eq!(duration.subsec_nanos(), 10_000_000);
>  ```

#### Arguments

- **\_self** : `Duration` \- No Documentation 🚧

#### Returns

- **arg0** : `u32` \- No Documentation 🚧