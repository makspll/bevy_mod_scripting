# transform\_point3

>  Transforms the given 3D vector as a point.
>  This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
>  `1.0`.
>  This method assumes that `self` contains a valid affine transform. It does not perform
>  a perspective divide, if `self` contains a perspective transform, or if you are unsure,
>  the [`Self::project_point3()`] method should be used instead.
>  # Panics
>  Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.

#### Arguments

- **\_self** : `Mat4` \- No Documentation 🚧
- **rhs** : `Vec3` \- No Documentation 🚧

#### Returns

- **arg0** : `Vec3` \- No Documentation 🚧