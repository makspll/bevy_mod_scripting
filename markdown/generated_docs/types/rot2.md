# Rot2

### Rot2

- **cos** : f32
- **sin** : f32

## Description

>  A counterclockwise 2D rotation.
> 
>  # Example
> 
>  ```
>  # use approx::assert_relative_eq;
>  # use bevy_math::{Rot2, Vec2};
>  use std::f32::consts::PI;
> 
>  // Create rotations from radians or degrees
>  let rotation1 = Rot2::radians(PI / 2.0);
>  let rotation2 = Rot2::degrees(45.0);
> 
>  // Get the angle back as radians or degrees
>  assert_eq!(rotation1.as_degrees(), 90.0);
>  assert_eq!(rotation2.as_radians(), PI / 4.0);
> 
>  // "Add" rotations together using `*`
>  assert_relative_eq!(rotation1 * rotation2, Rot2::degrees(135.0));
> 
>  // Rotate vectors
>  assert_relative_eq!(rotation1 * Vec2::X, Vec2::Y);
>  ```

## Functions

| Function | Summary |
| --- | --- |
| `angle_between(_self, other)` | [ Returns the angle in radians needed to make \`self\` and \`other\` coincide\.](./rot2/angle_between.md) |
| `angle_to(_self, other)` | [ Returns the angle in radians needed to make \`self\` and \`other\` coincide\.](./rot2/angle_to.md) |
| `as_degrees(_self)` | [ Returns the rotation in degrees in the \`\(\-180, 180\]\` range\.](./rot2/as_degrees.md) |
| `as_radians(_self)` | [ Returns the rotation in radians in the \`\(\-pi, pi\]\` range\.](./rot2/as_radians.md) |
| `as_turn_fraction(_self)` | [ Returns the rotation as a fraction of a full 360 degree turn\.](./rot2/as_turn_fraction.md) |
| `clone(_self)` | [No Documentation 🚧](./rot2/clone.md) |
| `degrees(degrees)` | [ Creates a \[\`Rot2\`\] from a counterclockwise angle in degrees\.  \# Note  The input rotation will always be clamped to the range \`\(\-180°, 180°\]](./rot2/degrees.md) |
| `eq(_self, other)` | [No Documentation 🚧](./rot2/eq.md) |
| `fast_renormalize(_self)` | [ Returns \`self\` after an approximate normalization, assuming the value is already nearly normalized\.](./rot2/fast_renormalize.md) |
| `from_sin_cos(sin, cos)` | [ Creates a \[\`Rot2\`\] from the sine and cosine of an angle in radians\.  The rotation is only valid if \`sin \* sin \+ cos \* cos == 1\.0\`\.  \# Panics  Panics if \`sin \* sin \+ cos \* cos \!= 1\.0\` when the \`glam\_assert\` feature is enabled\.](./rot2/from_sin_cos.md) |
| `inverse(_self)` | [ Returns the inverse of the rotation\. This is also the conjugate  of the unit complex number represe](./rot2/inverse.md) |
| `is_finite(_self)` | [ Returns \`true\` if the rotation is neither infinite nor NaN\.](./rot2/is_finite.md) |
| `is_nan(_self)` | [ Returns \`true\` if the rotation is NaN\.](./rot2/is_nan.md) |
| `is_near_identity(_self)` | [ Returns \`true\` if the rotation is near \[\`Rot2::IDENTITY\`\]\.](./rot2/is_near_identity.md) |
| `is_normalized(_self)` | [ Returns whether \`self\` has a length of \`1\.0\` or not\.  Uses a precision threshold of approximately \`1e\-4\`](./rot2/is_normalized.md) |
| `length(_self)` | [ Computes the length or norm of the complex number used to represent the rotation\.  The length is ty](./rot2/length.md) |
| `length_recip(_self)` | [ Computes \`1\.0 / self\.length\(\)\`\.  For valid results, \`self\` must \_not\_ have a length of zero\.](./rot2/length_recip.md) |
| `length_squared(_self)` | [ Computes the squared length or norm of the complex number used to represent the rotation\.  This is ](./rot2/length_squared.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./rot2/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./rot2/mul-1.md) |
| `mul-2(arg0, arg1)` | [No Documentation 🚧](./rot2/mul-2.md) |
| `nlerp(_self, end, s)` | [ Performs a linear interpolation between \`self\` and \`rhs\` based on  the value \`s\`, and normalizes th](./rot2/nlerp.md) |
| `normalize(_self)` | [ Returns \`self\` with a length of \`1\.0\`\.  Note that \[\`Rot2\`\] should typically already be normalized by design\.  Manual normalization is only needed when successive operations result in  accumulated floating point error, or if the rotation was constructed  with invalid values\.  \# Panics  Panics if \`self\` has a length of zero, NaN, or infinity when debug assertions are enabled\.](./rot2/normalize.md) |
| `radians(radians)` | [ Creates a \[\`Rot2\`\] from a counterclockwise angle in radians\.  \# Note  The input rotation will always be clamped to the range \`\(\-π, π\]](./rot2/radians.md) |
| `sin_cos(_self)` | [ Returns the sine and cosine of the rotation angle in radians\.](./rot2/sin_cos.md) |
| `slerp(_self, end, s)` | [ Performs a spherical linear interpolation between \`self\` and \`end\`  based on the value \`s\`\.  This c](./rot2/slerp.md) |
| `turn_fraction(fraction)` | [ Creates a \[\`Rot2\`\] from a counterclockwise fraction of a full turn of 360 degrees\.  \# Note  The input rotation will always be clamped to the range \`\(\-50%, 50%\]](./rot2/turn_fraction.md) |