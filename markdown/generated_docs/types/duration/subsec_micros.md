# subsec\_micros

>  Returns the fractional part of this `Duration`, in whole microseconds.
>  This method does **not** return the length of the duration when
>  represented by microseconds. The returned number always represents a
>  fractional portion of a second (i.e., it is less than one million).
>  # Examples
>  ```
>  use std::time::Duration;
>  let duration = Duration::from_micros(1_234_567);
>  assert_eq!(duration.as_secs(), 1);
>  assert_eq!(duration.subsec_micros(), 234_567);
>  ```

#### Arguments

- **\_self** : `Duration` \- No Documentation 🚧

#### Returns

- **arg0** : `u32` \- No Documentation 🚧