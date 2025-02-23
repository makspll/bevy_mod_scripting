# radians

>  Creates a [`Rot2`] from a counterclockwise angle in radians.
>  # Note
>  The input rotation will always be clamped to the range `(-π, π]` by design.
>  # Example
>  ```
>  # use bevy_math::Rot2;
>  # use approx::assert_relative_eq;
>  # use std::f32::consts::{FRAC_PI_2, PI};
>  let rot1 = Rot2::radians(3.0 * FRAC_PI_2);
>  let rot2 = Rot2::radians(-FRAC_PI_2);
>  assert_relative_eq!(rot1, rot2);
>  let rot3 = Rot2::radians(PI);
>  assert_relative_eq!(rot1 * rot1, rot3);
>  ```

#### Arguments

- **radians** : `f32` \- No Documentation 🚧

#### Returns

- **arg0** : `Rot2` \- No Documentation 🚧