# degrees

>  Creates a [`Rot2`] from a counterclockwise angle in degrees.
>  # Note
>  The input rotation will always be clamped to the range `(-180°, 180°]` by design.
>  # Example
>  ```
>  # use bevy_math::Rot2;
>  # use approx::assert_relative_eq;
>  let rot1 = Rot2::degrees(270.0);
>  let rot2 = Rot2::degrees(-90.0);
>  assert_relative_eq!(rot1, rot2);
>  let rot3 = Rot2::degrees(180.0);
>  assert_relative_eq!(rot1 * rot1, rot3);
>  ```

#### Arguments

- **degrees** : `f32` \- No Documentation 🚧

#### Returns

- **arg0** : `Rot2` \- No Documentation 🚧