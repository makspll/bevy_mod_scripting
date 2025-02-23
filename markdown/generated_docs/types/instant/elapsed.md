# elapsed

>  Returns the amount of time elapsed since this instant.
>  # Panics
>  Previous Rust versions panicked when the current time was earlier than self. Currently this
>  method returns a Duration of zero in that case. Future versions may reintroduce the panic.
>  See [Monotonicity].
>  [Monotonicity]: Instant#monotonicity
>  # Examples
>  ```no_run
>  use std::thread::sleep;
>  use std::time::{Duration, Instant};
>  let instant = Instant::now();
>  let three_secs = Duration::from_secs(3);
>  sleep(three_secs);
>  assert!(instant.elapsed() >= three_secs);
>  ```

#### Arguments

- **\_self** : `Instant` \- No Documentation 🚧

#### Returns

- **arg0** : `Duration` \- No Documentation 🚧