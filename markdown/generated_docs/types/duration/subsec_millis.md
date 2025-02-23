# subsec\_millis

>  Returns the fractional part of this `Duration`, in whole milliseconds.
>  This method does **not** return the length of the duration when
>  represented by milliseconds. The returned number always represents a
>  fractional portion of a second (i.e., it is less than one thousand).
>  # Examples
>  ```
>  use std::time::Duration;
>  let duration = Duration::from_millis(5_432);
>  assert_eq!(duration.as_secs(), 5);
>  assert_eq!(duration.subsec_millis(), 432);
>  ```

#### Arguments

- **\_self** : `Duration` \- No Documentation 🚧

#### Returns

- **arg0** : `u32` \- No Documentation 🚧