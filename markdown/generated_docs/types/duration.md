# Duration

Opaque Type\. 🔒

## Description

> No Documentation 🚧

## Functions

| Function | Summary |
| --- | --- |
| `abs_diff(_self, other)` | [ Computes the absolute difference between \`self\` and \`other\`\.  \# Examples  \`\`\`  use std::time::Duration;  assert\_eq\!\(Duration::new\(100, 0\)\.abs\_diff\(Duration::new\(80, 0\)\), Duration::new\(20, 0\)\);  assert\_eq\!\(Duration::new\(100, 400\_000\_000\)\.abs\_diff\(Duration::new\(110, 0\)\), Duration::new\(9, 600\_000\_000\)\);  \`](./duration/abs_diff.md) |
| `add(_self, rhs)` | [No Documentation 🚧](./duration/add.md) |
| `as_micros(_self)` | [ Returns the total number of whole microseconds contained by this \`Duration\`\.  \# Examples  \`\`\`  use std::time::Duration;  let duration = Duration::new\(5, 730\_023\_852\);  assert\_eq\!\(duration\.as\_micros\(\), 5\_730\_023\);  \`](./duration/as_micros.md) |
| `as_millis(_self)` | [ Returns the total number of whole milliseconds contained by this \`Duration\`\.  \# Examples  \`\`\`  use std::time::Duration;  let duration = Duration::new\(5, 730\_023\_852\);  assert\_eq\!\(duration\.as\_millis\(\), 5\_730\);  \`\`\`](./duration/as_millis.md) |
| `as_nanos(_self)` | [ Returns the total number of nanoseconds contained by this \`Duration\`\.  \# Examples  \`\`\`  use std::time::Duration;  let duration = Duration::new\(5, 730\_023\_852\);  assert\_eq\!\(duration\.as\_nanos\(\), 5\_730\_023\_852\);  \`\`\`](./duration/as_nanos.md) |
| `as_secs(_self)` | [ Returns the number of \_whole\_ seconds contained by this \`Duration\`\.  The returned value does not in](./duration/as_secs.md) |
| `as_secs_f32(_self)` | [ Returns the number of seconds contained by this \`Duration\` as \`f32\`\.  The returned value includes t](./duration/as_secs_f32.md) |
| `as_secs_f64(_self)` | [ Returns the number of seconds contained by this \`Duration\` as \`f64\`\.  The returned value includes t](./duration/as_secs_f64.md) |
| `assert_receiver_is_total_eq(_self)` | [No Documentation 🚧](./duration/assert_receiver_is_total_eq.md) |
| `clone(_self)` | [No Documentation 🚧](./duration/clone.md) |
| `div(_self, rhs)` | [No Documentation 🚧](./duration/div.md) |
| `div_duration_f32(_self, rhs)` | [ Divides \`Duration\` by \`Duration\` and returns \`f32\`\.  \# Examples  \`\`\`  use std::time::Duration;  let dur1 = Duration::new\(2, 700\_000\_000\);  let dur2 = Duration::new\(5, 400\_000\_000\);  assert\_eq\!\(dur1\.div\_duration\_f32\(dur2\), 0\.5\);  \`\`\`](./duration/div_duration_f32.md) |
| `div_duration_f64(_self, rhs)` | [ Divides \`Duration\` by \`Duration\` and returns \`f64\`\.  \# Examples  \`\`\`  use std::time::Duration;  let dur1 = Duration::new\(2, 700\_000\_000\);  let dur2 = Duration::new\(5, 400\_000\_000\);  assert\_eq\!\(dur1\.div\_duration\_f64\(dur2\), 0\.5\);  \`\`\`](./duration/div_duration_f64.md) |
| `div_f32(_self, rhs)` | [ Divides \`Duration\` by \`f32\`\.  \# Panics  This method will panic if result is negative, overflows \`Duration\`](./duration/div_f32.md) |
| `div_f64(_self, rhs)` | [ Divides \`Duration\` by \`f64\`\.  \# Panics  This method will panic if result is negative, overflows \`Duration\`](./duration/div_f64.md) |
| `eq(_self, other)` | [No Documentation 🚧](./duration/eq.md) |
| `from_micros(micros)` | [ Creates a new \`Duration\` from the specified number of microseconds\.  \# Examples  \`\`\`  use std::time::Duration;  let duration = Duration::from\_micros\(1\_000\_002\);  assert\_eq\!\(1, duration\.as\_secs\(\)\);  assert\_eq\!\(2\_000, duration\.subsec\_nanos\(\)\);  \`](./duration/from_micros.md) |
| `from_millis(millis)` | [ Creates a new \`Duration\` from the specified number of milliseconds\.  \# Examples  \`\`\`  use std::time::Duration;  let duration = Duration::from\_millis\(2\_569\);  assert\_eq\!\(2, duration\.as\_secs\(\)\);  assert\_eq\!\(569\_000\_000, duration\.subsec\_nanos\(\)\);  \`](./duration/from_millis.md) |
| `from_nanos(nanos)` | [ Creates a new \`Duration\` from the specified number of nanoseconds\.  Note: Using this on the return ](./duration/from_nanos.md) |
| `from_secs(secs)` | [ Creates a new \`Duration\` from the specified number of whole seconds\.  \# Examples  \`\`\`  use std::time::Duration;  let duration = Duration::from\_secs\(5\);  assert\_eq\!\(5, duration\.as\_secs\(\)\);  assert\_eq\!\(0, duration\.subsec\_nanos\(\)\);  \`\`\`](./duration/from_secs.md) |
| `from_secs_f32(secs)` | [ Creates a new \`Duration\` from the specified number of seconds represented  as \`f32\`\.  \# Panics  Thi](./duration/from_secs_f32.md) |
| `from_secs_f64(secs)` | [ Creates a new \`Duration\` from the specified number of seconds represented  as \`f64\`\.  \# Panics  Thi](./duration/from_secs_f64.md) |
| `is_zero(_self)` | [ Returns true if this \`Duration\` spans no time\.  \# Examples  \`\`\`  use std::time::Duration;  assert\!\(Duration::ZERO\.is\_zero\(\)\);  assert\!\(Duration::new\(0, 0\)\.is\_zero\(\)\);  assert\!\(Duration::from\_nanos\(0\)\.is\_zero\(\)\);  assert\!\(Duration::from\_secs\(0\)\.is\_zero\(\)\);  assert\!\(\!Duration::new\(1, 1\)\.is\_zero\(\)\);  assert\!\(\!Duration::from\_nanos\(1\)\.is\_zero\(\)\);  assert\!\(\!Duration::from\_secs\(1\)\.is\_zero\(\)\);  \`\`\`](./duration/is_zero.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./duration/mul.md) |
| `mul_f32(_self, rhs)` | [ Multiplies \`Duration\` by \`f32\`\.  \# Panics  This method will panic if result is negative, overflows \`Duration\`](./duration/mul_f32.md) |
| `mul_f64(_self, rhs)` | [ Multiplies \`Duration\` by \`f64\`\.  \# Panics  This method will panic if result is negative, overflows \`Duration\`](./duration/mul_f64.md) |
| `new(secs, nanos)` | [ Creates a new \`Duration\` from the specified number of whole seconds and  additional nanoseconds\.  I](./duration/new.md) |
| `saturating_add(_self, rhs)` | [ Saturating \`Duration\` addition\. Computes \`self \+ other\`, returning \[\`Duration::MAX\`\]  if overflow occurred\.  \# Examples  \`\`\`  \#\!\[feature\(duration\_constants\)\]](./duration/saturating_add.md) |
| `saturating_mul(_self, rhs)` | [ Saturating \`Duration\` multiplication\. Computes \`self \* other\`, returning  \[\`Duration::MAX\`\] if overflow occurred\.  \# Examples  \`\`\`  \#\!\[feature\(duration\_constants\)\]  use std::time::Duration;  assert\_eq\!\(Duration::new\(0, 500\_000\_001\)\.saturating\_mul\(2\), Duration::new\(1, 2\)\);  assert\_eq\!\(Duration::new\(u64::MAX \- 1, 0\)\.saturating\_mul\(2\), Duration::MAX\);  \`\`\`](./duration/saturating_mul.md) |
| `saturating_sub(_self, rhs)` | [ Saturating \`Duration\` subtraction\. Computes \`self \- other\`, returning \[\`Duration::ZERO\`\]  if the result would be negative or if overflow occurred\.  \# Examples  \`\`\`  use std::time::Duration;  assert\_eq\!\(Duration::new\(0, 1\)\.saturating\_sub\(Duration::new\(0, 0\)\), Duration::new\(0, 1\)\);  assert\_eq\!\(Duration::new\(0, 0\)\.saturating\_sub\(Duration::new\(0, 1\)\), Duration::ZERO\);  \`\`\`](./duration/saturating_sub.md) |
| `sub(_self, rhs)` | [No Documentation 🚧](./duration/sub.md) |
| `subsec_micros(_self)` | [ Returns the fractional part of this \`Duration\`, in whole microseconds\.  This method does \*\*not\*\* re](./duration/subsec_micros.md) |
| `subsec_millis(_self)` | [ Returns the fractional part of this \`Duration\`, in whole milliseconds\.  This method does \*\*not\*\* re](./duration/subsec_millis.md) |
| `subsec_nanos(_self)` | [ Returns the fractional part of this \`Duration\`, in nanoseconds\.  This method does \*\*not\*\* return th](./duration/subsec_nanos.md) |