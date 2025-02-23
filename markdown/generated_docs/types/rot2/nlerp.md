# nlerp

>  Performs a linear interpolation between `self` and `rhs` based on
>  the value `s`, and normalizes the rotation afterwards.
>  When `s == 0.0`, the result will be equal to `self`.
>  When `s == 1.0`, the result will be equal to `rhs`.
>  This is slightly more efficient than [`slerp`](Self::slerp), and produces a similar result
>  when the difference between the two rotations is small. At larger differences,
>  the result resembles a kind of ease-in-out effect.
>  If you would like the angular velocity to remain constant, consider using [`slerp`](Self::slerp) instead.
>  # Details
>  `nlerp` corresponds to computing an angle for a point at position `s` on a line drawn
>  between the endpoints of the arc formed by `self` and `rhs` on a unit circle,
>  and normalizing the result afterwards.
>  Note that if the angles are opposite like 0 and π, the line will pass through the origin,
>  and the resulting angle will always be either `self` or `rhs` depending on `s`.
>  If `s` happens to be `0.5` in this case, a valid rotation cannot be computed, and `self`
>  will be returned as a fallback.
>  # Example
>  ```
>  # use bevy_math::Rot2;
>  #
>  let rot1 = Rot2::IDENTITY;
>  let rot2 = Rot2::degrees(135.0);
>  let result1 = rot1.nlerp(rot2, 1.0 / 3.0);
>  assert_eq!(result1.as_degrees(), 28.675055);
>  let result2 = rot1.nlerp(rot2, 0.5);
>  assert_eq!(result2.as_degrees(), 67.5);
>  ```

#### Arguments

- **\_self** : `Rot2` \- No Documentation 🚧
- **end** : `Rot2` \- No Documentation 🚧
- **s** : `f32` \- No Documentation 🚧

#### Returns

- **arg0** : `Rot2` \- No Documentation 🚧