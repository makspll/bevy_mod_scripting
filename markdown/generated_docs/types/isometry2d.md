# Isometry2d

### Isometry2d

- **rotation** : bevy\_math::rotation2d::Rot2
- **translation** : glam::Vec2

## Description

>  An isometry in two dimensions, representing a rotation followed by a translation.
>  This can often be useful for expressing relative positions and transformations from one position to another.
> 
>  In particular, this type represents a distance-preserving transformation known as a *rigid motion* or a *direct motion*,
>  and belongs to the special [Euclidean group] SE(2). This includes translation and rotation, but excludes reflection.
> 
>  For the three-dimensional version, see [`Isometry3d`].
> 
>  [Euclidean group]: https://en.wikipedia.org/wiki/Euclidean_group
> 
>  # Example
> 
>  Isometries can be created from a given translation and rotation:
> 
>  ```
>  # use bevy_math::{Isometry2d, Rot2, Vec2};
>  #
>  let iso = Isometry2d::new(Vec2::new(2.0, 1.0), Rot2::degrees(90.0));
>  ```
> 
>  Or from separate parts:
> 
>  ```
>  # use bevy_math::{Isometry2d, Rot2, Vec2};
>  #
>  let iso1 = Isometry2d::from_translation(Vec2::new(2.0, 1.0));
>  let iso2 = Isometry2d::from_rotation(Rot2::degrees(90.0));
>  ```
> 
>  The isometries can be used to transform points:
> 
>  ```
>  # use approx::assert_abs_diff_eq;
>  # use bevy_math::{Isometry2d, Rot2, Vec2};
>  #
>  let iso = Isometry2d::new(Vec2::new(2.0, 1.0), Rot2::degrees(90.0));
>  let point = Vec2::new(4.0, 4.0);
> 
>  // These are equivalent
>  let result = iso.transform_point(point);
>  let result = iso * point;
> 
>  assert_eq!(result, Vec2::new(-2.0, 5.0));
>  ```
> 
>  Isometries can also be composed together:
> 
>  ```
>  # use bevy_math::{Isometry2d, Rot2, Vec2};
>  #
>  # let iso = Isometry2d::new(Vec2::new(2.0, 1.0), Rot2::degrees(90.0));
>  # let iso1 = Isometry2d::from_translation(Vec2::new(2.0, 1.0));
>  # let iso2 = Isometry2d::from_rotation(Rot2::degrees(90.0));
>  #
>  assert_eq!(iso1 * iso2, iso);
>  ```
> 
>  One common operation is to compute an isometry representing the relative positions of two objects
>  for things like intersection tests. This can be done with an inverse transformation:
> 
>  ```
>  # use bevy_math::{Isometry2d, Rot2, Vec2};
>  #
>  let circle_iso = Isometry2d::from_translation(Vec2::new(2.0, 1.0));
>  let rectangle_iso = Isometry2d::from_rotation(Rot2::degrees(90.0));
> 
>  // Compute the relative position and orientation between the two shapes
>  let relative_iso = circle_iso.inverse() * rectangle_iso;
> 
>  // Or alternatively, to skip an extra rotation operation:
>  let relative_iso = circle_iso.inverse_mul(rectangle_iso);
>  ```

## Functions

| Function | Summary |
| --- | --- |
| `clone(_self)` | [No Documentation 🚧](./isometry2d/clone.md) |
| `eq(_self, other)` | [No Documentation 🚧](./isometry2d/eq.md) |
| `from_rotation(rotation)` | [ Create a two\-dimensional isometry from a rotation\.](./isometry2d/from_rotation.md) |
| `from_translation(translation)` | [ Create a two\-dimensional isometry from a translation\.](./isometry2d/from_translation.md) |
| `from_xy(x, y)` | [ Create a two\-dimensional isometry from a translation with the given \`x\` and \`y\` components\.](./isometry2d/from_xy.md) |
| `inverse(_self)` | [ The inverse isometry that undoes this one\.](./isometry2d/inverse.md) |
| `inverse_mul(_self, rhs)` | [ Compute \`iso1\.inverse\(\) \* iso2\` in a more efficient way for one\-shot cases\.  If the same isometry is used multiple times, it is more efficient to instead compute  the inverse once and use that for each transformation\.](./isometry2d/inverse_mul.md) |
| `inverse_transform_point(_self, point)` | [ Transform a point by rotating and translating it using the inverse of this isometry\.  This is more ](./isometry2d/inverse_transform_point.md) |
| `mul(_self, rhs)` | [No Documentation 🚧](./isometry2d/mul.md) |
| `mul-1(arg0, arg1)` | [No Documentation 🚧](./isometry2d/mul-1.md) |
| `mul-2(arg0, arg1)` | [No Documentation 🚧](./isometry2d/mul-2.md) |
| `new(translation, rotation)` | [ Create a two\-dimensional isometry from a rotation and a translation\.](./isometry2d/new.md) |
| `transform_point(_self, point)` | [ Transform a point by rotating and translating it using this isometry\.](./isometry2d/transform_point.md) |