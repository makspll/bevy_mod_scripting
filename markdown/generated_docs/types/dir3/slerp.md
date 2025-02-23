# slerp

>  Performs a spherical linear interpolation between `self` and `rhs`
>  based on the value `s`.
>  This corresponds to interpolating between the two directions at a constant angular velocity.
>  When `s == 0.0`, the result will be equal to `self`.
>  When `s == 1.0`, the result will be equal to `rhs`.
>  # Example
>  ```
>  # use bevy_math::Dir3;
>  # use approx::{assert_relative_eq, RelativeEq};
>  #
>  let dir1 = Dir3::X;
>  let dir2 = Dir3::Y;
>  let result1 = dir1.slerp(dir2, 1.0 / 3.0);
>  assert_relative_eq!(
>      result1,
>      Dir3::from_xyz(0.75_f32.sqrt(), 0.5, 0.0).unwrap(),
>      epsilon = 0.000001
>  );
>  let result2 = dir1.slerp(dir2, 0.5);
>  assert_relative_eq!(result2, Dir3::from_xyz(0.5_f32.sqrt(), 0.5_f32.sqrt(), 0.0).unwrap());
>  ```

#### Arguments

- **\_self** : `Dir3` \- No Documentation 🚧
- **rhs** : `Dir3` \- No Documentation 🚧
- **s** : `f32` \- No Documentation 🚧

#### Returns

- **arg0** : `Dir3` \- No Documentation 🚧