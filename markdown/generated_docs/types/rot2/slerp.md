# slerp

>  Performs a spherical linear interpolation between `self` and `end`
>  based on the value `s`.
>  This corresponds to interpolating between the two angles at a constant angular velocity.
>  When `s == 0.0`, the result will be equal to `self`.
>  When `s == 1.0`, the result will be equal to `rhs`.
>  If you would like the rotation to have a kind of ease-in-out effect, consider
>  using the slightly more efficient [`nlerp`](Self::nlerp) instead.
>  # Example
>  ```
>  # use bevy_math::Rot2;
>  #
>  let rot1 = Rot2::IDENTITY;
>  let rot2 = Rot2::degrees(135.0);
>  let result1 = rot1.slerp(rot2, 1.0 / 3.0);
>  assert_eq!(result1.as_degrees(), 45.0);
>  let result2 = rot1.slerp(rot2, 0.5);
>  assert_eq!(result2.as_degrees(), 67.5);
>  ```

#### Arguments

- **\_self** : `Rot2` \- No Documentation 🚧
- **end** : `Rot2` \- No Documentation 🚧
- **s** : `f32` \- No Documentation 🚧

#### Returns

- **arg0** : `Rot2` \- No Documentation 🚧