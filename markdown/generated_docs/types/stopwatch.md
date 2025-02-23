# Stopwatch

### Stopwatch

- **elapsed** : bevy\_utils::Duration
- **is\_paused** : bool

## Description

>  A Stopwatch is a struct that tracks elapsed time when started.
> 
>  Note that in order to advance the stopwatch [`tick`](Stopwatch::tick) **MUST** be called.
>  # Examples
> 
>  ```
>  # use bevy_time::*;
>  use std::time::Duration;
>  let mut stopwatch = Stopwatch::new();
>  assert_eq!(stopwatch.elapsed_secs(), 0.0);
> 
>  stopwatch.tick(Duration::from_secs_f32(1.0)); // tick one second
>  assert_eq!(stopwatch.elapsed_secs(), 1.0);
> 
>  stopwatch.pause();
>  stopwatch.tick(Duration::from_secs_f32(1.0)); // paused stopwatches don't tick
>  assert_eq!(stopwatch.elapsed_secs(), 1.0);
> 
>  stopwatch.reset(); // reset the stopwatch
>  assert!(stopwatch.is_paused());
>  assert_eq!(stopwatch.elapsed_secs(), 0.0);
>  ```

## Functions

| Function | Summary |
| --- | --- |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./stopwatch/assert_receiver_is_total_eq.md) |
| `clone(_self)` | [No Documentation 🚧](./stopwatch/clone.md) |
| `elapsed(_self)` | [ Returns the elapsed time since the last \[\`reset\`\]\(Stopwatch::reset\)  of the stopwatch\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  use std::time::Duration;  let mut stopwatch = Stopwatch::new\(\);  stopwatch\.tick\(Duration::from\_secs\(1\)\);  assert\_eq\!\(stopwatch\.elapsed\(\), Duration::from\_secs\(1\)\);  \`\`\`  \# See Also  \[\`elapsed\_secs\`\]\(Stopwatch::elapsed\_secs\)](./stopwatch/elapsed.md) |
| `elapsed_secs(_self)` | [ Returns the elapsed time since the last \[\`reset\`\]\(Stopwatch::reset\)  of the stopwatch, in seconds\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  use std::time::Duration;  let mut stopwatch = Stopwatch::new\(\);  stopwatch\.tick\(Duration::from\_secs\(1\)\);  assert\_eq\!\(stopwatch\.elapsed\_secs\(\), 1\.0\);  \`\`\`  \# See Also  \[\`elapsed\`\]\(Stopwatch::elapsed\)](./stopwatch/elapsed_secs.md) |
| `elapsed_secs_f64(_self)` | [ Returns the elapsed time since the last \[\`reset\`\]\(Stopwatch::reset\)  of the stopwatch, in seconds, as f64\.  \# See Also  \[\`elapsed\`\]\(Stopwatch::elapsed\)](./stopwatch/elapsed_secs_f64.md) |
| `eq(_self, other)` | [No Documentation 🚧](./stopwatch/eq.md) |
| `is_paused(_self)` | [ Returns \`true\` if the stopwatch is paused\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  let mut stopwatch = Stopwatch::new\(\);  assert\!\(\!stopwatch\.is\_paused\(\)\);  stopwatch\.pause\(\);  assert\!\(stopwatch\.is\_paused\(\)\);  stopwatch\.unpause\(\);  assert\!\(\!stopwatch\.is\_paused\(\)\);  \`\`\`](./stopwatch/is_paused.md) |
| `new()` | [ Create a new unpaused \`Stopwatch\` with no elapsed time\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  let stopwatch = Stopwatch::new\(\);  assert\_eq\!\(stopwatch\.elapsed\_secs\(\), 0\.0\);  assert\_eq\!\(stopwatch\.is\_paused\(\), false\);  \`\`\`](./stopwatch/new.md) |
| `pause(_self)` | [ Pauses the stopwatch\. Any call to \[\`tick\`\]\(Stopwatch::tick\) while  paused will not have any effect on the elapsed time\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  use std::time::Duration;  let mut stopwatch = Stopwatch::new\(\);  stopwatch\.pause\(\);  stopwatch\.tick\(Duration::from\_secs\_f32\(1\.5\)\);  assert\!\(stopwatch\.is\_paused\(\)\);  assert\_eq\!\(stopwatch\.elapsed\_secs\(\), 0\.0\);  \`\`\`](./stopwatch/pause.md) |
| `reset(_self)` | [ Resets the stopwatch\. The reset doesn't affect the paused state of the stopwatch\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  use std::time::Duration;  let mut stopwatch = Stopwatch::new\(\);  stopwatch\.tick\(Duration::from\_secs\_f32\(1\.5\)\);  stopwatch\.reset\(\);  assert\_eq\!\(stopwatch\.elapsed\_secs\(\), 0\.0\);  \`\`\`](./stopwatch/reset.md) |
| `set_elapsed(_self, time)` | [ Sets the elapsed time of the stopwatch\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  use std::time::Duration;  let mut stopwatch = Stopwatch::new\(\);  stopwatch\.set\_elapsed\(Duration::from\_secs\_f32\(1\.0\)\);  assert\_eq\!\(stopwatch\.elapsed\_secs\(\), 1\.0\);  \`\`\`](./stopwatch/set_elapsed.md) |
| `unpause(_self)` | [ Unpauses the stopwatch\. Resume the effect of ticking on elapsed time\.  \# Examples  \`\`\`  \# use bevy\_time::\*;  use std::time::Duration;  let mut stopwatch = Stopwatch::new\(\);  stopwatch\.pause\(\);  stopwatch\.tick\(Duration::from\_secs\_f32\(1\.0\)\);  stopwatch\.unpause\(\);  stopwatch\.tick\(Duration::from\_secs\_f32\(1\.0\)\);  assert\!\(\!stopwatch\.is\_paused\(\)\);  assert\_eq\!\(stopwatch\.elapsed\_secs\(\), 1\.0\);  \`\`\`](./stopwatch/unpause.md) |