# transform\_vector3

>  Transforms the give 3D vector as a direction.
>  This is the equivalent of multiplying the 3D vector as a 4D vector where `w` is
>  `0.0`.
>  This method assumes that `self` contains a valid affine transform.
>  # Panics
>  Will panic if the 3rd row of `self` is not `(0, 0, 0, 1)` when `glam_assert` is enabled.

#### Arguments

- **\_self** : `Mat4` \- No Documentation 🚧
- **rhs** : `Vec3` \- No Documentation 🚧

#### Returns

- **arg0** : `Vec3` \- No Documentation 🚧