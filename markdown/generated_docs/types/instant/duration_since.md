# duration\_since

>  Returns the amount of time elapsed from another instant to this one,
>  or zero duration if that instant is later than this one.
>  # Panics
>  Previous Rust versions panicked when `earlier` was later than `self`. Currently this
>  method saturates. Future versions may reintroduce the panic in some circumstances.
>  See [Monotonicity].
>  [Monotonicity]: Instant#monotonicity
>  # Examples
>  ```no_run
>  use std::time::{Duration, Instant};
>  use std::thread::sleep;
>  let now = Instant::now();
>  sleep(Duration::new(1, 0));
>  let new_now = Instant::now();
>  println!("{:?}", new_now.duration_since(now));
>  println!("{:?}", now.duration_since(new_now)); // 0ns
>  ```

#### Arguments

- **\_self** : `Instant` \- No Documentation 🚧
- **earlier** : `Instant` \- No Documentation 🚧

#### Returns

- **arg0** : `Duration` \- No Documentation 🚧