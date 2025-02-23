# Isometry3d

### Isometry3d

- **rotation** : glam::Quat
- **translation** : glam::Vec3A

## Description

>  An isometry in three dimensions, representing a rotation followed by a translation.
>  This can often be useful for expressing relative positions and transformations from one position to another.
> 
>  In particular, this type represents a distance-preserving transformation known as a *rigid motion* or a *direct motion*,
>  and belongs to the special [Euclidean group] SE(3). This includes translation and rotation, but excludes reflection.
> 
>  For the two-dimensional version, see [`Isometry2d`].
> 
>  [Euclidean group]: https://en.wikipedia.org/wiki/Euclidean_group
> 
>  # Example
> 
>  Isometries can be created from a given translation and rotation:
> 
>  ```
>  # use bevy_math::{Isometry3d, Quat, Vec3};
>  # use std::f32::consts::FRAC_PI_2;
>  #
>  let iso = Isometry3d::new(Vec3::new(2.0, 1.0, 3.0), Quat::from_rotation_z(FRAC_PI_2));
>  ```
> 
>  Or from separate parts:
> 
>  ```
>  # use bevy_math::{Isometry3d, Quat, Vec3};
>  # use std::f32::consts::FRAC_PI_2;
>  #
>  let iso1 = Isometry3d::from_translation(Vec3::new(2.0, 1.0, 3.0));
>  let iso2 = Isometry3d::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
>  ```
> 
>  The isometries can be used to transform points:
> 
>  ```
>  # use approx::assert_relative_eq;
>  # use bevy_math::{Isometry3d, Quat, Vec3};
>  # use std::f32::consts::FRAC_PI_2;
>  #
>  let iso = Isometry3d::new(Vec3::new(2.0, 1.0, 3.0), Quat::from_rotation_z(FRAC_PI_2));
>  let point = Vec3::new(4.0, 4.0, 4.0);
> 
>  // These are equivalent
>  let result = iso.transform_point(point);
>  let result = iso * point;
> 
>  assert_relative_eq!(result, Vec3::new(-2.0, 5.0, 7.0));
>  ```
> 
>  Isometries can also be composed together:
> 
>  ```
>  # use bevy_math::{Isometry3d, Quat, Vec3};
>  # use std::f32::consts::FRAC_PI_2;
>  #
>  # let iso = Isometry3d::new(Vec3::new(2.0, 1.0, 3.0), Quat::from_rotation_z(FRAC_PI_2));
>  # let iso1 = Isometry3d::from_translation(Vec3::new(2.0, 1.0, 3.0));
>  # let iso2 = Isometry3d::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
>  #
>  assert_eq!(iso1 * iso2, iso);
>  ```
> 
>  One common operation is to compute an isometry representing the relative positions of two objects
>  for things like intersection tests. This can be done with an inverse transformation:
> 
>  ```
>  # use bevy_math::{Isometry3d, Quat, Vec3};
>  # use std::f32::consts::FRAC_PI_2;
>  #
>  let sphere_iso = Isometry3d::from_translation(Vec3::new(2.0, 1.0, 3.0));
>  let cuboid_iso = Isometry3d::from_rotation(Quat::from_rotation_z(FRAC_PI_2));
> 
>  // Compute the relative position and orientation between the two shapes
>  let relative_iso = sphere_iso.inverse() * cuboid_iso;
> 
>  // Or alternatively, to skip an extra rotation operation:
>  let relative_iso = sphere_iso.inverse_mul(cuboid_iso);
>  ```

## Functions

| Function | Summary |
| --- | --- |
| `clone(_self)` | [No Documentation 🚧](./isometry3d/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./isometry3d/eq.md) |
| `from_rotation(rotation)` | [ Create a three\-dimensional isometry from a rotation\.](./isometry3d/from_rotation.md) |
| `from_xyz(x, y, z)` | [ Create a three\-dimensional isometry from a translation with the given \`x\`, \`y\`, and \`z\` components\.](./isometry3d/from_xyz.md) |
| `inverse(_self)` | [ The inverse isometry that undoes this one\.](./isometry3d/inverse.md) |
| `inverse_mul(_self, rhs)` | [ Compute \`iso1\.inverse\(\) \* iso2\` in a more efficient way for one\-shot cases\.  If the same isometry is used multiple times, it is more efficient to instead compute  the inverse once and use that for each transformation\.](./isometry3d/inverse_mul.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./isometry3d/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./isometry3d/mul-1.md) |
| `mul-2(arg0, arg1)` | [No Documentation 🚧](./isometry3d/mul-2.md) |
| `mul-3(arg0, arg1)` | [No Documentation 🚧](./isometry3d/mul-3.md) |