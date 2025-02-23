# turn\_fraction

>  Creates a [`Rot2`] from a counterclockwise fraction of a full turn of 360 degrees.
>  # Note
>  The input rotation will always be clamped to the range `(-50%, 50%]` by design.
>  # Example
>  ```
>  # use bevy_math::Rot2;
>  # use approx::assert_relative_eq;
>  let rot1 = Rot2::turn_fraction(0.75);
>  let rot2 = Rot2::turn_fraction(-0.25);
>  assert_relative_eq!(rot1, rot2);
>  let rot3 = Rot2::turn_fraction(0.5);
>  assert_relative_eq!(rot1 * rot1, rot3);
>  ```

#### Arguments

- **fraction** : `f32` \- No Documentation 🚧

#### Returns

- **arg0** : `Rot2` \- No Documentation 🚧