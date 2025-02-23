# fast\_renormalize

>  Returns `self` after an approximate normalization, assuming the value is already nearly normalized.
>  Useful for preventing numerical error accumulation.
>  # Example
>  The following seemingly benign code would start accumulating errors over time,
>  leading to `dir` eventually not being normalized anymore.
>  ```
>  # use bevy_math::prelude::*;
>  # let N: usize = 200;
>  let mut dir = Dir3::X;
>  let quaternion = Quat::from_euler(EulerRot::XYZ, 1.0, 2.0, 3.0);
>  for i in 0..N {
>      dir = quaternion * dir;
>  }
>  ```
>  Instead, do the following.
>  ```
>  # use bevy_math::prelude::*;
>  # let N: usize = 200;
>  let mut dir = Dir3::X;
>  let quaternion = Quat::from_euler(EulerRot::XYZ, 1.0, 2.0, 3.0);
>  for i in 0..N {
>      dir = quaternion * dir;
>      dir = dir.fast_renormalize();
>  }
>  ```

#### Arguments

- **\_self** : `Dir3` \- No Documentation 🚧

#### Returns

- **arg0** : `Dir3` \- No Documentation 🚧