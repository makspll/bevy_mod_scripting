# saturating\_duration\_since

>  Returns the amount of time elapsed from another instant to this one,
>  or zero duration if that instant is later than this one.
>  # Examples
>  ```no_run
>  use std::time::{Duration, Instant};
>  use std::thread::sleep;
>  let now = Instant::now();
>  sleep(Duration::new(1, 0));
>  let new_now = Instant::now();
>  println!("{:?}", new_now.saturating_duration_since(now));
>  println!("{:?}", now.saturating_duration_since(new_now)); // 0ns
>  ```

#### Arguments

- **\_self** : `Instant` \- No Documentation 🚧
- **earlier** : `Instant` \- No Documentation 🚧

#### Returns

- **arg0** : `Duration` \- No Documentation 🚧